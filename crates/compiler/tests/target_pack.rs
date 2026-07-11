use std::{fs, path::PathBuf};

use compiler::{
    backend::emit_mir_function_to_object_with_entry_symbol,
    mir::{
        MirBasicBlock, MirBlockId, MirCleanupBoundary, MirFunction, MirFunctionId, MirInstruction,
        MirTerminator, MirValueId,
    },
    module::{FunctionSymbolIdentity, ModuleName, PackageNamespace},
    source::{ByteSpan, SourceFileId},
    target_pack::{ArtifactKind, TargetPack, TargetPackError, TargetPackManifest},
    types::{PrimitiveType, TypeArena, TypeRecord},
};
use cranelift_codegen::{
    ir::{AbiParam, Function, InstBuilder, UserFuncName, types},
    settings,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

fn fixture_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("neu-target-pack-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("bin")).unwrap();
    fs::create_dir_all(root.join("runtime")).unwrap();
    root
}

fn manifest(linker: &str, startup_shim: &str) -> TargetPackManifest {
    TargetPackManifest::new(
        Triple::host(),
        "macho",
        "macho",
        linker,
        startup_shim,
        "_start",
        "neu_lang_main",
        1,
    )
    .unwrap()
}

fn startup_shim_fixture(symbol: &str) -> Vec<u8> {
    let file = SourceFileId::from_raw(700);
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
                0,
                span,
            )],
            MirTerminator::return_value(MirValueId::from_raw(0), span),
        )],
        MirCleanupBoundary::empty(),
    )
    .with_symbol_identity(FunctionSymbolIdentity::new(
        ModuleName::parse("runtime").unwrap(),
        PackageNamespace::parse("bootstrap").unwrap(),
        "start",
    ))
    .with_entry(true);
    emit_mir_function_to_object_with_entry_symbol(&function, &types, symbol).unwrap()
}

fn startup_shim_call_fixture() -> Vec<u8> {
    let triple = Triple::host();
    let isa_builder = cranelift_codegen::isa::lookup_by_name(&triple.to_string()).unwrap();
    let isa = isa_builder
        .finish(settings::Flags::new(settings::builder()))
        .unwrap();
    let mut module =
        ObjectModule::new(ObjectBuilder::new(isa, "startup", default_libcall_names()).unwrap());
    let mut signature = module.make_signature();
    signature.returns.push(AbiParam::new(types::I64));
    let start = module
        .declare_function("start", Linkage::Export, &signature)
        .unwrap();
    let language_main = module
        .declare_function("neu_lang_main", Linkage::Import, &signature)
        .unwrap();
    let mut context = cranelift_codegen::Context::new();
    context.func = Function::with_name_signature(UserFuncName::user(0, start.as_u32()), signature);
    let mut builder_context = FunctionBuilderContext::new();
    {
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let block = builder.create_block();
        builder.switch_to_block(block);
        let callee = module.declare_func_in_func(language_main, builder.func);
        let call = builder.ins().call(callee, &[]);
        let result = builder.inst_results(call)[0];
        builder.ins().return_(&[result]);
        builder.seal_block(block);
        builder.finalize();
    }
    module.define_function(start, &mut context).unwrap();
    module.finish().emit().unwrap()
}

