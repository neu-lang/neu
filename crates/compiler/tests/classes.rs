use compiler::{
    ast::AstNodeKind,
    module::{ModuleName, PackageNamespace},
    parser::parse_source,
    source::SourceFileId,
    type_check::{
        ConstructorDiagnosticKind, DispatchDiagnosticKind, check_m0069_constructor_calls,
        check_m0070_dispatch, class_lifecycle_facts, type_m0068_class_types,
    },
};
use std::{fs, path::PathBuf};
use target_lexicon::Triple;

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
    assert_eq!(types.fields().len(), 2);
    assert!(check_m0069_constructor_calls(&parsed, &types).is_empty());
    let lifecycle = class_lifecycle_facts(&parsed);
    assert_eq!(lifecycle[0].initialization_order(), ["x", "y"]);
    assert_eq!(lifecycle[0].destruction_order(), ["y", "x"]);

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

#[test]
fn associates_method_declarations_with_their_class() {
    let parsed = parse_source(
        SourceFileId::from_raw(6804),
        "class Point(val x: Int) { fun value(): Int { return x; } }",
    );
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    assert_eq!(parsed.function_declarations.len(), 1);
    assert_eq!(
        parsed.function_declarations[0].owner,
        Some(parsed.class_declarations[0].declaration)
    );
}

#[test]
fn preserves_method_dispatch_modifiers_and_visibility() {
    let parsed = parse_source(
        SourceFileId::from_raw(6805),
        "class Base { open fun value(): Int { return 1; } } class Child: Base() { override fun value(): Int { return 2; } }",
    );
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    assert_eq!(parsed.function_declarations.len(), 2);
    assert!(parsed.function_declarations[0].is_open);
    assert!(parsed.function_declarations[1].is_override);
    assert!(check_m0070_dispatch(&parsed).is_empty());

    let invalid = parse_source(
        SourceFileId::from_raw(6806),
        "class Base { fun value(): Int { return 1; } } class Child: Base() { fun value(): Int { return 2; } }",
    );
    assert_eq!(
        check_m0070_dispatch(&invalid)[0].kind(),
        DispatchDiagnosticKind::MissingOverrideMarker
    );

    let incomplete = parse_source(
        SourceFileId::from_raw(6807),
        "interface Readable { fun read(): Int; } class Item: Readable {}",
    );
    assert!(
        check_m0070_dispatch(&incomplete)
            .iter()
            .any(|diagnostic| diagnostic.kind() == DispatchDiagnosticKind::MissingInterfaceMethod)
    );
}

#[test]
fn invalid_class_source_stops_before_backend() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-class-boundary-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let error = compiler::driver::compile_source_to_executable(
        "class Point { protected val x: Int; } public fun main(): Int { return 0; }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6808),
            ModuleName::parse("classes").unwrap(),
            PackageNamespace::root(),
            Triple::host(),
            root.join("target-packs"),
            workspace.join("program"),
        ),
    )
    .unwrap_err();
    assert!(format!("{error:?}").contains("ParseDiagnostics"));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn constructs_a_minimal_owned_class_through_the_target_pack() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-class-smoke-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let output = compiler::driver::compile_source_to_executable(
        "class Point(val value: Int) {} public fun main(): Int { val point: Point = new Point(7); return point.value; }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6809),
            ModuleName::parse("classes").unwrap(),
            PackageNamespace::root(),
            Triple::host(),
            root.join("target-packs"),
            workspace.join("program"),
        ),
    )
    .unwrap();
    assert_eq!(
        std::process::Command::new(output).status().unwrap().code(),
        Some(7)
    );
    let _ = fs::remove_dir_all(workspace);
}
