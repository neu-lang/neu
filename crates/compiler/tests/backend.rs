use compiler::{
    backend::{
        CraneliftLoweringError, emit_mir_module_to_object, lower_mir_function_to_cranelift,
        lower_mir_module_to_cranelift,
    },
    hir::{CheckedHirSource, lower_checked_hir_source},
    mir::{
        MirArithmetic, MirBasicBlock, MirBlockId, MirCleanupBoundary, MirFunction, MirFunctionId,
        MirInstruction, MirLocalId, MirTerminator, MirUnary, MirValueId,
    },
    source::{ByteSpan, SourceFileId},
    type_check::{
        ExecutableSourceTypes, apply_m0028_direct_call_results, check_m0028_direct_calls,
        type_m0028_executable_core_in, type_m0028_function_signatures_in,
    },
    types::{PrimitiveType, TypeArena, TypeRecord},
};

#[test]
fn m0031_lowers_int_constant_return_to_verified_cranelift_ir() {
    let file = SourceFileId::from_raw(400);
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
                42,
                span,
            )],
            MirTerminator::return_value(MirValueId::from_raw(0), span),
        )],
        MirCleanupBoundary::empty(),
    );

    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();

    assert!(ir.contains("-> i64"), "{ir}");
    assert!(ir.contains("iconst.i64 42"));
    assert!(ir.contains("return"));
}

