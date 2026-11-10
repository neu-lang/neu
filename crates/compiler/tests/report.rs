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

    let rendered = project.rendered();

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

    let rendered = project.rendered();

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

    let rendered = project.rendered();

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

    let rendered = project.rendered();

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

    let rendered = project.rendered();
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