#[test]
fn m0032_resolves_valid_target_pack() {
    let root = fixture_root("valid");
    let linker = root.join("bin/linker");
    let shim = root.join("runtime/startup.o");
    fs::write(&linker, b"linker").unwrap();
    fs::write(&shim, startup_shim_call_fixture()).unwrap();

    let pack = TargetPack::resolve(
        &root,
        manifest("bin/linker", "runtime/startup.o"),
        Triple::host(),
    )
    .unwrap();

    assert_eq!(pack.target(), Triple::host());
    assert_eq!(pack.linker_path(), linker.canonicalize().unwrap());
    assert_eq!(pack.startup_shim_path(), shim.canonicalize().unwrap());
    assert_eq!(pack.entry_symbol(), "_start");
    assert_eq!(pack.language_entry_symbol(), "neu_lang_main");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_rejects_target_mismatch() {
    let root = fixture_root("target-mismatch");
    fs::write(root.join("bin/linker"), b"linker").unwrap();
    fs::write(root.join("runtime/startup.o"), startup_shim_call_fixture()).unwrap();
    let other_target = "x86_64-unknown-linux-gnu".parse::<Triple>().unwrap();

    assert_eq!(
        TargetPack::resolve(
            &root,
            manifest("bin/linker", "runtime/startup.o"),
            other_target
        ),
        Err(TargetPackError::TargetMismatch)
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_rejects_unsafe_and_missing_artifacts() {
    let root = fixture_root("unsafe");
    fs::write(root.join("bin/linker"), b"linker").unwrap();
    fs::write(root.join("runtime/startup.o"), startup_shim_call_fixture()).unwrap();

    assert_eq!(
        TargetPack::resolve(
            &root,
            manifest("../outside", "runtime/startup.o"),
            Triple::host()
        ),
        Err(TargetPackError::TraversalArtifactPath(ArtifactKind::Linker))
    );
    assert_eq!(
        TargetPack::resolve(
            &root,
            manifest("/tmp/host-linker", "runtime/startup.o"),
            Triple::host(),
        ),
        Err(TargetPackError::AbsoluteArtifactPath(ArtifactKind::Linker))
    );
    assert_eq!(
        TargetPack::resolve(
            &root,
            manifest("bin/missing", "runtime/startup.o"),
            Triple::host()
        ),
        Err(TargetPackError::MissingArtifact(ArtifactKind::Linker))
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_rejects_invalid_manifest() {
    assert_eq!(
        TargetPackManifest::new(
            Triple::host(),
            "",
            "macho",
            "bin/linker",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        ),
        Err(TargetPackError::InvalidManifest)
    );
    assert_eq!(
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/linker",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            0,
        ),
        Err(TargetPackError::InvalidManifest)
    );
}

#[test]
fn m0032_loads_target_pack_manifest_from_toml() {
    let root = fixture_root("toml");
    fs::write(root.join("bin/linker"), b"linker").unwrap();
    fs::write(root.join("runtime/startup.o"), startup_shim_call_fixture()).unwrap();
    let manifest = format!(
        "[target]\ntriple = \"{}\"\nobject_format = \"macho\"\nexecutable_format = \"macho\"\n[linker]\npath = \"bin/linker\"\n[startup_shim]\npath = \"runtime/startup.o\"\n[entry]\nplatform_symbol = \"_start\"\nlanguage_symbol = \"neu_lang_main\"\ntrap_exit_code = 1\n",
        Triple::host()
    );

    let pack = TargetPack::resolve_toml(&root, &manifest, Triple::host()).unwrap();

    assert_eq!(pack.entry_symbol(), "_start");
    assert_eq!(pack.language_entry_symbol(), "neu_lang_main");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_rejects_malformed_target_pack_toml() {
    assert_eq!(
        TargetPackManifest::from_toml("[target]\ntriple = \"not a triple\"\n"),
        Err(TargetPackError::InvalidManifest)
    );
    assert_eq!(
        TargetPackManifest::from_toml(
            "[target]\ntriple = \"aarch64-apple-darwin\"\nobject_format = \"macho\"\n"
        ),
        Err(TargetPackError::InvalidManifest)
    );
}

#[test]
fn m0032_rejects_malformed_startup_shim_object() {
    let root = fixture_root("malformed-startup");
    fs::write(root.join("bin/linker"), b"linker").unwrap();
    fs::write(root.join("runtime/startup.o"), b"not an object").unwrap();

    assert_eq!(
        TargetPack::resolve(
            &root,
            manifest("bin/linker", "runtime/startup.o"),
            Triple::host()
        ),
        Err(TargetPackError::InvalidStartupShim)
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_rejects_startup_shim_format_mismatch() {
    let root = fixture_root("startup-format-mismatch");
    fs::write(root.join("bin/linker"), b"linker").unwrap();
    fs::write(root.join("runtime/startup.o"), startup_shim_call_fixture()).unwrap();
    let mismatch = TargetPackManifest::new(
        Triple::host(),
        "elf",
        "macho",
        "bin/linker",
        "runtime/startup.o",
        "_start",
        "neu_lang_main",
        1,
    )
    .unwrap();

    assert_eq!(
        TargetPack::resolve(&root, mismatch, Triple::host()),
        Err(TargetPackError::StartupShimFormatMismatch)
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_rejects_startup_shim_without_platform_entry() {
    let root = fixture_root("startup-entry-missing");
    fs::write(root.join("bin/linker"), b"linker").unwrap();
    fs::write(
        root.join("runtime/startup.o"),
        startup_shim_fixture("other_start"),
    )
    .unwrap();

    assert_eq!(
        TargetPack::resolve(
            &root,
            manifest("bin/linker", "runtime/startup.o"),
            Triple::host()
        ),
        Err(TargetPackError::MissingStartupEntrySymbol)
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_accepts_startup_shim_language_entry_relocation() {
    let root = fixture_root("startup-language-entry");
    fs::write(root.join("bin/linker"), b"linker").unwrap();
    fs::write(root.join("runtime/startup.o"), startup_shim_call_fixture()).unwrap();

    assert!(
        TargetPack::resolve(
            &root,
            manifest("bin/linker", "runtime/startup.o"),
            Triple::host()
        )
        .is_ok()
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_rejects_startup_shim_without_language_entry_relocation() {
    let root = fixture_root("startup-language-missing");
    fs::write(root.join("bin/linker"), b"linker").unwrap();
    fs::write(
        root.join("runtime/startup.o"),
        startup_shim_fixture("start"),
    )
    .unwrap();

    assert_eq!(
        TargetPack::resolve(
            &root,
            manifest("bin/linker", "runtime/startup.o"),
            Triple::host()
        ),
        Err(TargetPackError::MissingLanguageEntryRelocation)
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn m0032_rejects_startup_shim_language_entry_mismatch() {
    let root = fixture_root("startup-language-mismatch");
    fs::write(root.join("bin/linker"), b"linker").unwrap();
    fs::write(root.join("runtime/startup.o"), startup_shim_call_fixture()).unwrap();
    let mismatch = TargetPackManifest::new(
        Triple::host(),
        "macho",
        "macho",
        "bin/linker",
        "runtime/startup.o",
        "_start",
        "other_main",
        1,
    )
    .unwrap();

    assert_eq!(
        TargetPack::resolve(&root, mismatch, Triple::host()),
        Err(TargetPackError::MissingLanguageEntryRelocation)
    );
    let _ = fs::remove_dir_all(root);
}
