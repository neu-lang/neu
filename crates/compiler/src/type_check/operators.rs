//! Integer, primitive, and string operators, compile-time constants, and unsupported expressions.

use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompileTimeValue {
    Bool(bool),
    Int(i64),
    Float(u64),
    Byte(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompileTimeConstant {
    pub(crate) declaration: AstNodeId,
    pub(crate) ty: TypeId,
    pub(crate) value: CompileTimeValue,
}

impl CompileTimeConstant {
    pub fn new(declaration: AstNodeId, ty: TypeId, value: CompileTimeValue) -> Self {
        Self {
            declaration,
            ty,
            value,
        }
    }

    pub fn declaration(self) -> AstNodeId {
        self.declaration
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }

    pub fn value(self) -> CompileTimeValue {
        self.value
    }
}

pub fn type_string_operations(
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
    types: &mut TypeArena,
    array_types: &[ParsedArrayType],
) {
    let Some(string_type) = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::String)).then_some(record.id())
    }) else {
        return;
    };
    let byte_type = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::Byte)).then_some(record.id())
    });
    let int_type = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::Int)).then_some(record.id())
    });
    let bool_type = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::Bool)).then_some(record.id())
    });

    for literal in &parsed.string_literals {
        report.replace_expression_type(ExpressionType::new(literal.expression, string_type));
    }
    report.retain_diagnostics(|diagnostic| {
        diagnostic.kind() != TypeCheckDiagnosticKind::TypeMismatch
            || !parsed
                .string_literals
                .iter()
                .any(|literal| literal.expression == diagnostic.node())
    });

    for name in &parsed.name_references {
        let Some(binding) = parsed
            .local_binding_names
            .iter()
            .find(|binding| binding.name == name.name)
        else {
            continue;
        };
        if let Some(ty) = report.declaration_signature(binding.binding) {
            report.replace_expression_type(ExpressionType::new(name.reference, ty));
        }
    }

    for parameter in &parsed.function_parameters {
        let primitives = PrimitiveTypeIds::module_owned(types);
        let Some(parameter_type) = resolve_annotation_type(
            parameter.annotation,
            &parsed.type_name_references,
            array_types,
            &primitives,
            &[],
            types,
        ) else {
            continue;
        };
        for reference in parsed.name_references.iter().filter(|reference| {
            reference.name == parameter.name
                && parsed
                    .arena
                    .node(reference.reference)
                    .is_some_and(|node| node.span.start() >= parameter.name_span.start())
        }) {
            report
                .replace_expression_type(ExpressionType::new(reference.reference, parameter_type));
        }
    }

    for member in &parsed.member_expressions {
        if report.expression_type(member.expression).is_some() {
            continue;
        }
        let class_name_receiver = parsed
            .name_references
            .iter()
            .find(|reference| reference.reference == member.receiver)
            .filter(|reference| {
                !parsed
                    .local_binding_names
                    .iter()
                    .any(|binding| binding.name == reference.name)
            })
            .map(|reference| reference.name.as_str());
        if class_name_receiver.is_some_and(|class_name| {
            parsed.class_declarations.iter().any(|class| {
                class.name == class_name
                    && parsed.function_declarations.iter().any(|function| {
                        function.owner == Some(class.declaration)
                            && function.name == member.name
                            && function.is_static
                    })
            })
        }) {
            report.retain_diagnostics(|diagnostic| {
                diagnostic.node() != member.expression
                    || diagnostic.rule() != TypeRuleDiagnostic::MemberExpressionDeferred
            });
            continue;
        }
        if report
            .expression_type(member.receiver)
            .is_some_and(|receiver| {
                matches!(
                    types.get(receiver).map(|record| record.kind()),
                    Some(TypeKind::DynamicArray(_))
                )
            })
        {
            continue;
        }
        // `task.cancel()` is typed by the call pass for any task, whatever its result.
        if member.name == "cancel"
            && report
                .expression_type(member.receiver)
                .is_some_and(|receiver| {
                    matches!(
                        types.get(receiver).map(|record| record.kind()),
                        Some(TypeKind::Task(_))
                    )
                })
        {
            continue;
        }
        if member.name == "length" && report.expression_type(member.receiver) == Some(string_type) {
            if let Some(int_type) = int_type {
                report.replace_expression_type(ExpressionType::new(member.expression, int_type));
            }
            report.retain_diagnostics(|diagnostic| {
                diagnostic.node() != member.expression
                    || diagnostic.rule() != TypeRuleDiagnostic::MemberExpressionDeferred
            });
            continue;
        }
        report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
            TypeRuleDiagnostic::MemberExpressionDeferred,
            member.expression,
        ));
    }

    for index in &parsed.index_expressions {
        if report.expression_type(index.array) != Some(string_type) {
            continue;
        }
        if report.expression_type(index.index) != int_type {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::StringIndexTypeMismatch,
                index.index,
            ));
            continue;
        }
        if let Some(byte_type) = byte_type {
            report.replace_expression_type(ExpressionType::new(index.expression, byte_type));
        }
        let literal_length = parsed
            .string_literals
            .iter()
            .find(|literal| literal.expression == index.array)
            .map(|literal| literal.bytes.len())
            .or_else(|| {
                let name = parsed
                    .name_references
                    .iter()
                    .find(|name| name.reference == index.array)?;
                let binding = parsed
                    .local_binding_names
                    .iter()
                    .find(|binding| binding.name == name.name)?;
                let declaration = parsed
                    .local_declarations
                    .iter()
                    .find(|declaration| declaration.declaration == binding.binding)?;
                let initializer = declaration.initializer?;
                parsed
                    .string_literals
                    .iter()
                    .find(|literal| literal.expression == initializer)
                    .map(|literal| literal.bytes.len())
            });
        let static_index = parsed
            .integer_literals
            .iter()
            .find(|literal| literal.expression == index.index)
            .and_then(|literal| literal.value.map(|value| value as i128))
            .or_else(|| {
                let unary = parsed
                    .unary_expressions
                    .iter()
                    .find(|unary| unary.expression == index.index)?;
                if unary.operator != crate::parser::ParsedUnaryOperator::Minus {
                    return None;
                }
                parsed
                    .integer_literals
                    .iter()
                    .find(|literal| literal.expression == unary.operand)
                    .and_then(|literal| literal.value.map(|value| -(value as i128)))
            });
        if let (Some(length), Some(value)) = (literal_length, static_index)
            && (value < 0 || value >= length as i128)
        {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::StringIndexOutOfBounds,
                index.index,
            ));
        }
    }

    for binary in &parsed.binary_expressions {
        let left = report.expression_type(binary.left);
        let right = report.expression_type(binary.right);
        if left != Some(string_type) || right != Some(string_type) {
            continue;
        }
        match binary.operator {
            ParsedBinaryOperator::Equal | ParsedBinaryOperator::NotEqual => {
                if let Some(bool_type) = bool_type {
                    report
                        .replace_expression_type(ExpressionType::new(binary.expression, bool_type));
                }
                report.retain_diagnostics(|diagnostic| {
                    let declaration = parsed
                        .local_declarations
                        .iter()
                        .find(|declaration| declaration.initializer == Some(binary.expression))
                        .map(|declaration| declaration.declaration);
                    diagnostic.kind() != TypeCheckDiagnosticKind::TypeMismatch
                        || (diagnostic.node() != binary.expression
                            && Some(diagnostic.node()) != declaration)
                });
            }
            ParsedBinaryOperator::Plus => {
                report.replace_expression_type(ExpressionType::new(binary.expression, string_type));
                report.retain_diagnostics(|diagnostic| {
                    let declaration = parsed
                        .local_declarations
                        .iter()
                        .find(|declaration| declaration.initializer == Some(binary.expression))
                        .map(|declaration| declaration.declaration);
                    diagnostic.kind() != TypeCheckDiagnosticKind::TypeMismatch
                        || (diagnostic.node() != binary.expression
                            && Some(diagnostic.node()) != declaration)
                });
            }
            _ => report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::StringOperationUnsupported,
                binary.expression,
            )),
        }
    }

    for call in &parsed.call_expressions {
        let is_clone = parsed
            .name_references
            .iter()
            .find(|name| name.reference == call.callee)
            .is_some_and(|name| name.name == "clone");
        if is_clone
            && call.arguments.len() == 1
            && report.expression_type(call.arguments[0]) == Some(string_type)
        {
            report.replace_expression_type(ExpressionType::new(call.expression, string_type));
            report.retain_diagnostics(|diagnostic| {
                diagnostic.node() != call.expression
                    || diagnostic.rule() != TypeRuleDiagnostic::DirectCallDeferred
            });
        }
    }

    for binary in &parsed.binary_expressions {
        let left = report.expression_type(binary.left);
        let right = report.expression_type(binary.right);
        if left != Some(string_type) || right != Some(string_type) {
            continue;
        }
        match binary.operator {
            ParsedBinaryOperator::Equal | ParsedBinaryOperator::NotEqual => {
                if let Some(bool_type) = bool_type {
                    report
                        .replace_expression_type(ExpressionType::new(binary.expression, bool_type));
                }
            }
            ParsedBinaryOperator::Plus => {
                report.replace_expression_type(ExpressionType::new(binary.expression, string_type));
            }
            _ => {}
        }
    }
}

