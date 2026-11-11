//! Direct calls, overload resolution, and indirect calls.

use super::*;

pub struct ExecutableSourceTypes<'a> {
    pub(crate) package: &'a PackageNamespace,
    pub(crate) parsed: &'a ParseOutput,
    pub(crate) signatures: &'a [FunctionSignature],
    pub(crate) expression_types: &'a [ExpressionType],
    pub(crate) class_types: Option<&'a ClassTypeReport>,
    pub(crate) type_arena: Option<&'a TypeArena>,
}

impl<'a> ExecutableSourceTypes<'a> {
    pub fn new(
        package: &'a PackageNamespace,
        parsed: &'a ParseOutput,
        signatures: &'a [FunctionSignature],
        expression_types: &'a [ExpressionType],
    ) -> Self {
        Self {
            package,
            parsed,
            signatures,
            expression_types,
            class_types: None,
            type_arena: None,
        }
    }

    pub fn with_class_types(mut self, class_types: &'a ClassTypeReport) -> Self {
        self.class_types = Some(class_types);
        self
    }

    pub fn with_type_arena(mut self, type_arena: &'a TypeArena) -> Self {
        self.type_arena = Some(type_arena);
        self
    }

    pub fn package(&self) -> &PackageNamespace {
        self.package
    }

    pub fn parsed(&self) -> &ParseOutput {
        self.parsed
    }

    pub fn signatures(&self) -> &[FunctionSignature] {
        self.signatures
    }

