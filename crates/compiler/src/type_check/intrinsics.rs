//! Built-in `assert`, `fail`, and `precondition` calls, and test declarations.

use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TestDiagnosticKind {
    NotPublic,
    NotTopLevel,
    ParametersNotAllowed,
    GenericNotAllowed,
    SuspendNotAllowed,
    ReturnTypeNotAllowed,
    BodyRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TestDiagnostic {
    pub(crate) kind: TestDiagnosticKind,
    pub(crate) span: ByteSpan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntrinsicDiagnosticKind {
    ArgumentCount,
    ConditionType,
    MessageType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntrinsicDiagnostic {
    pub(crate) kind: IntrinsicDiagnosticKind,
    pub(crate) span: ByteSpan,
}

impl IntrinsicDiagnostic {
    pub fn kind(self) -> IntrinsicDiagnosticKind {
        self.kind
    }

    pub fn span(self) -> ByteSpan {
        self.span
    }
}

pub fn validate_intrinsic_calls(
    parsed: &ParseOutput,
    expression_types: &[ExpressionType],
    types: &TypeArena,
) -> Vec<IntrinsicDiagnostic> {
    let primitive = |kind| {
        types
            .records()
            .iter()
            .find(|record| record.kind() == &TypeKind::Primitive(kind))
            .map(|record| record.id())
    };
    let bool_type = primitive(PrimitiveType::Bool);
    let int_type = primitive(PrimitiveType::Int);
    let string_type = primitive(PrimitiveType::String);
    let mut diagnostics = Vec::new();
    for call in &parsed.call_expressions {
        let Some(name) = parsed
            .name_references
            .iter()
            .find(|reference| reference.reference == call.callee)
            .map(|reference| reference.name.as_str())
        else {
            continue;
        };
        let expected = if matches!(name, "assert" | "precondition") {
            2
        } else if name == "fail" {
            1
        } else {
            continue;
        };
        let span = parsed
            .arena
            .node(call.expression)
            .map(|node| node.span)
            .expect("intrinsic call has an AST node");
        if call.arguments.len() != expected {
            diagnostics.push(IntrinsicDiagnostic {
                kind: IntrinsicDiagnosticKind::ArgumentCount,
                span,
            });
            continue;
        }
        let condition_is_bool = expression_types
            .iter()
            .find(|entry| entry.expression() == call.arguments[0])
            .map(|entry| entry.ty())
            == bool_type
            || parsed.binary_expressions.iter().any(|binary| {
                binary.expression == call.arguments[0]
                    && matches!(
                        binary.operator,
                        ParsedBinaryOperator::Equal
                            | ParsedBinaryOperator::NotEqual
                            | ParsedBinaryOperator::Less
                            | ParsedBinaryOperator::Greater
                            | ParsedBinaryOperator::LessEqual
                            | ParsedBinaryOperator::GreaterEqual
                    )
                    && expression_types
                        .iter()
                        .find(|entry| entry.expression() == binary.left)
                        .map(|entry| entry.ty())
                        == int_type
                    && expression_types
                        .iter()
                        .find(|entry| entry.expression() == binary.right)
                        .map(|entry| entry.ty())
                        == int_type
            });
        if matches!(name, "assert" | "precondition") && !condition_is_bool {
            diagnostics.push(IntrinsicDiagnostic {
                kind: IntrinsicDiagnosticKind::ConditionType,
                span,
            });
        }
        let message = call.arguments[if matches!(name, "assert" | "precondition") {
            1
        } else {
            0
        }];
        if expression_types
            .iter()
            .find(|entry| entry.expression() == message)
            .map(|entry| entry.ty())
            != string_type
        {
            diagnostics.push(IntrinsicDiagnostic {
                kind: IntrinsicDiagnosticKind::MessageType,
                span,
            });
        }
    }
    diagnostics
}

pub fn apply_intrinsic_call_facts(
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
    types: &TypeArena,
) {
    let Some(void_type) = types
        .records()
        .iter()
        .find(|record| record.kind() == &TypeKind::Primitive(PrimitiveType::Void))
        .map(|record| record.id())
    else {
        return;
    };
    for call in &parsed.call_expressions {
        let Some(name) = parsed
            .name_references
            .iter()
            .find(|reference| reference.reference == call.callee)
            .map(|reference| reference.name.as_str())
        else {
            continue;
        };
        if matches!(name, "assert" | "fail" | "precondition") {
            report.replace_expression_type(ExpressionType::new(call.expression, void_type));
            report.retain_diagnostics(|diagnostic| {
                diagnostic.node() != call.expression
                    || !matches!(diagnostic.rule(), TypeRuleDiagnostic::DirectCallDeferred)
            });
        }
    }
}

impl TestDiagnostic {
    pub fn kind(self) -> TestDiagnosticKind {
        self.kind
    }

    pub fn span(self) -> ByteSpan {
        self.span
    }
}

pub fn validate_test_declarations(parsed: &ParseOutput) -> Vec<TestDiagnostic> {
    parsed
        .function_declarations
        .iter()
        .filter(|function| function.is_test)
        .flat_map(|function| {
            let Some(span) = parsed
                .arena
                .node(function.declaration)
                .map(|node| node.span)
            else {
                return Vec::new();
            };
            let mut diagnostics = Vec::new();
            if function.visibility != "public" {
                diagnostics.push(TestDiagnostic {
                    kind: TestDiagnosticKind::NotPublic,
                    span,
                });
            }
            if !function.top_level {
                diagnostics.push(TestDiagnostic {
                    kind: TestDiagnosticKind::NotTopLevel,
                    span,
                });
            }
            if !function.parameters.is_empty() {
                diagnostics.push(TestDiagnostic {
                    kind: TestDiagnosticKind::ParametersNotAllowed,
                    span,
                });
            }
            if parsed
                .generic_parameters
                .iter()
                .any(|parameter| parameter.owner == Some(function.declaration))
            {
                diagnostics.push(TestDiagnostic {
                    kind: TestDiagnosticKind::GenericNotAllowed,
                    span,
                });
            }
            if function.is_suspend {
                diagnostics.push(TestDiagnostic {
                    kind: TestDiagnosticKind::SuspendNotAllowed,
                    span,
                });
            }
            if function.return_annotation.is_some() {
                diagnostics.push(TestDiagnostic {
                    kind: TestDiagnosticKind::ReturnTypeNotAllowed,
                    span,
                });
            }
            if function.body.is_none() {
                diagnostics.push(TestDiagnostic {
                    kind: TestDiagnosticKind::BodyRequired,
                    span,
                });
            }
            diagnostics
        })
        .collect()
}