pub fn type_grouped_expressions(
    grouped_expressions: &[ParsedGroupedExpression],
    known_expression_types: &[ExpressionType],
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for grouped in grouped_expressions {
        let Some(inner) = known_expression_types
            .iter()
            .find(|entry| entry.expression() == grouped.inner)
        else {
            continue;
        };
        report.record_expression_type(ExpressionType::new(grouped.expression, inner.ty()));
    }

    report
}

pub fn type_executable_int_operators(
    unary_expressions: &[ParsedUnaryExpression],
    binary_expressions: &[ParsedBinaryExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    known_expression_types: &[ExpressionType],
    int_type: TypeId,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }
    let mut completed = Vec::new();
    loop {
        let mut changed = false;

        for unary in unary_expressions {
            if completed.contains(&unary.expression)
                || !is_executable_int_unary_operator(unary.operator)
            {
                continue;
            }
            let Some(operand_type) = report.expression_type(unary.operand) else {
                continue;
            };

            completed.push(unary.expression);
            changed = true;
            if operand_type == int_type {
                report.record_expression_type(ExpressionType::new(unary.expression, int_type));
            } else {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    unary.operand,
                    int_type,
                    operand_type,
                ));
            }
        }

        for binary in binary_expressions {
            if completed.contains(&binary.expression)
                || !is_executable_int_operator(binary.operator)
            {
                continue;
            }
            let (Some(left_type), Some(right_type)) = (
                report.expression_type(binary.left),
                report.expression_type(binary.right),
            ) else {
                continue;
            };

            completed.push(binary.expression);
            changed = true;
            if left_type != int_type {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    binary.left,
                    int_type,
                    left_type,
                ));
            } else if right_type != int_type {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    binary.right,
                    int_type,
                    right_type,
                ));
            } else {
                report.record_expression_type(ExpressionType::new(binary.expression, int_type));
            }
        }

        for grouped in grouped_expressions {
            if completed.contains(&grouped.expression) {
                continue;
            }
            let Some(inner_type) = report.expression_type(grouped.inner) else {
                continue;
            };

            completed.push(grouped.expression);
            changed = true;
            report.record_expression_type(ExpressionType::new(grouped.expression, inner_type));
        }

        if !changed {
            break;
        }
    }

    report
}

