//! Spawn, await, channel, and cancellation typing, and the structure checks for them.

use super::*;

pub fn type_concurrency_operations(
    parsed: &ParseOutput,
    types: &mut TypeArena,
    report: &mut TypeCheckReport,
) {
    let unit = PrimitiveTypeIds::module_owned(types).void_id;
    let int = PrimitiveTypeIds::module_owned(types).int_id;
    for call in &parsed.call_expressions {
        if let Some(member) = parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
            && member.name == "cancel"
        {
            let Some(receiver_type) = report.expression_type(member.receiver) else {
                continue;
            };
            let is_task = types
                .get(receiver_type)
                .is_some_and(|record| matches!(record.kind(), TypeKind::Task(_)));
            if !is_task {
                continue;
            }
            if call.arguments.is_empty() {
                report.replace_expression_type(ExpressionType::new(call.expression, unit));
            } else {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::InvalidTaskOperation,
                    call.expression,
                ));
            }
            continue;
        }
        let Some(name) = parsed
            .name_references
            .iter()
            .find(|reference| reference.reference == call.callee)
            .map(|reference| reference.name.as_str())
        else {
            continue;
        };
        // ADR-0122: every send and receive can block, so both are valid only in
        // suspend functions.
        if matches!(name, "send" | "receive") {
            let suspend = parsed
                .function_declarations
                .iter()
                .find(|function| function.declaration == call.function)
                .is_some_and(|function| function.is_suspend);
            let already_reported = report.diagnostics().iter().any(|diagnostic| {
                diagnostic.node() == call.expression
                    && diagnostic.rule() == TypeRuleDiagnostic::ChannelOpOutsideSuspend
            });
            if !suspend && !already_reported {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::ChannelOpOutsideSuspend,
                    call.expression,
                ));
            }
        }
        match name {
            "channel" => {
                let Some(element_node) = call
                    .generic_arguments
                    .first()
                    .copied()
                    .filter(|_| call.generic_arguments.len() == 1)
                else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidChannelOperation,
                        call.expression,
                    ));
                    continue;
                };
                let Some(element) = resolve_builtin_generic_argument(parsed, element_node, types)
                else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidChannelOperation,
                        call.expression,
                    ));
                    continue;
                };
                let valid_capacity = call.arguments.len() == 1
                    && report.expression_type(call.arguments[0]) == Some(int);
                if !valid_capacity {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidChannelOperation,
                        call.expression,
                    ));
                    continue;
                }
                report.replace_expression_type(ExpressionType::new(
                    call.expression,
                    types.channel(element),
                ));
            }
            "send" => {
                if call.arguments.len() != 2 {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidChannelOperation,
                        call.expression,
                    ));
                    continue;
                }
                let Some(channel_type) = report.expression_type(call.arguments[0]) else {
                    continue;
                };
                let Some(value_type) = report.expression_type(call.arguments[1]) else {
                    continue;
                };
                let valid = types
                    .get(channel_type)
                    .and_then(|record| match record.kind() {
                        TypeKind::Channel(channel) => Some(channel.element()),
                        _ => None,
                    })
                    .is_some_and(|element| value_type == element);
                if valid {
                    report.replace_expression_type(ExpressionType::new(call.expression, unit));
                } else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidChannelOperation,
                        call.expression,
                    ));
                }
            }
            "receive" => {
                let Some(argument) = call
                    .arguments
                    .first()
                    .copied()
                    .filter(|_| call.arguments.len() == 1)
                else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidChannelOperation,
                        call.expression,
                    ));
                    continue;
                };
                let Some(channel) = report.expression_type(argument) else {
                    continue;
                };
                let Some(element) = types.get(channel).and_then(|record| match record.kind() {
                    TypeKind::Channel(channel) => Some(channel.element()),
                    _ => None,
                }) else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidChannelOperation,
                        call.expression,
                    ));
                    continue;
                };
                report.replace_expression_type(ExpressionType::new(
                    call.expression,
                    types.channel_result(element),
                ));
            }
            "close" => {
                if call.arguments.len() != 1 {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidChannelOperation,
                        call.expression,
                    ));
                    continue;
                }
                let Some(channel) = report.expression_type(call.arguments[0]) else {
                    continue;
                };
                let valid = types
                    .get(channel)
                    .is_some_and(|record| matches!(record.kind(), TypeKind::Channel(_)));
                if valid {
                    report.replace_expression_type(ExpressionType::new(call.expression, unit));
                } else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidChannelOperation,
                        call.expression,
                    ));
                }
            }
            "spawn" => {
                let Some(argument) = call
                    .arguments
                    .first()
                    .copied()
                    .filter(|_| call.arguments.len() == 1)
                else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidTaskOperation,
                        call.expression,
                    ));
                    continue;
                };
                let Some(argument_type) = report.expression_type(argument) else {
                    continue;
                };
                let Some(function_type) = types.get(argument_type).and_then(|record| match record
                    .kind()
                {
                    TypeKind::Function(function) => Some(function.return_type()),
                    _ => None,
                }) else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidTaskOperation,
                        call.expression,
                    ));
                    continue;
                };
                report.replace_expression_type(ExpressionType::new(
                    call.expression,
                    types.task(function_type),
                ));
            }
            "await" => {
                let Some(argument) = call
                    .arguments
                    .first()
                    .copied()
                    .filter(|_| call.arguments.len() == 1)
                else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidTaskOperation,
                        call.expression,
                    ));
                    continue;
                };
                let Some(argument_type) = report.expression_type(argument) else {
                    continue;
                };
                let Some(result) =
                    types
                        .get(argument_type)
                        .and_then(|record| match record.kind() {
                            TypeKind::Task(task) => Some(task.result()),
                            _ => None,
                        })
                else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::InvalidTaskOperation,
                        call.expression,
                    ));
                    continue;
                };
                // A no-result task yields no value, so an `await` of one may be a statement
                // but not the initializer of a local (SPEC, ADR-0101).
                if result == unit
                    && parsed
                        .local_declarations
                        .iter()
                        .any(|declaration| declaration.initializer == Some(call.expression))
                {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::AwaitVoidResult,
                        call.expression,
                    ));
                }
                report.replace_expression_type(ExpressionType::new(call.expression, result));
            }
            "cancel" => {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::InvalidTaskOperation,
                    call.expression,
                ));
            }
            _ => {}
        }
    }
}

