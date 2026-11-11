//! Generic classes (ADR-0083) and Array fields inside classes (ADR-0126): the
//! compiler support that a storage-embedding `Vector<T>` needs.

use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    module::{ModuleName, PackageNamespace},
    source::SourceFileId,
};

fn run_exit_code(name: &str, source: &str) -> Option<i32> {
    let workspace =
        std::env::temp_dir().join(format!("neu-generic-class-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(81000 + name.len()),
            ModuleName::parse("generic.class").unwrap(),
            PackageNamespace::root(),
            workspace.join("program"),
        ),
    )
    .expect("program should compile");
    let code = Command::new(executable).status().unwrap().code();
    let _ = fs::remove_dir_all(workspace);
    code
}

#[test]
fn explicit_type_arguments_construct_a_generic_class() {
    assert_eq!(
        run_exit_code(
            "construct",
            "class Box<T>(var value: T) {\n    func get(): T {\n        return value;\n    }\n}\n\npublic func main(): Int {\n    val b: Box<Int> = new Box<Int>(7);\n    return b.get();\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn method_on_generic_instance_takes_and_returns_the_type_argument() {
    assert_eq!(
        run_exit_code(
            "method",
            "class Cell<T>(var value: T) {\n    func put(next: T) {\n        this.value = next;\n    }\n    func get(): T {\n        return value;\n    }\n}\n\npublic func main(): Int {\n    val c: Cell<Int> = new Cell<Int>(1);\n    c.put(9);\n    return c.get();\n}\n",
        ),
        Some(9)
    );
}

#[test]
fn class_method_calls_an_array_field() {
    assert_eq!(
        run_exit_code(
            "array_field",
            "class Counter(var items: Array<Int>) {\n    func count(): Int {\n        return items.size();\n    }\n}\n\npublic func main(): Int {\n    var values: Array<Int> = new Int[];\n    values.add(4);\n    val c: Counter = new Counter(values);\n    return c.count();\n}\n",
        ),
        Some(1)
    );
}

#[test]
fn generic_class_with_an_array_field_stores_and_counts_elements() {
    assert_eq!(
        run_exit_code(
            "generic_storage",
            "class Stack<T>(var items: Array<T>) {\n    func push(value: T) {\n        items.add(value);\n    }\n    func len(): Int {\n        return items.size();\n    }\n}\n\npublic func main(): Int {\n    var values: Array<Int> = new Int[];\n    val s: Stack<Int> = new Stack<Int>(values);\n    s.push(4);\n    s.push(5);\n    return s.len();\n}\n",
        ),
        Some(2)
    );
}

#[test]
fn generic_class_holds_a_bool_argument() {
    assert_eq!(
        run_exit_code(
            "bool_argument",
            "class Box<T>(var value: T) {\n    func get(): T {\n        return value;\n    }\n}\n\npublic func main(): Int {\n    val b: Box<Bool> = new Box<Bool>(true);\n    if (b.get()) {\n        return 7;\n    }\n    return 1;\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn generic_class_holds_a_float_argument() {
    assert_eq!(
        run_exit_code(
            "float_argument",
            "class Box<T>(var value: T) {\n    func get(): T {\n        return value;\n    }\n}\n\npublic func main(): Int {\n    val b: Box<Float> = new Box<Float>(2.5);\n    val f: Float = b.get();\n    if (f > 2.0) {\n        return 7;\n    }\n    return 1;\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn generic_class_holds_a_class_argument() {
    assert_eq!(
        run_exit_code(
            "class_argument",
            "class Cell(var n: Int) {\n}\n\nclass Box<T>(var value: T) {\n    func get(): T {\n        return value;\n    }\n}\n\npublic func main(): Int {\n    val c: Cell = new Cell(7);\n    val b: Box<Cell> = new Box<Cell>(c);\n    return b.get().n;\n}\n",
        ),
        Some(7)
    );
}