pub(crate) fn is_executable_int_operator(operator: ParsedBinaryOperator) -> bool {
    matches!(
        operator,
        ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
            | ParsedBinaryOperator::Percent
            | ParsedBinaryOperator::Exponent
            | ParsedBinaryOperator::BitwiseAnd
            | ParsedBinaryOperator::BitwiseOr
            | ParsedBinaryOperator::BitwiseXor
            | ParsedBinaryOperator::ShiftLeft
            | ParsedBinaryOperator::ShiftRight
    )
}

pub(crate) fn is_executable_int_unary_operator(
    operator: crate::parser::ParsedUnaryOperator,
) -> bool {
    matches!(
        operator,
        crate::parser::ParsedUnaryOperator::Plus
            | crate::parser::ParsedUnaryOperator::Minus
            | crate::parser::ParsedUnaryOperator::BitwiseNot
    )
}

pub fn type_primitive_operators(
    unary_expressions: &[ParsedUnaryExpression],
    binary_expressions: &[ParsedBinaryExpression],
    known_expression_types: &[ExpressionType],
    bool_type: TypeId,
    float_type: TypeId,
    byte_type: TypeId,
    void_type: TypeId,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }

    for unary in unary_expressions {
        let Some(operand_type) = report.expression_type(unary.operand) else {
            continue;
        };
        if unary.operator == crate::parser::ParsedUnaryOperator::Not {
            if operand_type == bool_type {
                report.record_expression_type(ExpressionType::new(unary.expression, bool_type));
            } else {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    unary.operand,
                    bool_type,
                    operand_type,
                ));
            }
        } else if matches!(
            unary.operator,
            crate::parser::ParsedUnaryOperator::Plus | crate::parser::ParsedUnaryOperator::Minus
        ) && operand_type == float_type
        {
            report.record_expression_type(ExpressionType::new(unary.expression, float_type));
        }
    }

    for binary in binary_expressions {
        let (Some(left), Some(right)) = (
            report.expression_type(binary.left),
            report.expression_type(binary.right),
        ) else {
            continue;
        };
        let (expected, result) = match binary.operator {
            ParsedBinaryOperator::LogicalAnd | ParsedBinaryOperator::LogicalOr => {
                (Some(bool_type), Some(bool_type))
            }
            ParsedBinaryOperator::Equal | ParsedBinaryOperator::NotEqual
                if left == right && left != void_type =>
            {
                (None, Some(bool_type))
            }
            ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
                if left == float_type && right == float_type =>
            {
                (Some(float_type), Some(float_type))
            }
            ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
                if left == float_type || right == float_type =>
            {
                (Some(float_type), Some(float_type))
            }
            ParsedBinaryOperator::Less
            | ParsedBinaryOperator::Greater
            | ParsedBinaryOperator::LessEqual
            | ParsedBinaryOperator::GreaterEqual
                if left == float_type && right == float_type =>
            {
                (Some(float_type), Some(bool_type))
            }
            ParsedBinaryOperator::Less
            | ParsedBinaryOperator::Greater
            | ParsedBinaryOperator::LessEqual
            | ParsedBinaryOperator::GreaterEqual
                if left == float_type || right == float_type =>
            {
                (Some(float_type), Some(bool_type))
            }
            ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
            | ParsedBinaryOperator::Percent
            | ParsedBinaryOperator::ShiftLeft
            | ParsedBinaryOperator::ShiftRight
            | ParsedBinaryOperator::BitwiseAnd
            | ParsedBinaryOperator::BitwiseOr
            | ParsedBinaryOperator::BitwiseXor
                if left == byte_type && right == byte_type =>
            {
                (Some(byte_type), Some(byte_type))
            }
            ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
            | ParsedBinaryOperator::Percent
            | ParsedBinaryOperator::ShiftLeft
            | ParsedBinaryOperator::ShiftRight
            | ParsedBinaryOperator::BitwiseAnd
            | ParsedBinaryOperator::BitwiseOr
            | ParsedBinaryOperator::BitwiseXor
                if left == byte_type || right == byte_type =>
            {
                (Some(byte_type), Some(byte_type))
            }
            _ => (None, None),
        };
        let Some(result) = result else {
            continue;
        };
        if let Some(expected) = expected {
            if left != expected {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    binary.left,
                    expected,
                    left,
                ));
                continue;
            }
            if right != expected {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    binary.right,
                    expected,
                    right,
                ));
                continue;
            }
        } else if left != right {
            continue;
        }
        report.record_expression_type(ExpressionType::new(binary.expression, result));
    }
    report
}

