use std::{fs, path::PathBuf};

use compiler::{
    backend::emit_mir_function_to_object_with_entry_symbol,
    linker::LinkInvocation,
    mir::{
        MirBasicBlock, MirBlockId, MirCleanupBoundary, MirFunction, MirFunctionId, MirInstruction,
        MirTerminator, MirValueId,
    },
    module::{FunctionSymbolIdentity, ModuleName, PackageNamespace},
    source::{ByteSpan, SourceFileId},
    target_pack::TargetPack,
    types::{PrimitiveType, TypeArena, TypeRecord},
};
use target_lexicon::Triple;

fn smoke_function() -> (MirFunction, TypeArena) {
    let file = SourceFileId::from_raw(900);
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
        ModuleName::parse("examples").unwrap(),
        PackageNamespace::parse("current").unwrap(),
        "main",
    ))
    .with_entry(true);
    (function, types)
}

#[test]
fn links_and_runs_host_target_pack_smoke() {
    let root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target-packs/aarch64-apple-darwin");
    let manifest = fs::read_to_string(root.join("manifest.toml")).unwrap();
    let pack = TargetPack::resolve_toml(&root, &manifest, Triple::host()).unwrap();
    let (function, types) = smoke_function();
    let temp = std::env::temp_dir().join(format!("neu-host-pack-{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp);
    fs::create_dir_all(&temp).unwrap();
    let object = temp.join("program.o");
    let executable = temp.join("program");
    fs::write(
        &object,
        emit_mir_function_to_object_with_entry_symbol(
            &function,
            &types,
            pack.language_entry_symbol(),
        )
        .unwrap(),
    )
    .unwrap();

    let invocation = LinkInvocation::new(&pack, &object, &executable).unwrap();
    invocation.execute().unwrap();
    invocation
        .verify_main_result(17, pack.trap_exit_code())
        .unwrap();
    let _ = fs::remove_dir_all(temp);
}