pub(crate) fn resolve_builtin_generic_argument(
    parsed: &ParseOutput,
    argument: AstNodeId,
    types: &mut TypeArena,
) -> Option<TypeId> {
    let primitives = PrimitiveTypeIds::module_owned(types);
    let name = parsed
        .type_name_references
        .iter()
        .find(|reference| reference.reference == argument)
        .map(|reference| reference.name.clone())
        .or_else(|| {
            let argument_span = parsed.arena.node(argument).map(|node| node.span)?;
            parsed
                .type_name_references
                .iter()
                .find(|reference| {
                    parsed
                        .arena
                        .node(reference.reference)
                        .is_some_and(|node| node.span == argument_span)
                })
                .map(|reference| reference.name.clone())
        })
        .or_else(|| {
            parsed.type_name_references.iter().find_map(|reference| {
                reference
                    .generic_arguments
                    .iter()
                    .position(|candidate| *candidate == argument)
                    .and_then(|index| reference.generic_argument_names.get(index))
                    .cloned()
            })
        })?;
    primitives.type_for_primitive_name(&name)
}

pub fn validate_concurrency_structure(
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
    types: &TypeArena,
) {
    for call in &parsed.call_expressions {
        let member_cancel = parsed.member_expressions.iter().any(|member| {
            member.expression == call.callee
                && member.name == "cancel"
                && report.expression_type(member.receiver).is_some_and(|ty| {
                    types
                        .get(ty)
                        .is_some_and(|record| matches!(record.kind(), TypeKind::Task(_)))
                })
        });
        let name = if member_cancel {
            "cancel"
        } else if let Some(name) = parsed
            .name_references
            .iter()
            .find(|reference| reference.reference == call.callee)
            .map(|reference| reference.name.as_str())
        {
            name
        } else {
            continue;
        };
        let Some(span) = parsed.arena.node(call.expression).map(|node| node.span) else {
            continue;
        };
        let in_scope = parsed.scope_statements.iter().any(|scope| {
            scope.function == call.function && concurrency_span_contains(scope.span, span)
        });
        match name {
            "spawn" if !in_scope => {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::SpawnOutsideScope,
                    call.expression,
                ))
            }
            "await" => {
                let suspend = parsed
                    .function_declarations
                    .iter()
                    .find(|function| function.declaration == call.function)
                    .is_some_and(|function| function.is_suspend);
                if !suspend {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::AwaitOutsideSuspend,
                        call.expression,
                    ));
                }
            }
            "cancel" if !in_scope => {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::InvalidTaskOperation,
                    call.expression,
                ))
            }
            _ => {}
        }
    }
}

pub fn validate_task_member_cancellation_structure(
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
    types: &TypeArena,
) {
    for call in &parsed.call_expressions {
        let Some(member) = parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee && member.name == "cancel")
        else {
            continue;
        };
        let Some(receiver_type) = report.expression_type(member.receiver) else {
            continue;
        };
        let Some(receiver_record) = types.get(receiver_type) else {
            continue;
        };
        if !matches!(receiver_record.kind(), TypeKind::Task(_)) {
            if !matches!(receiver_record.kind(), TypeKind::Nominal(_)) {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::InvalidTaskOperation,
                    call.expression,
                ));
            }
            continue;
        }
        let in_scope = parsed.scope_statements.iter().any(|scope| {
            scope.function == call.function
                && parsed
                    .arena
                    .node(call.expression)
                    .is_some_and(|node| concurrency_span_contains(scope.span, node.span))
        });
        if !in_scope
            && !report.diagnostics().iter().any(|diagnostic| {
                diagnostic.node() == call.expression
                    && diagnostic.rule() == TypeRuleDiagnostic::InvalidTaskOperation
            })
        {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::InvalidTaskOperation,
                call.expression,
            ));
        }
    }
}

pub(crate) fn concurrency_span_contains(outer: ByteSpan, inner: ByteSpan) -> bool {
    outer.file() == inner.file() && outer.start() <= inner.start() && inner.end() <= outer.end()
}
