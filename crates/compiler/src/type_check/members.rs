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
        if new.type_arguments.is_empty() {
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
            .type_arguments
            .iter()
            .map(|name| {
                primitives.type_for_primitive_name(name).or_else(|| {
                    classes
                        .classes()
                        .iter()
                        .find(|class| class.name() == name)
                        .map(|class| class.type_id())
                })
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
