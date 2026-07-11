use compiler::{
    ast::AstNodeId,
    hir::{
        HirBinaryOperator, HirExpression, HirExpressionId, HirFunction, HirFunctionId, HirLocal,
        HirModule, HirParameter, HirReturn, HirSafetyFacts, HirUnaryOperator,
    },
    mir::{
        MirBasicBlock, MirBlockId, MirCleanupBoundary, MirFunction, MirFunctionId, MirInstruction,
        MirLocal, MirLocalId, MirModule, MirTerminator, MirValueId, lower_hir_to_mir,
    },
    module::ModuleName,
    ownership_effects::{EffectKind, OwnershipEffectContract, infer_parameter_effects},
    source::{ByteSpan, SourceFileId},
    types::{PrimitiveType, TypeArena, TypeId, TypeRecord},
};

#[test]
fn m0064_cleanup_boundary_tracks_owned_string_values() {
    let file = SourceFileId::from_raw(1064);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let string = types.insert(TypeRecord::primitive(PrimitiveType::String));
    let function = HirFunction::new(
        HirFunctionId::from_raw(0),
        ModuleName::parse("strings").unwrap(),
        compiler::module::PackageNamespace::root(),
        span,
        false,
        string,
        vec![HirParameter::new(
            compiler::hir::HirParameterId::from_raw(0),
            span,
            string,
        )],
        vec![HirLocal::new(
            compiler::hir::HirLocalId::from_raw(0),
            span,
            string,
            false,
        )],
        vec![],
        vec![],
        HirSafetyFacts::executable_subset_checked(),
        vec![],
    );
    let boundary = MirCleanupBoundary::for_function(&function, &types);
    assert_eq!(boundary.owned_locals(), &[MirLocalId::from_raw(0)]);
    assert_eq!(boundary.owned_parameters(), &[MirValueId::from_raw(0)]);
    assert!(boundary.returns_owned());
}

#[test]
fn m0062_hir_to_mir_preserves_ownership_effect_contract() {
    let file = SourceFileId::from_raw(1006);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let contract = OwnershipEffectContract::new(
        AstNodeId::from_raw(1007),
        infer_parameter_effects(1, &[(0, EffectKind::Read)]),
        true,
    );
    let hir_function = compiler::hir::HirFunction::new(
        compiler::hir::HirFunctionId::from_raw(0),
        ModuleName::parse("app").unwrap(),
        compiler::module::PackageNamespace::root(),
        span,
        false,
        int,
        vec![],
        vec![],
        vec![HirExpression::int_literal(
            compiler::hir::HirExpressionId::from_raw(0),
            span,
            int,
            1,
        )],
        vec![HirReturn::new(
            span,
            compiler::hir::HirExpressionId::from_raw(0),
        )],
        HirSafetyFacts::executable_subset_checked(),
        vec![],
    )
    .with_control_flow(vec![])
    .with_effect_contract(contract.clone());
    let mir = lower_hir_to_mir(
        &HirModule::new(ModuleName::parse("app").unwrap(), vec![hir_function]),
        &types,
    )
    .unwrap();

    assert_eq!(mir.functions()[0].effect_contract(), Some(&contract));
}

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
    assert!(mir.functions()[0].is_entry());
}

