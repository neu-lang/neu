use compiler::{
    ast::AstNodeKind,
    module::{ModuleName, PackageNamespace},
    parser::parse_source,
    source::SourceFileId,
    type_check::{
        ConstructorDiagnosticKind, DispatchDiagnosticKind, TypeRuleDiagnostic,
        check_m0069_constructor_calls, check_m0070_dispatch, class_lifecycle_facts,
        type_m0068_class_types,
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
fn preserves_explicit_superclass_constructor_arguments() {
    let parsed = parse_source(
        SourceFileId::from_raw(6819),
        "class Base(val value: Int) {} class Child: Base(7) {}",
    );
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    assert_eq!(parsed.class_declarations[1].superclass_arguments.len(), 1);

    let invalid = parse_source(
        SourceFileId::from_raw(6825),
        "class Base(val value: Int) {} class Child: Base() {}",
    );
    let (_, types) = type_m0068_class_types(
        &invalid,
        &ModuleName::parse("classes").unwrap(),
        &PackageNamespace::root(),
    );
    assert_eq!(
        check_m0069_constructor_calls(&invalid, &types)[0].kind(),
        ConstructorDiagnosticKind::SuperclassArgumentCountMismatch
    );
}

#[test]
fn constructs_superclass_fields_before_derived_object_use() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace =
        std::env::temp_dir().join(format!("neu-super-constructor-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let output = compiler::driver::compile_source_to_executable(
        "class Base(val value: Int) {} class Child: Base(17) {} public fun main(): Int { val child: Child = new Child(); return child.value; }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6820),
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
        Some(17)
    );
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn supports_class_typed_function_parameters_and_returns() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace =
        std::env::temp_dir().join(format!("neu-class-signature-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let output = compiler::driver::compile_source_to_executable(
        "class Point(val value: Int) {} fun identity(point: Point): Point { return point; } public fun main(): Int { val point: Point = identity(new Point(19)); return point.value; }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6823),
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
        Some(19)
    );
    let _ = fs::remove_dir_all(workspace);
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
        "class Base { fun value(): Int { return 1; } } class Child: Base() { override fun value(): Int { return 2; } }",
    );
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    assert_eq!(parsed.function_declarations.len(), 2);
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

    let hiding = parse_source(
        SourceFileId::from_raw(6821),
        "class Base(val value: Int) {} class Child: Base() { var value: Int; }",
    );
    let (_, hiding_types) = type_m0068_class_types(
        &hiding,
        &ModuleName::parse("classes").unwrap(),
        &PackageNamespace::root(),
    );
    assert!(
        hiding_types
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.rule() == TypeRuleDiagnostic::FieldHiding)
    );

    let incompatible = parse_source(
        SourceFileId::from_raw(6822),
        "class Base { fun value(): Int { return 1; } } class Child: Base() { override fun value(): Bool { return true; } }",
    );
    assert!(
        check_m0070_dispatch(&incompatible)
            .iter()
            .any(|diagnostic| diagnostic.kind() == DispatchDiagnosticKind::IncompatibleOverride)
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

#[test]
fn dispatches_a_same_module_class_method() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-class-method-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let output = compiler::driver::compile_source_to_executable(
        "class Point(val value: Int) { fun answer(): Int { return 9; } } public fun main(): Int { val point: Point = new Point(7); return point.answer(); }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6810),
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
        Some(9)
    );
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn mutates_a_var_field_and_rejects_val_field_writes() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace =
        std::env::temp_dir().join(format!("neu-class-field-write-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let output = compiler::driver::compile_source_to_executable(
        "class Point(var value: Int) {} public fun main(): Int { val point: Point = new Point(7); point.value = 8; return point.value; }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6811),
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
        Some(8)
    );
    let _ = fs::remove_dir_all(workspace);

    let readonly =
        std::env::temp_dir().join(format!("neu-class-field-readonly-{}", std::process::id()));
    let _ = fs::remove_dir_all(&readonly);
    fs::create_dir_all(&readonly).unwrap();
    let error = compiler::driver::compile_source_to_executable(
        "class Point(val value: Int) {} public fun main(): Int { val point: Point = new Point(7); point.value = 8; return 0; }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6812),
            ModuleName::parse("classes").unwrap(),
            PackageNamespace::root(),
            Triple::host(),
            root.join("target-packs"),
            readonly.join("program"),
        ),
    )
    .unwrap_err();
    assert!(format!("{error:?}").contains("ImmutableFieldMutation"));
    let _ = fs::remove_dir_all(readonly);
}

#[test]
fn dispatches_a_class_through_an_interface_type() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace =
        std::env::temp_dir().join(format!("neu-interface-smoke-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let output = compiler::driver::compile_source_to_executable(
        "interface Answer { fun answer(): Int; } class Point: Answer { fun answer(): Int { return 11; } } public fun main(): Int { val value: Answer = new Point(); return value.answer(); }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6813),
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
        Some(11)
    );
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn accepts_derived_to_base_assignment() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-derived-base-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let output = compiler::driver::compile_source_to_executable(
        "class Base() {} class Child: Base() {} public fun main(): Int { val base: Base = new Child(); return 13; }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6814),
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
        Some(13)
    );
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn class_method_reads_its_implicit_this_receiver() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-method-this-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let output = compiler::driver::compile_source_to_executable(
        "class Point(val value: Int) { fun read(): Int { return this.value; } } public fun main(): Int { val point: Point = new Point(17); return point.read(); }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6815),
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
        Some(17)
    );
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn class_method_reads_a_bare_inherited_field() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-bare-field-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let output = compiler::driver::compile_source_to_executable(
        "class Base(val value: Int) {} class Child: Base(23) { fun read(): Int { return value; } } public fun main(): Int { val child: Child = new Child(); return child.read(); }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6824),
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
        Some(23)
    );
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn open_method_declaration_is_rejected() {
    let parsed = parse_source(
        SourceFileId::from_raw(6830),
        "class Base { open fun value(): Int { return 1; } }",
    );
    assert!(!parsed.diagnostics.is_empty());
}

#[test]
fn default_overridable_method_dispatches_through_a_base_type() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace =
        std::env::temp_dir().join(format!("neu-virtual-dispatch-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let output = compiler::driver::compile_source_to_executable(
        "class Base { fun value(): Int { return 1; } } class Child: Base() { override fun value(): Int { return 2; } } public fun main(): Int { val base: Base = new Child(); return base.value(); }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6831),
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
        Some(2)
    );
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn final_class_is_accepted_and_final_method_cannot_be_overridden() {
    let parsed = parse_source(
        SourceFileId::from_raw(6832),
        "final class Sealed { final fun value(): Int { return 7; } }",
    );
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);

    let invalid = parse_source(
        SourceFileId::from_raw(6833),
        "class Base { final fun value(): Int { return 1; } } class Child: Base() { override fun value(): Int { return 2; } }",
    );
    assert!(!check_m0070_dispatch(&invalid).is_empty());
}

#[test]
fn class_method_can_call_immediate_super_method() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = std::env::temp_dir().join(format!("neu-super-method-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let output = compiler::driver::compile_source_to_executable(
        "class Base { fun value(): Int { return 2; } } class Child: Base() { override fun value(): Int { return super.value() + 1; } } public fun main(): Int { val child: Child = new Child(); return child.value(); }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(6816),
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
        Some(3)
    );
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn lifecycle_orders_constructor_fields_before_body_fields() {
    let parsed = parse_source(
        SourceFileId::from_raw(6817),
        "class Record(val first: Int) { var second: Int; }",
    );
    let lifecycle = class_lifecycle_facts(&parsed);
    assert_eq!(lifecycle[0].initialization_order(), ["first", "second"]);
    assert_eq!(lifecycle[0].destruction_order(), ["second", "first"]);
}

#[test]
fn lifecycle_orders_superclass_fields_before_derived_fields() {
    let parsed = parse_source(
        SourceFileId::from_raw(6818),
        "class Base(val base: Int) {} class Child: Base() { var child: Int; }",
    );
    let lifecycle = class_lifecycle_facts(&parsed);
    assert_eq!(lifecycle[1].initialization_order(), ["base", "child"]);
    assert_eq!(lifecycle[1].destruction_order(), ["child", "base"]);
}
