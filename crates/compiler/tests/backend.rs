use compiler::{
    backend::{CraneliftLoweringError, lower_mir_function_to_cranelift},
    mir::{
        MirArithmetic, MirBasicBlock, MirBlockId, MirCleanupBoundary, MirFunction, MirFunctionId,
        MirInstruction, MirTerminator, MirValueId,
    },
    source::{ByteSpan, SourceFileId},
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
            vec![MirInstruction::CheckedArithmetic {
                output: MirValueId::from_raw(2),
                operation: MirArithmetic::Exponent,
                left: MirValueId::from_raw(0),
                right: MirValueId::from_raw(1),
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