    pub fn expression_types(&self) -> &[ExpressionType] {
        self.expression_types
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectCallDiagnosticKind {
    InvalidCallTarget,
    NoMatchingOverload,
    AmbiguousOverload,
    DuplicateOverload,
    ArgumentCountMismatch,
    ArgumentTypeMismatch,
    RecursiveCallUnsupported,
    /// A call whose leading `precondition` fails for constant arguments (ADR-0124).
    PreconditionViolation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectCallDiagnostic {
    pub(crate) kind: DirectCallDiagnosticKind,
    pub(crate) span: ByteSpan,
}

impl DirectCallDiagnostic {
    pub fn new(kind: DirectCallDiagnosticKind, span: ByteSpan) -> Self {
        Self { kind, span }
    }

    pub fn kind(self) -> DirectCallDiagnosticKind {
        self.kind
    }

    pub fn span(self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectCallExpressionType {
    pub(crate) expression_span: ByteSpan,
    pub(crate) ty: TypeId,
}

impl DirectCallExpressionType {
    pub fn expression_span(self) -> ByteSpan {
        self.expression_span
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DirectCallReport {
    pub(crate) expression_types: Vec<ExpressionType>,
    pub(crate) source_expression_types: Vec<DirectCallExpressionType>,
    pub(crate) resolved_declarations: Vec<ResolvedCallDeclaration>,
    pub(crate) diagnostics: Vec<DirectCallDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedCallDeclaration {
    pub(crate) call: AstNodeId,
    pub(crate) declaration: AstNodeId,
}

impl ResolvedCallDeclaration {
    pub fn call(self) -> AstNodeId {
        self.call
    }

    pub fn declaration(self) -> AstNodeId {
        self.declaration
    }
}

impl DirectCallReport {
    pub fn expression_types(&self) -> &[ExpressionType] {
        &self.expression_types
    }

    pub fn source_expression_types(&self) -> &[DirectCallExpressionType] {
        &self.source_expression_types
    }

    pub fn resolved_declarations(&self) -> &[ResolvedCallDeclaration] {
        &self.resolved_declarations
    }

    pub fn diagnostics(&self) -> &[DirectCallDiagnostic] {
        &self.diagnostics
    }
}

pub fn check_direct_calls(sources: &[ExecutableSourceTypes<'_>]) -> DirectCallReport {
    let mut report = DirectCallReport::default();
    let mut resolved_calls = Vec::new();
    let mut call_edges: Vec<(ByteSpan, ByteSpan)> = Vec::new();

    for (source_index, source) in sources.iter().enumerate() {
        for (index, function) in source.parsed.function_declarations.iter().enumerate() {
            if source.parsed.function_declarations[index + 1..]
                .iter()
                .any(|other| {
                    other.owner == function.owner
                        && other.name == function.name
                        && same_function_parameter_types(source.parsed, function, other)
                })
                && let Some(node) = source.parsed.arena.node(function.declaration)
            {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::DuplicateOverload,
                    node.span,
                ));
            }
        }
        for (call_index, call) in source.parsed.call_expressions.iter().enumerate() {
            if is_function_typed_local_or_parameter(source, call) {
                continue;
            }
            let callee_name = source
                .parsed
                .name_references
                .iter()
                .find(|reference| reference.reference == call.callee)
                .map(|reference| (reference.name.clone(), reference.name_span));
            let member = source
                .parsed
                .member_expressions
                .iter()
                .find(|member| member.expression == call.callee);
            if member.is_some_and(|member| is_enum_variant_call(source, member, call)) {
                continue;
            }
            if member.is_some_and(|member| is_enum_static_call(source, member, call)) {
                continue;
            }
            let Some((target_name, diagnostic_span)) =
                callee_name.or_else(|| member.map(|member| (member.name.clone(), member.span)))
            else {
                let span = source
                    .parsed
                    .arena
                    .node(call.callee)
                    .expect("parsed call callee")
                    .span;
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::InvalidCallTarget,
                    span,
                ));
                continue;
            };
            if member.is_none()
                && matches!(
                    target_name.as_str(),
                    "clone"
                        | "spawn"
                        | "await"
                        | "cancel"
                        | "channel"
                        | "send"
                        | "receive"
                        | "close"
                        | "assert"
                        | "fail"
                        | "precondition"
                )
            {
                continue;
            }
            if member.is_some_and(|member| member.name == "cancel")
                && !sources.iter().any(|candidate| {
                    candidate
                        .parsed
                        .function_declarations
                        .iter()
                        .any(|function| function.owner.is_some() && function.name == "cancel")
                })
            {
                continue;
            }
            if member
                .is_some_and(|member| matches!(member.name.as_str(), "size" | "add" | "remove"))
            {
                continue;
            }
            // A member call on a generic instance is typed and targeted by
            // members.rs, with the receiver's type arguments substituted (ADR-0126).
            if member.is_some_and(|member| receiver_is_generic_instance(source, member.receiver)) {
                continue;
            }
            if member.is_some_and(|member| {
                source
                    .parsed
                    .index_expressions
                    .iter()
                    .any(|index| index.expression == member.receiver)
            }) {
                continue;
            }
            let targets: Vec<_> = sources
                .iter()
                .enumerate()
                .filter(|(_, candidate)| candidate.package == source.package)
                .flat_map(|(candidate_index, candidate)| {
                    candidate
                        .parsed
                        .function_declarations
                        .iter()
                        .filter(|function| {
                            function.name == target_name
                                && (member.is_none() || function.owner.is_some())
                                && member.is_none_or(|member| {
                                    let class_name_receiver =
                                        is_class_name_receiver(source, member.receiver);
                                    member_receiver_class_name(
                                        source,
                                        member.receiver,
                                        call.function,
                                    )
                                    .and_then(|class_name| {
                                        function.owner.and_then(|owner| {
                                            if class_name_receiver {
                                                Some(class_static_owner_matches(
                                                    candidate.parsed,
                                                    owner,
                                                    &class_name,
                                                ))
                                            } else {
                                                candidate
                                                    .parsed
                                                    .class_declarations
                                                    .iter()
                                                    .find(|class| class.declaration == owner)
                                                    .map(|class| class.name == class_name)
                                            }
                                        })
                                    })
                                    .unwrap_or(true)
                                        && function.is_static == class_name_receiver
                                })
                        })
                        .filter_map(move |function| {
                            candidate
                                .signatures
                                .iter()
                                .position(|signature| signature.declaration == function.declaration)
                                .map(|signature_index| (candidate_index, signature_index))
                        })
                })
                .collect();
            let mut arguments = call.arguments.clone();
            if let Some(member) = member
                && !is_class_name_receiver(source, member.receiver)
            {
                arguments.insert(0, member.receiver);
            }
            let Some(argument_types) = arguments
                .iter()
                .enumerate()
                .map(|(index, argument)| {
                    if index == 0
                        && let Some(member) = member
                        && !is_class_name_receiver(source, member.receiver)
                        && let Some(class_name) =
                            member_receiver_class_name(source, member.receiver, call.function)
                        && let Some(class_type) = source.class_types.and_then(|classes| {
                            classes
                                .classes
                                .iter()
                                .find(|class| class.name == class_name)
                        })
                    {
                        return Some(class_type.type_id);
                    }
                    source
                        .expression_types
                        .iter()
                        .find(|typed| typed.expression == *argument)
                        .map(|typed| typed.ty)
                })
                .collect::<Option<Vec<_>>>()
            else {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::InvalidCallTarget,
                    diagnostic_span,
                ));
                continue;
            };
            let target_count = targets.len();
            let arity_matches = targets
                .into_iter()
                .filter(|(candidate_index, signature_index)| {
                    sources[*candidate_index].signatures[*signature_index]
                        .parameter_types()
                        .len()
                        == argument_types.len()
                })
                .collect::<Vec<_>>();
            if arity_matches.is_empty() {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    if target_count == 0 {
                        DirectCallDiagnosticKind::InvalidCallTarget
                    } else if target_count == 1 {
                        DirectCallDiagnosticKind::ArgumentCountMismatch
                    } else {
                        DirectCallDiagnosticKind::NoMatchingOverload
                    },
                    diagnostic_span,
                ));
                continue;
            }
            let exact_matches = arity_matches
                .iter()
                .copied()
                .filter(|(candidate_index, signature_index)| {
                    sources[*candidate_index].signatures[*signature_index]
                        .parameter_types()
                        .iter()
                        .zip(&argument_types)
                        .all(|(expected, actual)| expected == actual)
                })
                .collect::<Vec<_>>();
            let compatible_matches = if exact_matches.is_empty() {
                arity_matches
                    .iter()
                    .copied()
                    .filter(|(candidate_index, signature_index)| {
                        sources[*candidate_index].signatures[*signature_index]
                            .parameter_types()
                            .iter()
                            .zip(&argument_types)
                            .all(|(expected, actual)| {
                                overload_type_compatible(source, *actual, *expected)
                            })
                    })
                    .collect::<Vec<_>>()
            } else {
                exact_matches
            };
            if compatible_matches.is_empty() {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    if target_count == 1 {
                        DirectCallDiagnosticKind::ArgumentTypeMismatch
                    } else {
                        DirectCallDiagnosticKind::NoMatchingOverload
                    },
                    diagnostic_span,
                ));
                continue;
            }
            if compatible_matches.len() > 1 {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::AmbiguousOverload,
                    diagnostic_span,
                ));
                continue;
            }
            let (target_source_index, target_signature_index) = compatible_matches[0];
            let target_source = &sources[target_source_index];
            let target = &target_source.signatures[target_signature_index];
            let has_body = target_source
                .parsed
                .function_declarations
                .iter()
                .any(|function| {
                    function.declaration == target.declaration && function.body.is_some()
                });
            let interface_contract = member.is_some_and(|member| {
                member_receiver_class_name(source, member.receiver, call.function)
                    .and_then(|name| {
                        source
                            .class_types?
                            .classes
                            .iter()
                            .find(|class| class.name == name)
                    })
                    .is_some_and(|class| class.interface)
            });
            if !has_body && !interface_contract {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::InvalidCallTarget,
                    diagnostic_span,
                ));
                continue;
            }
            let source_function_span = source
                .parsed
                .arena
                .node(call.function)
                .expect("parsed call owner function")
                .span;
            let target_span = target_source
                .parsed
                .arena
                .node(target.declaration)
                .expect("parsed call target function")
                .span;
            call_edges.push((source_function_span, target_span));
            resolved_calls.push((
                source_index,
                call_index,
                source_function_span,
                target_span,
                target_source_index,
                target_signature_index,
            ));
        }
    }

    for (
        source_index,
        call_index,
        source_function_span,
        target_span,
        target_source_index,
        target_signature_index,
    ) in resolved_calls
    {
        let source = &sources[source_index];
        let call = &source.parsed.call_expressions[call_index];
        let target = &sources[target_source_index].signatures[target_signature_index];
        if call_path_exists(&call_edges, target_span, source_function_span) {
            let span = source
                .parsed
                .arena
                .node(call.expression)
                .expect("parsed call expression")
                .span;
            report.diagnostics.push(DirectCallDiagnostic::new(
                DirectCallDiagnosticKind::RecursiveCallUnsupported,
                span,
            ));
            continue;
        }
        let mut arguments = call.arguments.clone();
        if let Some(member) = source
            .parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
            && !is_class_name_receiver(source, member.receiver)
        {
            arguments.insert(0, member.receiver);
        }
        if arguments.len() != target.parameter_types.len() {
            let span = source
                .parsed
                .arena
                .node(call.expression)
                .expect("parsed call expression")
                .span;
            report.diagnostics.push(DirectCallDiagnostic::new(
                DirectCallDiagnosticKind::ArgumentCountMismatch,
                span,
            ));
            continue;
        }
        let mut argument_error = false;
        for (index, (argument, parameter)) in
            arguments.iter().zip(&target.parameter_types).enumerate()
        {
            if index == 0
                && source
                    .parsed
                    .member_expressions
                    .iter()
                    .any(|member| member.expression == call.callee)
            {
                continue;
            }
            let actual_type = source
                .expression_types
                .iter()
                .find(|typed| typed.expression == *argument)
                .map(|typed| typed.ty);
            if actual_type
                .is_none_or(|actual| !overload_type_compatible(source, actual, *parameter))
            {
                let span = source
                    .parsed
                    .arena
                    .node(*argument)
                    .expect("parsed argument")
                    .span;
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::ArgumentTypeMismatch,
                    span,
                ));
                argument_error = true;
            }
        }
        if !argument_error {
            let call_span = source
                .parsed
                .arena
                .node(call.expression)
                .expect("parsed call expression")
                .span;
            // A plain call's arguments line up with the callee's parameters. A member
            // call inserts its receiver, which is not checked here.
            let is_plain_call = !source
                .parsed
                .member_expressions
                .iter()
                .any(|member| member.expression == call.callee);
            if is_plain_call
                && crate::precondition::leading_precondition_fails(
                    sources[target_source_index].parsed,
                    target.declaration,
                    source.parsed,
                    &call.arguments,
                ) == Some(true)
            {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::PreconditionViolation,
                    call_span,
                ));
                continue;
            }
            report
                .expression_types
                .push(ExpressionType::new(call.expression, target.return_type));
            report
                .source_expression_types
                .push(DirectCallExpressionType {
                    expression_span: call_span,
                    ty: target.return_type,
                });
            report.resolved_declarations.push(ResolvedCallDeclaration {
                call: call.expression,
                declaration: target.declaration,
            });
        }
    }
    report
}

