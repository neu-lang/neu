use compiler::{
    hir::{
        HirBinaryOperator, HirExpression, HirExpressionId, HirFunction, HirFunctionId, HirModule,
        HirReturn, HirSafetyFacts, HirUnaryOperator,
    },
    mir::{
        MirBasicBlock, MirBlockId, MirCleanupBoundary, MirFunction, MirFunctionId, MirInstruction,
        MirLocal, MirLocalId, MirModule, MirTerminator, MirValueId, lower_hir_to_mir,
    },
    module::ModuleName,
    source::{ByteSpan, SourceFileId},
    types::{PrimitiveType, TypeArena, TypeId, TypeRecord},
};

#[test]
fn m0032_hir_to_mir_preserves_function_symbol_identity() {
    let file = SourceFileId::from_raw(304);
    let span = ByteSpan::new(file, 0, 8).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let hir = HirModule::new(
        ModuleName::parse("app").unwrap(),
        vec![
            HirFunction::new(
                HirFunctionId::from_raw(0),
                ModuleName::parse("app").unwrap(),
                compiler::module::PackageNamespace::parse("demo").unwrap(),
                span,
                true,
                int,
                vec![],
                vec![],
                vec![HirExpression::int_literal(
                    HirExpressionId::from_raw(0),
                    span,
                    int,
                    0,
                )],
                vec![HirReturn::new(span, HirExpressionId::from_raw(0))],
                HirSafetyFacts::executable_subset_checked(),
                vec![],
            )
            .with_symbol_identity(compiler::module::FunctionSymbolIdentity::new(
                ModuleName::parse("app").unwrap(),
                compiler::module::PackageNamespace::parse("demo").unwrap(),
                "main",
            )),
        ],
    );

    let mir = lower_hir_to_mir(&hir, &types).unwrap();

    assert_eq!(mir.functions()[0].symbol_identity().unwrap().name(), "main");
}

#[test]
fn m0030_mir_model_preserves_ordered_source_mapped_runtime_facts() {
    let file = SourceFileId::from_raw(300);
    let span = ByteSpan::new(file, 0, 4).unwrap();
    let int = TypeId::from_raw(1);
    let function = MirFunction::new(
        MirFunctionId::from_raw(0),
        span,
        vec![(MirValueId::from_raw(0), int)],
        int,
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
    assert_eq!(module.functions()[0].return_type(), int);
    assert_eq!(module.functions()[0].blocks()[0].terminator().span(), span);
    assert!(module.functions()[0].cleanup_boundary().is_empty());
}

#[test]
fn m0030_hir_integer_function_lowers_to_ordered_mir_block() {
    let file = SourceFileId::from_raw(301);
    let span = ByteSpan::new(file, 0, 8).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let hir = HirModule::new(
        ModuleName::parse("app").unwrap(),
        vec![HirFunction::new(
            HirFunctionId::from_raw(0),
            ModuleName::parse("app").unwrap(),
            compiler::module::PackageNamespace::parse("app").unwrap(),
            span,
            false,
            int,
            vec![],
            vec![],
            vec![
                HirExpression::int_literal(HirExpressionId::from_raw(0), span, int, 1),
                HirExpression::int_literal(HirExpressionId::from_raw(1), span, int, 2),
                HirExpression::binary(
                    HirExpressionId::from_raw(2),
                    span,
                    int,
                    HirBinaryOperator::Plus,
                    HirExpressionId::from_raw(0),
                    HirExpressionId::from_raw(1),
                ),
            ],
            vec![HirReturn::new(span, HirExpressionId::from_raw(2))],
            HirSafetyFacts::executable_subset_checked(),
            vec![],
        )],
    );
    let mir = lower_hir_to_mir(&hir, &types).unwrap();
    assert_eq!(mir.functions()[0].blocks()[0].instructions().len(), 3);
    assert_eq!(mir.functions()[0].return_type(), int);
    assert_eq!(mir.functions()[0].blocks()[0].terminator().span(), span);
}

#[test]
fn m0030_hir_to_mir_requires_owning_type_arena() {
    let file = SourceFileId::from_raw(303);
    let span = ByteSpan::new(file, 0, 8).unwrap();
    let mut owning_types = TypeArena::new();
    let int = owning_types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let hir = HirModule::new(
        ModuleName::parse("app").unwrap(),
        vec![HirFunction::new(
            HirFunctionId::from_raw(0),
            ModuleName::parse("app").unwrap(),
            compiler::module::PackageNamespace::parse("app").unwrap(),
            span,
            false,
            int,
            vec![],
            vec![],
            vec![HirExpression::int_literal(
                HirExpressionId::from_raw(0),
                span,
                int,
                42,
            )],
            vec![HirReturn::new(span, HirExpressionId::from_raw(0))],
            HirSafetyFacts::executable_subset_checked(),
            vec![],
        )],
    );

    assert!(lower_hir_to_mir(&hir, &owning_types).is_ok());
    assert_eq!(
        lower_hir_to_mir(&hir, &TypeArena::new()),
        Err(compiler::mir::MirLoweringError::UnsupportedRuntimeType)
    );
    let mut foreign_types = TypeArena::new();
    foreign_types.insert(TypeRecord::primitive(PrimitiveType::Bool));
    assert_eq!(
        lower_hir_to_mir(&hir, &foreign_types),
        Err(compiler::mir::MirLoweringError::UnsupportedRuntimeType)
    );
}

#[test]
fn m0030_hir_unary_ints_lower_to_mir() {
    let file = SourceFileId::from_raw(304);
    let span = ByteSpan::new(file, 0, 8).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let hir = HirModule::new(
        ModuleName::parse("app").unwrap(),
        vec![HirFunction::new(
            HirFunctionId::from_raw(0),
            ModuleName::parse("app").unwrap(),
            compiler::module::PackageNamespace::parse("app").unwrap(),
            span,
            false,
            int,
            vec![],
            vec![],
            vec![
                HirExpression::int_literal(HirExpressionId::from_raw(0), span, int, 42),
                HirExpression::unary(
                    HirExpressionId::from_raw(1),
                    span,
                    int,
                    HirUnaryOperator::Plus,
                    HirExpressionId::from_raw(0),
                ),
                HirExpression::unary(
                    HirExpressionId::from_raw(2),
                    span,
                    int,
                    HirUnaryOperator::Minus,
                    HirExpressionId::from_raw(1),
                ),
                HirExpression::unary(
                    HirExpressionId::from_raw(3),
                    span,
                    int,
                    HirUnaryOperator::BitwiseNot,
                    HirExpressionId::from_raw(2),
                ),
            ],
            vec![HirReturn::new(span, HirExpressionId::from_raw(3))],
            HirSafetyFacts::executable_subset_checked(),
            vec![],
        )],
    );
    let mir = lower_hir_to_mir(&hir, &types).unwrap();
    assert_eq!(mir.functions()[0].blocks()[0].instructions().len(), 4);
}

#[test]
fn m0030_mir_function_preserves_declared_return_type() {
    let file = SourceFileId::from_raw(302);
    let span = ByteSpan::new(file, 0, 4).unwrap();
    let int = TypeId::from_raw(1);
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
                1,
                span,
            )],
            MirTerminator::return_value(MirValueId::from_raw(0), span),
        )],
        MirCleanupBoundary::empty(),
    );

    assert_eq!(function.return_type(), int);
}