#[test]
fn m0032_hir_to_mir_preserves_entry_classification() {
    m0032_hir_to_mir_preserves_function_symbol_identity();
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
    assert!(!mir.functions()[0].is_entry());
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

#[test]
fn m0035_mir_preserves_non_integer_constants_and_unit_return() {
    let file = SourceFileId::from_raw(905);
    let span = ByteSpan::new(file, 0, 4).unwrap();
    let bool_output = MirValueId::from_raw(0);
    let float_output = MirValueId::from_raw(1);
    let byte_output = MirValueId::from_raw(2);

    let bool_constant = MirInstruction::bool_constant(bool_output, true, span);
    let float_constant = MirInstruction::float_constant(float_output, (-0.0f64).to_bits(), span);
    let byte_constant = MirInstruction::byte_constant(byte_output, 255, span);

    assert_eq!(
        bool_constant,
        MirInstruction::BoolConstant {
            output: bool_output,
            value: true,
            span
        }
    );
    assert_eq!(
        float_constant,
        MirInstruction::FloatConstant {
            output: float_output,
            bits: (-0.0f64).to_bits(),
            span,
        }
    );
    assert_eq!(
        byte_constant,
        MirInstruction::ByteConstant {
            output: byte_output,
            value: 255,
            span
        }
    );
    assert_eq!(
        MirTerminator::return_unit(span),
        MirTerminator::ReturnUnit { span }
    );
}

#[test]
fn m0035_hir_to_mir_preserves_boolean_not_and_comparison_operations() {
    let file = SourceFileId::from_raw(911);
    let span = ByteSpan::new(file, 0, 4).unwrap();
    let mut types = TypeArena::new();
    let bool_type = types.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let hir = HirModule::new(
        ModuleName::parse("app").unwrap(),
        vec![HirFunction::new(
            HirFunctionId::from_raw(0),
            ModuleName::parse("app").unwrap(),
            compiler::module::PackageNamespace::parse("app").unwrap(),
            span,
            false,
            bool_type,
            vec![],
            vec![],
            vec![
                HirExpression::bool_literal(HirExpressionId::from_raw(0), span, bool_type, true),
                HirExpression::unary(
                    HirExpressionId::from_raw(1),
                    span,
                    bool_type,
                    HirUnaryOperator::Not,
                    HirExpressionId::from_raw(0),
                ),
                HirExpression::binary(
                    HirExpressionId::from_raw(2),
                    span,
                    bool_type,
                    HirBinaryOperator::Equal,
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
    assert!(matches!(
        mir.functions()[0].blocks()[0].instructions()[1],
        MirInstruction::LogicalNot { .. }
    ));
    assert!(matches!(
        mir.functions()[0].blocks()[0].instructions()[2],
        MirInstruction::Compare {
            operation: compiler::mir::MirComparison::Equal,
            ..
        }
    ));
}

#[test]
fn m0035_hir_to_mir_lowers_logical_or_with_short_circuit_cfg() {
    let file = SourceFileId::from_raw(912);
    let span = ByteSpan::new(file, 0, 4).unwrap();
    let mut types = TypeArena::new();
    let bool_type = types.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let hir = HirModule::new(
        ModuleName::parse("app").unwrap(),
        vec![HirFunction::new(
            HirFunctionId::from_raw(0),
            ModuleName::parse("app").unwrap(),
            compiler::module::PackageNamespace::parse("app").unwrap(),
            span,
            false,
            bool_type,
            vec![],
            vec![],
            vec![
                HirExpression::bool_literal(HirExpressionId::from_raw(0), span, bool_type, true),
                HirExpression::bool_literal(HirExpressionId::from_raw(1), span, bool_type, false),
                HirExpression::binary(
                    HirExpressionId::from_raw(2),
                    span,
                    bool_type,
                    HirBinaryOperator::LogicalOr,
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
    assert_eq!(mir.functions()[0].blocks().len(), 4);
    assert!(matches!(
        mir.functions()[0].blocks()[0].terminator(),
        MirTerminator::BranchIf { .. }
    ));
}

#[test]
fn m0035_hir_to_mir_lowers_logical_and_with_short_circuit_cfg() {
    let file = SourceFileId::from_raw(921);
    let span = ByteSpan::new(file, 0, 6).unwrap();
    let mut types = TypeArena::new();
    let bool_type = types.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let hir = HirModule::new(
        ModuleName::parse("app").unwrap(),
        vec![HirFunction::new(
            HirFunctionId::from_raw(0),
            ModuleName::parse("app").unwrap(),
            compiler::module::PackageNamespace::parse("app").unwrap(),
            span,
            false,
            bool_type,
            vec![],
            vec![],
            vec![
                HirExpression::bool_literal(HirExpressionId::from_raw(0), span, bool_type, true),
                HirExpression::bool_literal(HirExpressionId::from_raw(1), span, bool_type, false),
                HirExpression::binary(
                    HirExpressionId::from_raw(2),
                    span,
                    bool_type,
                    HirBinaryOperator::LogicalAnd,
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
    assert_eq!(mir.functions()[0].blocks().len(), 4);
    assert!(matches!(
        mir.functions()[0].blocks()[0].terminator(),
        MirTerminator::BranchIf { .. }
    ));
    assert!(mir.functions()[0].blocks().iter().any(|block| {
        block
            .instructions()
            .iter()
            .any(|instruction| matches!(instruction, MirInstruction::StoreLocal { .. }))
    }));
}

#[test]
fn m0035_hir_to_mir_lowers_primitive_parameter_reads() {
    let file = SourceFileId::from_raw(925);
    let span = ByteSpan::new(file, 0, 6).unwrap();
    let mut types = TypeArena::new();
    let float_type = types.insert(TypeRecord::primitive(PrimitiveType::Float));
    let parameter = compiler::hir::HirParameter::new(
        compiler::hir::HirParameterId::from_raw(0),
        span,
        float_type,
    );
    let hir = HirModule::new(
        ModuleName::parse("app").unwrap(),
        vec![HirFunction::new(
            HirFunctionId::from_raw(0),
            ModuleName::parse("app").unwrap(),
            compiler::module::PackageNamespace::parse("app").unwrap(),
            span,
            false,
            float_type,
            vec![parameter],
            vec![],
            vec![HirExpression::parameter_read(
                HirExpressionId::from_raw(0),
                span,
                float_type,
                compiler::hir::HirParameterId::from_raw(0),
            )],
            vec![HirReturn::new(span, HirExpressionId::from_raw(0))],
            HirSafetyFacts::executable_subset_checked(),
            vec![],
        )],
    );

    let mir = lower_hir_to_mir(&hir, &types).unwrap();
    assert_eq!(mir.functions()[0].parameters().len(), 1);
    assert!(matches!(
        mir.functions()[0].blocks()[0].instructions()[0],
        MirInstruction::ParameterRead { .. }
    ));
}

#[test]
fn m0035_hir_to_mir_lowers_primitive_literals_and_unit_return() {
    let file = SourceFileId::from_raw(908);
    let span = ByteSpan::new(file, 0, 4).unwrap();
    let mut types = TypeArena::new();
    let bool_type = types.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let float_type = types.insert(TypeRecord::primitive(PrimitiveType::Float));
    let byte_type = types.insert(TypeRecord::primitive(PrimitiveType::Byte));
    let unit_type = types.insert(TypeRecord::primitive(PrimitiveType::Unit));
    let package = compiler::module::PackageNamespace::parse("demo").unwrap();
    let module_name = ModuleName::parse("app").unwrap();
    let function = |id, ty, expression| {
        HirFunction::new(
            HirFunctionId::from_raw(id),
            module_name.clone(),
            package.clone(),
            span,
            false,
            ty,
            vec![],
            vec![],
            vec![expression],
            vec![HirReturn::new(span, HirExpressionId::from_raw(0))],
            HirSafetyFacts::executable_subset_checked(),
            vec![],
        )
    };
    let hir = HirModule::new(
        module_name.clone(),
        vec![
            function(
                0,
                bool_type,
                HirExpression::bool_literal(HirExpressionId::from_raw(0), span, bool_type, true),
            ),
            function(
                1,
                float_type,
                HirExpression::float_literal(HirExpressionId::from_raw(0), span, float_type, 1.5),
            ),
            function(
                2,
                byte_type,
                HirExpression::byte_literal(HirExpressionId::from_raw(0), span, byte_type, 255),
            ),
            function(
                3,
                unit_type,
                HirExpression::unit_literal(HirExpressionId::from_raw(0), span, unit_type),
            ),
        ],
    );

    let mir = lower_hir_to_mir(&hir, &types).unwrap();
    assert!(matches!(
        mir.functions()[0].blocks()[0].instructions()[0],
        MirInstruction::BoolConstant { .. }
    ));
    assert!(matches!(
        mir.functions()[1].blocks()[0].instructions()[0],
        MirInstruction::FloatConstant { .. }
    ));
    assert!(matches!(
        mir.functions()[2].blocks()[0].instructions()[0],
        MirInstruction::ByteConstant { .. }
    ));
    assert!(matches!(
        mir.functions()[3].blocks()[0].instructions()[0],
        MirInstruction::UnitConstant { .. }
    ));
    assert_eq!(
        mir.functions()[3].blocks()[0].terminator(),
        MirTerminator::ReturnUnit { span }
    );
}
