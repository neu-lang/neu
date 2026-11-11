//! Member typing for class bodies (ADR-0126): a bare name inside a method that
//! refers to a field of the method's class, or of a superclass, has the field's
//! type. A parameter or local of the same name shadows the field, so such names
//! are left to their own passes.

use super::*;

/// Records the field type of each bare field reference in a class method.
pub fn apply_bare_field_references(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    report: &mut TypeCheckReport,
) {
    for name in &parsed.name_references {
        let Some(method) = enclosing_method(parsed, name.name_span) else {
            continue;
        };
        let Some(class) = method.owner else {
            continue;
        };
        if method_declares_name(parsed, method, &name.name, name.name_span) {
            continue;
        }
        let Some(field) = classes.fields().iter().find(|field| {
            field.name() == name.name
                && class_or_superclass_owns_field(parsed, class, field.owner())
        }) else {
            continue;
        };
        if !private_field_accessible(parsed, field, name.reference) {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::PrivateFieldAccess,
                name.reference,
            ));
            continue;
        }
        report.replace_expression_type(ExpressionType::new(name.reference, field.type_id()));
    }
}

/// The method whose body contains the span, if any.
fn enclosing_method(
    parsed: &ParseOutput,
    span: ByteSpan,
) -> Option<&crate::parser::ParsedFunctionDeclaration> {
    parsed.function_declarations.iter().find(|method| {
        method.owner.is_some()
            && method
                .body
                .and_then(|body| parsed.arena.node(body))
                .is_some_and(|body| {
                    body.span.file() == span.file()
                        && body.span.start() <= span.start()
                        && span.end() <= body.span.end()
                })
    })
}

/// Whether a parameter or local named `name` is declared inside the method body.
fn method_declares_name(
    parsed: &ParseOutput,
    method: &crate::parser::ParsedFunctionDeclaration,
    name: &str,
    reference: ByteSpan,
) -> bool {
    let Some(body) = method.body.and_then(|body| parsed.arena.node(body)) else {
        return false;
    };
    let inside = |span: ByteSpan| {
        span.file() == reference.file()
            && body.span.start() <= span.start()
            && span.end() <= body.span.end()
    };
    parsed
        .function_parameters
        .iter()
        .any(|parameter| parameter.function == method.declaration && parameter.name == name)
        || parsed
            .local_binding_names
            .iter()
            .any(|binding| binding.name == name && inside(binding.name_span))
}

/// Types `new C<A, ...>(...)` as the generic instance `C<A, ...>` (ADR-0083).
pub(crate) fn apply_generic_new_expressions(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    types: &mut TypeArena,
    primitives: PrimitiveTypeIds,
    report: &mut TypeCheckReport,
) {
    for new in &parsed.new_expressions {
        if new.type_argument_references.is_empty() {
            continue;
        }
        let Some(class) = classes
            .classes()
            .iter()
            .find(|class| class.name() == new.type_name)
        else {
            continue;
        };
        let Some(identity) = (match types.get(class.type_id()).map(|record| record.kind()) {
            Some(TypeKind::Nominal(identity)) => Some(identity.clone()),
            _ => None,
        }) else {
            continue;
        };
        let arguments: Option<Vec<TypeId>> = new
            .type_argument_references
            .iter()
            .map(|argument| {
                super::generics::resolve_type_reference(
                    &parsed.type_name_references,
                    &parsed.array_types,
                    *argument,
                    &primitives,
                    classes.classes(),
                    &[],
                    types,
                )
            })
            .collect();
        let Some(arguments) = arguments else {
            continue;
        };
        let instance = types.generic_instance(GenericTypeIdentity::new(identity, arguments));
        report.replace_expression_type(ExpressionType::new(new.expression, instance));
    }
}

