//! ADR-0136: `Send` and `Share` declared on interfaces, and capabilities derived
//! for classes from their fields.
//!
//! Not written yet, because they cannot isolate their rule in the current compiler:
//! - bound tests (`func f<T: Send>` called as `f<Declared>(value)`): explicit generic
//!   function calls are not accepted yet, even with `Int`, so a rejection would come
//!   from that gap rather than the capability rule (ADR-0109);
//! - the recursive-cycle rule (ADR-0136 rule 8): a nullable class-typed field such as
//!   `Node?` is an unsupported form today.

use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    module::{ModuleName, PackageNamespace},
    source::SourceFileId,
};

/// Compiles and runs `source`. Returns the exit code, or the compile error text.
fn compile_and_run(name: &str, source: &str) -> Result<Option<i32>, String> {
    let workspace = std::env::temp_dir().join(format!(
        "neu-interface-capabilities-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let compiled = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(137_200),
            ModuleName::parse(name).unwrap(),
            PackageNamespace::root(),
            &executable,
        ),
    );
    let outcome = match compiled {
        Ok(_) => Ok(Command::new(&executable).status().unwrap().code()),
        Err(error) => Err(format!("{error:?}")),
    };
    let _ = fs::remove_dir_all(workspace);
    outcome
}

/// Asserts that `source` is rejected by a semantic rule. A parser rejection fails
/// here, because the rule under test would not have run.
fn assert_semantic_rejection(name: &str, source: &str) {
    match compile_and_run(name, source) {
        Ok(code) => panic!("expected a rejection, but the program ran with {code:?}"),
        Err(error) => assert!(
            !error.contains("ParseDiagnostics"),
            "rejected by the parser instead of the rule under test: {error}"
        ),
    }
}

#[test]
fn interface_declaring_send_and_share_accepts_a_fieldless_implementer() {
    // ADR-0136 syntax and rules 1 and 3. Fails today: the `&` conjunction in an
    // interface header is not parsed.
    let source = r#"
interface Sendable: Send & Share {
    func id(): Int;
}

class Plain: Sendable {
    func id(): Int {
        return 4;
    }
}

func main(): Int {
    val value: Sendable = new Plain();
    return value.id();
}
"#;
    assert_eq!(
        compile_and_run("capability.fieldless.implementer", source),
        Ok(Some(4))
    );
}

#[test]
fn class_with_a_string_field_cannot_implement_a_share_interface() {
    // ADR-0037: String satisfies Send but not Share, so the field fails the declared
    // Share requirement (ADR-0136 rules 1 and 3). Fails today: the class is accepted.
    let source = r#"
interface Shared: Share {
    func id(): Int;
}

class Labeled(val name: String): Shared {
    func id(): Int {
        return 1;
    }
}

func main(): Int {
    return 0;
}
"#;
    assert_semantic_rejection("capability.string.share", source);
}

#[test]
fn unknown_capability_name_on_an_interface_is_rejected() {
    // ADR-0136 syntax: only Send and Share are accepted names. Fails today: the
    // program is accepted.
    let source = r#"
interface Hashy: Hash {
    func id(): Int;
}

func main(): Int {
    return 0;
}
"#;
    assert_semantic_rejection("capability.unknown.name", source);
}

#[test]
fn class_field_of_an_undeclared_interface_type_cannot_implement_a_send_interface() {
    // ADR-0136 rule 5: an interface-typed field satisfies a capability only when that
    // interface declares it. Fails today: the class is accepted.
    let source = r#"
interface Undeclared {
    func id(): Int;
}

interface Sendable: Send {
    func id(): Int;
}

class Holder(val item: Undeclared): Sendable {
    func id(): Int {
        return 1;
    }
}

func main(): Int {
    return 0;
}
"#;
    assert_semantic_rejection("capability.field.undeclared", source);
}

#[test]
fn class_field_of_a_declared_interface_type_can_implement_a_send_interface() {
    // ADR-0136 rule 5, positive side. Passes today; it guards the accepted case.
    let source = r#"
interface Declared: Send {
    func id(): Int;
}

interface Sendable: Send {
    func id(): Int;
}

class Holder(val item: Declared): Sendable {
    func id(): Int {
        return 2;
    }
}

func main(): Int {
    return 0;
}
"#;
    assert_eq!(
        compile_and_run("capability.field.declared", source),
        Ok(Some(0))
    );
}