#[test]
fn m0035_lowers_bool_byte_float_and_unit_returns() {
    let file = SourceFileId::from_raw(906);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let bool_type = types.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let byte_type = types.insert(TypeRecord::primitive(PrimitiveType::Byte));
    let float_type = types.insert(TypeRecord::primitive(PrimitiveType::Float));
    let unit_type = types.insert(TypeRecord::primitive(PrimitiveType::Unit));

    let bool_function = MirFunction::new(
        MirFunctionId::from_raw(50),
        span,
        vec![],
        bool_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![MirInstruction::bool_constant(
                MirValueId::from_raw(0),
                true,
                span,
            )],
            MirTerminator::return_value(MirValueId::from_raw(0), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let bool_ir = lower_mir_function_to_cranelift(&bool_function, &types).unwrap();
    assert!(bool_ir.contains("-> i8"), "{bool_ir}");
    assert!(bool_ir.contains("iconst.i8 1"), "{bool_ir}");

    let byte_function = MirFunction::new(
        MirFunctionId::from_raw(51),
        span,
        vec![],
        byte_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![MirInstruction::byte_constant(
                MirValueId::from_raw(0),
                255,
                span,
            )],
            MirTerminator::return_value(MirValueId::from_raw(0), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let byte_ir = lower_mir_function_to_cranelift(&byte_function, &types).unwrap();
    assert!(byte_ir.contains("-> i8"), "{byte_ir}");
    assert!(byte_ir.contains("iconst.i8 -1"), "{byte_ir}");

    let float_function = MirFunction::new(
        MirFunctionId::from_raw(52),
        span,
        vec![],
        float_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![MirInstruction::float_constant(
                MirValueId::from_raw(0),
                1.5f64.to_bits(),
                span,
            )],
            MirTerminator::return_value(MirValueId::from_raw(0), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let float_ir = lower_mir_function_to_cranelift(&float_function, &types).unwrap();
    assert!(float_ir.contains("-> f64"), "{float_ir}");

    let unit_function = MirFunction::new(
        MirFunctionId::from_raw(53),
        span,
        vec![],
        unit_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![MirInstruction::unit_constant(span)],
            MirTerminator::return_unit(span),
        )],
        MirCleanupBoundary::empty(),
    );
    let unit_ir = lower_mir_function_to_cranelift(&unit_function, &types).unwrap();
    assert!(unit_ir.contains("return"), "{unit_ir}");
    assert!(!unit_ir.contains("->"), "{unit_ir}");
}

#[test]
fn m0031_rejects_unsupported_mir_instruction() {
    let file = SourceFileId::from_raw(401);
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
            vec![MirInstruction::DirectCall {
                output: MirValueId::from_raw(2),
                callee: MirFunctionId::from_raw(0),
                arguments: vec![],
                span,
            }],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );

    assert_eq!(
        lower_mir_function_to_cranelift(&function, &types),
        Err(CraneliftLoweringError::UnsupportedInstruction)
    );
}

#[test]
fn m0031_lowers_checked_addition_with_overflow_trap() {
    let file = SourceFileId::from_raw(402);
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
            vec![
                MirInstruction::int_constant(MirValueId::from_raw(0), 40, span),
                MirInstruction::int_constant(MirValueId::from_raw(1), 2, span),
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(2),
                    operation: MirArithmetic::Add,
                    left: MirValueId::from_raw(0),
                    right: MirValueId::from_raw(1),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );

    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();

    assert!(ir.contains("iadd"), "{ir}");
    assert!(ir.contains("int_ovf"), "{ir}");
}

#[test]
fn m0031_lowers_checked_subtraction_with_overflow_trap() {
    let file = SourceFileId::from_raw(403);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(3),
        span,
        vec![],
        int,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::int_constant(MirValueId::from_raw(0), 40, span),
                MirInstruction::int_constant(MirValueId::from_raw(1), 2, span),
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(2),
                    operation: MirArithmetic::Subtract,
                    left: MirValueId::from_raw(0),
                    right: MirValueId::from_raw(1),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );

    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();

    assert!(ir.contains("isub"), "{ir}");
    assert!(ir.contains("int_ovf"), "{ir}");
}

#[test]
fn m0031_lowers_checked_multiplication_with_overflow_trap() {
    let file = SourceFileId::from_raw(404);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(4),
        span,
        vec![],
        int,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::int_constant(MirValueId::from_raw(0), 6, span),
                MirInstruction::int_constant(MirValueId::from_raw(1), 7, span),
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(2),
                    operation: MirArithmetic::Multiply,
                    left: MirValueId::from_raw(0),
                    right: MirValueId::from_raw(1),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );

    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();
    assert!(ir.contains("imul"), "{ir}");
    assert!(ir.contains("smulhi"), "{ir}");
    assert!(ir.contains("int_ovf"), "{ir}");
}

#[test]
fn m0031_lowers_checked_division() {
    let file = SourceFileId::from_raw(405);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(5),
        span,
        vec![],
        int,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::int_constant(MirValueId::from_raw(0), 84, span),
                MirInstruction::int_constant(MirValueId::from_raw(1), 2, span),
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(2),
                    operation: MirArithmetic::Divide,
                    left: MirValueId::from_raw(0),
                    right: MirValueId::from_raw(1),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();
    assert!(ir.contains("sdiv"), "{ir}");
}

#[test]
fn m0031_lowers_checked_remainder() {
    let file = SourceFileId::from_raw(406);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(6),
        span,
        vec![],
        int,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::int_constant(MirValueId::from_raw(0), 85, span),
                MirInstruction::int_constant(MirValueId::from_raw(1), 2, span),
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(2),
                    operation: MirArithmetic::Remainder,
                    left: MirValueId::from_raw(0),
                    right: MirValueId::from_raw(1),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();
    assert!(ir.contains("srem"), "{ir}");
}

#[test]
fn m0031_lowers_bitwise_operations() {
    let file = SourceFileId::from_raw(407);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(7),
        span,
        vec![],
        int,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::int_constant(MirValueId::from_raw(0), 6, span),
                MirInstruction::int_constant(MirValueId::from_raw(1), 3, span),
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(2),
                    operation: MirArithmetic::BitwiseAnd,
                    left: MirValueId::from_raw(0),
                    right: MirValueId::from_raw(1),
                    span,
                },
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(3),
                    operation: MirArithmetic::BitwiseOr,
                    left: MirValueId::from_raw(2),
                    right: MirValueId::from_raw(1),
                    span,
                },
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(4),
                    operation: MirArithmetic::BitwiseXor,
                    left: MirValueId::from_raw(3),
                    right: MirValueId::from_raw(0),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(4), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();
    assert!(ir.contains("band"), "{ir}");
    assert!(ir.contains("bor"), "{ir}");
    assert!(ir.contains("bxor"), "{ir}");
}

