use std::{collections::HashMap, fmt::Write as _};

use cranelift_codegen::{
    ir::{
        AbiParam, ExtFuncData, ExternalName, FuncRef, Function, InstBuilder, MemFlagsData,
        Signature, TrapCode, UserExternalName, UserFuncName, Value,
        condcodes::{FloatCC, IntCC},
        immediates::Ieee64,
        types,
    },
    isa::CallConv,
    settings,
    settings::Configurable,
    verify_function,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{FuncId, Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

use crate::{
    mir::{
        MirArithmetic, MirComparison, MirFunction, MirInstruction, MirModule, MirTerminator,
        MirUnary, MirValueId,
    },
    types::{PrimitiveType, TypeArena, TypeId, TypeKind},
};

const INVALID_SHIFT_COUNT_TRAP: TrapCode = TrapCode::unwrap_user(1);
const NEGATIVE_EXPONENT_TRAP: TrapCode = TrapCode::unwrap_user(2);

fn codegen_flags() -> settings::Flags {
    let mut builder = settings::builder();
    builder
        .set("is_pic", "false")
        .expect("known Cranelift setting");
    settings::Flags::new(builder)
}

struct LoweringContext<'a> {
    function: &'a MirFunction,
    type_arena: &'a TypeArena,
    function_return_types: &'a HashMap<crate::mir::MirFunctionId, TypeId>,
    runtime: Option<&'a RuntimeFunctions>,
}

#[derive(Clone, Copy)]
struct RuntimeFunctions {
    malloc: FuncId,
    memcpy: FuncId,
    memcmp: FuncId,
    call_conv: CallConv,
}

impl RuntimeFunctions {
    fn reference(&self, function: &mut Function, id: FuncId, memory: bool) -> FuncRef {
        let mut signature = Signature::new(self.call_conv);
        if memory {
            signature.params.push(AbiParam::new(types::I64));
            signature.params.push(AbiParam::new(types::I64));
            signature.params.push(AbiParam::new(types::I64));
        } else {
            signature.params.push(AbiParam::new(types::I64));
        }
        signature.returns.push(AbiParam::new(types::I64));
        let signature = function.import_signature(signature);
        let user_name = function.declare_imported_user_function(UserExternalName {
            namespace: 0,
            index: id.as_u32(),
        });
        function.import_function(ExtFuncData {
            name: ExternalName::user(user_name),
            signature,
            colocated: true,
            patchable: false,
        })
    }
}

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

pub fn lower_mir_module_to_cranelift(
    module: &MirModule,
    type_arena: &TypeArena,
) -> Result<Vec<String>, CraneliftLoweringError> {
    let target = Triple::host();
    let isa_builder = cranelift_codegen::isa::lookup(target.clone())
        .map_err(|_| CraneliftLoweringError::TargetIsaUnavailable)?;
    let isa = isa_builder
        .finish(codegen_flags())
        .map_err(|_| CraneliftLoweringError::TargetIsaUnavailable)?;
    let mut object_module = ObjectModule::new(
        ObjectBuilder::new(isa, "neu", default_libcall_names())
            .map_err(|_| CraneliftLoweringError::ObjectBuilderFailed)?,
    );
    let mut function_ids = HashMap::new();
    let runtime = declare_runtime_functions(&mut object_module, &target)?;
    let function_return_types = module
        .functions()
        .iter()
        .map(|function| (function.id(), function.return_type()))
        .collect::<HashMap<_, _>>();
    for function in module.functions() {
        let identity = function
            .symbol_identity()
            .ok_or(CraneliftLoweringError::MissingFunctionIdentity)?;
        let signature = mir_signature(function, type_arena, &target)?;
        let function_id = object_module
            .declare_function(&bootstrap_symbol(identity), Linkage::Local, &signature)
            .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
        function_ids.insert(function.id(), function_id);
    }

    let mut output = Vec::new();
    for function in module.functions() {
        let clif_function = lower_mir_function_with_module(
            function,
            type_arena,
            &target,
            Some(&mut object_module),
            &function_ids,
            &function_return_types,
            Some(&runtime),
        )?;
        output.push(clif_function.display().to_string());
    }
    Ok(output)
}

pub fn emit_mir_module_to_object(
    module: &MirModule,
    type_arena: &TypeArena,
    language_entry_symbol: &str,
) -> Result<Vec<u8>, CraneliftLoweringError> {
    emit_mir_module_to_object_for_target(module, type_arena, language_entry_symbol, Triple::host())
}

pub fn emit_mir_module_to_object_for_target(
    module: &MirModule,
    type_arena: &TypeArena,
    language_entry_symbol: &str,
    target: Triple,
) -> Result<Vec<u8>, CraneliftLoweringError> {
    let isa_builder = cranelift_codegen::isa::lookup(target.clone())
        .map_err(|_| CraneliftLoweringError::TargetIsaUnavailable)?;
    let isa = isa_builder
        .finish(codegen_flags())
        .map_err(|_| CraneliftLoweringError::TargetIsaUnavailable)?;
    let mut object_module = ObjectModule::new(
        ObjectBuilder::new(isa, "neu", default_libcall_names())
            .map_err(|_| CraneliftLoweringError::ObjectBuilderFailed)?,
    );
    let mut function_ids = HashMap::new();
    let runtime = declare_runtime_functions(&mut object_module, &target)?;
    let function_return_types = module
        .functions()
        .iter()
        .map(|function| (function.id(), function.return_type()))
        .collect::<HashMap<_, _>>();
    for function in module.functions() {
        let identity = function
            .symbol_identity()
            .ok_or(CraneliftLoweringError::MissingFunctionIdentity)?;
        let signature = mir_signature(function, type_arena, &target)?;
        let symbol = if function.is_entry() {
            if language_entry_symbol.is_empty() {
                return Err(CraneliftLoweringError::MissingLanguageEntrySymbol);
            }
            language_entry_symbol.to_owned()
        } else {
            bootstrap_symbol(identity)
        };
        let linkage = if function.is_entry() {
            Linkage::Export
        } else {
            Linkage::Local
        };
        let function_id = object_module
            .declare_function(&symbol, linkage, &signature)
            .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
        function_ids.insert(function.id(), function_id);
    }

    for function in module.functions() {
        let clif_function = lower_mir_function_with_module(
            function,
            type_arena,
            &target,
            Some(&mut object_module),
            &function_ids,
            &function_return_types,
            Some(&runtime),
        )?;
        let function_id = function_ids
            .get(&function.id())
            .copied()
            .ok_or(CraneliftLoweringError::ObjectDefinitionFailed)?;
        let mut context = cranelift_codegen::Context::for_function(clif_function);
        object_module
            .define_function(function_id, &mut context)
            .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
    }
    object_module
        .finish()
        .emit()
        .map_err(|_| CraneliftLoweringError::ObjectEmissionFailed)
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
    let isa_builder = cranelift_codegen::isa::lookup(target.clone())
        .map_err(|_| CraneliftLoweringError::TargetIsaUnavailable)?;
    let isa = isa_builder
        .finish(codegen_flags())
        .map_err(|_| CraneliftLoweringError::TargetIsaUnavailable)?;
    let mut module = ObjectModule::new(
        ObjectBuilder::new(isa, "neu", default_libcall_names())
            .map_err(|_| CraneliftLoweringError::ObjectBuilderFailed)?,
    );
    let runtime = declare_runtime_functions(&mut module, &target)?;
    let clif_function =
        lower_mir_function_with_runtime(function, type_arena, &target, Some(&runtime))?;
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
    lower_mir_function_with_runtime(function, type_arena, target, None)
}

fn lower_mir_function_with_runtime(
    function: &MirFunction,
    type_arena: &TypeArena,
    target: &Triple,
    runtime: Option<&RuntimeFunctions>,
) -> Result<Function, CraneliftLoweringError> {
    lower_mir_function_with_module(
        function,
        type_arena,
        target,
        None,
        &HashMap::new(),
        &HashMap::new(),
        runtime,
    )
}

fn lower_mir_function_with_module(
    function: &MirFunction,
    type_arena: &TypeArena,
    target: &Triple,
    module: Option<&mut ObjectModule>,
    function_ids: &HashMap<crate::mir::MirFunctionId, FuncId>,
    function_return_types: &HashMap<crate::mir::MirFunctionId, TypeId>,
    runtime: Option<&RuntimeFunctions>,
) -> Result<Function, CraneliftLoweringError> {
    require_bootstrap_runtime_type(function.return_type(), type_arena)?;
    if function.blocks().is_empty() {
        return Err(CraneliftLoweringError::UnsupportedFunctionShape);
    }

    let function_index = u32::try_from(function.id().index())
        .map_err(|_| CraneliftLoweringError::FunctionIdOutOfRange)?;
    let signature = mir_signature(function, type_arena, target)?;
    let mut clif_function =
        Function::with_name_signature(UserFuncName::user(0, function_index), signature);
    let mut builder_context = FunctionBuilderContext::new();
    let mut values = HashMap::new();

    {
        let mut builder = FunctionBuilder::new(&mut clif_function, &mut builder_context);
        let mut module = module;
        let lowering_context = LoweringContext {
            function,
            type_arena,
            function_return_types,
            runtime,
        };
        let mut clif_blocks = HashMap::new();
        let mut locals = HashMap::new();
        let mut array_locals = HashMap::new();
        for local in function.locals() {
            if matches!(
                type_arena.get(local.ty()).map(|record| record.kind()),
                Some(TypeKind::Primitive(PrimitiveType::Unit))
            ) {
                continue;
            }
            if let Some(local_type) = cranelift_type(local.ty(), type_arena) {
                locals.insert(local.id(), builder.declare_var(local_type));
            } else if let Some((element_type, length)) = array_shape(local.ty(), type_arena) {
                let element_type = cranelift_type(element_type, type_arena)
                    .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)?;
                for index in 0..length {
                    array_locals.insert((local.id(), index), builder.declare_var(element_type));
                }
            } else {
                return Err(CraneliftLoweringError::UnsupportedRuntimeType);
            }
        }
        for mir_block in function.blocks() {
            clif_blocks.insert(mir_block.id(), builder.create_block());
        }

        let entry_block = *clif_blocks
            .get(&function.blocks()[0].id())
            .ok_or(CraneliftLoweringError::UnsupportedFunctionShape)?;
        builder.append_block_params_for_function_params(entry_block);
        for ((value_id, _), value) in function
            .parameters()
            .iter()
            .zip(builder.block_params(entry_block).iter().copied())
        {
            values.insert(*value_id, value);
        }

        for mir_block in function.blocks() {
            let clif_block = *clif_blocks
                .get(&mir_block.id())
                .ok_or(CraneliftLoweringError::UnsupportedTerminator)?;
            builder.switch_to_block(clif_block);
            for instruction in mir_block.instructions() {
                lower_instruction(
                    instruction,
                    &mut builder,
                    &mut values,
                    &locals,
                    &array_locals,
                    &mut module,
                    function_ids,
                    &lowering_context,
                )?;
            }
            lower_terminator(mir_block.terminator(), &clif_blocks, &mut builder, &values)?;
        }
        for clif_block in clif_blocks.values().copied() {
            builder.seal_block(clif_block);
        }
        builder.finalize();
    }

    let flags = codegen_flags();
    verify_function(&clif_function, &flags)
        .map_err(|_| CraneliftLoweringError::VerificationFailed)?;
    Ok(clif_function)
}