pub(crate) fn is_function_typed_local_or_parameter(
    source: &ExecutableSourceTypes<'_>,
    call: &crate::parser::ParsedCallExpression,
) -> bool {
    let Some(name) = source
        .parsed
        .name_references
        .iter()
        .find(|reference| reference.reference == call.callee)
        .map(|reference| reference.name.as_str())
    else {
        return false;
    };
    source.parsed.function_parameters.iter().any(|parameter| {
        parameter.function == call.function
            && parameter.name == name
            && source
                .parsed
                .arena
                .node(parameter.annotation)
                .is_some_and(|node| node.kind == AstNodeKind::FunctionType)
    }) || source.parsed.local_binding_names.iter().any(|binding| {
        binding.name == name
            && source
                .parsed
                .local_declarations
                .iter()
                .find(|declaration| declaration.declaration == binding.binding)
                .and_then(|declaration| declaration.annotation)
                .and_then(|annotation| source.parsed.arena.node(annotation))
                .is_some_and(|node| node.kind == AstNodeKind::FunctionType)
            || initialized_by_lambda(source.parsed, binding.binding)
    })
}

/// Whether a local binding's initializer is a lambda expression. Such a local
/// is function-typed even without an annotation (ADR-0125).
pub(crate) fn initialized_by_lambda(parsed: &ParseOutput, binding: AstNodeId) -> bool {
    parsed
        .local_declarations
        .iter()
        .find(|declaration| declaration.declaration == binding)
        .and_then(|declaration| declaration.initializer)
        .is_some_and(|initializer| {
            parsed
                .lambda_expressions
                .iter()
                .any(|lambda| lambda.expression == initializer)
        })
}