/// Types a method call on a generic instance by substituting the receiver's
/// type arguments for the class's type parameters in the method's signature.
pub fn apply_generic_instance_calls(
    parsed: &ParseOutput,
    signatures: &[FunctionSignature],
    types: &mut TypeArena,
    report: &mut TypeCheckReport,
) {
    for call in &parsed.call_expressions {
        let Some(member) = parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
        else {
            continue;
        };
        let Some(receiver_type) = report.expression_type(member.receiver) else {
            continue;
        };
        let Some(TypeKind::GenericInstance(identity)) =
            types.get(receiver_type).map(|record| record.kind().clone())
        else {
            continue;
        };
        let class = identity.declaration().declaration();
        let Some(method) = parsed
            .function_declarations
            .iter()
            .find(|method| method.owner == Some(class) && method.name == member.name)
        else {
            continue;
        };
        let Some(signature) = signatures
            .iter()
            .find(|signature| signature.declaration == method.declaration)
        else {
            continue;
        };
        let mut substitution = GenericSubstitution::new();
        for (parameter, argument) in signature
            .generic_parameters()
            .iter()
            .copied()
            .zip(identity.arguments().iter().copied())
        {
            substitution.insert(parameter, argument);
        }
        let specialized = signature.specialize(&substitution, types);
        // An instance method's signature lists its receiver first. The call's
        // arguments line up with the declared parameters after it.
        let receiver_slots = usize::from(!method.is_static);
        for (argument, parameter) in call
            .arguments
            .iter()
            .zip(specialized.parameter_types().iter().skip(receiver_slots))
        {
            if report.expression_type(*argument) != Some(*parameter) {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::GenericArgumentMismatch,
                    *argument,
                ));
            }
        }
        report.replace_expression_type(ExpressionType::new(
            call.expression,
            specialized.return_type(),
        ));
        report.retain_diagnostics(|diagnostic| {
            (diagnostic.node() != call.expression && diagnostic.node() != member.expression)
                || !matches!(
                    diagnostic.rule(),
                    TypeRuleDiagnostic::DirectCallDeferred
                        | TypeRuleDiagnostic::MemberExpressionDeferred
                )
        });
    }
}

/// The call-to-method targets of member calls on generic instances, for HIR
/// lowering. The direct-call checker does not resolve these calls (ADR-0126).
pub(crate) fn generic_instance_call_targets(
    parsed: &ParseOutput,
    types: &TypeArena,
    report: &TypeCheckReport,
) -> Vec<crate::type_check::ResolvedCallDeclaration> {
    let mut targets = Vec::new();
    for call in &parsed.call_expressions {
        let Some(member) = parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
        else {
            continue;
        };
        let Some(receiver_type) = report.expression_type(member.receiver) else {
            continue;
        };
        let Some(TypeKind::GenericInstance(identity)) =
            types.get(receiver_type).map(|record| record.kind())
        else {
            continue;
        };
        let class = identity.declaration().declaration();
        let Some(method) = parsed
            .function_declarations
            .iter()
            .find(|method| method.owner == Some(class) && method.name == member.name)
        else {
            continue;
        };
        targets.push(crate::type_check::ResolvedCallDeclaration {
            call: call.expression,
            declaration: method.declaration,
        });
    }
    targets
}

/// Types a field read or assignment target on a generic instance by substituting
/// the receiver's type arguments for the class's type parameters in the field's
/// type (ADR-0126). A `val` field is not assignable. Method calls are typed by
/// their own pass.
pub(crate) fn apply_generic_instance_fields(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    types: &mut TypeArena,
    report: &mut TypeCheckReport,
) {
    for member in &parsed.member_expressions {
        if parsed
            .call_expressions
            .iter()
            .any(|call| call.callee == member.expression)
        {
            continue;
        }
        let Some(receiver_type) = report.expression_type(member.receiver) else {
            continue;
        };
        let Some(TypeKind::GenericInstance(identity)) =
            types.get(receiver_type).map(|record| record.kind().clone())
        else {
            continue;
        };
        let class = identity.declaration().declaration();
        let Some(field) = classes.fields().iter().find(|field| {
            field.name() == member.name
                && class_or_superclass_owns_field(parsed, class, field.owner())
        }) else {
            continue;
        };
        if !private_field_accessible(parsed, field, member.expression) {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::PrivateFieldAccess,
                member.expression,
            ));
        }
        let field_type = field.type_id();
        let mut symbols = SymbolInterner::new();
        let parameter_types =
            build_generic_parameter_types(&parsed.generic_parameters, &mut symbols, types);
        let mut substitution = GenericSubstitution::new();
        for (parameter, argument) in parsed
            .generic_parameters
            .iter()
            .filter(|parameter| parameter.owner == Some(class))
            .filter_map(|parameter| {
                parameter_types
                    .iter()
                    .find(|record| record.parameter() == parameter.parameter)
            })
            .zip(identity.arguments().iter().copied())
        {
            substitution.insert(parameter.ty(), argument);
        }
        let specialized = substitution.apply(field_type, types);
        report.replace_expression_type(ExpressionType::new(member.expression, specialized));
        if parsed
            .assignment_statements
            .iter()
            .any(|assignment| assignment.target == member.expression)
            && !field.mutable()
        {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ImmutableFieldMutation,
                member.expression,
            ));
        }
    }
}

