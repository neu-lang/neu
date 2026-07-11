use std::{collections::HashMap, fmt::Write as _};

use cranelift_codegen::{
    ir::{
        AbiParam, Function, InstBuilder, Signature, TrapCode, UserFuncName, Value,
        condcodes::IntCC, types,
    },
    isa::CallConv,
    settings, verify_function,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

use crate::{
    mir::{MirArithmetic, MirFunction, MirInstruction, MirTerminator, MirUnary, MirValueId},
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
    if !function.parameters().is_empty() || function.blocks().len() != 1 {
        return Err(CraneliftLoweringError::UnsupportedFunctionShape);
    }

    let function_index = u32::try_from(function.id().index())
        .map_err(|_| CraneliftLoweringError::FunctionIdOutOfRange)?;
    let mut signature = Signature::new(CallConv::triple_default(target));
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
            let is_min = builder.ins().icmp_imm(IntCC::Equal, operand, i64::MIN);
            builder.ins().trapnz(is_min, TrapCode::INTEGER_OVERFLOW);
            values.insert(*output, builder.ins().ineg(operand));
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
