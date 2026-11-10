use compiler::module::{FunctionSymbolIdentity, ModuleName, PackageNamespace};
use compiler::{
    backend::emit_mir_function_to_object_for_target,
    mir::{
        MirBasicBlock, MirBlockId, MirCleanupBoundary, MirFunction, MirFunctionId, MirInstruction,
        MirTerminator, MirValueId,
    },
    source::{ByteSpan, SourceFileId},
    types::{PrimitiveType, TypeArena, TypeRecord},
};
use object::{Object, ObjectSymbol};
use target_lexicon::Triple;

fn int_function() -> (MirFunction, TypeArena) {
    let file = SourceFileId::from_raw(1300);
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
                7,
                span,
            )],
            MirTerminator::return_value(MirValueId::from_raw(0), span),
        )],
        MirCleanupBoundary::empty(),
    )
    .with_symbol_identity(FunctionSymbolIdentity::new(
        ModuleName::parse("targets").unwrap(),
        PackageNamespace::root(),
        "answer",
    ));
    (function, types)
}

/// Symbol names an object defines or imports, for checking which runtime pieces
/// were emitted for a target.
fn symbol_names(bytes: &[u8]) -> Vec<String> {
    let file = object::File::parse(bytes).expect("object parses");
    file.symbols()
        .filter_map(|symbol| symbol.name().ok().map(str::to_owned))
        .collect()
}

fn has(names: &[String], symbol: &str) -> bool {
    names
        .iter()
        .any(|name| name.trim_start_matches('_') == symbol)
}

#[test]
fn scheduler_uses_ucontext_on_unix_and_fibers_on_windows() {
    let (function, types) = int_function();
    for (triple, expects_ucontext) in [
        ("x86_64-unknown-linux-gnu", true),
        ("aarch64-unknown-linux-gnu", true),
        ("x86_64-apple-darwin", true),
        ("aarch64-apple-darwin", true),
        ("x86_64-pc-windows-gnu", false),
        ("x86_64-pc-windows-msvc", false),
    ] {
        let target: Triple = triple.parse().unwrap();
        let bytes = emit_mir_function_to_object_for_target(&function, &types, "main", target)
            .unwrap_or_else(|error| panic!("{triple} failed to emit: {error:?}"));
        let names = symbol_names(&bytes);

        assert!(has(&names, "neu_sched_current"), "{triple}: {names:?}");
        assert!(has(&names, "neu_sched_spawn"), "{triple}: {names:?}");
        if expects_ucontext {
            assert!(has(&names, "swapcontext"), "{triple}: {names:?}");
            assert!(has(&names, "makecontext"), "{triple}: {names:?}");
            assert!(!has(&names, "CreateFiber"), "{triple}: {names:?}");
        } else {
            assert!(has(&names, "CreateFiber"), "{triple}: {names:?}");
            assert!(has(&names, "SwitchToFiber"), "{triple}: {names:?}");
            assert!(!has(&names, "swapcontext"), "{triple}: {names:?}");
        }
    }
}