fn mir_signature(
    function: &MirFunction,
    type_arena: &TypeArena,
    target: &Triple,
) -> Result<Signature, CraneliftLoweringError> {
    let mut signature = Signature::new(CallConv::triple_default(target));
    for (_, parameter_type) in function.parameters() {
        let parameter_type = cranelift_type(*parameter_type, type_arena)
            .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)?;
        signature.params.push(AbiParam::new(parameter_type));
    }
    if let Some(return_type) = cranelift_type(function.return_type(), type_arena) {
        signature.returns.push(AbiParam::new(return_type));
    }
    Ok(signature)
}

fn declare_runtime_functions(
    module: &mut ObjectModule,
    target: &Triple,
) -> Result<RuntimeFunctions, CraneliftLoweringError> {
    let mut malloc_signature = Signature::new(CallConv::triple_default(target));
    malloc_signature.params.push(AbiParam::new(types::I64));
    malloc_signature.returns.push(AbiParam::new(types::I64));
    let mut memory_signature = Signature::new(CallConv::triple_default(target));
    memory_signature.params.push(AbiParam::new(types::I64));
    memory_signature.params.push(AbiParam::new(types::I64));
    memory_signature.params.push(AbiParam::new(types::I64));
    memory_signature.returns.push(AbiParam::new(types::I64));
    let malloc = module
        .declare_function("malloc", Linkage::Import, &malloc_signature)
        .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
    let memcpy = module
        .declare_function("memcpy", Linkage::Import, &memory_signature)
        .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
    let memcmp = module
        .declare_function("memcmp", Linkage::Import, &memory_signature)
        .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
    Ok(RuntimeFunctions {
        malloc,
        memcpy,
        memcmp,
        call_conv: CallConv::triple_default(target),
    })
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

fn require_bootstrap_runtime_type(
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
                | PrimitiveType::String
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
        Some(TypeKind::Primitive(PrimitiveType::String)) => Some(types::I64),
        _ => None,
    }
}

