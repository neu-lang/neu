use std::{fs, path::PathBuf, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    parser::parse_source,
    source::SourceFileId,
};
use target_lexicon::Triple;

fn options(output: &std::path::Path) -> SourceDriverOptions {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    SourceDriverOptions::new(
        SourceFileId::from_raw(6400),
        compiler::module::ModuleName::parse("strings").unwrap(),
        compiler::module::PackageNamespace::root(),
        Triple::host(),
        repo_root.join("target-packs"),
        output,
    )
}

#[test]
fn parses_valid_string_literal_and_rejects_invalid_escape() {
    let valid = parse_source(
        SourceFileId::from_raw(6401),
        r#"func read(): String { return "hi\n"; }"#,
    );
    assert!(valid.lex_diagnostics.is_empty());
    assert!(valid.diagnostics.is_empty(), "{:?}", valid.diagnostics);
    assert_eq!(valid.string_literals[0].bytes, b"hi\n");

    let utf8 = parse_source(
        SourceFileId::from_raw(6403),
        r#"func read(): String { return "café"; }"#,
    );
    assert!(utf8.lex_diagnostics.is_empty());
    assert!(utf8.diagnostics.is_empty(), "{:?}", utf8.diagnostics);

    let invalid = compiler::lexer::lex(SourceFileId::from_raw(6402), r#""bad\q""#);
    assert!(
        invalid
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.kind
                == compiler::lexer::DiagnosticKind::InvalidStringEscape)
    );
}

#[test]
fn compiles_string_length_index_equality_concat_and_clone() {
    let workspace = std::env::temp_dir().join(format!("neu-string-driver-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public func main(): Int {
            val left: String = "ab";
            val right: String = clone(left) + "c";
            return right.length;
        }
    "#;
    let output = compile_source_to_executable(source, options(&executable)).unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(3));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn compiles_string_byte_index_and_utf8_byte_length() {
    let workspace = std::env::temp_dir().join(format!("neu-string-bytes-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public func main(): Int {
            val text: String = "é";
            val first: Byte = text[0];
            return text.length;
        }
    "#;
    let output = compile_source_to_executable(source, options(&executable)).unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(2));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn compiles_string_equality_inequality_and_empty_concat() {
    let workspace =
        std::env::temp_dir().join(format!("neu-string-equality-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public func main(): Int {
            val equal: Bool = "a" == "a";
            val unequal: Bool = "a" != "ab";
            val empty: String = "" + "";
            return empty.length + 7;
        }
    "#;
    let output = compile_source_to_executable(source, options(&executable)).unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn string_equality_results_cover_unequal_lengths_and_empty_values() {
    let workspace = std::env::temp_dir().join(format!(
        "neu-string-equality-results-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public func score(value: Bool): Int {
            if (value) { return 1; }
            return 0;
        }
        public func main(): Int {
            return score("a" == "a")
                + score("a" != "ab")
                + score("ab" != "a")
                + score("" == "")
                + 3;
        }
    "#;
    let output = compile_source_to_executable(source, options(&executable)).unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn string_parameter_return_and_clone_preserve_ownership() {
    let workspace =
        std::env::temp_dir().join(format!("neu-string-functions-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public func identity(value: String): String {
            return value;
        }
        public func main(): Int {
            val original: String = "ab";
            val copied: String = clone(original);
            val returned: String = identity(copied);
            return original.length + returned.length + 3;
        }
    "#;
    let output = compile_source_to_executable(source, options(&executable)).unwrap();
    assert_eq!(Command::new(output).status().unwrap().code(), Some(7));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn dynamic_string_index_traps() {
    let workspace =
        std::env::temp_dir().join(format!("neu-string-dynamic-index-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public func main(): Int {
            val text: String = "a";
            val index: Int = 1;
            val byte: Byte = text[index];
            return 7;
        }
    "#;
    let output = compile_source_to_executable(source, options(&executable)).unwrap();
    assert!(!Command::new(output).status().unwrap().success());
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn string_read_only_use_preserves_source_and_move_is_diagnosed() {
    let workspace = std::env::temp_dir().join(format!("neu-string-move-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public func consume(value: String): String {
            return value;
        }
        public func main(): Int {
            var text: String = "value";
            val consumed: String = consume(text);
            return text.length;
        }
    "#;
    let error = compile_source_to_executable(source, options(&executable)).unwrap_err();
    assert!(format!("{error:?}").contains("UseAfterMove"), "{error:?}");
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn statically_invalid_string_index_is_rejected() {
    let workspace = std::env::temp_dir().join(format!("neu-string-index-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public func main(): Int {
            val text: String = "a";
            val byte: Byte = text[2];
            return 0;
        }
    "#;
    let error = compile_source_to_executable(source, options(&executable)).unwrap_err();
    assert!(
        format!("{error:?}").contains("StringIndexOutOfBounds"),
        "{error:?}"
    );
    let _ = fs::remove_dir_all(workspace);
}