pub(crate) fn is_enum_variant_call(
    source: &ExecutableSourceTypes<'_>,
    member: &crate::parser::ParsedMemberExpression,
    call: &crate::parser::ParsedCallExpression,
) -> bool {
    let Some(receiver_name) = source
        .parsed
        .name_references
        .iter()
        .find(|reference| reference.reference == member.receiver)
        .map(|reference| reference.name.as_str())
    else {
        return false;
    };
    let Some(classes) = source.class_types else {
        return false;
    };
    let Some(class) = classes.classes().iter().find(|class| {
        class.name() == receiver_name
            && source.parsed.enum_variants.iter().any(|variant| {
                variant.enum_declaration == class.declaration() && variant.name == member.name
            })
    }) else {
        return false;
    };
    source
        .parsed
        .enum_variants
        .iter()
        .find(|variant| {
            variant.enum_declaration == class.declaration() && variant.name == member.name
        })
        .is_some_and(|variant| variant.arguments.len() == call.arguments.len())
}

pub(crate) fn is_enum_static_call(
    source: &ExecutableSourceTypes<'_>,
    member: &crate::parser::ParsedMemberExpression,
    call: &crate::parser::ParsedCallExpression,
) -> bool {
    let Some(receiver_name) = source
        .parsed
        .name_references
        .iter()
        .find(|reference| reference.reference == member.receiver)
        .map(|reference| reference.name.as_str())
    else {
        return false;
    };
    let Some(classes) = source.class_types else {
        return false;
    };
    let Some(class) = classes.classes().iter().find(|class| {
        class.name() == receiver_name
            && source
                .parsed
                .enum_variants
                .iter()
                .any(|variant| variant.enum_declaration == class.declaration())
    }) else {
        return false;
    };
    source.parsed.function_declarations.iter().any(|function| {
        function.owner == Some(class.declaration())
            && function.name == member.name
            && function.is_static
            && function.parameters.len() == call.arguments.len()
    })
}