pub fn type_static_integer_diagnostics(
    literals: &[ParsedIntegerLiteral],
    grouped: &[ParsedGroupedExpression],
    unary: &[ParsedUnaryExpression],
    binaries: &[ParsedBinaryExpression],
) -> Vec<TypeCheckDiagnostic> {
    fn is_constant_expression(
        node: AstNodeId,
        literals: &[ParsedIntegerLiteral],
        grouped: &[ParsedGroupedExpression],
        unary: &[ParsedUnaryExpression],
        binaries: &[ParsedBinaryExpression],
    ) -> bool {
        if literals.iter().any(|literal| literal.expression == node) {
            return true;
        }
        if let Some(group) = grouped.iter().find(|group| group.expression == node) {
            return is_constant_expression(group.inner, literals, grouped, unary, binaries);
        }
        if let Some(expression) = unary
            .iter()
            .find(|expression| expression.expression == node)
        {
            return matches!(
                expression.operator,
                crate::parser::ParsedUnaryOperator::Plus
                    | crate::parser::ParsedUnaryOperator::Minus
                    | crate::parser::ParsedUnaryOperator::BitwiseNot
            ) && is_constant_expression(
                expression.operand,
                literals,
                grouped,
                unary,
                binaries,
            );
        }
        let Some(expression) = binaries
            .iter()
            .find(|expression| expression.expression == node)
        else {
            return false;
        };
        is_executable_int_operator(expression.operator)
            && is_constant_expression(expression.left, literals, grouped, unary, binaries)
            && is_constant_expression(expression.right, literals, grouped, unary, binaries)
    }

    fn is_min_int_magnitude(
        node: AstNodeId,
        literals: &[ParsedIntegerLiteral],
        grouped: &[ParsedGroupedExpression],
    ) -> bool {
        if let Some(literal) = literals.iter().find(|literal| literal.expression == node) {
            return literal.value == Some((i64::MAX as u64) + 1);
        }
        grouped
            .iter()
            .find(|group| group.expression == node)
            .is_some_and(|group| is_min_int_magnitude(group.inner, literals, grouped))
    }

    fn checked_power(base: i64, mut exponent: i64) -> Option<i64> {
        let mut result = 1_i64;
        let mut factor = base;
        while exponent > 0 {
            if exponent % 2 != 0 {
                result = result.checked_mul(factor)?;
            }
            exponent /= 2;
            if exponent > 0 {
                factor = factor.checked_mul(factor)?;
            }
        }
        Some(result)
    }

    fn evaluate(
        node: AstNodeId,
        literals: &[ParsedIntegerLiteral],
        grouped: &[ParsedGroupedExpression],
        unary: &[ParsedUnaryExpression],
        binaries: &[ParsedBinaryExpression],
    ) -> Result<Option<i64>, TypeRuleDiagnostic> {
        if let Some(literal) = literals.iter().find(|literal| literal.expression == node) {
            return literal
                .value
                .and_then(|value| i64::try_from(value).ok())
                .map(Some)
                .ok_or(TypeRuleDiagnostic::IntegerLiteralOutOfRange);
        }
        if let Some(group) = grouped.iter().find(|group| group.expression == node) {
            return evaluate(group.inner, literals, grouped, unary, binaries);
        }
        if let Some(expression) = unary
            .iter()
            .find(|expression| expression.expression == node)
        {
            if expression.operator == crate::parser::ParsedUnaryOperator::Minus
                && is_min_int_magnitude(expression.operand, literals, grouped)
            {
                return Ok(Some(i64::MIN));
            }
            let value = evaluate(expression.operand, literals, grouped, unary, binaries)?;
            return match (expression.operator, value) {
                (_, None) => Ok(None),
                (crate::parser::ParsedUnaryOperator::Plus, Some(value)) => Ok(Some(value)),
                (crate::parser::ParsedUnaryOperator::Minus, Some(value)) => value
                    .checked_neg()
                    .map(Some)
                    .ok_or(TypeRuleDiagnostic::IntegerOverflow),
                (crate::parser::ParsedUnaryOperator::BitwiseNot, Some(value)) => Ok(Some(!value)),
                (crate::parser::ParsedUnaryOperator::Not, Some(_)) => Ok(None),
            };
        }
        let Some(expression) = binaries
            .iter()
            .find(|expression| expression.expression == node)
        else {
            return Ok(None);
        };
        let (Some(left), Some(right)) = (
            evaluate(expression.left, literals, grouped, unary, binaries)?,
            evaluate(expression.right, literals, grouped, unary, binaries)?,
        ) else {
            return Ok(None);
        };
        let value = match expression.operator {
            ParsedBinaryOperator::Plus => left.checked_add(right),
            ParsedBinaryOperator::Minus => left.checked_sub(right),
            ParsedBinaryOperator::Star => left.checked_mul(right),
            ParsedBinaryOperator::Slash => {
                return if right == 0 {
                    Err(TypeRuleDiagnostic::DivisionByZero)
                } else {
                    left.checked_div(right)
                        .map(Some)
                        .ok_or(TypeRuleDiagnostic::IntegerOverflow)
                };
            }
            ParsedBinaryOperator::Percent => {
                return if right == 0 {
                    Err(TypeRuleDiagnostic::DivisionByZero)
                } else {
                    left.checked_rem(right)
                        .map(Some)
                        .ok_or(TypeRuleDiagnostic::IntegerOverflow)
                };
            }
            ParsedBinaryOperator::Exponent => {
                if right < 0 {
                    return Err(TypeRuleDiagnostic::NegativeExponent);
                }
                checked_power(left, right)
            }
            ParsedBinaryOperator::ShiftLeft | ParsedBinaryOperator::ShiftRight
                if !(0..64).contains(&right) =>
            {
                return Err(TypeRuleDiagnostic::InvalidShiftCount);
            }
            ParsedBinaryOperator::ShiftLeft => left.checked_shl(right as u32),
            ParsedBinaryOperator::ShiftRight => Some(left >> right),
            ParsedBinaryOperator::BitwiseAnd => Some(left & right),
            ParsedBinaryOperator::BitwiseOr => Some(left | right),
            ParsedBinaryOperator::BitwiseXor => Some(left ^ right),
            _ => return Ok(None),
        };
        value.map(Some).ok_or(TypeRuleDiagnostic::IntegerOverflow)
    }

    let expressions: Vec<_> = literals
        .iter()
        .map(|literal| literal.expression)
        .chain(grouped.iter().map(|expression| expression.expression))
        .chain(unary.iter().map(|expression| expression.expression))
        .chain(binaries.iter().map(|expression| expression.expression))
        .filter(|expression| {
            is_constant_expression(*expression, literals, grouped, unary, binaries)
        })
        .collect();
    let children: Vec<_> = grouped
        .iter()
        .filter(|expression| {
            is_constant_expression(expression.expression, literals, grouped, unary, binaries)
        })
        .map(|expression| expression.inner)
        .chain(
            unary
                .iter()
                .filter(|expression| {
                    is_constant_expression(
                        expression.expression,
                        literals,
                        grouped,
                        unary,
                        binaries,
                    )
                })
                .map(|expression| expression.operand),
        )
        .chain(
            binaries
                .iter()
                .filter(|expression| {
                    is_constant_expression(
                        expression.expression,
                        literals,
                        grouped,
                        unary,
                        binaries,
                    )
                })
                .flat_map(|expression| [expression.left, expression.right]),
        )
        .collect();

    let mut diagnostics = Vec::new();
    for expression in expressions
        .into_iter()
        .filter(|expression| !children.contains(expression))
    {
        if let Err(rule) = evaluate(expression, literals, grouped, unary, binaries) {
            diagnostics.push(TypeCheckDiagnostic::static_integer(rule, expression));
        }
    }
    diagnostics
}

