use std::{collections::HashMap, fmt::Write as _};

use cranelift_codegen::{
    ir::{
        AbiParam, Function, InstBuilder, Signature, TrapCode, UserFuncName, Value,
        condcodes::{FloatCC, IntCC},
        immediates::Ieee64,
        types,
    },
    isa::CallConv,
    settings, verify_function,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

use crate::{
    mir::{
        MirArithmetic, MirComparison, MirFunction, MirInstruction, MirTerminator, MirUnary,
        MirValueId,
    },
    types::{PrimitiveType, TypeArena, TypeKind},
};

const INVALID_SHIFT_COUNT_TRAP: TrapCode = TrapCode::unwrap_user(1);
const NEGATIVE_EXPONENT_TRAP: TrapCode = TrapCode::unwrap_user(2);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CraneliftLoweringError {
    UnsupportedRuntimeType,
    UnsupportedFunctionShape,
    UnsupportedInstruction,
    UnsupportedTerminator,
    MissingValue,
    FunctionIdOutOfRange,
    MissingFunctionIdentity,
    MissingLanguageEntrySymbol,
    TargetIsaUnavailable,
    ObjectBuilderFailed,
    ObjectDefinitionFailed,
    ObjectEmissionFailed,
    VerificationFailed,
}

pub fn lower_mir_function_to_cranelift(
    function: &MirFunction,
    type_arena: &TypeArena,
) -> Result<String, CraneliftLoweringError> {
    Ok(lower_mir_function(function, type_arena, &Triple::host())?
        .display()
        .to_string())
}

pub fn emit_mir_function_to_object(
    function: &MirFunction,
    type_arena: &TypeArena,
) -> Result<Vec<u8>, CraneliftLoweringError> {
    emit_mir_function_to_object_impl(function, type_arena, None, Triple::host())
}

pub fn emit_mir_function_to_object_with_entry_symbol(
    function: &MirFunction,
    type_arena: &TypeArena,
    language_entry_symbol: &str,
) -> Result<Vec<u8>, CraneliftLoweringError> {
    emit_mir_function_to_object_impl(
        function,
        type_arena,
        Some(language_entry_symbol),
        Triple::host(),
    )
}

pub fn emit_mir_function_to_object_for_target(
    function: &MirFunction,
    type_arena: &TypeArena,
    language_entry_symbol: &str,
    target: Triple,
) -> Result<Vec<u8>, CraneliftLoweringError> {
    emit_mir_function_to_object_impl(function, type_arena, Some(language_entry_symbol), target)
}

fn emit_mir_function_to_object_impl(
    function: &MirFunction,
    type_arena: &TypeArena,
    language_entry_symbol: Option<&str>,
    target: Triple,
) -> Result<Vec<u8>, CraneliftLoweringError> {
    let identity = function
        .symbol_identity()
        .ok_or(CraneliftLoweringError::MissingFunctionIdentity)?;
    let clif_function = lower_mir_function(function, type_arena, &target)?;
    let isa_builder = cranelift_codegen::isa::lookup(target.clone())
        .map_err(|_| CraneliftLoweringError::TargetIsaUnavailable)?;
    let isa = isa_builder
        .finish(settings::Flags::new(settings::builder()))
        .map_err(|_| CraneliftLoweringError::TargetIsaUnavailable)?;
    let mut module = ObjectModule::new(
        ObjectBuilder::new(isa, "neu", default_libcall_names())
            .map_err(|_| CraneliftLoweringError::ObjectBuilderFailed)?,
    );
    let symbol = if function.is_entry() {
        let symbol = language_entry_symbol
            .filter(|symbol| !symbol.is_empty())
            .ok_or(CraneliftLoweringError::MissingLanguageEntrySymbol)?;
        symbol.to_owned()
    } else {
        bootstrap_symbol(identity)
    };
    let linkage = if function.is_entry() {
        Linkage::Export
    } else {
        Linkage::Local
    };
    let function_id = module
        .declare_function(&symbol, linkage, &clif_function.signature)
        .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
    let mut context = cranelift_codegen::Context::for_function(clif_function);
    module
        .define_function(function_id, &mut context)
        .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
    module
        .finish()
        .emit()
        .map_err(|_| CraneliftLoweringError::ObjectEmissionFailed)
}

fn lower_mir_function(
    function: &MirFunction,
    type_arena: &TypeArena,
    target: &Triple,
) -> Result<Function, CraneliftLoweringError> {
    require_bootstrap_int(function.return_type(), type_arena)?;
    if !function.parameters().is_empty() || function.blocks().is_empty() {
        return Err(CraneliftLoweringError::UnsupportedFunctionShape);
    }

    let function_index = u32::try_from(function.id().index())
        .map_err(|_| CraneliftLoweringError::FunctionIdOutOfRange)?;
    let mut signature = Signature::new(CallConv::triple_default(target));
    if let Some(return_type) = cranelift_type(function.return_type(), type_arena) {
        signature.returns.push(AbiParam::new(return_type));
    }
    let mut clif_function =
        Function::with_name_signature(UserFuncName::user(0, function_index), signature);
    let mut builder_context = FunctionBuilderContext::new();
    let mut values = HashMap::new();

    {
        let mut builder = FunctionBuilder::new(&mut clif_function, &mut builder_context);
        let mut clif_blocks = HashMap::new();
        let mut locals = HashMap::new();
        for local in function.locals() {
            let local_type = cranelift_type(local.ty(), type_arena)
                .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)?;
            locals.insert(local.id(), builder.declare_var(local_type));
        }
        for mir_block in function.blocks() {
            clif_blocks.insert(mir_block.id(), builder.create_block());
        }

        for mir_block in function.blocks() {
            let clif_block = *clif_blocks
                .get(&mir_block.id())
                .ok_or(CraneliftLoweringError::UnsupportedTerminator)?;
            builder.switch_to_block(clif_block);
            for instruction in mir_block.instructions() {
                lower_instruction(instruction, &mut builder, &mut values, &locals)?;
            }
            lower_terminator(mir_block.terminator(), &clif_blocks, &mut builder, &values)?;
        }
        for clif_block in clif_blocks.values().copied() {
            builder.seal_block(clif_block);
        }
        builder.finalize();
    }

    let flags = settings::Flags::new(settings::builder());
    verify_function(&clif_function, &flags)
        .map_err(|_| CraneliftLoweringError::VerificationFailed)?;
    Ok(clif_function)
}

