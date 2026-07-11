use compiler::{
    ast::AstNodeKind,
    module::{ModuleName, PackageNamespace},
    parser::parse_source,
    source::SourceFileId,
    type_check::type_m0068_class_types,
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