pub fn type_unsupported_expressions(arena: &AstArena) -> TypeCheckReport {
    type_unsupported_expressions_with_operators(arena, None)
}

pub(crate) fn type_unsupported_expressions_with_operators(
    arena: &AstArena,
    executable_operators: Option<(&[ParsedUnaryExpression], &[ParsedBinaryExpression])>,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for node in arena.nodes() {
        if executable_operators.is_some_and(|(unary, binary)| {
            unary.iter().any(|expression| {
                expression.expression == node.id
                    && (is_executable_int_unary_operator(expression.operator)
                        || expression.operator == crate::parser::ParsedUnaryOperator::Not)
            }) || binary.iter().any(|expression| {
                expression.expression == node.id
                    && (is_executable_int_operator(expression.operator)
                        || is_primitive_binary_operator(expression.operator))
            })
        }) {
            continue;
        }
        let Some(rule) = unsupported_expression_rule(node.kind) else {
            continue;
        };
        report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(rule, node.id));
    }

    report
}

pub(crate) fn is_primitive_binary_operator(operator: ParsedBinaryOperator) -> bool {
    matches!(
        operator,
        ParsedBinaryOperator::LogicalOr
            | ParsedBinaryOperator::LogicalAnd
            | ParsedBinaryOperator::Equal
            | ParsedBinaryOperator::NotEqual
            | ParsedBinaryOperator::Less
            | ParsedBinaryOperator::Greater
            | ParsedBinaryOperator::LessEqual
            | ParsedBinaryOperator::GreaterEqual
            | ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
            | ParsedBinaryOperator::Percent
            | ParsedBinaryOperator::BitwiseAnd
            | ParsedBinaryOperator::BitwiseOr
            | ParsedBinaryOperator::BitwiseXor
            | ParsedBinaryOperator::ShiftLeft
            | ParsedBinaryOperator::ShiftRight
    )
}

