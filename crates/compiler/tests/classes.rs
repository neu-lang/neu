use compiler::{
    ast::AstNodeKind,
    module::{ModuleName, PackageNamespace},
    parser::parse_source,
    source::SourceFileId,
    type_check::{
        ConstructorDiagnosticKind, check_m0069_constructor_calls, type_m0068_class_types,
    },
};

#[test]
fn parses_class_interface_and_field_surface() {
    let parsed = parse_source(
        SourceFileId::from_raw(6800),
        "class Child: Base(), Readable { private val count: Int; public var ready: Bool; } interface Readable {}",
    );
    assert!(
        parsed.lex_diagnostics.is_empty(),
        "{:?}",
        parsed.lex_diagnostics
    );
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    assert!(parsed.node_kinds().contains(&AstNodeKind::ClassDeclaration));
    let (_, types) = type_m0068_class_types(
        &parsed,
        &ModuleName::parse("classes").unwrap(),
        &PackageNamespace::root(),
    );
    assert_eq!(types.classes().len(), 2);
    assert_eq!(types.fields().len(), 2);
    assert_eq!(types.fields()[1].name(), "ready");
}

#[test]
fn rejects_protected_fields_and_missing_field_types() {
    let parsed = parse_source(
        SourceFileId::from_raw(6801),
        "class Invalid { protected val secret: Int; private val missing; }",
    );
    assert!(!parsed.diagnostics.is_empty());
}

#[test]
fn parses_primary_constructor_and_new_expression() {
    let parsed = parse_source(
        SourceFileId::from_raw(6802),
        "class Point(val x: Int, var y: Int) {} fun make(): Point { return new Point(1, 2); }",
    );
    assert!(
        parsed.lex_diagnostics.is_empty(),
        "{:?}",
        parsed.lex_diagnostics
    );
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    assert_eq!(parsed.new_expressions.len(), 1);
    assert_eq!(parsed.class_declarations[0].constructor_parameters.len(), 2);
    assert!(parsed.class_declarations[0].constructor_parameters[0].field);
    let (_, types) = type_m0068_class_types(
        &parsed,
        &ModuleName::parse("classes").unwrap(),
        &PackageNamespace::root(),
    );
    assert_eq!(types.classes()[0].constructor_parameter_count(), 2);
    assert!(check_m0069_constructor_calls(&parsed, &types).is_empty());

    let invalid = parse_source(
        SourceFileId::from_raw(6803),
        "class Point(val x: Int) {} fun make(): Point { return new Missing(1); }",
    );
    let (_, invalid_types) = type_m0068_class_types(
        &invalid,
        &ModuleName::parse("classes").unwrap(),
        &PackageNamespace::root(),
    );
    assert_eq!(
        check_m0069_constructor_calls(&invalid, &invalid_types)[0].kind(),
        ConstructorDiagnosticKind::UnknownClass
    );
}