/// Types a field read through a subclass whose superclass is generic: the field's
/// type has the superclass's type parameters replaced by the type arguments the
/// subclass gives (ADR-0083).
pub(crate) fn apply_inherited_field_types(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    types: &mut TypeArena,
    report: &mut TypeCheckReport,
) {
    for member in &parsed.member_expressions {
        let Some(receiver_type) = report.expression_type(member.receiver) else {
            continue;
        };
        let Some(class) = classes
            .classes()
            .iter()
            .find(|class| class.type_id == receiver_type)
        else {
            continue;
        };
        let Some(field) = classes.fields().iter().find(|field| {
            field.name() == member.name
                && class_or_superclass_owns_field(parsed, class.declaration, field.owner())
        }) else {
            continue;
        };
        if field.owner() == class.declaration {
            continue;
        }
        let Some(mapping) = inherited_parameter_types(classes, class.declaration, field.owner())
        else {
            continue;
        };
        if mapping.is_empty() {
            continue;
        }
        let mut substitution = GenericSubstitution::new();
        for (parameter, argument) in mapping {
            substitution.insert(parameter, argument);
        }
        let specialized = substitution.apply(field.type_id(), types);
        report.replace_expression_type(ExpressionType::new(member.expression, specialized));
    }
}

/// The type arguments a receiver gives its class's type parameters, as pairs of
/// the parameter and the argument.
type ReceiverSubstitution = Vec<(TypeId, TypeId)>;

/// The class that a receiver's type denotes, with the substitution for that
/// class's own type parameters (the receiver's type arguments). A nominal class
/// has no substitution.
fn receiver_class(
    classes: &ClassTypeReport,
    types: &TypeArena,
    receiver_type: TypeId,
) -> Option<(AstNodeId, ReceiverSubstitution)> {
    if let Some(TypeKind::GenericInstance(identity)) =
        types.get(receiver_type).map(|record| record.kind())
    {
        let declaration = identity.declaration().declaration();
        let record = classes
            .classes()
            .iter()
            .find(|class| class.declaration == declaration && !class.interface)?;
        let own = record
            .generic_parameters
            .iter()
            .copied()
            .zip(identity.arguments().iter().copied())
            .collect();
        return Some((declaration, own));
    }
    let class = classes
        .classes()
        .iter()
        .find(|class| class.type_id == receiver_type && !class.interface)?;
    Some((class.declaration, Vec::new()))
}

/// The inherited method that `call` invokes on a class receiver, with the class it
/// is declared in and the receiver's own type arguments.
fn inherited_call(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    types: &TypeArena,
    receiver_type: Option<TypeId>,
    member: &crate::parser::ParsedMemberExpression,
) -> Option<(AstNodeId, AstNodeId, ReceiverSubstitution)> {
    let (class, own) = receiver_class(classes, types, receiver_type?)?;
    let owner = inherited_method_owner(parsed, classes, class, &member.name)?;
    let method = parsed.function_declarations.iter().find(|method| {
        method.owner == Some(owner) && method.name == member.name && !method.is_static
    })?;
    Some((class, method.declaration, own))
}