pub(crate) fn unsupported_expression_rule(kind: AstNodeKind) -> Option<TypeRuleDiagnostic> {
    match kind {
        AstNodeKind::CallExpression => Some(TypeRuleDiagnostic::DirectCallDeferred),
        AstNodeKind::MemberExpression => Some(TypeRuleDiagnostic::MemberExpressionDeferred),
        AstNodeKind::BinaryExpression => Some(TypeRuleDiagnostic::BinaryExpressionDeferred),
        AstNodeKind::UnaryExpression => Some(TypeRuleDiagnostic::UnaryExpressionDeferred),
        AstNodeKind::IfExpression => Some(TypeRuleDiagnostic::IfValueDeferred),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CompileTimeEvaluationError {
    NotConstant,
    DependencyCycle,
}

pub fn validate_compile_time_constants(
    parsed: &ParseOutput,
    expression_types: &[ExpressionType],
    type_arena: &TypeArena,
    report: &mut TypeCheckReport,
) {
    for declaration_id in &parsed.const_declarations {
        let Some(declaration) = parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.declaration == *declaration_id)
        else {
            continue;
        };
        let Some(initializer) = declaration.initializer else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConstInitializerRequired,
                *declaration_id,
            ));
            continue;
        };
        let Some(ty) = expression_types
            .iter()
            .find(|entry| entry.expression() == initializer)
            .map(|entry| entry.ty())
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConstInitializerNotConstant,
                initializer,
            ));
            continue;
        };

        match evaluate_constant(initializer, parsed, expression_types, type_arena) {
            Ok(value) => report.record_compile_time_constant(CompileTimeConstant::new(
                *declaration_id,
                ty,
                value,
            )),
            Err(CompileTimeEvaluationError::DependencyCycle) => {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::ConstDependencyCycle,
                    initializer,
                ));
            }
            Err(CompileTimeEvaluationError::NotConstant) => {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::ConstInitializerNotConstant,
                    initializer,
                ));
            }
        }
    }
}

