use std::collections::HashMap;

use cranelift_codegen::{
    ir::{
        AbiParam, Function, InstBuilder, Signature, TrapCode, UserFuncName, Value,
        condcodes::IntCC, types,
    },
    isa::CallConv,
    settings, verify_function,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use target_lexicon::Triple;

use crate::{
    mir::{MirArithmetic, MirFunction, MirInstruction, MirTerminator, MirValueId},
    types::{PrimitiveType, TypeArena, TypeKind},
};

const INVALID_SHIFT_COUNT_TRAP: TrapCode = TrapCode::unwrap_user(1);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CraneliftLoweringError {
    UnsupportedRuntimeType,
    UnsupportedFunctionShape,
    UnsupportedInstruction,
    UnsupportedTerminator,
    MissingValue,
    FunctionIdOutOfRange,
    VerificationFailed,
}

pub fn lower_mir_function_to_cranelift(
    function: &MirFunction,
    type_arena: &TypeArena,
) -> Result<String, CraneliftLoweringError> {
    require_bootstrap_int(function.return_type(), type_arena)?;
    if !function.parameters().is_empty() || function.blocks().len() != 1 {
        return Err(CraneliftLoweringError::UnsupportedFunctionShape);
    }

    let function_index = u32::try_from(function.id().index())
        .map_err(|_| CraneliftLoweringError::FunctionIdOutOfRange)?;
    let mut signature = Signature::new(CallConv::triple_default(&Triple::host()));
    signature.returns.push(AbiParam::new(types::I64));
    let mut clif_function =
        Function::with_name_signature(UserFuncName::user(0, function_index), signature);
    let mut builder_context = FunctionBuilderContext::new();
    let mut values = HashMap::new();

    {
        let mut builder = FunctionBuilder::new(&mut clif_function, &mut builder_context);
        let mir_block = &function.blocks()[0];
        let clif_block = builder.create_block();
        builder.switch_to_block(clif_block);
        builder.seal_block(clif_block);

        for instruction in mir_block.instructions() {
            lower_instruction(instruction, &mut builder, &mut values)?;
        }
        lower_terminator(mir_block.terminator(), &mut builder, &values)?;
        builder.finalize();
    }

    let flags = settings::Flags::new(settings::builder());
    verify_function(&clif_function, &flags)
        .map_err(|_| CraneliftLoweringError::VerificationFailed)?;
    Ok(clif_function.display().to_string())
}

fn require_bootstrap_int(
    ty: crate::types::TypeId,
    type_arena: &TypeArena,
) -> Result<(), CraneliftLoweringError> {
    matches!(
        type_arena.get(ty).map(|record| record.kind()),
        Some(TypeKind::Primitive(PrimitiveType::Int))
    )
    .then_some(())
    .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)
}

