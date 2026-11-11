//! Array literals, dynamic arrays, and array element types.

use super::*;

pub fn type_array_expressions(
    types: &mut TypeArena,
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
) {
    type_array_expressions_with_classes(types, parsed, report, &[]);
}

pub fn type_array_expressions_with_classes(
    types: &mut TypeArena,
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
    classes: &[ClassTypeRecord],
) {
    let primitives = PrimitiveTypeIds::module_owned(types);
    let const_lengths = report
        .compile_time_constants()
        .iter()
        .filter_map(|constant| match constant.value() {
            CompileTimeValue::Int(value) => u64::try_from(value)
                .ok()
                .map(|value| (constant.declaration(), value)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let resolve_type = |node: AstNodeId, types: &mut TypeArena| -> Option<TypeId> {
        if let Some(reference) = parsed
            .type_name_references
            .iter()
            .find(|reference| reference.reference == node)
        {
            if reference.name == "Array" && reference.generic_argument_names.len() == 1 {
                let element_name = &reference.generic_argument_names[0];
                let element = primitives
                    .type_for_primitive_name(element_name)
                    .or_else(|| {
                        parsed
                            .generic_parameters
                            .iter()
                            .find(|parameter| parameter.name == *element_name)
                            .and_then(|parameter| {
                                types.records().iter().find_map(|record| {
                                    matches!(
                                        record.kind(),
                                        TypeKind::GenericParameter(generic)
                                            if generic.declaration() == parameter.parameter
                                    )
                                    .then_some(record.id())
                                })
                            })
                    })?;
                return Some(types.dynamic_array(element));
            }
            return primitives.type_for_primitive_name(&reference.name);
        }
        let array = parsed
            .array_types
            .iter()
            .find(|array| array.array == node)?;
        let element =
            resolve_array_element_type(array.element_type, parsed, types, primitives, classes)?;
        Some(types.array(
            element,
            array.length.or_else(|| {
                resolve_named_const_length(array.length_name.as_deref(), parsed, &const_lengths)
            })?,
        ))
    };

    for declaration in &parsed.local_declarations {
        let Some(annotation) = declaration.annotation else {
            continue;
        };
        let Some(ty) = resolve_type(annotation, types) else {
            continue;
        };
        report.record_declaration_signature(DeclarationSignature::new(declaration.declaration, ty));
        if let (Some(initializer), Some(TypeKind::Array(array))) = (
            declaration.initializer,
            types.get(ty).map(|record| record.kind()),
        ) && let Some(literal) = parsed
            .array_literals
            .iter()
            .find(|literal| literal.expression == initializer)
        {
            if literal.elements.len() as u64 != array.length() {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::ArrayLiteralLengthMismatch,
                    literal.expression,
                ));
            }
            for element in &literal.elements {
                if let Some(element_type) = report.expression_type(*element)
                    && !array_element_assignable(element_type, array.element(), classes)
                {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::ArrayElementTypeMismatch,
                        *element,
                    ));
                }
            }
        }
    }

    for expression in &parsed.new_expressions {
        if !expression.dynamic_array {
            continue;
        }
        if let Some(ty) = parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.initializer == Some(expression.expression))
            .and_then(|declaration| declaration.annotation)
            .and_then(|annotation| resolve_type(annotation, types))
        {
            report.replace_expression_type(ExpressionType::new(expression.expression, ty));
        }
    }

    for reference in &parsed.name_references {
        let Some(binding) = parsed
            .local_binding_names
            .iter()
            .find(|binding| binding.name == reference.name)
        else {
            continue;
        };
        if let Some(ty) = report.declaration_signature(binding.binding) {
            report.replace_expression_type(ExpressionType::new(reference.reference, ty));
        }
    }

    for literal in &parsed.array_literals {
        let ty = parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.initializer == Some(literal.expression))
            .and_then(|declaration| declaration.annotation)
            .and_then(|annotation| resolve_type(annotation, types))
            .or_else(|| {
                let element = literal
                    .elements
                    .first()
                    .and_then(|element| report.expression_type(*element))?;
                Some(types.array(element, literal.elements.len() as u64))
            });
        if let Some(ty) = ty {
            report.replace_expression_type(ExpressionType::new(literal.expression, ty));
        }
    }

    for index in &parsed.index_expressions {
        let Some(array_ty) = report.expression_type(index.array) else {
            continue;
        };
        let Some(index_ty) = report.expression_type(index.index) else {
            continue;
        };
        if index_ty != primitives.int_id {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ArrayIndexTypeMismatch,
                index.index,
            ));
            continue;
        }
        match types.get(array_ty).map(|record| record.kind()) {
            Some(TypeKind::Array(array)) => {
                report.replace_expression_type(ExpressionType::new(
                    index.expression,
                    array.element(),
                ));
                if let Some(value) = parsed
                    .integer_literals
                    .iter()
                    .find(|literal| literal.expression == index.index)
                    .and_then(|literal| literal.value)
                    && value >= array.length()
                {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::ArrayIndexOutOfBounds,
                        index.index,
                    ));
                }
            }
            Some(TypeKind::DynamicArray(array)) => {
                report.replace_expression_type(ExpressionType::new(
                    index.expression,
                    array.element(),
                ));
            }
            _ => {}
        }
    }

    for assignment in &parsed.assignment_statements {
        let Some(index) = parsed
            .index_expressions
            .iter()
            .find(|index| index.expression == assignment.target)
        else {
            continue;
        };
        let Some(binding) = parsed.local_binding_names.iter().find(|binding| {
            binding.name
                == parsed
                    .name_references
                    .iter()
                    .find(|name| name.reference == index.array)
                    .map(|name| name.name.as_str())
                    .unwrap_or_default()
        }) else {
            continue;
        };
        if binding.kind == LocalBindingKind::Immutable {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ImmutableArrayMutation,
                assignment.target,
            ));
        }
    }
}