pub(crate) fn evaluate_constant(
    expression: AstNodeId,
    parsed: &ParseOutput,
    expression_types: &[ExpressionType],
    type_arena: &TypeArena,
) -> Result<CompileTimeValue, CompileTimeEvaluationError> {
    if let Some(literal) = parsed
        .literal_expressions
        .iter()
        .find(|literal| literal.expression == expression)
    {
        return match literal.kind {
            ParsedLiteralKind::BoolTrue => Ok(CompileTimeValue::Bool(true)),
            ParsedLiteralKind::BoolFalse => Ok(CompileTimeValue::Bool(false)),
            ParsedLiteralKind::Float => parsed
                .float_literals
                .iter()
                .find(|literal| literal.expression == expression)
                .and_then(|literal| literal.bits)
                .map(CompileTimeValue::Float)
                .ok_or(CompileTimeEvaluationError::NotConstant),
            ParsedLiteralKind::AcceptedInteger => parsed
                .integer_literals
                .iter()
                .find(|literal| literal.expression == expression)
                .and_then(|literal| literal.value)
                .and_then(|value| i64::try_from(value).ok())
                .map(|value| {
                    if expression_type_is(
                        expression,
                        expression_types,
                        type_arena,
                        PrimitiveType::Byte,
                    ) {
                        CompileTimeValue::Byte(value as u8)
                    } else {
                        CompileTimeValue::Int(value)
                    }
                })
                .ok_or(CompileTimeEvaluationError::NotConstant),
            ParsedLiteralKind::AcceptedString | ParsedLiteralKind::Null => {
                Err(CompileTimeEvaluationError::NotConstant)
            }
        };
    }
    if let Some(grouped) = parsed
        .grouped_expressions
        .iter()
        .find(|grouped| grouped.expression == expression)
    {
        return evaluate_constant(grouped.inner, parsed, expression_types, type_arena);
    }
    if let Some(unary) = parsed
        .unary_expressions
        .iter()
        .find(|unary| unary.expression == expression)
    {
        let operand = evaluate_constant(unary.operand, parsed, expression_types, type_arena)?;
        return evaluate_unary(unary.operator, operand);
    }
    if let Some(binary) = parsed
        .binary_expressions
        .iter()
        .find(|binary| binary.expression == expression)
    {
        let left = evaluate_constant(binary.left, parsed, expression_types, type_arena)?;
        let right = evaluate_constant(binary.right, parsed, expression_types, type_arena)?;
        return evaluate_binary(binary.operator, left, right);
    }
    if let Some(name) = parsed
        .name_references
        .iter()
        .find(|name| name.reference == expression)
        && parsed.local_binding_names.iter().any(|binding| {
            parsed.const_declarations.contains(&binding.binding) && binding.name == name.name
        })
    {
        return Err(CompileTimeEvaluationError::DependencyCycle);
    }
    Err(CompileTimeEvaluationError::NotConstant)
}

pub(crate) fn expression_type_is(
    expression: AstNodeId,
    expression_types: &[ExpressionType],
    type_arena: &TypeArena,
    primitive: PrimitiveType,
) -> bool {
    expression_types
        .iter()
        .find(|entry| entry.expression() == expression)
        .and_then(|entry| type_arena.get(entry.ty()))
        .is_some_and(|record| record.kind() == &TypeKind::Primitive(primitive))
}

pub(crate) fn evaluate_unary(
    operator: ParsedUnaryOperator,
    value: CompileTimeValue,
) -> Result<CompileTimeValue, CompileTimeEvaluationError> {
    match (operator, value) {
        (ParsedUnaryOperator::Not, CompileTimeValue::Bool(value)) => {
            Ok(CompileTimeValue::Bool(!value))
        }
        (ParsedUnaryOperator::Plus, CompileTimeValue::Int(value)) => {
            Ok(CompileTimeValue::Int(value))
        }
        (ParsedUnaryOperator::Minus, CompileTimeValue::Int(value)) => value
            .checked_neg()
            .map(CompileTimeValue::Int)
            .ok_or(CompileTimeEvaluationError::NotConstant),
        (ParsedUnaryOperator::BitwiseNot, CompileTimeValue::Int(value)) => {
            Ok(CompileTimeValue::Int(!value))
        }
        (ParsedUnaryOperator::Plus, CompileTimeValue::Float(value)) => {
            Ok(CompileTimeValue::Float(value))
        }
        (ParsedUnaryOperator::Minus, CompileTimeValue::Float(value)) => {
            Ok(CompileTimeValue::Float((-f64::from_bits(value)).to_bits()))
        }
        (ParsedUnaryOperator::BitwiseNot, CompileTimeValue::Byte(value)) => {
            Ok(CompileTimeValue::Byte(!value))
        }
        _ => Err(CompileTimeEvaluationError::NotConstant),
    }
}