pub(crate) fn class_static_owner_matches(
    parsed: &ParseOutput,
    owner: AstNodeId,
    receiver_name: &str,
) -> bool {
    let Some(owner_name) = parsed
        .class_declarations
        .iter()
        .find(|class| class.declaration == owner)
        .map(|class| class.name.as_str())
    else {
        return false;
    };
    let mut current = Some(receiver_name.to_owned());
    while let Some(name) = current {
        if name == owner_name {
            return true;
        }
        current = parsed
            .class_declarations
            .iter()
            .find(|class| class.name == name)
            .and_then(|class| class.superclass.clone());
    }
    false
}

pub(crate) fn is_class_name_receiver(
    source: &ExecutableSourceTypes<'_>,
    receiver: AstNodeId,
) -> bool {
    let Some(name) = source
        .parsed
        .name_references
        .iter()
        .find(|reference| reference.reference == receiver)
    else {
        return false;
    };
    !source
        .parsed
        .local_binding_names
        .iter()
        .any(|binding| binding.name == name.name)
        && source
            .parsed
            .class_declarations
            .iter()
            .any(|class| class.name == name.name)
}

pub(crate) fn member_receiver_class_name(
    source: &ExecutableSourceTypes<'_>,
    receiver: AstNodeId,
    function_declaration: AstNodeId,
) -> Option<String> {
    let receiver_name = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == receiver)
        .map(|name| name.name.as_str());
    if matches!(receiver_name, Some("this" | "super")) {
        let owner = source
            .parsed
            .function_declarations
            .iter()
            .find(|function| function.declaration == function_declaration)?
            .owner?;
        let class = source
            .parsed
            .class_declarations
            .iter()
            .find(|class| class.declaration == owner)?;
        return if receiver_name == Some("super") {
            class.superclass.clone()
        } else {
            Some(class.name.clone())
        };
    }
    if let Some(expression) = source
        .parsed
        .new_expressions
        .iter()
        .find(|expression| expression.expression == receiver)
    {
        return Some(expression.type_name.clone());
    }
    let name = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == receiver)?;
    if let Some(parameter) =
        source.parsed.function_parameters.iter().find(|parameter| {
            parameter.function == function_declaration && parameter.name == name.name
        })
    {
        return source
            .parsed
            .type_name_references
            .iter()
            .find(|reference| reference.reference == parameter.annotation)
            .map(|reference| reference.name.clone());
    }
    let Some(binding) = source
        .parsed
        .local_binding_names
        .iter()
        .find(|binding| binding.name == name.name)
    else {
        return source
            .expression_types
            .iter()
            .find(|typed| typed.expression == receiver)
            .and_then(|typed| {
                source
                    .class_types?
                    .classes
                    .iter()
                    .find(|class| class.type_id == typed.ty)
            })
            .map(|class| class.name.clone());
    };
    let declaration = source
        .parsed
        .local_declarations
        .iter()
        .find(|declaration| declaration.declaration == binding.binding)?;
    let local_name = declaration.initializer.and_then(|initializer| {
        source
            .parsed
            .new_expressions
            .iter()
            .find(|expression| expression.expression == initializer)
            .map(|expression| expression.type_name.clone())
    });
    if local_name.is_some() {
        return local_name;
    }
    source
        .expression_types
        .iter()
        .find(|typed| typed.expression == receiver)
        .and_then(|typed| {
            source
                .class_types?
                .classes
                .iter()
                .find(|class| class.type_id == typed.ty)
        })
        .map(|class| class.name.clone())
}

