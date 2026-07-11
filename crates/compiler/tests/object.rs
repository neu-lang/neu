use compiler::{
    backend::{emit_mir_function_to_object, emit_mir_function_to_object_with_entry_symbol},
    mir::{
        MirBasicBlock, MirBlockId, MirCleanupBoundary, MirFunction, MirFunctionId, MirInstruction,
        MirTerminator, MirValueId,
    },
    module::{FunctionSymbolIdentity, ModuleName, PackageNamespace},
    source::{ByteSpan, SourceFileId},
    types::{PrimitiveType, TypeArena, TypeRecord},
};
use object::{Object, ObjectSymbol};

#[test]
fn m0032_emits_host_object_for_int_return() {
    let file = SourceFileId::from_raw(500);
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
        ModuleName::parse("app").unwrap(),
        PackageNamespace::parse("demo").unwrap(),
        "main",
    ));

    let object_bytes = emit_mir_function_to_object(&function, &types).unwrap();
    let object_file = object::File::parse(object_bytes.as_slice()).unwrap();

    let symbol_names: Vec<_> = object_file
        .symbols()
        .filter_map(|symbol| symbol.name().ok())
        .collect();
    assert!(
        symbol_names
            .iter()
            .any(|name| name.contains("neu_fn_3_617070_4_64656d6f_4_6d61696e"))
    );
}

#[test]
fn m0032_object_emission_rejects_missing_function_identity() {
    let file = SourceFileId::from_raw(501);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(1),
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
    );

    assert_eq!(
        emit_mir_function_to_object(&function, &types),
        Err(compiler::backend::CraneliftLoweringError::MissingFunctionIdentity)
    );
}

#[test]
fn m0032_emits_canonical_language_entry_symbol() {
    let file = SourceFileId::from_raw(502);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(2),
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
        ModuleName::parse("app").unwrap(),
        PackageNamespace::parse("demo").unwrap(),
        "main",
    ))
    .with_entry(true);

    let object_bytes =
        emit_mir_function_to_object_with_entry_symbol(&function, &types, "neu_lang_main").unwrap();
    let object_file = object::File::parse(object_bytes.as_slice()).unwrap();
    assert!(object_file.symbols().any(|symbol| {
        symbol
            .name()
            .is_ok_and(|name| name.contains("neu_lang_main"))
    }));
    assert_eq!(
        emit_mir_function_to_object_with_entry_symbol(&function, &types, ""),
        Err(compiler::backend::CraneliftLoweringError::MissingLanguageEntrySymbol)
    );
}
