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
        r#"fun read(): String { return "hi\n"; }"#,
    );
    assert!(valid.lex_diagnostics.is_empty());
    assert!(valid.diagnostics.is_empty(), "{:?}", valid.diagnostics);

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
        public fun main(): Int {
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
fn string_read_only_use_preserves_source_and_move_is_diagnosed() {
    let workspace = std::env::temp_dir().join(format!("neu-string-move-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let source = r#"
        public fun main(): Int {
            var text: String = "value";
            var other: String = text;
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
        public fun main(): Int {
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