#[test]
fn m0031_lowers_checked_shifts() {
    let file = SourceFileId::from_raw(408);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(8),
        span,
        vec![],
        int,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::int_constant(MirValueId::from_raw(0), 8, span),
                MirInstruction::int_constant(MirValueId::from_raw(1), 1, span),
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(2),
                    operation: MirArithmetic::ShiftLeft,
                    left: MirValueId::from_raw(0),
                    right: MirValueId::from_raw(1),
                    span,
                },
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(3),
                    operation: MirArithmetic::ShiftRight,
                    left: MirValueId::from_raw(2),
                    right: MirValueId::from_raw(1),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(3), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();
    assert!(ir.contains("ishl"), "{ir}");
    assert!(ir.contains("sshr"), "{ir}");
    assert!(ir.contains("trapnz"), "{ir}");
}

#[test]
fn m0031_lowers_unary_int_operations() {
    let file = SourceFileId::from_raw(409);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(9),
        span,
        vec![],
        int,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::int_constant(MirValueId::from_raw(0), 8, span),
                MirInstruction::Unary {
                    output: MirValueId::from_raw(1),
                    operation: MirUnary::Plus,
                    operand: MirValueId::from_raw(0),
                    span,
                },
                MirInstruction::Unary {
                    output: MirValueId::from_raw(2),
                    operation: MirUnary::Negate,
                    operand: MirValueId::from_raw(1),
                    span,
                },
                MirInstruction::Unary {
                    output: MirValueId::from_raw(3),
                    operation: MirUnary::BitwiseNot,
                    operand: MirValueId::from_raw(2),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(3), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();
    assert!(ir.contains("ineg"), "{ir}");
    assert!(ir.contains("bnot"), "{ir}");
    assert!(ir.contains("int_ovf"), "{ir}");
}

#[test]
fn m0035_lowers_float_byte_bool_and_comparison_operations() {
    let file = SourceFileId::from_raw(913);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let bool_type = types.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let float_type = types.insert(TypeRecord::primitive(PrimitiveType::Float));
    let byte_type = types.insert(TypeRecord::primitive(PrimitiveType::Byte));

    let float_function = MirFunction::new(
        MirFunctionId::from_raw(60),
        span,
        vec![],
        float_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::float_constant(MirValueId::from_raw(0), 1.5f64.to_bits(), span),
                MirInstruction::float_constant(MirValueId::from_raw(1), 2.0f64.to_bits(), span),
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(2),
                    operation: MirArithmetic::Multiply,
                    left: MirValueId::from_raw(0),
                    right: MirValueId::from_raw(1),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let float_ir = lower_mir_function_to_cranelift(&float_function, &types).unwrap();
    assert!(float_ir.contains("fmul"), "{float_ir}");
    assert!(!float_ir.contains("imul"), "{float_ir}");

    let byte_function = MirFunction::new(
        MirFunctionId::from_raw(61),
        span,
        vec![],
        byte_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::byte_constant(MirValueId::from_raw(0), 2, span),
                MirInstruction::byte_constant(MirValueId::from_raw(1), 3, span),
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(2),
                    operation: MirArithmetic::Multiply,
                    left: MirValueId::from_raw(0),
                    right: MirValueId::from_raw(1),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let byte_ir = lower_mir_function_to_cranelift(&byte_function, &types).unwrap();
    assert!(byte_ir.contains("imul"), "{byte_ir}");

    let bool_function = MirFunction::new(
        MirFunctionId::from_raw(62),
        span,
        vec![],
        bool_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::bool_constant(MirValueId::from_raw(0), true, span),
                MirInstruction::LogicalNot {
                    output: MirValueId::from_raw(1),
                    operand: MirValueId::from_raw(0),
                    span,
                },
                MirInstruction::Compare {
                    output: MirValueId::from_raw(2),
                    operation: compiler::mir::MirComparison::Equal,
                    left: MirValueId::from_raw(0),
                    right: MirValueId::from_raw(1),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let bool_ir = lower_mir_function_to_cranelift(&bool_function, &types).unwrap();
    assert!(bool_ir.contains("icmp"), "{bool_ir}");
    assert!(bool_ir.contains("select"), "{bool_ir}");
}

#[test]
fn m0035_lowers_conditional_mir_cfg_to_cranelift() {
    let file = SourceFileId::from_raw(920);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let bool_type = types.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let int_type = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(70),
        span,
        vec![],
        int_type,
        vec![],
        vec![
            MirBasicBlock::new(
                MirBlockId::from_raw(0),
                vec![MirInstruction::bool_constant(
                    MirValueId::from_raw(0),
                    true,
                    span,
                )],
                MirTerminator::branch_if(
                    MirValueId::from_raw(0),
                    MirBlockId::from_raw(1),
                    MirBlockId::from_raw(2),
                    span,
                ),
            ),
            MirBasicBlock::new(
                MirBlockId::from_raw(1),
                vec![MirInstruction::int_constant(
                    MirValueId::from_raw(1),
                    1,
                    span,
                )],
                MirTerminator::return_value(MirValueId::from_raw(1), span),
            ),
            MirBasicBlock::new(
                MirBlockId::from_raw(2),
                vec![MirInstruction::int_constant(
                    MirValueId::from_raw(2),
                    2,
                    span,
                )],
                MirTerminator::return_value(MirValueId::from_raw(2), span),
            ),
        ],
        MirCleanupBoundary::empty(),
    );

    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();
    assert!(ir.contains("brif"), "{ir}");
    assert!(ir.contains("block1"), "{ir}");
    assert!(ir.contains("block2"), "{ir}");
    let _ = bool_type;
}

#[test]
fn m0035_lowers_short_circuit_result_local_through_cfg() {
    let file = SourceFileId::from_raw(922);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let bool_type = types.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let local = MirLocalId::from_raw(0);
    let function = MirFunction::new(
        MirFunctionId::from_raw(71),
        span,
        vec![],
        bool_type,
        vec![compiler::mir::MirLocal::new(local, bool_type, span)],
        vec![
            MirBasicBlock::new(
                MirBlockId::from_raw(0),
                vec![MirInstruction::bool_constant(
                    MirValueId::from_raw(0),
                    true,
                    span,
                )],
                MirTerminator::branch_if(
                    MirValueId::from_raw(0),
                    MirBlockId::from_raw(1),
                    MirBlockId::from_raw(2),
                    span,
                ),
            ),
            MirBasicBlock::new(
                MirBlockId::from_raw(1),
                vec![
                    MirInstruction::bool_constant(MirValueId::from_raw(1), false, span),
                    MirInstruction::StoreLocal {
                        local,
                        value: MirValueId::from_raw(1),
                        span,
                    },
                ],
                MirTerminator::Branch {
                    target: MirBlockId::from_raw(3),
                    span,
                },
            ),
            MirBasicBlock::new(
                MirBlockId::from_raw(2),
                vec![
                    MirInstruction::bool_constant(MirValueId::from_raw(2), true, span),
                    MirInstruction::StoreLocal {
                        local,
                        value: MirValueId::from_raw(2),
                        span,
                    },
                ],
                MirTerminator::Branch {
                    target: MirBlockId::from_raw(3),
                    span,
                },
            ),
            MirBasicBlock::new(
                MirBlockId::from_raw(3),
                vec![MirInstruction::LoadLocal {
                    output: MirValueId::from_raw(3),
                    local,
                    span,
                }],
                MirTerminator::return_value(MirValueId::from_raw(3), span),
            ),
        ],
        MirCleanupBoundary::empty(),
    );

    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();
    assert!(ir.contains("brif"), "{ir}");
    assert!(ir.contains("return"), "{ir}");
}

#[test]
fn m0035_lowers_typed_primitive_parameters() {
    let file = SourceFileId::from_raw(923);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let bool_type = types.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let float_type = types.insert(TypeRecord::primitive(PrimitiveType::Float));
    let byte_type = types.insert(TypeRecord::primitive(PrimitiveType::Byte));

    let bool_function = MirFunction::new(
        MirFunctionId::from_raw(72),
        span,
        vec![(MirValueId::from_raw(0), bool_type)],
        bool_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![MirInstruction::ParameterRead {
                output: MirValueId::from_raw(3),
                parameter: compiler::mir::MirParameterId::from_raw(0),
                span,
            }],
            MirTerminator::return_value(MirValueId::from_raw(3), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let bool_ir = lower_mir_function_to_cranelift(&bool_function, &types).unwrap();
    assert!(bool_ir.contains("i8"), "{bool_ir}");
    assert!(bool_ir.contains("return"), "{bool_ir}");

    let float_function = MirFunction::new(
        MirFunctionId::from_raw(73),
        span,
        vec![(MirValueId::from_raw(1), float_type)],
        float_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![],
            MirTerminator::return_value(MirValueId::from_raw(1), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let float_ir = lower_mir_function_to_cranelift(&float_function, &types).unwrap();
    assert!(float_ir.contains("f64"), "{float_ir}");

    let byte_function = MirFunction::new(
        MirFunctionId::from_raw(74),
        span,
        vec![(MirValueId::from_raw(2), byte_type)],
        byte_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let byte_ir = lower_mir_function_to_cranelift(&byte_function, &types).unwrap();
    assert!(byte_ir.contains("i8"), "{byte_ir}");
}

#[test]
fn m0035_lowers_primitive_direct_call_through_module_context() {
    let file = SourceFileId::from_raw(927);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let bool_type = types.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let helper_identity = compiler::module::FunctionSymbolIdentity::new(
        compiler::module::ModuleName::parse("app").unwrap(),
        compiler::module::PackageNamespace::parse("demo").unwrap(),
        "helper",
    );
    let caller_identity = compiler::module::FunctionSymbolIdentity::new(
        compiler::module::ModuleName::parse("app").unwrap(),
        compiler::module::PackageNamespace::parse("demo").unwrap(),
        "caller",
    );
    let helper = MirFunction::new(
        MirFunctionId::from_raw(1),
        span,
        vec![],
        bool_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![MirInstruction::bool_constant(
                MirValueId::from_raw(0),
                true,
                span,
            )],
            MirTerminator::return_value(MirValueId::from_raw(0), span),
        )],
        MirCleanupBoundary::empty(),
    )
    .with_symbol_identity(helper_identity);
    let caller = MirFunction::new(
        MirFunctionId::from_raw(0),
        span,
        vec![],
        bool_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![MirInstruction::DirectCall {
                output: MirValueId::from_raw(0),
                callee: MirFunctionId::from_raw(1),
                arguments: vec![],
                span,
            }],
            MirTerminator::return_value(MirValueId::from_raw(0), span),
        )],
        MirCleanupBoundary::empty(),
    )
    .with_symbol_identity(caller_identity);
    let module = compiler::mir::MirModule::new(
        compiler::module::ModuleName::parse("app").unwrap(),
        vec![caller, helper],
    );

    let ir = lower_mir_module_to_cranelift(&module, &types).unwrap();
    assert_eq!(ir.len(), 2);
    assert!(ir[0].contains("call"), "{}", ir[0]);
    let object = emit_mir_module_to_object(&module, &types, "neu_main").unwrap();
    assert!(!object.is_empty());
}

#[test]
fn m0035_lowers_unit_direct_call_without_abi_result() {
    let file = SourceFileId::from_raw(928);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let unit_type = types.insert(TypeRecord::primitive(PrimitiveType::Unit));
    let module_name = compiler::module::ModuleName::parse("app").unwrap();
    let package = compiler::module::PackageNamespace::parse("demo").unwrap();
    let helper = MirFunction::new(
        MirFunctionId::from_raw(1),
        span,
        vec![],
        unit_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![MirInstruction::unit_constant(span)],
            MirTerminator::return_unit(span),
        )],
        MirCleanupBoundary::empty(),
    )
    .with_symbol_identity(compiler::module::FunctionSymbolIdentity::new(
        module_name.clone(),
        package.clone(),
        "unit_helper",
    ));
    let caller = MirFunction::new(
        MirFunctionId::from_raw(0),
        span,
        vec![],
        unit_type,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![MirInstruction::DirectCall {
                output: MirValueId::from_raw(0),
                callee: MirFunctionId::from_raw(1),
                arguments: vec![],
                span,
            }],
            MirTerminator::return_unit(span),
        )],
        MirCleanupBoundary::empty(),
    )
    .with_symbol_identity(compiler::module::FunctionSymbolIdentity::new(
        module_name.clone(),
        package.clone(),
        "unit_caller",
    ));
    let module = compiler::mir::MirModule::new(module_name, vec![caller, helper]);

    let ir = lower_mir_module_to_cranelift(&module, &types).unwrap();
    assert!(ir[0].contains("call"), "{}", ir[0]);
    assert!(!ir[0].contains("->"), "{}", ir[0]);
    assert!(
        !emit_mir_module_to_object(&module, &types, "neu_main")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn m0035_source_float_helper_call_reaches_object_emission() {
    let parsed = compiler::parser::parse_source(
        SourceFileId::from_raw(929),
        "fun helper(): Float { return 1.5; } fun caller(): Float { return helper(); }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());
    let mut types = TypeArena::new();
    let signatures = type_m0028_function_signatures_in(
        &mut types,
        &parsed.function_declarations,
        &parsed.function_parameters,
        &parsed.type_name_references,
    );
    assert_eq!(signatures.len(), 2);

    let mut report = type_m0028_executable_core_in(
        &mut types,
        &parsed.arena,
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.integer_literals,
        &parsed.grouped_expressions,
        &parsed.unary_expressions,
        &parsed.binary_expressions,
        &parsed.assignment_statements,
        &compiler::name_resolution::ResolutionTable::new(),
        &[],
    );
    let package = compiler::module::PackageNamespace::parse("app").unwrap();
    let calls = check_m0028_direct_calls(&[ExecutableSourceTypes::new(
        &package,
        &parsed,
        &signatures,
        report.expression_types(),
    )]);
    assert!(calls.diagnostics().is_empty());
    apply_m0028_direct_call_results(&mut report, &parsed, &calls);

    let hir = lower_checked_hir_source(CheckedHirSource::new(
        compiler::module::ModuleName::parse("app").unwrap(),
        package,
        &parsed,
        &signatures,
        report.expression_types(),
        true,
    ))
    .unwrap();
    let mir = compiler::mir::lower_hir_to_mir(&hir, &types).unwrap();
    let object = emit_mir_module_to_object(&mir, &types, "neu_main").unwrap();
    assert!(!object.is_empty());
}

#[test]
fn m0031_lowers_checked_exponentiation() {
    let file = SourceFileId::from_raw(410);
    let span = ByteSpan::new(file, 0, 10).unwrap();
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let function = MirFunction::new(
        MirFunctionId::from_raw(10),
        span,
        vec![],
        int,
        vec![],
        vec![MirBasicBlock::new(
            MirBlockId::from_raw(0),
            vec![
                MirInstruction::int_constant(MirValueId::from_raw(0), 2, span),
                MirInstruction::int_constant(MirValueId::from_raw(1), 3, span),
                MirInstruction::CheckedArithmetic {
                    output: MirValueId::from_raw(2),
                    operation: MirArithmetic::Exponent,
                    left: MirValueId::from_raw(0),
                    right: MirValueId::from_raw(1),
                    span,
                },
            ],
            MirTerminator::return_value(MirValueId::from_raw(2), span),
        )],
        MirCleanupBoundary::empty(),
    );
    let ir = lower_mir_function_to_cranelift(&function, &types).unwrap();
    assert!(ir.contains("imul"), "{ir}");
    assert!(ir.contains("trapnz"), "{ir}");
    assert!(ir.contains("trap user2"), "{ir}");
}
