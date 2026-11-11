//! Qualified and named import forms (ADR-0095, ADR-0117).
//!
//! A qualified import (`import "./math"` with an optional alias) makes the
//! package visible through its qualifier in the importing file only. A named
//! import (`import {add} from "./math"`) exposes only the listed names and
//! creates no qualifier.

use std::{fs, process::Command};

use compiler::{
    driver::{
        SourceDriverOptions, compile_virtual_project_to_executable, validate_virtual_project,
    },
    module::{
        ModuleName, PackageGraphDiagnosticKind, PackageNamespace, VirtualPackageGraph,
        VirtualSource,
    },
    source::SourceFileId,
};

fn math_package() -> VirtualSource {
    VirtualSource::new(
        "src/math/add.neu",
        "package math\n\npublic func add(): Int {\n    return 7\n}\n",
    )
}

fn graph_error(main: &str) -> Vec<PackageGraphDiagnosticKind> {
    VirtualPackageGraph::build(
        "src/main.neu",
        [VirtualSource::new("src/main.neu", main), math_package()],
    )
    .expect_err("graph should be rejected")
    .into_iter()
    .map(|diagnostic| diagnostic.kind)
    .collect()
}

fn run_exit_code(name: &str, main: &str) -> Option<i32> {
    let workspace =
        std::env::temp_dir().join(format!("neu-import-forms-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let project = validate_virtual_project(
        "src/main.neu",
        [VirtualSource::new("src/main.neu", main), math_package()],
    )
    .unwrap();
    let executable = compile_virtual_project_to_executable(
        &project,
        SourceDriverOptions::new(
            SourceFileId::from_raw(31500),
            ModuleName::parse("import.forms").unwrap(),
            PackageNamespace::root(),
            workspace.join("program"),
        ),
    )
    .unwrap();
    let code = Command::new(executable).status().unwrap().code();
    let _ = fs::remove_dir_all(workspace);
    code
}

#[test]
fn qualified_import_exposes_the_package_qualifier() {
    assert_eq!(
        run_exit_code(
            "qualified",
            "import \"./math\"\n\npublic func main(): Int {\n    return math.add()\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn qualified_import_with_alias_exposes_the_alias_qualifier() {
    assert_eq!(
        run_exit_code(
            "alias",
            "import \"./math\" as m\n\npublic func main(): Int {\n    return m.add()\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn named_import_exposes_the_name_unqualified() {
    assert_eq!(
        run_exit_code(
            "named",
            "import {add} from \"./math\"\n\npublic func main(): Int {\n    return add()\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn named_import_does_not_make_its_package_a_qualifier() {
    let kinds = graph_error(
        "import {add} from \"./math\"\n\npublic func main(): Int {\n    return math.add()\n}\n",
    );
    assert!(
        kinds.contains(&PackageGraphDiagnosticKind::QualifierNotImported),
        "{kinds:?}"
    );
}

#[test]
fn alias_replaces_the_package_qualifier() {
    let kinds = graph_error(
        "import \"./math\" as m\n\npublic func main(): Int {\n    return math.add()\n}\n",
    );
    assert!(
        kinds.contains(&PackageGraphDiagnosticKind::QualifierNotImported),
        "{kinds:?}"
    );
}

#[test]
fn local_named_like_a_package_qualifier_compiles_and_runs() {
    assert_eq!(
        run_exit_code(
            "shadow",
            "import {add} from \"./math\"\n\nclass Box(var v: Int) {\n}\n\npublic func main(): Int {\n    val math: Box = new Box(7);\n    return math.v\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn identifiers_that_contain_a_qualifier_name_are_untouched() {
    assert_eq!(
        run_exit_code(
            "contains",
            "import \"./math\"\n\nclass Box(var v: Int) {\n}\n\npublic func main(): Int {\n    val mymath: Box = new Box(7);\n    return mymath.v + math.add() - 7\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn named_import_of_a_private_name_is_rejected() {
    let graph = VirtualPackageGraph::build(
        "src/main.neu",
        [
            VirtualSource::new(
                "src/main.neu",
                "import {hidden} from \"./math\"\n\npublic func main(): Int {\n    return hidden()\n}\n",
            ),
            VirtualSource::new(
                "src/math/add.neu",
                "package math\n\nprivate func hidden(): Int {\n    return 7\n}\n",
            ),
        ],
    );
    let kinds: Vec<_> = graph
        .expect_err("private name should be rejected")
        .into_iter()
        .map(|diagnostic| diagnostic.kind)
        .collect();
    assert!(
        kinds.contains(&PackageGraphDiagnosticKind::InaccessibleImport),
        "{kinds:?}"
    );
}

#[test]
fn duplicate_name_within_one_named_import_is_rejected() {
    let kinds = graph_error(
        "import {add, add} from \"./math\"\n\npublic func main(): Int {\n    return add()\n}\n",
    );
    assert!(
        kinds.contains(&PackageGraphDiagnosticKind::DuplicateNamedImport),
        "{kinds:?}"
    );
}

#[test]
fn named_import_colliding_with_a_local_declaration_is_rejected() {
    let kinds = graph_error(
        "import {add} from \"./math\"\n\npublic func main(): Int {\n    return add()\n}\n\nfunc add(): Int {\n    return 0\n}\n",
    );
    assert!(
        kinds.contains(&PackageGraphDiagnosticKind::NamedImportCollision),
        "{kinds:?}"
    );
}

#[test]
fn named_import_of_a_public_class_is_accepted() {
    let graph = VirtualPackageGraph::build(
        "src/main.neu",
        [
            VirtualSource::new(
                "src/main.neu",
                "import {Box} from \"./models\"\n\npublic func main(): Int {\n    return 1\n}\n",
            ),
            VirtualSource::new(
                "src/models/box.neu",
                "package models\n\npublic class Box(var v: Int) {\n}\n",
            ),
        ],
    );
    assert!(graph.is_ok(), "{:?}", graph.err());
}

#[test]
fn named_import_of_a_private_class_is_rejected() {
    let kinds = VirtualPackageGraph::build(
        "src/main.neu",
        [
            VirtualSource::new(
                "src/main.neu",
                "import {Box} from \"./models\"\n\npublic func main(): Int {\n    return 1\n}\n",
            ),
            VirtualSource::new(
                "src/models/box.neu",
                "package models\n\nprivate class Box(var v: Int) {\n}\n",
            ),
        ],
    )
    .expect_err("private class should be rejected")
    .into_iter()
    .map(|diagnostic| diagnostic.kind)
    .collect::<Vec<_>>();
    assert!(
        kinds.contains(&PackageGraphDiagnosticKind::InaccessibleImport),
        "{kinds:?}"
    );
}

#[test]
fn named_import_of_a_public_class_compiles_and_runs() {
    let workspace =
        std::env::temp_dir().join(format!("neu-import-forms-class-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let project = validate_virtual_project(
        "src/main.neu",
        [
            VirtualSource::new(
                "src/main.neu",
                "import {Box} from \"./models\"\n\npublic func main(): Int {\n    val b: Box = new Box(7);\n    return b.v;\n}\n",
            ),
            VirtualSource::new(
                "src/models/box.neu",
                "package models\n\npublic class Box(var v: Int) {\n}\n",
            ),
        ],
    )
    .unwrap();
    let executable = compile_virtual_project_to_executable(
        &project,
        SourceDriverOptions::new(
            SourceFileId::from_raw(31501),
            ModuleName::parse("import.class").unwrap(),
            PackageNamespace::root(),
            workspace.join("program"),
        ),
    )
    .unwrap();
    assert_eq!(Command::new(executable).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn named_import_of_a_public_enum_is_accepted() {
    let graph = VirtualPackageGraph::build(
        "src/main.neu",
        [
            VirtualSource::new(
                "src/main.neu",
                "import {Color} from \"./models\"\n\npublic func main(): Int {\n    return 1\n}\n",
            ),
            VirtualSource::new(
                "src/models/color.neu",
                "package models\n\npublic enum Color {\n    Red,\n    Blue\n}\n",
            ),
        ],
    );
    assert!(graph.is_ok(), "{:?}", graph.err());
}

#[test]
fn qualified_access_without_any_import_is_rejected() {
    let kinds = graph_error("public func main(): Int {\n    return math.add();\n}\n");
    assert!(
        kinds.contains(&PackageGraphDiagnosticKind::QualifierNotImported),
        "{kinds:?}"
    );
}