pub(crate) fn evaluate_binary(
    operator: ParsedBinaryOperator,
    left: CompileTimeValue,
    right: CompileTimeValue,
) -> Result<CompileTimeValue, CompileTimeEvaluationError> {
    match (operator, left, right) {
        (
            ParsedBinaryOperator::LogicalAnd,
            CompileTimeValue::Bool(left),
            CompileTimeValue::Bool(right),
        ) => Ok(CompileTimeValue::Bool(left && right)),
        (
            ParsedBinaryOperator::LogicalOr,
            CompileTimeValue::Bool(left),
            CompileTimeValue::Bool(right),
        ) => Ok(CompileTimeValue::Bool(left || right)),
        (ParsedBinaryOperator::Equal, left, right) => Ok(CompileTimeValue::Bool(left == right)),
        (ParsedBinaryOperator::NotEqual, left, right) => Ok(CompileTimeValue::Bool(left != right)),
        (operator, CompileTimeValue::Int(left), CompileTimeValue::Int(right)) => {
            let value = match operator {
                ParsedBinaryOperator::Plus => left.checked_add(right),
                ParsedBinaryOperator::Minus => left.checked_sub(right),
                ParsedBinaryOperator::Star => left.checked_mul(right),
                ParsedBinaryOperator::Slash => left.checked_div(right),
                ParsedBinaryOperator::Percent => left.checked_rem(right),
                ParsedBinaryOperator::Exponent => u32::try_from(right)
                    .ok()
                    .and_then(|right| left.checked_pow(right)),
                ParsedBinaryOperator::BitwiseAnd => Some(left & right),
                ParsedBinaryOperator::BitwiseOr => Some(left | right),
                ParsedBinaryOperator::BitwiseXor => Some(left ^ right),
                ParsedBinaryOperator::ShiftLeft => u32::try_from(right)
                    .ok()
                    .and_then(|right| left.checked_shl(right)),
                ParsedBinaryOperator::ShiftRight => u32::try_from(right)
                    .ok()
                    .and_then(|right| left.checked_shr(right)),
                ParsedBinaryOperator::Less => return Ok(CompileTimeValue::Bool(left < right)),
                ParsedBinaryOperator::Greater => return Ok(CompileTimeValue::Bool(left > right)),
                ParsedBinaryOperator::LessEqual => {
                    return Ok(CompileTimeValue::Bool(left <= right));
                }
                ParsedBinaryOperator::GreaterEqual => {
                    return Ok(CompileTimeValue::Bool(left >= right));
                }
                _ => None,
            };
            value
                .map(CompileTimeValue::Int)
                .ok_or(CompileTimeEvaluationError::NotConstant)
        }
        (operator, CompileTimeValue::Float(left), CompileTimeValue::Float(right)) => {
            let left = f64::from_bits(left);
            let right = f64::from_bits(right);
            let value = match operator {
                ParsedBinaryOperator::Plus => CompileTimeValue::Float((left + right).to_bits()),
                ParsedBinaryOperator::Minus => CompileTimeValue::Float((left - right).to_bits()),
                ParsedBinaryOperator::Star => CompileTimeValue::Float((left * right).to_bits()),
                ParsedBinaryOperator::Slash => CompileTimeValue::Float((left / right).to_bits()),
                ParsedBinaryOperator::Less => CompileTimeValue::Bool(left < right),
                ParsedBinaryOperator::Greater => CompileTimeValue::Bool(left > right),
                ParsedBinaryOperator::LessEqual => CompileTimeValue::Bool(left <= right),
                ParsedBinaryOperator::GreaterEqual => CompileTimeValue::Bool(left >= right),
                _ => return Err(CompileTimeEvaluationError::NotConstant),
            };
            Ok(value)
        }
        (operator, CompileTimeValue::Byte(left), CompileTimeValue::Byte(right)) => {
            let value = match operator {
                ParsedBinaryOperator::Plus => left.checked_add(right),
                ParsedBinaryOperator::Minus => left.checked_sub(right),
                ParsedBinaryOperator::Star => left.checked_mul(right),
                ParsedBinaryOperator::Slash => left.checked_div(right),
                ParsedBinaryOperator::Percent => left.checked_rem(right),
                ParsedBinaryOperator::BitwiseAnd => Some(left & right),
                ParsedBinaryOperator::BitwiseOr => Some(left | right),
                ParsedBinaryOperator::BitwiseXor => Some(left ^ right),
                ParsedBinaryOperator::ShiftLeft => left.checked_shl(u32::from(right)),
                ParsedBinaryOperator::ShiftRight => left.checked_shr(u32::from(right)),
                _ => None,
            };
            value
                .map(CompileTimeValue::Byte)
                .ok_or(CompileTimeEvaluationError::NotConstant)
        }
        _ => Err(CompileTimeEvaluationError::NotConstant),
    }
}
