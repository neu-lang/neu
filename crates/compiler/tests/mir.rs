use compiler::{
    mir::{
        MirBasicBlock, MirBlockId, MirCleanupBoundary, MirFunction, MirFunctionId, MirInstruction,
        MirLocal, MirLocalId, MirModule, MirTerminator, MirValueId,
    },
    module::ModuleName,
    source::{ByteSpan, SourceFileId},
    types::TypeId,
};

#[test]
fn m0030_mir_model_preserves_ordered_source_mapped_runtime_facts() {
    let file = SourceFileId::from_raw(300);
    let span = ByteSpan::new(file, 0, 4).unwrap();
    let int = TypeId::from_raw(1);
    let function = MirFunction::new(
        MirFunctionId::from_raw(0),
        span,
        vec![(MirValueId::from_raw(0), int)],
        vec![MirLocal::new(MirLocalId::from_raw(0), int, span)],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::int_constant(MirValueId::from_raw(1), 1, span),
                MirInstruction::checked_add(
                    MirValueId::from_raw(2),
                    MirValueId::from_raw(0),
                    MirValueId::from_raw(1),
                    span,
                ),
            ],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let module = MirModule::new(ModuleName::parse("app").unwrap(), vec![function]);

    assert_eq!(module.functions()[0].blocks()[0].instructions().len(), 2);
    assert_eq!(module.functions()[0].blocks()[0].terminator().span(), span);
    assert!(module.functions()[0].cleanup_boundary().is_empty());
}