pub(crate) fn overload_type_compatible(
    source: &ExecutableSourceTypes<'_>,
    actual: TypeId,
    expected: TypeId,
) -> bool {
    if actual == expected {
        return true;
    }
    if let Some(arena) = source.type_arena {
        match (
            arena.get(actual).map(|record| record.kind()),
            arena.get(expected).map(|record| record.kind()),
        ) {
            (Some(TypeKind::DynamicArray(actual)), Some(TypeKind::DynamicArray(expected))) => {
                return actual.element() == expected.element()
                    || matches!(
                        arena.get(expected.element()).map(|record| record.kind()),
                        Some(TypeKind::GenericParameter(_))
                    );
            }
            (_, Some(TypeKind::GenericParameter(_))) => return true,
            (
                Some(TypeKind::GenericInstance(actual)),
                Some(TypeKind::GenericInstance(expected)),
            ) => {
                return actual.declaration() == expected.declaration()
                    && actual.arguments().len() == expected.arguments().len()
                    && actual.arguments().iter().zip(expected.arguments()).all(
                        |(actual, expected)| {
                            *actual == *expected
                                || matches!(
                                    arena.get(*expected).map(|record| record.kind()),
                                    Some(TypeKind::GenericParameter(_))
                                )
                        },
                    );
            }
            (Some(TypeKind::Nominal(actual)), Some(TypeKind::GenericInstance(expected))) => {
                return actual == expected.declaration();
            }
            (Some(TypeKind::GenericInstance(actual)), Some(TypeKind::Nominal(expected))) => {
                return actual.declaration() == expected;
            }
            _ => {}
        }
    }
    let Some(classes) = source.class_types else {
        return false;
    };
    let Some(actual) = classes.classes.iter().find(|class| class.type_id == actual) else {
        return false;
    };
    let Some(expected) = classes
        .classes
        .iter()
        .find(|class| class.type_id == expected)
    else {
        return false;
    };
    if expected.interface {
        return actual.interfaces.iter().any(|name| name == &expected.name)
            || actual.superclass.as_deref().is_some_and(|parent| {
                classes
                    .classes
                    .iter()
                    .find(|class| class.name == parent)
                    .is_some_and(|parent| {
                        overload_type_compatible(source, parent.type_id, expected.type_id)
                    })
            });
    }
    actual.superclass.as_deref().is_some_and(|parent| {
        classes
            .classes
            .iter()
            .find(|class| class.name == parent)
            .is_some_and(|parent| {
                overload_type_compatible(source, parent.type_id, expected.type_id)
            })
    })
}