fn bootstrap_symbol(identity: &crate::module::FunctionSymbolIdentity) -> String {
    format!(
        "neu_fn_{}_{}_{}",
        encode_symbol_component(identity.module().as_str()),
        encode_symbol_component(identity.package().as_str()),
        encode_symbol_component(identity.name()),
    )
}

fn encode_symbol_component(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.as_bytes() {
        let _ = write!(encoded, "{byte:02x}");
    }
    format!("{}_{}", value.len(), encoded)
}

fn require_bootstrap_int(
    ty: crate::types::TypeId,
    type_arena: &TypeArena,
) -> Result<(), CraneliftLoweringError> {
    matches!(
        type_arena.get(ty).map(|record| record.kind()),
        Some(TypeKind::Primitive(
            PrimitiveType::Bool
                | PrimitiveType::Int
                | PrimitiveType::Float
                | PrimitiveType::Byte
                | PrimitiveType::Unit
        ))
    )
    .then_some(())
    .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)
}

fn cranelift_type(ty: crate::types::TypeId, type_arena: &TypeArena) -> Option<types::Type> {
    match type_arena.get(ty).map(|record| record.kind()) {
        Some(TypeKind::Primitive(PrimitiveType::Bool | PrimitiveType::Byte)) => Some(types::I8),
        Some(TypeKind::Primitive(PrimitiveType::Int)) => Some(types::I64),
        Some(TypeKind::Primitive(PrimitiveType::Float)) => Some(types::F64),
        Some(TypeKind::Primitive(PrimitiveType::Unit)) => None,
        _ => None,
    }
}

