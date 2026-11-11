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
            "class Counter(var items: Int[]) {\n    func count(): Int {\n        return items.size();\n    }\n}\n\npublic func main(): Int {\n    var values: Int[] = new Int[];\n    values.add(4);\n    val c: Counter = new Counter(values);\n    return c.count();\n}\n",
        ),
        Some(1)
    );
}

#[test]
fn generic_class_with_an_array_field_stores_and_counts_elements() {
    assert_eq!(
        run_exit_code(
            "generic_storage",
            "class Stack<T>(var items: T[]) {\n    func push(value: T) {\n        items.add(value);\n    }\n    func len(): Int {\n        return items.size();\n    }\n}\n\npublic func main(): Int {\n    var values: Int[] = new Int[];\n    val s: Stack<Int> = new Stack<Int>(values);\n    s.push(4);\n    s.push(5);\n    return s.len();\n}\n",
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

#[test]
fn generic_class_field_read_returns_the_type_argument() {
    assert_eq!(
        run_exit_code(
            "field_read_int",
            "class Box<T>(var value: T) {\n}\n\npublic func main(): Int {\n    val b: Box<Int> = new Box<Int>(7);\n    return b.value;\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn generic_class_field_read_of_a_bool_argument_decodes_it() {
    assert_eq!(
        run_exit_code(
            "field_read_bool",
            "class Box<T>(var value: T) {\n}\n\npublic func main(): Int {\n    val b: Box<Bool> = new Box<Bool>(true);\n    if (b.value) {\n        return 7;\n    }\n    return 1;\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn field_assignment_through_a_generic_instance_writes_the_field() {
    assert_eq!(
        run_exit_code(
            "field_write_int",
            "class Box<T>(var value: T) {\n}\n\npublic func main(): Int {\n    val b: Box<Int> = new Box<Int>(1);\n    b.value = 9;\n    return b.value;\n}\n",
        ),
        Some(9)
    );
}

#[test]
fn field_assignment_through_a_generic_instance_encodes_a_bool() {
    assert_eq!(
        run_exit_code(
            "field_write_bool",
            "class Box<T>(var value: T) {\n}\n\npublic func main(): Int {\n    val b: Box<Bool> = new Box<Bool>(false);\n    b.value = true;\n    if (b.value) {\n        return 7;\n    }\n    return 1;\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn field_assignment_through_a_generic_instance_rejects_a_val_field() {
    let workspace =
        std::env::temp_dir().join(format!("neu-generic-class-readonly-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let error = compile_source_to_executable(
        "class Box<T>(val value: T) {\n}\n\npublic func main(): Int {\n    val b: Box<Int> = new Box<Int>(1);\n    b.value = 9;\n    return 0;\n}\n",
        SourceDriverOptions::new(
            SourceFileId::from_raw(81990),
            ModuleName::parse("generic.class").unwrap(),
            PackageNamespace::root(),
            workspace.join("program"),
        ),
    )
    .unwrap_err();
    assert!(format!("{error:?}").contains("ImmutableFieldMutation"));
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn field_assignment_through_a_generic_instance_inside_a_block_writes_the_field() {
    assert_eq!(
        run_exit_code(
            "field_write_in_block",
            "class Box<T>(var value: T) {\n}\n\npublic func main(): Int {\n    val b: Box<Int> = new Box<Int>(1);\n    if (b.value == 1) {\n        b.value = 9;\n    }\n    return b.value;\n}\n",
        ),
        Some(9)
    );
}

#[test]
fn class_implements_a_generic_interface_with_a_type_argument() {
    assert_eq!(
        run_exit_code(
            "generic_interface_direct",
            "interface Counter<T> {\n    func next(): T;\n}\n\nclass Source(var n: Int): Counter<Int> {\n    func next(): Int {\n        return n;\n    }\n}\n\npublic func main(): Int {\n    val s: Source = new Source(7);\n    return s.next();\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn generic_superclass_method_returns_the_type_argument() {
    assert_eq!(
        run_exit_code(
            "superclass_method",
            "class Base<T>(var value: T) {\n    func get(): T {\n        return value;\n    }\n}\n\nclass Child: Base<Int>(7) {\n}\n\npublic func main(): Int {\n    val c: Child = new Child();\n    return c.get();\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn generic_superclass_field_read_decodes_the_type_argument() {
    assert_eq!(
        run_exit_code(
            "superclass_field",
            "class Base<T>(var value: T) {\n}\n\nclass Child: Base<Bool>(true) {\n}\n\npublic func main(): Int {\n    val c: Child = new Child();\n    if (c.value) {\n        return 7;\n    }\n    return 1;\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn generic_superclass_override_with_the_substituted_return_type_is_accepted() {
    assert_eq!(
        run_exit_code(
            "superclass_override",
            "class Base<T>(var value: T) {\n    func get(): T {\n        return value;\n    }\n}\n\nclass Child: Base<Int>(7) {\n    func get(): Int {\n        return 3;\n    }\n}\n\npublic func main(): Int {\n    val c: Child = new Child();\n    return c.get();\n}\n",
        ),
        Some(3)
    );
}

#[test]
fn generic_superclass_bool_result_is_decoded() {
    assert_eq!(
        run_exit_code(
            "superclass_bool_result",
            "class Base<T>(var value: T) {\n    func get(): T {\n        return value;\n    }\n}\n\nclass Child: Base<Bool>(true) {\n}\n\npublic func main(): Int {\n    val c: Child = new Child();\n    if (c.get()) {\n        return 7;\n    }\n    return 1;\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn generic_subclass_inherits_a_method_with_its_own_type_argument() {
    assert_eq!(
        run_exit_code(
            "generic_subclass",
            "class Base<T>(var value: T) {\n    func get(): T {\n        return value;\n    }\n}\n\nclass Child<U>(var extra: U): Base<U>(extra) {\n}\n\npublic func main(): Int {\n    val c: Child<Int> = new Child<Int>(7);\n    return c.get();\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn superclass_type_argument_mismatch_is_rejected() {
    let workspace =
        std::env::temp_dir().join(format!("neu-superclass-mismatch-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let error = compile_source_to_executable(
        "class Base<T>(var value: T) {} class Child: Base<Int>(true) {} public func main(): Int { return 7; }",
        SourceDriverOptions::new(
            SourceFileId::from_raw(81995),
            ModuleName::parse("generic.class").unwrap(),
            PackageNamespace::root(),
            workspace.join("program"),
        ),
    )
    .unwrap_err();
    assert!(format!("{error:?}").contains("TypeMismatch"), "{error:?}");
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn constructor_argument_forwards_through_a_three_level_chain() {
    assert_eq!(
        run_exit_code(
            "chain_forward",
            "class A<T>(var value: T) {\n}\n\nclass B<U>(var extra: U): A<U>(extra) {\n}\n\nclass C: B<Int>(7) {\n}\n\npublic func main(): Int {\n    val c: C = new C();\n    return c.value;\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn bool_argument_forwards_through_a_chain_and_is_decoded() {
    assert_eq!(
        run_exit_code(
            "chain_bool",
            "class A<T>(var value: T) {\n}\n\nclass B<U>(var extra: U): A<U>(extra) {\n}\n\nclass C(var flag: Bool): B<Bool>(flag) {\n}\n\npublic func main(): Int {\n    val c: C = new C(true);\n    if (c.value) {\n        return 7;\n    }\n    return 1;\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn bool_literal_through_a_chain_is_decoded() {
    assert_eq!(
        run_exit_code(
            "chain_bool_literal",
            "class A<T>(var value: T) {\n}\n\nclass B<U>(var extra: U): A<U>(extra) {\n}\n\nclass C: B<Bool>(true) {\n}\n\npublic func main(): Int {\n    val c: C = new C();\n    if (c.value) {\n        return 7;\n    }\n    return 1;\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn nested_generic_argument_in_a_new_expression_and_method() {
    assert_eq!(
        run_exit_code(
            "nested_new",
            "class Box<T>(var value: T) {\n    func get(): T {\n        return value;\n    }\n}\n\npublic func main(): Int {\n    var items: Int[] = new Int[];\n    items.add(7);\n    val boxed: Box<Int[]> = new Box<Int[]>(items);\n    val inner: Int[] = boxed.get();\n    return inner[0];\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn nested_generic_superclass_argument_forwards_a_nested_value() {
    assert_eq!(
        run_exit_code(
            "nested_superclass",
            "class Base<T>(var value: T) {\n    func get(): T {\n        return value;\n    }\n}\n\nclass Child(var items: Int[]): Base<Int[]>(items) {\n}\n\npublic func main(): Int {\n    var items: Int[] = new Int[];\n    items.add(7);\n    val c: Child = new Child(items);\n    val inner: Int[] = c.get();\n    return inner[0];\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn box_of_box_reads_the_inner_value() {
    assert_eq!(
        run_exit_code(
            "box_of_box",
            "class Box<T>(var value: T) {\n    func get(): T {\n        return value;\n    }\n}\n\npublic func main(): Int {\n    val boxed: Box<Box<Int>> = new Box<Box<Int>>(new Box<Int>(7));\n    val inner: Box<Int> = boxed.get();\n    return inner.get();\n}\n",
        ),
        Some(7)
    );
}

#[test]
fn nested_generic_annotation_must_match_its_value() {
    let workspace =
        std::env::temp_dir().join(format!("neu-nested-mismatch-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let error = compile_source_to_executable(
        "class Box<T>(var value: T) {} public func main(): Int { val b: Box<Int[]> = new Box<Int>(3); return 7; }",
        SourceDriverOptions::new(
            SourceFileId::from_raw(81996),
            ModuleName::parse("generic.class").unwrap(),
            PackageNamespace::root(),
            workspace.join("program"),
        ),
    )
    .unwrap_err();
    assert!(format!("{error:?}").contains("TypeMismatch"), "{error:?}");
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn nested_generic_arguments_are_invariant() {
    let workspace =
        std::env::temp_dir().join(format!("neu-nested-invariant-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let error = compile_source_to_executable(
        "class Box<T>(var value: T) {} public func main(): Int { var bools: Bool[] = new Bool[]; val b: Box<Int[]> = new Box<Bool[]>(bools); return 7; }",
        SourceDriverOptions::new(
            SourceFileId::from_raw(81997),
            ModuleName::parse("generic.class").unwrap(),
            PackageNamespace::root(),
            workspace.join("program"),
        ),
    )
    .unwrap_err();
    assert!(format!("{error:?}").contains("TypeMismatch"), "{error:?}");
    let _ = fs::remove_dir_all(workspace);
}