pub(crate) fn array_element_assignable(
    actual: TypeId,
    expected: TypeId,
    classes: &[ClassTypeRecord],
) -> bool {
    if actual == expected {
        return true;
    }
    let Some(actual) = classes.iter().find(|class| class.type_id == actual) else {
        return false;
    };
    let Some(expected) = classes.iter().find(|class| class.type_id == expected) else {
        return false;
    };
    if expected.interface {
        return actual.interfaces.iter().any(|name| name == &expected.name)
            || actual.superclass.as_deref().is_some_and(|parent| {
                classes
                    .iter()
                    .find(|class| class.name == parent)
                    .is_some_and(|parent| {
                        array_element_assignable(parent.type_id, expected.type_id, classes)
                    })
            });
    }
    actual.superclass.as_deref().is_some_and(|parent| {
        classes
            .iter()
            .find(|class| class.name == parent)
            .is_some_and(|parent| {
                array_element_assignable(parent.type_id, expected.type_id, classes)
            })
    })
}

pub fn type_dynamic_array_operations(
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
    types: &TypeArena,
) {
    let dynamic_declarations = parsed
        .local_declarations
        .iter()
        .filter(|declaration| {
            declaration
                .annotation
                .and_then(|annotation| {
                    parsed
                        .type_name_references
                        .iter()
                        .find(|reference| reference.reference == annotation)
                })
                .is_some_and(|reference| reference.name == "Array")
        })
        .map(|declaration| declaration.declaration)
        .collect::<Vec<_>>();
    report.retain_diagnostics(|diagnostic| {
        !dynamic_declarations.contains(&diagnostic.node())
            || diagnostic.rule() != TypeRuleDiagnostic::MissingAnnotationType
    });
    let int_type = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::Int)).then_some(record.id())
    });
    let void_type = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::Void)).then_some(record.id())
    });
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
        if !matches!(
            types.get(receiver_type).map(|record| record.kind()),
            Some(TypeKind::DynamicArray(_))
        ) {
            continue;
        }
        let (result, mutating, valid_arity) = match member.name.as_str() {
            "size" => (int_type, false, call.arguments.is_empty()),
            "add" => (void_type, true, matches!(call.arguments.len(), 1 | 2)),
            "remove" => (void_type, true, call.arguments.len() == 1),
            _ => (None, false, false),
        };
        if !valid_arity {
            continue;
        }
        if mutating
            && let Some(name) = parsed
                .name_references
                .iter()
                .find(|name| name.reference == member.receiver)
            && let Some(binding) = parsed
                .local_binding_names
                .iter()
                .find(|binding| binding.name == name.name)
            && binding.kind == LocalBindingKind::Immutable
        {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ImmutableArrayMutation,
                member.expression,
            ));
            continue;
        }
        if let Some(result) = result {
            report.replace_expression_type(ExpressionType::new(call.expression, result));
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
}

pub(crate) fn resolve_array_element_type(
    node: AstNodeId,
    parsed: &ParseOutput,
    types: &mut TypeArena,
    primitives: PrimitiveTypeIds,
    classes: &[ClassTypeRecord],
) -> Option<TypeId> {
    if let Some(reference) = parsed
        .type_name_references
        .iter()
        .find(|reference| reference.reference == node)
    {
        return primitives
            .type_for_primitive_name(&reference.name)
            .or_else(|| {
                classes
                    .iter()
                    .find(|class| class.name == reference.name)
                    .map(|class| class.type_id)
            });
    }
    let array = parsed
        .array_types
        .iter()
        .find(|array| array.array == node)?;
    let element =
        resolve_array_element_type(array.element_type, parsed, types, primitives, classes)?;
    Some(
        types.array(
            element,
            array.length.or_else(|| {
                resolve_named_const_length(array.length_name.as_deref(), parsed, &[])
            })?,
        ),
    )
}

pub(crate) fn resolve_named_const_length(
    name: Option<&str>,
    parsed: &ParseOutput,
    const_lengths: &[(AstNodeId, u64)],
) -> Option<u64> {
    let name = name?;
    let binding = parsed.local_binding_names.iter().find(|binding| {
        binding.name == name && parsed.const_declarations.contains(&binding.binding)
    })?;
    let declaration = parsed
        .local_declarations
        .iter()
        .find(|declaration| declaration.declaration == binding.binding)?;
    let initializer = declaration.initializer?;
    const_lengths
        .iter()
        .find(|(declaration, _)| *declaration == binding.binding)
        .map(|(_, value)| *value)
        .or_else(|| {
            parsed
                .integer_literals
                .iter()
                .find(|literal| literal.expression == initializer)
                .and_then(|literal| literal.value)
        })
}