fn lower_basic_arithmetic(
    operation: MirArithmetic,
    left: Value,
    right: Value,
    builder: &mut FunctionBuilder<'_>,
) -> Value {
    let value_type = builder.func.dfg.value_type(left);
    if value_type == types::F64 {
        return match operation {
            MirArithmetic::Add => builder.ins().fadd(left, right),
            MirArithmetic::Subtract => builder.ins().fsub(left, right),
            MirArithmetic::Multiply => builder.ins().fmul(left, right),
            MirArithmetic::Divide => builder.ins().fdiv(left, right),
            _ => unreachable!("unsupported Float arithmetic operation"),
        };
    }

    if value_type == types::I8 {
        let left_wide = builder.ins().uextend(types::I64, left);
        let right_wide = builder.ins().uextend(types::I64, right);
        let value = match operation {
            MirArithmetic::Add => {
                let result = builder.ins().iadd(left_wide, right_wide);
                let overflow =
                    builder
                        .ins()
                        .icmp_imm(IntCC::UnsignedGreaterThan, result, i64::from(u8::MAX));
                builder.ins().trapnz(overflow, TrapCode::INTEGER_OVERFLOW);
                builder.ins().ireduce(types::I8, result)
            }
            MirArithmetic::Subtract => {
                let underflow = builder
                    .ins()
                    .icmp(IntCC::UnsignedLessThan, left_wide, right_wide);
                builder.ins().trapnz(underflow, TrapCode::INTEGER_OVERFLOW);
                let result = builder.ins().isub(left_wide, right_wide);
                builder.ins().ireduce(types::I8, result)
            }
            MirArithmetic::Multiply => {
                let result = builder.ins().imul(left_wide, right_wide);
                let overflow =
                    builder
                        .ins()
                        .icmp_imm(IntCC::UnsignedGreaterThan, result, i64::from(u8::MAX));
                builder.ins().trapnz(overflow, TrapCode::INTEGER_OVERFLOW);
                builder.ins().ireduce(types::I8, result)
            }
            MirArithmetic::Divide => {
                let zero = builder.ins().icmp_imm(IntCC::Equal, right, 0);
                builder
                    .ins()
                    .trapnz(zero, TrapCode::INTEGER_DIVISION_BY_ZERO);
                builder.ins().udiv(left, right)
            }
            MirArithmetic::Remainder => {
                let zero = builder.ins().icmp_imm(IntCC::Equal, right, 0);
                builder
                    .ins()
                    .trapnz(zero, TrapCode::INTEGER_DIVISION_BY_ZERO);
                builder.ins().urem(left, right)
            }
            _ => unreachable!("unsupported Byte arithmetic operation"),
        };
        return value;
    }

    match operation {
        MirArithmetic::Add => {
            let sum = builder.ins().iadd(left, right);
            let left_sign_change = builder.ins().bxor(left, sum);
            let right_sign_change = builder.ins().bxor(right, sum);
            let signed_change = builder.ins().band(left_sign_change, right_sign_change);
            let overflow = builder
                .ins()
                .icmp_imm(IntCC::SignedLessThan, signed_change, 0);
            builder.ins().trapnz(overflow, TrapCode::INTEGER_OVERFLOW);
            sum
        }
        MirArithmetic::Subtract => {
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
            difference
        }
        MirArithmetic::Multiply => {
            let product = builder.ins().imul(left, right);
            let high_half = builder.ins().smulhi(left, right);
            let sign_extension = builder.ins().sshr_imm(product, 63);
            let overflow = builder
                .ins()
                .icmp(IntCC::NotEqual, high_half, sign_extension);
            builder.ins().trapnz(overflow, TrapCode::INTEGER_OVERFLOW);
            product
        }
        MirArithmetic::Divide => builder.ins().sdiv(left, right),
        MirArithmetic::Remainder => builder.ins().srem(left, right),
        _ => unreachable!("unsupported basic arithmetic operation"),
    }
}

fn int_cc(operation: MirComparison, unsigned: bool) -> IntCC {
    match operation {
        MirComparison::Equal => IntCC::Equal,
        MirComparison::NotEqual => IntCC::NotEqual,
        MirComparison::Less if unsigned => IntCC::UnsignedLessThan,
        MirComparison::Greater if unsigned => IntCC::UnsignedGreaterThan,
        MirComparison::LessEqual if unsigned => IntCC::UnsignedLessThanOrEqual,
        MirComparison::GreaterEqual if unsigned => IntCC::UnsignedGreaterThanOrEqual,
        MirComparison::Less => IntCC::SignedLessThan,
        MirComparison::Greater => IntCC::SignedGreaterThan,
        MirComparison::LessEqual => IntCC::SignedLessThanOrEqual,
        MirComparison::GreaterEqual => IntCC::SignedGreaterThanOrEqual,
    }
}