pub fn apply_direct_call_results(
    report: &mut TypeCheckReport,
    parsed: &ParseOutput,
    direct_calls: &DirectCallReport,
) {
    for call in &parsed.call_expressions {
        let span = parsed
            .arena
            .node(call.expression)
            .expect("parsed call expression")
            .span;
        let Some(ty) = direct_calls
            .source_expression_types
            .iter()
            .find(|typed| typed.expression_span == span)
            .map(|typed| typed.ty)
        else {
            continue;
        };
        // Direct calls are applied once per typing round, so a call's type replaces
        // any earlier one rather than adding a duplicate.
        report.replace_expression_type(ExpressionType::new(call.expression, ty));
        report.diagnostics.retain(|diagnostic| {
            !matches!(
                diagnostic.rule(),
                TypeRuleDiagnostic::MemberExpressionDeferred
            ) && (diagnostic.node() != call.expression
                || diagnostic.rule() != TypeRuleDiagnostic::DirectCallDeferred)
        });
    }
}

pub(crate) fn call_path_exists(
    call_edges: &[(ByteSpan, ByteSpan)],
    start: ByteSpan,
    goal: ByteSpan,
) -> bool {
    let mut pending = vec![start];
    let mut visited = Vec::new();

    while let Some(current) = pending.pop() {
        if current == goal {
            return true;
        }
        if visited.contains(&current) {
            continue;
        }
        visited.push(current);
        pending.extend(
            call_edges
                .iter()
                .filter_map(|(owner, target)| (*owner == current).then_some(*target)),
        );
    }

    false
}

pub fn check_indirect_calls(
    parsed: &ParseOutput,
    known_expression_types: &[ExpressionType],
    types: &TypeArena,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }
    for call in &parsed.call_expressions {
        let Some(callee_type) = report.expression_type(call.callee) else {
            continue;
        };
        let Some(function_type) = types
            .get(callee_type)
            .and_then(|record| match record.kind() {
                TypeKind::Function(function) => Some(function),
                _ => None,
            })
        else {
            continue;
        };
        if function_type.parameters().len() != call.arguments.len() {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::FunctionTypeApplicationDeferred,
                call.expression,
            ));
            continue;
        }
        let mut valid = true;
        for (argument, expected) in call.arguments.iter().zip(function_type.parameters()) {
            let Some(actual) = report.expression_type(*argument) else {
                valid = false;
                continue;
            };
            if actual != *expected {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    *argument, *expected, actual,
                ));
                valid = false;
            }
        }
        if valid {
            report.record_expression_type(ExpressionType::new(
                call.expression,
                function_type.return_type(),
            ));
        }
    }
    report
}

/// Whether an expression's type is a generic class instance.
pub(crate) fn receiver_is_generic_instance(
    source: &ExecutableSourceTypes<'_>,
    receiver: AstNodeId,
) -> bool {
    source
        .expression_types
        .iter()
        .find(|typed| typed.expression == receiver)
        .and_then(|typed| source.type_arena.and_then(|arena| arena.get(typed.ty)))
        .is_some_and(|record| matches!(record.kind(), TypeKind::GenericInstance(_)))
}
