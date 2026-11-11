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
