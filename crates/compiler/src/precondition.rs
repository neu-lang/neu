//! Compile-time decision of a leading `precondition` (ADR-0124).
//!
//! A call to a function whose first statement is `precondition(condition, _)` is
//! decided at compile time when every argument the condition uses is a constant,
//! and the condition depends only on the parameters. Anything else is left to run
//! time, as ADR-0123 specifies.

use crate::{
    ast::{AstNodeId, AstNodeKind},
    parser::{ParseOutput, ParsedBinaryOperator, ParsedLiteralKind, ParsedUnaryOperator},
    source::ByteSpan,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
    Int(i64),
    Bool(bool),
}

/// Whether the leading `precondition` of `function` fails for `arguments`, which
/// are expressions of the calling source `caller`. `Some(true)` means the
/// precondition fails, `Some(false)` means it holds, and `None` means the compiler
/// cannot decide it at compile time.
pub fn leading_precondition_fails(
    callee: &ParseOutput,
    function: AstNodeId,
    caller: &ParseOutput,
    arguments: &[AstNodeId],
) -> Option<bool> {
    let declaration = callee
        .function_declarations
        .iter()
        .find(|declaration| declaration.declaration == function)?;
    let body = callee.arena.node(declaration.body?)?.span;
    let call = callee.call_expressions.iter().find(|call| {
        call.function == function
            && name_of(callee, call.callee) == Some("precondition")
            && is_leading_statement(callee, body, call.expression)
    })?;
    let condition = *call.arguments.first()?;
    let mut environment = Vec::new();
    for (parameter, argument) in declaration.parameters.iter().zip(arguments) {
        let name = callee
            .function_parameters
            .iter()
            .find(|candidate| candidate.parameter == *parameter)?
            .name
            .clone();
        // A parameter whose argument is not a constant stays unbound, so a condition
        // that uses it is not decided.
        if let Some(value) = evaluate(caller, *argument, &[]) {
            environment.push((name, value));
        }
    }
    match evaluate(callee, condition, &environment)? {
        Value::Bool(holds) => Some(!holds),
        Value::Int(_) => None,
    }
}

fn name_of(parsed: &ParseOutput, reference: AstNodeId) -> Option<&str> {
    parsed
        .name_references
        .iter()
        .find(|candidate| candidate.reference == reference)
        .map(|candidate| candidate.name.as_str())
}

/// Whether `call` is the expression of a statement that begins the function body:
/// no statement starts before it in the body.
fn is_leading_statement(parsed: &ParseOutput, body: ByteSpan, call: AstNodeId) -> bool {
    let Some(call_start) = parsed.arena.node(call).map(|node| node.span.start()) else {
        return false;
    };
    let nodes = parsed.arena.nodes();
    let starts_statement = nodes.iter().any(|node| {
        node.kind == AstNodeKind::ExpressionStatement
            && starts_within(body, node.span)
            && node.span.start() == call_start
    });
    starts_statement
        && !nodes.iter().any(|node| {
            is_statement(node.kind)
                && starts_within(body, node.span)
                && node.span.start() < call_start
        })
}

fn is_statement(kind: AstNodeKind) -> bool {
    matches!(
        kind,
        AstNodeKind::VariableDeclarationStatement
            | AstNodeKind::AssignmentStatement
            | AstNodeKind::ReturnStatement
            | AstNodeKind::ExpressionStatement
            | AstNodeKind::ForStatement
            | AstNodeKind::BreakStatement
            | AstNodeKind::ContinueStatement
            | AstNodeKind::ScopeStatement
    )
}

fn starts_within(outer: ByteSpan, inner: ByteSpan) -> bool {
    outer.file() == inner.file() && outer.start() <= inner.start() && inner.end() <= outer.end()
}

/// Evaluates a pure expression over integer and `Bool` values. Names are looked up
/// in `environment`. Anything outside this subset, including overflow, is `None`.
fn evaluate(
    parsed: &ParseOutput,
    expression: AstNodeId,
    environment: &[(String, Value)],
) -> Option<Value> {
    if let Some(integer) = parsed
        .integer_literals
        .iter()
        .find(|candidate| candidate.expression == expression)
    {
        return i64::try_from(integer.value?).ok().map(Value::Int);
    }
    if let Some(literal) = parsed
        .literal_expressions
        .iter()
        .find(|candidate| candidate.expression == expression)
    {
        return match literal.kind {
            ParsedLiteralKind::BoolTrue => Some(Value::Bool(true)),
            ParsedLiteralKind::BoolFalse => Some(Value::Bool(false)),
            _ => None,
        };
    }
    if let Some(name) = name_of(parsed, expression) {
        return environment
            .iter()
            .find(|(bound, _)| bound == name)
            .map(|(_, value)| *value);
    }
    if let Some(grouped) = parsed
        .grouped_expressions
        .iter()
        .find(|candidate| candidate.expression == expression)
    {
        return evaluate(parsed, grouped.inner, environment);
    }
    if let Some(unary) = parsed
        .unary_expressions
        .iter()
        .find(|candidate| candidate.expression == expression)
    {
        let operand = evaluate(parsed, unary.operand, environment)?;
        return match (unary.operator, operand) {
            (ParsedUnaryOperator::Not, Value::Bool(value)) => Some(Value::Bool(!value)),
            (ParsedUnaryOperator::Minus, Value::Int(value)) => value.checked_neg().map(Value::Int),
            (ParsedUnaryOperator::Plus, Value::Int(value)) => Some(Value::Int(value)),
            _ => None,
        };
    }
    if let Some(binary) = parsed
        .binary_expressions
        .iter()
        .find(|candidate| candidate.expression == expression)
    {
        let left = evaluate(parsed, binary.left, environment)?;
        let right = evaluate(parsed, binary.right, environment)?;
        return evaluate_binary(binary.operator, left, right);
    }
    None
}

fn evaluate_binary(operator: ParsedBinaryOperator, left: Value, right: Value) -> Option<Value> {
    match (left, right) {
        (Value::Int(a), Value::Int(b)) => match operator {
            ParsedBinaryOperator::Less => Some(Value::Bool(a < b)),
            ParsedBinaryOperator::LessEqual => Some(Value::Bool(a <= b)),
            ParsedBinaryOperator::Greater => Some(Value::Bool(a > b)),
            ParsedBinaryOperator::GreaterEqual => Some(Value::Bool(a >= b)),
            ParsedBinaryOperator::Equal => Some(Value::Bool(a == b)),
            ParsedBinaryOperator::NotEqual => Some(Value::Bool(a != b)),
            ParsedBinaryOperator::Plus => a.checked_add(b).map(Value::Int),
            ParsedBinaryOperator::Minus => a.checked_sub(b).map(Value::Int),
            ParsedBinaryOperator::Star => a.checked_mul(b).map(Value::Int),
            _ => None,
        },
        (Value::Bool(a), Value::Bool(b)) => match operator {
            ParsedBinaryOperator::LogicalAnd => Some(Value::Bool(a && b)),
            ParsedBinaryOperator::LogicalOr => Some(Value::Bool(a || b)),
            ParsedBinaryOperator::Equal => Some(Value::Bool(a == b)),
            ParsedBinaryOperator::NotEqual => Some(Value::Bool(a != b)),
            _ => None,
        },
        _ => None,
    }
}