fn array_shape(ty: TypeId, type_arena: &TypeArena) -> Option<(TypeId, u64)> {
    match type_arena.get(ty).map(|record| record.kind()) {
        Some(TypeKind::Array(array)) => Some((array.element(), array.length())),
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

fn normalize_bool_value(
    value: Value,
    ty: TypeId,
    type_arena: &TypeArena,
    builder: &mut FunctionBuilder<'_>,
) -> Value {
    if !matches!(
        type_arena.get(ty).map(|record| record.kind()),
        Some(TypeKind::Primitive(PrimitiveType::Bool))
    ) {
        return value;
    }
    let nonzero = builder.ins().icmp_imm(IntCC::NotEqual, value, 0);
    let one = builder.ins().iconst(types::I8, 1);
    let zero = builder.ins().iconst(types::I8, 0);
    builder.ins().select(nonzero, one, zero)
}

#[allow(clippy::too_many_arguments)]
fn lower_instruction(
    instruction: &MirInstruction,
    builder: &mut FunctionBuilder<'_>,
    values: &mut HashMap<MirValueId, Value>,
    locals: &HashMap<crate::mir::MirLocalId, Variable>,
    array_locals: &HashMap<(crate::mir::MirLocalId, u64), Variable>,
    module: &mut Option<&mut ObjectModule>,
    function_ids: &HashMap<crate::mir::MirFunctionId, FuncId>,
    context: &LoweringContext<'_>,
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
        MirInstruction::ParameterRead {
            output, parameter, ..
        } => {
            let value = values
                .get(&MirValueId::from_raw(parameter.index()))
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let ty = context
                .function
                .parameters()
                .get(parameter.index())
                .map(|(_, ty)| *ty)
                .ok_or(CraneliftLoweringError::MissingValue)?;
            values.insert(
                *output,
                normalize_bool_value(value, ty, context.type_arena, builder),
            );
            Ok(())
        }
        MirInstruction::DirectCall {
            output,
            callee,
            arguments,
            ..
        } => {
            let function_id = function_ids
                .get(callee)
                .copied()
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let module = module
                .as_deref_mut()
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let function_ref = module.declare_func_in_func(function_id, builder.func);
            let arguments = arguments
                .iter()
                .map(|argument| {
                    values
                        .get(argument)
                        .copied()
                        .ok_or(CraneliftLoweringError::MissingValue)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let call = builder.ins().call(function_ref, &arguments);
            if let Some(value) = builder.inst_results(call).first().copied() {
                let ty = context
                    .function_return_types
                    .get(callee)
                    .copied()
                    .ok_or(CraneliftLoweringError::MissingValue)?;
                values.insert(
                    *output,
                    normalize_bool_value(value, ty, context.type_arena, builder),
                );
            }
            Ok(())
        }
        MirInstruction::ArrayInit {
            local, elements, ..
        } => {
            for (index, value) in elements.iter().enumerate() {
                let variable = array_locals
                    .get(&(*local, index as u64))
                    .copied()
                    .ok_or(CraneliftLoweringError::MissingValue)?;
                let value = values
                    .get(value)
                    .copied()
                    .ok_or(CraneliftLoweringError::MissingValue)?;
                builder.def_var(variable, value);
            }
            Ok(())
        }
        MirInstruction::ArrayLoad {
            output,
            local,
            index,
            ..
        } => {
            let index_value = values
                .get(index)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let (_, length) = context
                .function
                .locals()
                .iter()
                .find(|candidate| candidate.id() == *local)
                .and_then(|candidate| array_shape(candidate.ty(), context.type_arena))
                .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)?;
            let negative = builder
                .ins()
                .icmp_imm(IntCC::SignedLessThan, index_value, 0);
            let too_large = builder.ins().icmp_imm(
                IntCC::UnsignedGreaterThanOrEqual,
                index_value,
                length as i64,
            );
            builder.ins().trapnz(negative, TrapCode::unwrap_user(3));
            builder.ins().trapnz(too_large, TrapCode::unwrap_user(3));
            let mut result = None;
            for position in (0..length).rev() {
                let variable = array_locals
                    .get(&(*local, position))
                    .copied()
                    .ok_or(CraneliftLoweringError::MissingValue)?;
                let value = builder.use_var(variable);
                let matches = builder
                    .ins()
                    .icmp_imm(IntCC::Equal, index_value, position as i64);
                result = Some(match result {
                    Some(current) => builder.ins().select(matches, value, current),
                    None => value,
                });
            }
            values.insert(*output, result.ok_or(CraneliftLoweringError::MissingValue)?);
            Ok(())
        }
        MirInstruction::ArrayStore {
            local,
            index,
            value,
            ..
        } => {
            let index_value = values
                .get(index)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let (_, length) = context
                .function
                .locals()
                .iter()
                .find(|candidate| candidate.id() == *local)
                .and_then(|candidate| array_shape(candidate.ty(), context.type_arena))
                .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)?;
            let negative = builder
                .ins()
                .icmp_imm(IntCC::SignedLessThan, index_value, 0);
            let too_large = builder.ins().icmp_imm(
                IntCC::UnsignedGreaterThanOrEqual,
                index_value,
                length as i64,
            );
            builder.ins().trapnz(negative, TrapCode::unwrap_user(3));
            builder.ins().trapnz(too_large, TrapCode::unwrap_user(3));
            let replacement = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            for position in 0..length {
                let variable = array_locals
                    .get(&(*local, position))
                    .copied()
                    .ok_or(CraneliftLoweringError::MissingValue)?;
                let current = builder.use_var(variable);
                let matches = builder
                    .ins()
                    .icmp_imm(IntCC::Equal, index_value, position as i64);
                let selected = builder.ins().select(matches, replacement, current);
                builder.def_var(variable, selected);
            }
            Ok(())
        }
        MirInstruction::StringConstant { output, bytes, .. } => {
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let function_ref = runtime.reference(builder.func, runtime.malloc, false);
            let size = builder.ins().iconst(
                types::I64,
                i64::try_from(bytes.len() + 8)
                    .map_err(|_| CraneliftLoweringError::UnsupportedInstruction)?,
            );
            let call = builder.ins().call(function_ref, &[size]);
            let pointer = *builder
                .inst_results(call)
                .first()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.ins().trapz(pointer, TrapCode::unwrap_user(4));
            let length = builder.ins().iconst(
                types::I64,
                i64::try_from(bytes.len())
                    .map_err(|_| CraneliftLoweringError::UnsupportedInstruction)?,
            );
            builder.ins().store(MemFlagsData::new(), length, pointer, 0);
            for (offset, byte) in bytes.iter().copied().enumerate() {
                let value = builder.ins().iconst(types::I8, i64::from(byte));
                builder.ins().store(
                    MemFlagsData::new(),
                    value,
                    pointer,
                    i32::try_from(offset + 8)
                        .map_err(|_| CraneliftLoweringError::UnsupportedInstruction)?,
                );
            }
            values.insert(*output, pointer);
            Ok(())
        }
        MirInstruction::StringLength { output, value, .. } => {
            let pointer = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            values.insert(
                *output,
                builder
                    .ins()
                    .load(types::I64, MemFlagsData::new(), pointer, 0),
            );
            Ok(())
        }
        MirInstruction::StringIndex {
            output,
            value,
            index,
            ..
        } => {
            let pointer = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let index = values
                .get(index)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let length = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 0);
            let negative = builder.ins().icmp_imm(IntCC::SignedLessThan, index, 0);
            let too_large = builder
                .ins()
                .icmp(IntCC::UnsignedGreaterThanOrEqual, index, length);
            builder.ins().trapnz(negative, TrapCode::unwrap_user(3));
            builder.ins().trapnz(too_large, TrapCode::unwrap_user(3));
            let data = builder.ins().iadd_imm(pointer, 8);
            let address = builder.ins().iadd(data, index);
            let byte = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), address, 0);
            values.insert(*output, builder.ins().ireduce(types::I8, byte));
            Ok(())
        }
        MirInstruction::StringClone { output, value, .. } => {
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let source = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let length = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), source, 0);
            let size = builder.ins().iadd_imm(length, 8);
            let malloc_ref = runtime.reference(builder.func, runtime.malloc, false);
            let allocation = builder.ins().call(malloc_ref, &[size]);
            let destination = *builder
                .inst_results(allocation)
                .first()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.ins().trapz(destination, TrapCode::unwrap_user(4));
            let memcpy_ref = runtime.reference(builder.func, runtime.memcpy, true);
            builder.ins().call(memcpy_ref, &[destination, source, size]);
            values.insert(*output, destination);
            Ok(())
        }
        MirInstruction::StringConcat {
            output,
            left,
            right,
            ..
        } => {
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let left = values
                .get(left)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let right = values
                .get(right)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let left_length = builder.ins().load(types::I64, MemFlagsData::new(), left, 0);
            let right_length = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), right, 0);
            let length = builder.ins().iadd(left_length, right_length);
            let size = builder.ins().iadd_imm(length, 8);
            let malloc_ref = runtime.reference(builder.func, runtime.malloc, false);
            let allocation = builder.ins().call(malloc_ref, &[size]);
            let destination = *builder
                .inst_results(allocation)
                .first()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.ins().trapz(destination, TrapCode::unwrap_user(4));
            builder
                .ins()
                .store(MemFlagsData::new(), length, destination, 0);
            let memcpy_ref = runtime.reference(builder.func, runtime.memcpy, true);
            let destination_data = builder.ins().iadd_imm(destination, 8);
            let left_data = builder.ins().iadd_imm(left, 8);
            builder
                .ins()
                .call(memcpy_ref, &[destination_data, left_data, left_length]);
            let right_destination = builder.ins().iadd(destination_data, left_length);
            let right_data = builder.ins().iadd_imm(right, 8);
            builder
                .ins()
                .call(memcpy_ref, &[right_destination, right_data, right_length]);
            values.insert(*output, destination);
            Ok(())
        }
        MirInstruction::StringCompare {
            output,
            left,
            right,
            negate,
            ..
        } => {
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let left = values
                .get(left)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let right = values
                .get(right)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let left_length = builder.ins().load(types::I64, MemFlagsData::new(), left, 0);
            let right_length = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), right, 0);
            let same_length = builder.ins().icmp(IntCC::Equal, left_length, right_length);
            let shorter_left =
                builder
                    .ins()
                    .icmp(IntCC::UnsignedLessThan, left_length, right_length);
            let compare_length = builder
                .ins()
                .select(shorter_left, left_length, right_length);
            let left_data = builder.ins().iadd_imm(left, 8);
            let right_data = builder.ins().iadd_imm(right, 8);
            let memcmp_ref = runtime.reference(builder.func, runtime.memcmp, true);
            let comparison = builder
                .ins()
                .call(memcmp_ref, &[left_data, right_data, compare_length]);
            let comparison = *builder
                .inst_results(comparison)
                .first()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let equal = builder.ins().icmp_imm(IntCC::Equal, comparison, 0);
            let equal = builder.ins().band(equal, same_length);
            let result = if *negate {
                builder.ins().bxor_imm(equal, 1)
            } else {
                equal
            };
            values.insert(*output, result);
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
            let local_id = *local;
            let variable = locals
                .get(&local_id)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let value = builder.use_var(variable);
            let ty = context
                .function
                .locals()
                .iter()
                .find(|candidate| candidate.id() == local_id)
                .map(|candidate| candidate.ty())
                .ok_or(CraneliftLoweringError::MissingValue)?;
            values.insert(
                *output,
                normalize_bool_value(value, ty, context.type_arena, builder),
            );
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