/// Types a call to an inherited method by substituting the superclass's type
/// arguments into the method's signature (ADR-0083).
pub(crate) fn apply_inherited_calls(
    parsed: &ParseOutput,
    signatures: &[FunctionSignature],
    classes: &ClassTypeReport,
    types: &mut TypeArena,
    report: &mut TypeCheckReport,
) {
    for call in &parsed.call_expressions {
        let Some(member) = parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
        else {
            continue;
        };
        let receiver_type = report.expression_type(member.receiver);
        let Some((class, declaration, own)) =
            inherited_call(parsed, classes, types, receiver_type, member)
        else {
            continue;
        };
        let Some(signature) = signatures
            .iter()
            .find(|signature| signature.declaration == declaration)
        else {
            continue;
        };
        let owner = parsed
            .function_declarations
            .iter()
            .find(|method| method.declaration == declaration)
            .and_then(|method| method.owner);
        let Some(mapping) =
            owner.and_then(|owner| inherited_parameter_types(classes, class, owner))
        else {
            continue;
        };
        let mut substitution = GenericSubstitution::new();
        for (parameter, argument) in mapping {
            let argument = own
                .iter()
                .find(|(own_parameter, _)| *own_parameter == argument)
                .map_or(argument, |(_, ty)| *ty);
            substitution.insert(parameter, argument);
        }
        let specialized = signature.specialize(&substitution, types);
        // An instance method's signature lists its receiver first.
        for (argument, parameter) in call
            .arguments
            .iter()
            .zip(specialized.parameter_types().iter().skip(1))
        {
            if report.expression_type(*argument) != Some(*parameter) {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::GenericArgumentMismatch,
                    *argument,
                ));
            }
        }
        report.replace_expression_type(ExpressionType::new(
            call.expression,
            specialized.return_type(),
        ));
        report.retain_diagnostics(|diagnostic| {
            (diagnostic.node() != call.expression && diagnostic.node() != member.expression)
                || !matches!(
                    diagnostic.rule(),
                    TypeRuleDiagnostic::DirectCallDeferred
                        | TypeRuleDiagnostic::MemberExpressionDeferred
                )
        });
    }
}

/// The call targets of calls to inherited methods, for HIR lowering.
pub(crate) fn inherited_call_targets(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    types: &TypeArena,
    report: &TypeCheckReport,
) -> Vec<crate::type_check::ResolvedCallDeclaration> {
    let mut targets = Vec::new();
    for call in &parsed.call_expressions {
        let Some(member) = parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
        else {
            continue;
        };
        if let Some((_, declaration, _)) = inherited_call(
            parsed,
            classes,
            types,
            report.expression_type(member.receiver),
            member,
        ) {
            targets.push(crate::type_check::ResolvedCallDeclaration {
                call: call.expression,
                declaration,
            });
        }
    }
    targets
}

/// Checks each argument to a superclass constructor against the superclass's
/// constructor parameter, with the superclass's type parameters replaced by the
/// type arguments the subclass gives (ADR-0083).
pub(crate) fn check_superclass_arguments(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    types: &mut TypeArena,
    report: &mut TypeCheckReport,
) {
    for child in &parsed.class_declarations {
        let Some(parent_name) = child.superclass.as_deref() else {
            continue;
        };
        let Some(parent) = parsed
            .class_declarations
            .iter()
            .find(|candidate| candidate.name == parent_name)
        else {
            continue;
        };
        let mapping = inherited_parameter_types(classes, child.declaration, parent.declaration)
            .unwrap_or_default();
        let mut substitution = GenericSubstitution::new();
        for (parameter, argument) in mapping {
            substitution.insert(parameter, argument);
        }
        for (index, argument) in child.superclass_arguments.iter().enumerate() {
            let Some(parameter) = parent.constructor_parameters.get(index) else {
                continue;
            };
            let Some(field) = classes.fields().iter().find(|field| {
                field.owner() == parent.declaration && field.name() == parameter.name
            }) else {
                continue;
            };
            // A subclass constructor parameter forwarded to the superclass is a
            // name reference to its promoted field, which has the parameter's type.
            if report.expression_type(*argument).is_none()
                && let Some(name) = parsed
                    .name_references
                    .iter()
                    .find(|name| name.reference == *argument)
                && let Some(own) = classes
                    .fields()
                    .iter()
                    .find(|field| field.owner() == child.declaration && field.name() == name.name)
            {
                report.replace_expression_type(ExpressionType::new(*argument, own.type_id()));
            }
            let Some(actual) = report.expression_type(*argument) else {
                continue;
            };
            let expected = substitution.apply(field.type_id(), types);
            if actual != expected
                && !crate::type_check::calls::class_value_assignable(classes, actual, expected)
            {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    *argument, expected, actual,
                ));
            }
        }
    }
}
