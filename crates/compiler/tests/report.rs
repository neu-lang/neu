use std::path::{Path, PathBuf};

use compiler::{
    driver::{DriverError, compile_manifest_to_executable},
    module::{VirtualPackageGraph, VirtualSource},
    report::{LineOrigin, render_manifest_error},
};

struct Project {
    root: PathBuf,
}

impl Project {
    fn new(name: &str, files: &[(&str, &str)]) -> Self {
        let root = std::env::temp_dir().join(format!(
            "neu-report-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("neu.json"),
            format!(
                r#"{{"name":"report.{name}","entrypoint":"src/main.neu","srcs":["src/**/*.neu"],"dependencies":[]}}"#
            ),
        )
        .unwrap();
        for (path, text) in files {
            let path = root.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        Self { root }
    }

    fn manifest(&self) -> PathBuf {
        self.root.join("neu.json")
    }

    fn compile_error(&self) -> DriverError {
        compile_manifest_to_executable(self.manifest(), self.root.join("target/out"))
            .expect_err("project should fail to compile")
    }

    fn rendered(&self) -> String {
        render_manifest_error(self.manifest(), &self.compile_error())
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn bootstrap_line_origins_map_merged_lines_to_source_files() {
    let graph = VirtualPackageGraph::build(
        "src/main.neu",
        [
            VirtualSource::new(
                "src/main.neu",
                "import \"./math\" as arithmetic\n\npublic func main(): Int {\n    return arithmetic.add()\n}",
            ),
            VirtualSource::new(
                "src/math/add.neu",
                "package math\n\npublic func add(): Int {\n    return 7\n}",
            ),
        ],
    )
    .unwrap();

    let merged = graph.bootstrap_source();
    let origins = graph.bootstrap_line_origins();

    assert_eq!(merged.split('\n').count(), origins.len());
    let line_of = |needle: &str| {
        let index = merged
            .split('\n')
            .position(|line| line.contains(needle))
            .unwrap();
        origins[index].clone()
    };
    assert_eq!(
        line_of("func main"),
        LineOrigin {
            path: PathBuf::from("src/main.neu"),
            line: 3
        }
    );
    assert_eq!(
        line_of("return 7"),
        LineOrigin {
            path: PathBuf::from("src/math/add.neu"),
            line: 4
        }
    );
}

#[test]
fn parse_errors_report_file_line_column_and_kind() {
    let project = Project::new(
        "parse",
        &[(
            "src/main.neu",
            "public func main(): Int {\n    return 1 +;\n}\n",
        )],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(rendered.contains("error["), "{rendered}");
    assert!(rendered.contains("src/main.neu:2:"), "{rendered}");
    assert!(rendered.contains("    return 1 +;"), "{rendered}");
    assert!(rendered.contains('^'), "{rendered}");
    assert!(!rendered.contains("PackageGraphDiagnostic"), "{rendered}");
}

#[test]
fn use_after_move_points_at_use_and_move_origin() {
    let project = Project::new(
        "moved",
        &[(
            "src/main.neu",
            "class Box(var v: Int) {\n}\nfunc take(b: Box): Int { return b.v; }\npublic func main(): Int {\n    val b: Box = new Box(3);\n    val x: Int = take(b);\n    return take(b) + x;\n}\n",
        )],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(rendered.contains("error[use_after_move]"), "{rendered}");
    assert!(rendered.contains("src/main.neu:7:17"), "{rendered}");
    assert!(rendered.contains("note: value moved here"), "{rendered}");
    assert!(rendered.contains("src/main.neu:6:23"), "{rendered}");
}

#[test]
fn return_type_mismatch_reports_location() {
    let project = Project::new(
        "return",
        &[(
            "src/main.neu",
            "public func main(): Int { return \"hi\"; }\n",
        )],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(
        rendered.contains("error[return_type_mismatch]"),
        "{rendered}"
    );
    assert!(rendered.contains("src/main.neu:1:34"), "{rendered}");
}

#[test]
fn errors_in_imported_files_report_the_imported_path() {
    let project = Project::new(
        "imported",
        &[
            (
                "src/main.neu",
                "import \"./math\" as arithmetic\n\npublic func main(): Int {\n    return arithmetic.add()\n}\n",
            ),
            (
                "src/math/add.neu",
                "package math\n\npublic func add(): Int {\n    return \"seven\"\n}\n",
            ),
        ],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(rendered.contains("src/math/add.neu:4:12"), "{rendered}");
    assert!(rendered.contains("return \"seven\""), "{rendered}");
}

#[test]
fn identical_diagnostics_are_reported_once() {
    let project = Project::new(
        "dedupe",
        &[(
            "src/main.neu",
            "class Cell(var n: Int) {\n}\nfunc bump(c: Cell): Int { c.n = c.n + 1; return c.n; }\npublic suspend func main(): Int {\n    val cell: Cell = new Cell(0);\n    scope {\n        val a = spawn { -> bump(cell) };\n        val c = spawn { -> bump(cell) };\n    }\n    return 0;\n}\n",
        )],
    );

    let rendered = project.rendered().replace('\\', "/");
    let headers = rendered
        .lines()
        .filter(|line| line.starts_with("error["))
        .zip(rendered.lines().filter(|line| line.contains("--> ")))
        .collect::<Vec<_>>();

    assert!(!headers.is_empty(), "{rendered}");
    for (index, header) in headers.iter().enumerate() {
        assert!(
            !headers[index + 1..].contains(header),
            "duplicate diagnostic {header:?}:\n{rendered}"
        );
    }
}

#[test]
fn non_source_errors_render_without_debug_syntax() {
    let project = Project::new("missing", &[]);
    let error = DriverError::Io {
        operation: "read",
        path: Path::new("src/main.neu").to_path_buf(),
    };

    let rendered = render_manifest_error(project.manifest(), &error);

    assert!(rendered.starts_with("error"), "{rendered}");
    assert!(rendered.contains("src/main.neu"), "{rendered}");
    assert!(!rendered.contains("Io {"), "{rendered}");
}

#[test]
fn named_import_of_a_missing_name_points_at_the_import_with_a_message() {
    let project = Project::new(
        "missing_named_import",
        &[
            (
                "src/main.neu",
                "import {missing} from \"./math\"\n\npublic func main(): Int {\n    return 1\n}\n",
            ),
            (
                "src/math/add.neu",
                "package math\n\npublic func add(): Int {\n    return 7\n}\n",
            ),
        ],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(rendered.contains("src/main.neu:1:"), "{rendered}");
    assert!(
        rendered.contains("`missing` is not a public top-level declaration of `./math`"),
        "{rendered}"
    );
}

#[test]
fn private_member_through_an_alias_points_at_the_member_with_a_message() {
    let project = Project::new(
        "private_member",
        &[
            (
                "src/main.neu",
                "import \"./math\" as arithmetic\n\npublic func main(): Int {\n    return arithmetic.hidden()\n}\n",
            ),
            (
                "src/math/add.neu",
                "package math\n\nprivate func hidden(): Int {\n    return 7\n}\n",
            ),
        ],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(rendered.contains("src/main.neu:4:"), "{rendered}");
    assert!(
        rendered.contains("`hidden` is not public in `arithmetic`"),
        "{rendered}"
    );
}

#[test]
fn named_import_collision_points_at_the_imported_name() {
    let project = Project::new(
        "named_import_collision",
        &[
            (
                "src/main.neu",
                "import {add} from \"./math\"\n\npublic func main(): Int {\n    return 1\n}\n\nfunc add(): Int {\n    return 0\n}\n",
            ),
            (
                "src/math/add.neu",
                "package math\n\npublic func add(): Int {\n    return 7\n}\n",
            ),
        ],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(rendered.contains("src/main.neu:1:9"), "{rendered}");
    assert!(
        rendered.contains("named import add collides with another name"),
        "{rendered}"
    );
}

#[test]
fn duplicate_named_import_across_statements_is_rejected() {
    let project = Project::new(
        "duplicate_named_import",
        &[
            (
                "src/main.neu",
                "import {add} from \"./math\"\nimport {add} from \"./math\"\n\npublic func main(): Int {\n    return 1\n}\n",
            ),
            (
                "src/math/add.neu",
                "package math\n\npublic func add(): Int {\n    return 7\n}\n",
            ),
        ],
    );

    let rendered = project.rendered();

    assert!(
        rendered.contains("`add` is imported more than once"),
        "{rendered}"
    );
}

#[test]
fn named_import_alias_is_rejected_at_the_as_keyword() {
    let project = Project::new(
        "named_import_alias",
        &[
            (
                "src/main.neu",
                "import {add as plus} from \"./math\"\n\npublic func main(): Int {\n    return 1\n}\n",
            ),
            (
                "src/math/add.neu",
                "package math\n\npublic func add(): Int {\n    return 7\n}\n",
            ),
        ],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(rendered.contains("error[named_import_alias]"), "{rendered}");
    assert!(rendered.contains("src/main.neu:1:13"), "{rendered}");
    assert!(!rendered.contains("malformed"), "{rendered}");
}

#[test]
fn qualified_access_without_a_qualified_import_points_at_the_package_name() {
    let project = Project::new(
        "qualifier_not_imported",
        &[
            (
                "src/main.neu",
                "import {add} from \"./math\"\n\npublic func main(): Int {\n    return math.add()\n}\n",
            ),
            (
                "src/math/add.neu",
                "package math\n\npublic func add(): Int {\n    return 7\n}\n",
            ),
        ],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(
        rendered.contains("error[qualifier_not_imported]"),
        "{rendered}"
    );
    assert!(rendered.contains("src/main.neu:4:12"), "{rendered}");
    assert!(
        rendered.contains("`math` is not imported as a qualifier in this file"),
        "{rendered}"
    );
}

#[test]
fn lambda_statement_body_is_rejected_with_a_clear_message() {
    let project = Project::new(
        "lambda_statement_body",
        &[(
            "src/main.neu",
            "public func main(): Int {\n    val f: () -> Int = { -> val y = 3; y + 4 };\n    return f();\n}\n",
        )],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(
        rendered.contains("error[lambda_body_not_single_expression]"),
        "{rendered}"
    );
    assert!(rendered.contains("src/main.neu:2:"), "{rendered}");
    assert!(!rendered.contains("missing_expression"), "{rendered}");
}

#[test]
fn unknown_name_points_at_the_name_with_a_message() {
    let project = Project::new(
        "unknown_name",
        &[(
            "src/main.neu",
            "public func main(): Int {\n    return missing;\n}\n",
        )],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(rendered.contains("error[unknown_name]"), "{rendered}");
    assert!(rendered.contains("src/main.neu:2:12"), "{rendered}");
    assert!(
        rendered.contains("`missing` is not declared in this program"),
        "{rendered}"
    );
    assert!(!rendered.contains("missing_type"), "{rendered}");
}

#[test]
fn qualifier_without_a_package_points_at_the_qualifier() {
    let project = Project::new(
        "unknown_qualifier",
        &[(
            "src/main.neu",
            "public func main(): Int {\n    return math.add();\n}\n",
        )],
    );

    let rendered = project.rendered().replace('\\', "/");

    assert!(rendered.contains("error[unknown_name]"), "{rendered}");
    assert!(rendered.contains("src/main.neu:2:12"), "{rendered}");
    assert!(
        rendered.contains("`math` is not declared in this program"),
        "{rendered}"
    );
    assert!(!rendered.contains("deferred"), "{rendered}");
}