fn lower_instruction(
    instruction: &MirInstruction,
    builder: &mut FunctionBuilder<'_>,
    values: &mut HashMap<MirValueId, Value>,
) -> Result<(), CraneliftLoweringError> {
    match instruction {
        MirInstruction::IntConstant { output, value, .. } => {
            values.insert(*output, builder.ins().iconst(types::I64, *value));
            Ok(())
        }
        MirInstruction::CheckedArithmetic {
            output,
            operation: MirArithmetic::Add,
            left,
            right,
            ..
        } => {
            let left = values
                .get(left)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let right = values
                .get(right)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let sum = builder.ins().iadd(left, right);
            let left_sign_change = builder.ins().bxor(left, sum);
            let right_sign_change = builder.ins().bxor(right, sum);
            let signed_change = builder.ins().band(left_sign_change, right_sign_change);
            let overflow = builder
                .ins()
                .icmp_imm(IntCC::SignedLessThan, signed_change, 0);
            builder.ins().trapnz(overflow, TrapCode::INTEGER_OVERFLOW);
            values.insert(*output, sum);
            Ok(())
        }
        MirInstruction::CheckedArithmetic {
            output,
            operation: MirArithmetic::Subtract,
            left,
            right,
            ..
        } => {
            let left = values
                .get(left)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let right = values
                .get(right)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let difference = builder.ins().isub(left, right);
            let operand_sign_difference = builder.ins().bxor(left, right);
            let result_sign_difference = builder.ins().bxor(left, difference);
            let signed_change = builder
                .ins()
                .band(operand_sign_difference, result_sign_difference);
            let overflow = builder
                .ins()
                .icmp_imm(IntCC::SignedLessThan, signed_change, 0);
            builder.ins().trapnz(overflow, TrapCode::INTEGER_OVERFLOW);
            values.insert(*output, difference);
            Ok(())
        }
        MirInstruction::CheckedArithmetic {
            output,
            operation: MirArithmetic::Multiply,
            left,
            right,
            ..
        } => {
            let left = values
                .get(left)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let right = values
                .get(right)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let product = builder.ins().imul(left, right);
            let high_half = builder.ins().smulhi(left, right);
            let sign_extension = builder.ins().sshr_imm(product, 63);
            let overflow = builder
                .ins()
                .icmp(IntCC::NotEqual, high_half, sign_extension);
            builder.ins().trapnz(overflow, TrapCode::INTEGER_OVERFLOW);
            values.insert(*output, product);
            Ok(())
        }
        MirInstruction::CheckedArithmetic {
            output,
            operation: MirArithmetic::Divide,
            left,
            right,
            ..
        } => {
            let left = values
                .get(left)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let right = values
                .get(right)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            values.insert(*output, builder.ins().sdiv(left, right));
            Ok(())
        }
        MirInstruction::CheckedArithmetic {
            output,
            operation: MirArithmetic::Remainder,
            left,
            right,
            ..
        } => {
            let left = values
                .get(left)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let right = values
                .get(right)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            values.insert(*output, builder.ins().srem(left, right));
            Ok(())
        }
        MirInstruction::CheckedArithmetic {
            output,
            operation,
            left,
            right,
            ..
        } if matches!(
            operation,
            MirArithmetic::BitwiseAnd | MirArithmetic::BitwiseOr | MirArithmetic::BitwiseXor
        ) =>
        {
            let left = values
                .get(left)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let right = values
                .get(right)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let value = match operation {
                MirArithmetic::BitwiseAnd => builder.ins().band(left, right),
                MirArithmetic::BitwiseOr => builder.ins().bor(left, right),
                MirArithmetic::BitwiseXor => builder.ins().bxor(left, right),
                _ => unreachable!("guard accepts only bitwise operations"),
            };
            values.insert(*output, value);
            Ok(())
        }
        MirInstruction::CheckedArithmetic {
            output,
            operation,
            left,
            right,
            ..
        } if matches!(
            operation,
            MirArithmetic::ShiftLeft | MirArithmetic::ShiftRight
        ) =>
        {
            let left = values
                .get(left)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let right = values
                .get(right)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let invalid_count = builder
                .ins()
                .icmp_imm(IntCC::UnsignedGreaterThan, right, 63);
            builder
                .ins()
                .trapnz(invalid_count, INVALID_SHIFT_COUNT_TRAP);
            let value = match operation {
                MirArithmetic::ShiftLeft => builder.ins().ishl(left, right),
                MirArithmetic::ShiftRight => builder.ins().sshr(left, right),
                _ => unreachable!("guard accepts only shift operations"),
            };
            values.insert(*output, value);
            Ok(())
        }
        _ => Err(CraneliftLoweringError::UnsupportedInstruction),
    }
}

fn lower_terminator(
    terminator: MirTerminator,
    builder: &mut FunctionBuilder<'_>,
    values: &HashMap<MirValueId, Value>,
) -> Result<(), CraneliftLoweringError> {
    match terminator {
        MirTerminator::Return { value, .. } => {
            let value = values
                .get(&value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.ins().return_(&[value]);
            Ok(())
        }
        _ => Err(CraneliftLoweringError::UnsupportedTerminator),
    }
}