fn float_cc(operation: MirComparison) -> FloatCC {
    match operation {
        MirComparison::Equal => FloatCC::Equal,
        MirComparison::NotEqual => FloatCC::NotEqual,
        MirComparison::Less => FloatCC::LessThan,
        MirComparison::Greater => FloatCC::GreaterThan,
        MirComparison::LessEqual => FloatCC::LessThanOrEqual,
        MirComparison::GreaterEqual => FloatCC::GreaterThanOrEqual,
    }
}

fn lower_instruction(
    instruction: &MirInstruction,
    builder: &mut FunctionBuilder<'_>,
    values: &mut HashMap<MirValueId, Value>,
    locals: &HashMap<crate::mir::MirLocalId, Variable>,
) -> Result<(), CraneliftLoweringError> {
    match instruction {
        MirInstruction::IntConstant { output, value, .. } => {
            values.insert(*output, builder.ins().iconst(types::I64, *value));
            Ok(())
        }
        MirInstruction::BoolConstant { output, value, .. } => {
            values.insert(*output, builder.ins().iconst(types::I8, i64::from(*value)));
            Ok(())
        }
        MirInstruction::ByteConstant { output, value, .. } => {
            values.insert(*output, builder.ins().iconst(types::I8, i64::from(*value)));
            Ok(())
        }
        MirInstruction::FloatConstant { output, bits, .. } => {
            values.insert(*output, builder.ins().f64const(Ieee64::with_bits(*bits)));
            Ok(())
        }
        MirInstruction::UnitConstant { .. } => Ok(()),
        MirInstruction::Unary {
            output,
            operation: MirUnary::Plus,
            operand,
            ..
        } => {
            let operand = values
                .get(operand)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            values.insert(*output, operand);
            Ok(())
        }
        MirInstruction::Unary {
            output,
            operation: MirUnary::Negate,
            operand,
            ..
        } => {
            let operand = values
                .get(operand)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let value = if builder.func.dfg.value_type(operand) == types::F64 {
                builder.ins().fneg(operand)
            } else {
                let is_min = builder.ins().icmp_imm(IntCC::Equal, operand, i64::MIN);
                builder.ins().trapnz(is_min, TrapCode::INTEGER_OVERFLOW);
                builder.ins().ineg(operand)
            };
            values.insert(*output, value);
            Ok(())
        }
        MirInstruction::Unary {
            output,
            operation: MirUnary::BitwiseNot,
            operand,
            ..
        } => {
            let operand = values
                .get(operand)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            values.insert(*output, builder.ins().bnot(operand));
            Ok(())
        }
        MirInstruction::LogicalNot {
            output, operand, ..
        } => {
            let operand = values
                .get(operand)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let is_false = builder.ins().icmp_imm(IntCC::Equal, operand, 0);
            let one = builder.ins().iconst(types::I8, 1);
            let zero = builder.ins().iconst(types::I8, 0);
            values.insert(*output, builder.ins().select(is_false, one, zero));
            Ok(())
        }
        MirInstruction::Compare {
            output,
            operation,
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
            let left_type = builder.func.dfg.value_type(left);
            let condition = if left_type == types::F64 {
                builder.ins().fcmp(float_cc(*operation), left, right)
            } else {
                builder
                    .ins()
                    .icmp(int_cc(*operation, left_type == types::I8), left, right)
            };
            let one = builder.ins().iconst(types::I8, 1);
            let zero = builder.ins().iconst(types::I8, 0);
            values.insert(*output, builder.ins().select(condition, one, zero));
            Ok(())
        }
        MirInstruction::LoadLocal { output, local, .. } => {
            let local = locals
                .get(local)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            values.insert(*output, builder.use_var(local));
            Ok(())
        }
        MirInstruction::StoreLocal { local, value, .. } => {
            let local = locals
                .get(local)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let value = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.def_var(local, value);
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
            MirArithmetic::Add
                | MirArithmetic::Subtract
                | MirArithmetic::Multiply
                | MirArithmetic::Divide
                | MirArithmetic::Remainder
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
            let value = lower_basic_arithmetic(*operation, left, right, builder);
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
            let left_type = builder.func.dfg.value_type(left);
            let max_shift = if left_type == types::I8 { 7 } else { 63 };
            let invalid_count =
                builder
                    .ins()
                    .icmp_imm(IntCC::UnsignedGreaterThan, right, max_shift);
            builder
                .ins()
                .trapnz(invalid_count, INVALID_SHIFT_COUNT_TRAP);
            let value = match operation {
                MirArithmetic::ShiftLeft => builder.ins().ishl(left, right),
                MirArithmetic::ShiftRight if left_type == types::I8 => {
                    builder.ins().ushr(left, right)
                }
                MirArithmetic::ShiftRight => builder.ins().sshr(left, right),
                _ => unreachable!("guard accepts only shift operations"),
            };
            values.insert(*output, value);
            Ok(())
        }
        MirInstruction::CheckedArithmetic {
            output,
            operation: MirArithmetic::Exponent,
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

            let result = builder.declare_var(types::I64);
            let base = builder.declare_var(types::I64);
            let exponent = builder.declare_var(types::I64);
            let one = builder.ins().iconst(types::I64, 1);
            builder.def_var(result, one);
            builder.def_var(base, left);
            builder.def_var(exponent, right);

            let loop_block = builder.create_block();
            let negative_block = builder.create_block();
            let check_zero_block = builder.create_block();
            let multiply_block = builder.create_block();
            let done_block = builder.create_block();

            builder.ins().jump(loop_block, &[]);

            builder.switch_to_block(loop_block);
            let exponent_value = builder.use_var(exponent);
            let is_negative = builder
                .ins()
                .icmp_imm(IntCC::SignedLessThan, exponent_value, 0);
            builder
                .ins()
                .brif(is_negative, negative_block, &[], check_zero_block, &[]);

            builder.seal_block(negative_block);
            builder.switch_to_block(negative_block);
            builder.ins().trap(NEGATIVE_EXPONENT_TRAP);

            builder.seal_block(check_zero_block);
            builder.switch_to_block(check_zero_block);
            let exponent_value = builder.use_var(exponent);
            let is_zero = builder.ins().icmp_imm(IntCC::Equal, exponent_value, 0);
            builder
                .ins()
                .brif(is_zero, done_block, &[], multiply_block, &[]);

            builder.seal_block(multiply_block);
            builder.switch_to_block(multiply_block);
            let result_value = builder.use_var(result);
            let base_value = builder.use_var(base);
            let product = builder.ins().imul(result_value, base_value);
            let high_half = builder.ins().smulhi(result_value, base_value);
            let sign_extension = builder.ins().sshr_imm(product, 63);
            let overflow = builder
                .ins()
                .icmp(IntCC::NotEqual, high_half, sign_extension);
            builder.ins().trapnz(overflow, TrapCode::INTEGER_OVERFLOW);
            builder.def_var(result, product);
            let exponent_value = builder.use_var(exponent);
            let decremented = builder.ins().isub(exponent_value, one);
            builder.def_var(exponent, decremented);
            builder.ins().jump(loop_block, &[]);
            builder.seal_block(loop_block);

            builder.seal_block(done_block);
            builder.switch_to_block(done_block);
            values.insert(*output, builder.use_var(result));
            Ok(())
        }
        _ => Err(CraneliftLoweringError::UnsupportedInstruction),
    }
}

fn lower_terminator(
    terminator: MirTerminator,
    blocks: &HashMap<crate::mir::MirBlockId, cranelift_codegen::ir::Block>,
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
        MirTerminator::ReturnUnit { .. } => {
            builder.ins().return_(&[]);
            Ok(())
        }
        MirTerminator::Branch { target, .. } => {
            let target = blocks
                .get(&target)
                .copied()
                .ok_or(CraneliftLoweringError::UnsupportedTerminator)?;
            builder.ins().jump(target, &[]);
            Ok(())
        }
        MirTerminator::BranchIf {
            condition,
            then_target,
            else_target,
            ..
        } => {
            let condition = values
                .get(&condition)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let then_target = blocks
                .get(&then_target)
                .copied()
                .ok_or(CraneliftLoweringError::UnsupportedTerminator)?;
            let else_target = blocks
                .get(&else_target)
                .copied()
                .ok_or(CraneliftLoweringError::UnsupportedTerminator)?;
            builder
                .ins()
                .brif(condition, then_target, &[], else_target, &[]);
            Ok(())
        }
        MirTerminator::Trap { .. } => Err(CraneliftLoweringError::UnsupportedTerminator),
    }
}
