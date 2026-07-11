use std::{fs, path::PathBuf};

use compiler::{
    backend::emit_mir_function_to_object_for_target,
    driver::{SourceDriverOptions, compile_source_to_executable},
    linker::LinkInvocation,
    mir::{
        MirBasicBlock, MirBlockId, MirCleanupBoundary, MirFunction, MirFunctionId, MirInstruction,
        MirTerminator, MirValueId,
    },
    module::{FunctionSymbolIdentity, ModuleName, PackageNamespace},
    source::{ByteSpan, SourceFileId},
    target_pack::TargetPackRegistry,
    types::{PrimitiveType, TypeArena, TypeRecord},
};
use object::{Object, ObjectSymbol};
use target_lexicon::Triple;

fn target_pack_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target-packs")
}

fn bootstrap_function() -> (MirFunction, TypeArena) {
    let file = SourceFileId::from_raw(800);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(0),
        span,
        vec![],
        int,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![MirInstruction::int_constant(
                MirValueId::from_raw(0),
                17,
                span,
            )],
            MirTerminator::return_value(MirValueId::from_raw(0), span),
        )],
        MirCleanupBoundary::empty(),
    )
    .with_symbol_identity(FunctionSymbolIdentity::new(
        ModuleName::parse("examples.current").unwrap(),
        PackageNamespace::parse("examples").unwrap(),
        "main",
    ))
    .with_entry(true);
    (function, types)
}

#[test]
fn links_the_explicit_x86_64_target_pack_without_foreign_execution() {
    let target = "x86_64-unknown-linux-gnu".parse::<Triple>().unwrap();
    let registry = TargetPackRegistry::new(target_pack_root());
    let pack = registry.resolve(target.clone()).expect("x86 target pack");
    let (function, types) = bootstrap_function();
    let object_bytes = emit_mir_function_to_object_for_target(
        &function,
        &types,
        pack.language_entry_symbol(),
        target,
    )
    .unwrap();
    let object_file = object::File::parse(object_bytes.as_slice()).unwrap();
    assert_eq!(object_file.format(), object::BinaryFormat::Elf);
    assert_eq!(object_file.architecture(), object::Architecture::X86_64);

    let root = std::env::temp_dir().join(format!("neu-cross-target-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let object_path = root.join("program.o");
    let output_path = root.join("program");
    fs::write(&object_path, object_bytes).unwrap();
    let invocation = LinkInvocation::new(&pack, &object_path, &output_path).unwrap();
    invocation.execute().unwrap();

    let executable_bytes = fs::read(&output_path).unwrap();
    let executable = object::File::parse(executable_bytes.as_slice()).unwrap();
    assert_eq!(executable.format(), object::BinaryFormat::Elf);
    assert!(
        executable
            .symbols()
            .any(|symbol| { symbol.name().is_ok_and(|name| name == pack.entry_symbol()) })
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn links_string_object_through_the_x86_target_pack_without_execution() {
    let target = "x86_64-unknown-linux-gnu".parse::<Triple>().unwrap();
    let root = std::env::temp_dir().join(format!("neu-cross-string-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let output = root.join("program");
    let source = r#"
        public fun main(): Int {
            val text: String = "é" + "";
            return text.length;
        }
    "#;
    let executable = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(801),
            ModuleName::parse("strings").unwrap(),
            PackageNamespace::root(),
            target,
            target_pack_root(),
            &output,
        ),
    )
    .unwrap();
    let executable_bytes = fs::read(executable).unwrap();
    let executable = object::File::parse(executable_bytes.as_slice()).unwrap();
    assert_eq!(executable.format(), object::BinaryFormat::Elf);
    let _ = fs::remove_dir_all(root);
}

#[test]
fn links_class_object_through_the_x86_target_pack_without_execution() {
    let target = "x86_64-unknown-linux-gnu".parse::<Triple>().unwrap();
    let root = std::env::temp_dir().join(format!("neu-cross-class-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let output = root.join("program");
    let executable = compile_source_to_executable(
        "class Point(var value: Int) {} public fun main(): Int { val point: Point = new Point(7); return point.value; }",
        SourceDriverOptions::new(
            SourceFileId::from_raw(802),
            ModuleName::parse("classes").unwrap(),
            PackageNamespace::root(),
            target,
            target_pack_root(),
            &output,
        ),
    )
    .unwrap();
    let executable_bytes = fs::read(executable).unwrap();
    let executable = object::File::parse(executable_bytes.as_slice()).unwrap();
    assert_eq!(executable.format(), object::BinaryFormat::Elf);
    let _ = fs::remove_dir_all(root);
}
