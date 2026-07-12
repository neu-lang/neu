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
    function_parameter_types: &'a HashMap<crate::mir::MirFunctionId, Vec<TypeId>>,
    call_conv: CallConv,
    runtime: Option<&'a RuntimeFunctions>,
}

#[derive(Clone, Copy)]
struct RuntimeFunctions {
    malloc: FuncId,
    free: FuncId,
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
    let function_parameter_types = module
        .functions()
        .iter()
        .map(|function| {
            (
                function.id(),
                function
                    .parameters()
                    .iter()
                    .map(|(_, ty)| *ty)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    for function in module.functions() {
        let identity = function
            .symbol_identity()
            .ok_or(CraneliftLoweringError::MissingFunctionIdentity)?;
        let signature = mir_signature(function, type_arena, &target)?;
        let function_id = object_module
            .declare_function(
                &bootstrap_symbol(identity, function.parameters().iter().map(|(_, ty)| *ty)),
                Linkage::Local,
                &signature,
            )
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
            &function_parameter_types,
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
    let function_parameter_types = module
        .functions()
        .iter()
        .map(|function| {
            (
                function.id(),
                function
                    .parameters()
                    .iter()
                    .map(|(_, ty)| *ty)
                    .collect::<Vec<_>>(),
            )
        })
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
            bootstrap_symbol(identity, function.parameters().iter().map(|(_, ty)| *ty))
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
            &function_parameter_types,
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
        bootstrap_symbol(identity, function.parameters().iter().map(|(_, ty)| *ty))
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
        &HashMap::new(),
        runtime,
    )
}

#[allow(clippy::too_many_arguments)]
fn lower_mir_function_with_module(
    function: &MirFunction,
    type_arena: &TypeArena,
    target: &Triple,
    module: Option<&mut ObjectModule>,
    function_ids: &HashMap<crate::mir::MirFunctionId, FuncId>,
    function_return_types: &HashMap<crate::mir::MirFunctionId, TypeId>,
    function_parameter_types: &HashMap<crate::mir::MirFunctionId, Vec<TypeId>>,
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
    let mut aggregate_values = HashMap::new();
    let mut task_results = HashMap::new();
    let mut task_origins = HashMap::new();
    let mut task_locals = HashMap::new();
    let mut scope_tasks = Vec::new();

    {
        let mut builder = FunctionBuilder::new(&mut clif_function, &mut builder_context);
        let mut module = module;
        let lowering_context = LoweringContext {
            function,
            type_arena,
            function_return_types,
            function_parameter_types,
            call_conv: CallConv::triple_default(target),
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
        let block_params = builder.block_params(entry_block).to_vec();
        let mut parameter_cursor = 0usize;
        for (value_id, parameter_type) in function.parameters() {
            let mut flattened = Vec::new();
            flattened_cranelift_types(*parameter_type, type_arena, &mut flattened)?;
            let end = parameter_cursor + flattened.len();
            let parameter_values = block_params
                .get(parameter_cursor..end)
                .ok_or(CraneliftLoweringError::MissingValue)?
                .to_vec();
            parameter_cursor = end;
            if array_shape(*parameter_type, type_arena).is_some() {
                aggregate_values.insert(*value_id, parameter_values);
            } else if let Some(value) = parameter_values.first().copied() {
                values.insert(*value_id, value);
                if let Some(result) = task_result_type(*parameter_type, type_arena) {
                    task_results.insert(*value_id, result);
                }
            }
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
                    &mut aggregate_values,
                    &mut task_results,
                    &mut task_origins,
                    &mut task_locals,
                    &mut scope_tasks,
                    &locals,
                    &array_locals,
                    &mut module,
                    function_ids,
                    &lowering_context,
                )?;
            }
            lower_terminator(
                mir_block.terminator(),
                &clif_blocks,
                &mut builder,
                &values,
                &aggregate_values,
            )?;
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
        let mut flattened = Vec::new();
        flattened_cranelift_types(*parameter_type, type_arena, &mut flattened)?;
        signature
            .params
            .extend(flattened.into_iter().map(AbiParam::new));
    }
    let mut flattened = Vec::new();
    flattened_cranelift_types(function.return_type(), type_arena, &mut flattened)?;
    signature
        .returns
        .extend(flattened.into_iter().map(AbiParam::new));
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
    // The bootstrap object model passes interior pointers to cleanup. Keep the
    // compiler-owned free boundary local until a host runtime with typed
    // allocation metadata replaces it.
    let free = module
        .declare_function("neu_free", Linkage::Local, &malloc_signature)
        .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
    let memcpy = module
        .declare_function("memcpy", Linkage::Import, &memory_signature)
        .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
    let memcmp = module
        .declare_function("memcmp", Linkage::Import, &memory_signature)
        .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
    let mut context = cranelift_codegen::Context::new();
    context.func =
        Function::with_name_signature(UserFuncName::user(0, free.as_u32()), malloc_signature);
    let mut builder_context = FunctionBuilderContext::new();
    {
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let block = builder.create_block();
        builder.append_block_params_for_function_params(block);
        builder.switch_to_block(block);
        let pointer = builder.block_params(block)[0];
        builder.ins().return_(&[pointer]);
        builder.seal_block(block);
        builder.finalize();
    }
    module
        .define_function(free, &mut context)
        .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;

    Ok(RuntimeFunctions {
        malloc,
        free,
        memcpy,
        memcmp,
        call_conv: CallConv::triple_default(target),
    })
}

fn bootstrap_symbol(
    identity: &crate::module::FunctionSymbolIdentity,
    parameter_types: impl IntoIterator<Item = crate::types::TypeId>,
) -> String {
    let parameters = parameter_types
        .into_iter()
        .map(|ty| ty.index().to_string())
        .collect::<Vec<_>>()
        .join("_");
    format!(
        "neu_fn_{}_{}_{}_p{}",
        encode_symbol_component(identity.module().as_str()),
        encode_symbol_component(identity.package().as_str()),
        encode_symbol_component(identity.name()),
        parameters,
    )
}

pub fn bootstrap_specialized_symbol(
    identity: &crate::module::FunctionSymbolIdentity,
    specialization: &crate::types::GenericSpecializationIdentity,
    parameter_types: impl IntoIterator<Item = crate::types::TypeId>,
) -> String {
    format!(
        "{}_s{}",
        bootstrap_symbol(identity, parameter_types),
        encode_symbol_component(&specialization.mangle("specialization")),
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
        )) | Some(TypeKind::Array(_))
            | Some(TypeKind::DynamicArray(_))
            | Some(TypeKind::Task(_))
            | Some(TypeKind::Channel(_))
            | Some(TypeKind::ChannelResult(_))
            | Some(TypeKind::Nominal(_))
            | Some(TypeKind::GenericInstance(_))
            | Some(TypeKind::Function(_))
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
        Some(TypeKind::Nominal(_) | TypeKind::GenericInstance(_) | TypeKind::Function(_)) => {
            Some(types::I64)
        }
        Some(TypeKind::DynamicArray(_)) => Some(types::I64),
        Some(TypeKind::Task(_)) => Some(types::I64),
        Some(TypeKind::Channel(_)) | Some(TypeKind::ChannelResult(_)) => Some(types::I64),
        _ => None,
    }
}

fn flattened_cranelift_types(
    ty: TypeId,
    type_arena: &TypeArena,
    output: &mut Vec<types::Type>,
) -> Result<(), CraneliftLoweringError> {
    match type_arena.get(ty).map(|record| record.kind()) {
        Some(TypeKind::Array(array)) => {
            for _ in 0..array.length() {
                flattened_cranelift_types(array.element(), type_arena, output)?;
            }
            Ok(())
        }
        Some(TypeKind::Primitive(PrimitiveType::Unit)) => Ok(()),
        Some(_) => {
            output.push(
                cranelift_type(ty, type_arena)
                    .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)?,
            );
            Ok(())
        }
        None => Err(CraneliftLoweringError::UnsupportedRuntimeType),
    }
}

fn array_shape(ty: TypeId, type_arena: &TypeArena) -> Option<(TypeId, u64)> {
    match type_arena.get(ty).map(|record| record.kind()) {
        Some(TypeKind::Array(array)) => Some((array.element(), array.length())),
        _ => None,
    }
}

fn task_result_type(ty: TypeId, type_arena: &TypeArena) -> Option<TypeId> {
    match type_arena.get(ty).map(|record| record.kind()) {
        Some(TypeKind::Task(task)) => Some(task.result()),
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
    aggregate_values: &mut HashMap<MirValueId, Vec<Value>>,
    task_results: &mut HashMap<MirValueId, TypeId>,
    task_origins: &mut HashMap<MirValueId, MirValueId>,
    task_locals: &mut HashMap<crate::mir::MirLocalId, MirValueId>,
    scope_tasks: &mut Vec<Vec<MirValueId>>,
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
        MirInstruction::EnumConstruct {
            output,
            tag,
            payload,
            ..
        } => {
            let payload = values
                .get(payload)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let shift = builder.ins().iconst(types::I64, 8);
            let packed = builder.ins().ishl(payload, shift);
            let tag = builder.ins().iconst(types::I64, *tag);
            values.insert(*output, builder.ins().iadd(packed, tag));
            Ok(())
        }
        MirInstruction::EnumPayload {
            output,
            value,
            index,
            ..
        } => {
            if *index != 0 {
                return Err(CraneliftLoweringError::UnsupportedInstruction);
            }
            let value = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            values.insert(*output, builder.ins().sshr_imm(value, 8));
            Ok(())
        }
        MirInstruction::EnumTag { output, value, .. } => {
            let value = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let mask = builder.ins().iconst(types::I64, 0xff);
            values.insert(*output, builder.ins().band(value, mask));
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
        MirInstruction::Suspend { .. } | MirInstruction::Resume { .. } => Ok(()),
        MirInstruction::ScopeEnter { .. } => {
            scope_tasks.push(Vec::new());
            Ok(())
        }
        MirInstruction::ScopeExit { .. } => {
            let handles = scope_tasks.pop().unwrap_or_default();
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let free_ref = runtime.reference(builder.func, runtime.free, false);
            for origin in handles {
                if let Some(handle) = values.get(&origin).copied() {
                    builder.ins().call(free_ref, &[handle]);
                }
                task_results.remove(&origin);
                task_origins.retain(|_, candidate| *candidate != origin);
                task_locals.retain(|_, candidate| *candidate != origin);
            }
            Ok(())
        }
        MirInstruction::ParameterRead {
            output, parameter, ..
        } => {
            let parameter_id = MirValueId::from_raw(parameter.index());
            if let Some(aggregate) = aggregate_values.get(&parameter_id).cloned() {
                aggregate_values.insert(*output, aggregate);
                return Ok(());
            }
            let value = values
                .get(&parameter_id)
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
            if let Some(result) = task_result_type(ty, context.type_arena) {
                task_results.insert(*output, result);
                task_origins.insert(*output, parameter_id);
            }
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
            let mut call_arguments = Vec::new();
            for argument in arguments {
                if let Some(aggregate) = aggregate_values.get(argument) {
                    call_arguments.extend(aggregate.iter().copied());
                } else if let Some(value) = values.get(argument).copied() {
                    call_arguments.push(value);
                }
            }
            let call = builder.ins().call(function_ref, &call_arguments);
            let results = builder.inst_results(call).to_vec();
            let ty = context
                .function_return_types
                .get(callee)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            if array_shape(ty, context.type_arena).is_some() {
                aggregate_values.insert(*output, results);
            } else if let Some(value) = results.first().copied() {
                values.insert(
                    *output,
                    normalize_bool_value(value, ty, context.type_arena, builder),
                );
            }
            Ok(())
        }
        MirInstruction::TaskSpawn { output, callee, .. } => {
            let function_id = function_ids
                .get(callee)
                .copied()
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let result_type = context
                .function_return_types
                .get(callee)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let module = module
                .as_deref_mut()
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let function_ref = module.declare_func_in_func(function_id, builder.func);
            let call = builder.ins().call(function_ref, &[]);
            let result = builder.inst_results(call).first().copied();
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let size = builder.ins().iconst(types::I64, 16);
            let malloc_ref = runtime.reference(builder.func, runtime.malloc, false);
            let allocation = builder.ins().call(malloc_ref, &[size]);
            let handle = *builder
                .inst_results(allocation)
                .first()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.ins().trapz(handle, TrapCode::unwrap_user(7));
            let status = builder.ins().iconst(types::I8, 0);
            builder.ins().store(MemFlagsData::new(), status, handle, 0);
            if let Some(result) = result {
                builder.ins().store(MemFlagsData::new(), result, handle, 8);
            }
            values.insert(*output, handle);
            task_results.insert(*output, result_type);
            task_origins.insert(*output, *output);
            if let Some(scope) = scope_tasks.last_mut() {
                scope.push(*output);
            }
            Ok(())
        }
        MirInstruction::TaskAwait { output, task, .. } => {
            let handle = values
                .get(task)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let result_type = task_results
                .get(task)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let origin = task_origins.get(task).copied().unwrap_or(*task);
            let status = builder
                .ins()
                .load(types::I8, MemFlagsData::new(), handle, 0);
            builder.ins().trapnz(status, TrapCode::unwrap_user(8));
            if !matches!(
                context
                    .type_arena
                    .get(result_type)
                    .map(|record| record.kind()),
                Some(TypeKind::Primitive(PrimitiveType::Unit))
            ) {
                let value_type = cranelift_type(result_type, context.type_arena)
                    .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)?;
                let value = builder
                    .ins()
                    .load(value_type, MemFlagsData::new(), handle, 8);
                values.insert(
                    *output,
                    normalize_bool_value(value, result_type, context.type_arena, builder),
                );
            }
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let free_ref = runtime.reference(builder.func, runtime.free, false);
            builder.ins().call(free_ref, &[handle]);
            task_results.remove(task);
            task_results.remove(&origin);
            task_origins.remove(task);
            task_origins.remove(&origin);
            for scope in scope_tasks.iter_mut() {
                scope.retain(|candidate| *candidate != origin);
            }
            Ok(())
        }
        MirInstruction::TaskCancel { task, .. } => {
            let handle = values
                .get(task)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let status = builder.ins().iconst(types::I8, 1);
            builder.ins().store(MemFlagsData::new(), status, handle, 0);
            Ok(())
        }
        MirInstruction::ChannelCreate {
            output, capacity, ..
        } => {
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let capacity = values
                .get(capacity)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let zero = builder.ins().iconst(types::I64, 0);
            let negative = builder.ins().icmp(IntCC::SignedLessThan, capacity, zero);
            builder.ins().trapnz(negative, TrapCode::unwrap_user(10));
            let header_size = builder.ins().iconst(types::I64, 48);
            let malloc_ref = runtime.reference(builder.func, runtime.malloc, false);
            let header_call = builder.ins().call(malloc_ref, &[header_size]);
            let header = *builder
                .inst_results(header_call)
                .first()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.ins().trapz(header, TrapCode::unwrap_user(11));
            let has_capacity = builder.ins().icmp(IntCC::NotEqual, capacity, zero);
            let one = builder.ins().iconst(types::I64, 1);
            let data_count = builder.ins().select(has_capacity, capacity, one);
            let data_size = builder.ins().imul_imm(data_count, 8);
            let data_call = builder.ins().call(malloc_ref, &[data_size]);
            let data = *builder
                .inst_results(data_call)
                .first()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.ins().trapz(data, TrapCode::unwrap_user(11));
            builder
                .ins()
                .store(MemFlagsData::new(), capacity, header, 0);
            builder.ins().store(MemFlagsData::new(), zero, header, 8);
            builder.ins().store(MemFlagsData::new(), zero, header, 16);
            builder.ins().store(MemFlagsData::new(), zero, header, 24);
            builder.ins().store(MemFlagsData::new(), zero, header, 32);
            builder.ins().store(MemFlagsData::new(), data, header, 40);
            values.insert(*output, header);
            Ok(())
        }
        MirInstruction::ChannelSend {
            channel,
            value,
            element_type,
            ..
        } => {
            let pointer = values
                .get(channel)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let value = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let closed = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 32);
            builder.ins().trapnz(closed, TrapCode::unwrap_user(12));
            let capacity = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 0);
            let length = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 8);
            let has_capacity = builder.ins().icmp_imm(IntCC::NotEqual, capacity, 0);
            let full = builder
                .ins()
                .icmp(IntCC::UnsignedGreaterThanOrEqual, length, capacity);
            let full_and_bounded = builder.ins().band(has_capacity, full);
            builder
                .ins()
                .trapnz(full_and_bounded, TrapCode::unwrap_user(13));
            let data = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 40);
            let tail = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 24);
            let offset = builder.ins().imul_imm(tail, 8);
            let address = builder.ins().iadd(data, offset);
            let element_clif = cranelift_type(*element_type, context.type_arena)
                .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)?;
            builder.ins().store(MemFlagsData::new(), value, address, 0);
            let next_raw = builder.ins().iadd_imm(tail, 1);
            let one = builder.ins().iconst(types::I64, 1);
            let safe_capacity = builder.ins().select(has_capacity, capacity, one);
            let next_mod = builder.ins().urem(next_raw, safe_capacity);
            let zero = builder.ins().iconst(types::I64, 0);
            let next_tail = builder.ins().select(has_capacity, next_mod, zero);
            builder
                .ins()
                .store(MemFlagsData::new(), next_tail, pointer, 24);
            let new_length = builder.ins().iadd_imm(length, 1);
            builder
                .ins()
                .store(MemFlagsData::new(), new_length, pointer, 8);
            let _ = element_clif;
            Ok(())
        }
        MirInstruction::ChannelReceive {
            output,
            channel,
            element_type,
            ..
        } => {
            let pointer = values
                .get(channel)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let length = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 8);
            let closed = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 32);
            let empty = builder.ins().icmp_imm(IntCC::Equal, length, 0);
            let open = builder.ins().icmp_imm(IntCC::Equal, closed, 0);
            let empty_and_open = builder.ins().band(empty, open);
            builder
                .ins()
                .trapnz(empty_and_open, TrapCode::unwrap_user(14));
            let data = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 40);
            let head = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 16);
            let offset = builder.ins().imul_imm(head, 8);
            let address = builder.ins().iadd(data, offset);
            let element_clif = cranelift_type(*element_type, context.type_arena)
                .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)?;
            let loaded = builder
                .ins()
                .load(element_clif, MemFlagsData::new(), address, 0);
            let loaded = if element_clif == types::I8 {
                builder.ins().uextend(types::I64, loaded)
            } else if element_clif == types::I64 {
                loaded
            } else {
                return Err(CraneliftLoweringError::UnsupportedRuntimeType);
            };
            let zero = builder.ins().iconst(types::I64, 0);
            let payload = builder.ins().select(empty, zero, loaded);
            let one = builder.ins().iconst(types::I64, 1);
            let tag = builder.ins().select(empty, one, zero);
            let shifted_payload = builder.ins().ishl_imm(payload, 8);
            let result = builder.ins().iadd(shifted_payload, tag);
            values.insert(*output, result);
            let next_raw = builder.ins().iadd_imm(head, 1);
            let capacity = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 0);
            let has_capacity = builder.ins().icmp_imm(IntCC::NotEqual, capacity, 0);
            let one = builder.ins().iconst(types::I64, 1);
            let safe_capacity = builder.ins().select(has_capacity, capacity, one);
            let next_mod = builder.ins().urem(next_raw, safe_capacity);
            let zero = builder.ins().iconst(types::I64, 0);
            let next_head = builder.ins().select(has_capacity, next_mod, zero);
            builder
                .ins()
                .store(MemFlagsData::new(), next_head, pointer, 16);
            let remaining = builder.ins().iadd_imm(length, -1);
            let new_length = builder.ins().select(empty, length, remaining);
            builder
                .ins()
                .store(MemFlagsData::new(), new_length, pointer, 8);
            Ok(())
        }
        MirInstruction::ChannelClose { channel, .. } => {
            let pointer = values
                .get(channel)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let one = builder.ins().iconst(types::I64, 1);
            builder.ins().store(MemFlagsData::new(), one, pointer, 32);
            Ok(())
        }
        MirInstruction::DestroyChannel { value, .. } => {
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let pointer = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let data = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 40);
            let free_ref = runtime.reference(builder.func, runtime.free, false);
            builder.ins().call(free_ref, &[data]);
            builder.ins().call(free_ref, &[pointer]);
            Ok(())
        }
        MirInstruction::FunctionReference { output, callee, .. } => {
            let function_id = function_ids
                .get(callee)
                .copied()
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let module = module
                .as_deref_mut()
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let function_ref = module.declare_func_in_func(function_id, builder.func);
            values.insert(*output, builder.ins().func_addr(types::I64, function_ref));
            Ok(())
        }
        MirInstruction::IndirectCall {
            output,
            callee,
            function_type,
            arguments,
            ..
        } => {
            let function = context
                .type_arena
                .get(*function_type)
                .and_then(|record| match record.kind() {
                    TypeKind::Function(function) => Some(function),
                    _ => None,
                })
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let mut signature = Signature::new(context.call_conv);
            for parameter_type in function.parameters() {
                let mut parameter_types = Vec::new();
                flattened_cranelift_types(
                    *parameter_type,
                    context.type_arena,
                    &mut parameter_types,
                )?;
                signature
                    .params
                    .extend(parameter_types.into_iter().map(AbiParam::new));
            }
            let mut return_types = Vec::new();
            flattened_cranelift_types(
                function.return_type(),
                context.type_arena,
                &mut return_types,
            )?;
            signature
                .returns
                .extend(return_types.into_iter().map(AbiParam::new));
            let signature_ref = builder.import_signature(signature);
            let callee = values
                .get(callee)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let mut call_arguments = Vec::new();
            for argument in arguments {
                if let Some(aggregate) = aggregate_values.get(argument) {
                    call_arguments.extend(aggregate.iter().copied());
                } else if let Some(value) = values.get(argument).copied() {
                    call_arguments.push(value);
                }
            }
            let call = builder
                .ins()
                .call_indirect(signature_ref, callee, &call_arguments);
            let results = builder.inst_results(call).to_vec();
            if array_shape(function.return_type(), context.type_arena).is_some() {
                aggregate_values.insert(*output, results);
            } else if let Some(value) = results.first().copied() {
                values.insert(
                    *output,
                    normalize_bool_value(
                        value,
                        function.return_type(),
                        context.type_arena,
                        builder,
                    ),
                );
            }
            Ok(())
        }
        MirInstruction::VirtualCall {
            output,
            arguments,
            targets,
            ..
        }
        | MirInstruction::InterfaceCall {
            output,
            arguments,
            targets,
            ..
        } => {
            let module = module
                .as_deref_mut()
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let receiver = arguments
                .first()
                .and_then(|value| values.get(value).copied())
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let first = targets
                .first()
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let parameter_types = context
                .function_parameter_types
                .get(&first.callee())
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let return_type = context
                .function_return_types
                .get(&first.callee())
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let mut signature = Signature::new(context.call_conv);
            for parameter_type in parameter_types {
                let mut parameter_types = Vec::new();
                flattened_cranelift_types(
                    *parameter_type,
                    context.type_arena,
                    &mut parameter_types,
                )?;
                signature
                    .params
                    .extend(parameter_types.into_iter().map(AbiParam::new));
            }
            let mut return_types = Vec::new();
            flattened_cranelift_types(return_type, context.type_arena, &mut return_types)?;
            signature
                .returns
                .extend(return_types.into_iter().map(AbiParam::new));
            let signature_ref = builder.import_signature(signature);
            let mut selected = None;
            let mut matched = builder.ins().iconst(types::I8, 0);
            let tag = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), receiver, -8);
            for target in targets {
                let function_id = function_ids
                    .get(&target.callee())
                    .copied()
                    .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
                let function_ref = module.declare_func_in_func(function_id, builder.func);
                let address = builder.ins().func_addr(types::I64, function_ref);
                let expected = builder.ins().iconst(
                    types::I64,
                    i64::try_from(target.receiver_type().index()).unwrap_or(0),
                );
                let condition = builder.ins().icmp(IntCC::Equal, tag, expected);
                selected = Some(match selected {
                    Some(previous) => builder.ins().select(condition, address, previous),
                    None => address,
                });
                let one = builder.ins().iconst(types::I8, 1);
                matched = builder.ins().select(condition, one, matched);
            }
            let selected = selected.ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            builder.ins().trapz(matched, TrapCode::unwrap_user(5));
            let mut call_arguments = Vec::new();
            for argument in arguments {
                if let Some(aggregate) = aggregate_values.get(argument) {
                    call_arguments.extend(aggregate.iter().copied());
                } else if let Some(value) = values.get(argument).copied() {
                    call_arguments.push(value);
                }
            }
            let call = builder
                .ins()
                .call_indirect(signature_ref, selected, &call_arguments);
            let results = builder.inst_results(call).to_vec();
            if array_shape(return_type, context.type_arena).is_some() {
                aggregate_values.insert(*output, results);
            } else if let Some(value) = results.first().copied() {
                values.insert(
                    *output,
                    normalize_bool_value(value, return_type, context.type_arena, builder),
                );
            }
            Ok(())
        }
        MirInstruction::StaticSuperCall {
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
            let mut call_arguments = Vec::new();
            for argument in arguments {
                if let Some(value) = values.get(argument).copied() {
                    call_arguments.push(value);
                }
            }
            let call = builder.ins().call(function_ref, &call_arguments);
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
        MirInstruction::ArrayValue { output, local, .. } => {
            let (_, length) = context
                .function
                .locals()
                .iter()
                .find(|candidate| candidate.id() == *local)
                .and_then(|candidate| array_shape(candidate.ty(), context.type_arena))
                .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)?;
            let mut result = Vec::new();
            for position in 0..length {
                let variable = array_locals
                    .get(&(*local, position))
                    .copied()
                    .ok_or(CraneliftLoweringError::MissingValue)?;
                result.push(builder.use_var(variable));
            }
            aggregate_values.insert(*output, result);
            Ok(())
        }
        MirInstruction::NewObject {
            output,
            type_id,
            arguments,
            ..
        } => {
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let word_count = i64::try_from(arguments.len().max(1))
                .map_err(|_| CraneliftLoweringError::UnsupportedInstruction)?;
            let size = builder.ins().iconst(types::I64, (word_count + 2) * 8);
            let malloc_ref = runtime.reference(builder.func, runtime.malloc, false);
            let allocation = builder.ins().call(malloc_ref, &[size]);
            let allocation = *builder
                .inst_results(allocation)
                .first()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.ins().trapz(allocation, TrapCode::unwrap_user(4));
            builder
                .ins()
                .store(MemFlagsData::new(), size, allocation, 0);
            let type_tag = builder
                .ins()
                .iconst(types::I64, i64::try_from(type_id.index()).unwrap_or(0));
            builder
                .ins()
                .store(MemFlagsData::new(), type_tag, allocation, 8);
            let pointer = builder.ins().iadd_imm(allocation, 16);
            for (index, argument) in arguments.iter().enumerate() {
                let value = values
                    .get(argument)
                    .copied()
                    .ok_or(CraneliftLoweringError::MissingValue)?;
                let offset = i32::try_from(index * 8)
                    .map_err(|_| CraneliftLoweringError::UnsupportedInstruction)?;
                builder
                    .ins()
                    .store(MemFlagsData::new(), value, pointer, offset);
            }
            values.insert(*output, pointer);
            Ok(())
        }
        MirInstruction::DestroyObject { value, .. } => {
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let pointer = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let allocation = builder.ins().iadd_imm(pointer, -16);
            let free_ref = runtime.reference(builder.func, runtime.free, false);
            builder.ins().call(free_ref, &[allocation]);
            Ok(())
        }
        MirInstruction::DestroyDynamicArray { value, .. } => {
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let pointer = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let data = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 16);
            let data_allocation = builder.ins().iadd_imm(data, -16);
            let free_ref = runtime.reference(builder.func, runtime.free, false);
            builder.ins().call(free_ref, &[data_allocation]);
            let header_allocation = builder.ins().iadd_imm(pointer, -8);
            builder.ins().call(free_ref, &[header_allocation]);
            Ok(())
        }
        MirInstruction::DynamicArrayNew { output, .. } => {
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let size = builder.ins().iconst(types::I64, 32);
            let malloc_ref = runtime.reference(builder.func, runtime.malloc, false);
            let allocation = builder.ins().call(malloc_ref, &[size]);
            let allocation = *builder
                .inst_results(allocation)
                .first()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.ins().trapz(allocation, TrapCode::unwrap_user(6));
            builder
                .ins()
                .store(MemFlagsData::new(), size, allocation, 0);
            let pointer = builder.ins().iadd_imm(allocation, 8);
            let zero = builder.ins().iconst(types::I64, 0);
            let capacity = builder.ins().iconst(types::I64, 4);
            let data_size = builder.ins().iconst(types::I64, 48);
            let data_alloc = builder.ins().call(malloc_ref, &[data_size]);
            let data_alloc = *builder
                .inst_results(data_alloc)
                .first()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.ins().trapz(data_alloc, TrapCode::unwrap_user(6));
            builder
                .ins()
                .store(MemFlagsData::new(), data_size, data_alloc, 0);
            let data = builder.ins().iadd_imm(data_alloc, 16);
            builder.ins().store(MemFlagsData::new(), zero, pointer, 0);
            builder
                .ins()
                .store(MemFlagsData::new(), capacity, pointer, 8);
            builder.ins().store(MemFlagsData::new(), data, pointer, 16);
            values.insert(*output, pointer);
            Ok(())
        }
        MirInstruction::DynamicArraySize { output, array, .. } => {
            let pointer = values
                .get(array)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let length = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 0);
            values.insert(*output, length);
            Ok(())
        }
        MirInstruction::DynamicArrayLoad {
            output,
            array,
            index,
            ..
        } => {
            let pointer = values
                .get(array)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let index = values
                .get(index)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let length = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 0);
            let invalid = builder
                .ins()
                .icmp(IntCC::UnsignedGreaterThanOrEqual, index, length);
            builder.ins().trapnz(invalid, TrapCode::unwrap_user(7));
            let data = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 16);
            let offset = builder.ins().imul_imm(index, 8);
            let address = builder.ins().iadd(data, offset);
            values.insert(
                *output,
                builder
                    .ins()
                    .load(types::I64, MemFlagsData::new(), address, 0),
            );
            Ok(())
        }
        MirInstruction::DynamicArrayAdd {
            array,
            value,
            index,
            ..
        } => {
            let runtime = context
                .runtime
                .ok_or(CraneliftLoweringError::UnsupportedInstruction)?;
            let pointer = values
                .get(array)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let value = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let length = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 0);
            let capacity = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 8);
            let position = if let Some(index) = index {
                let index = values
                    .get(index)
                    .copied()
                    .ok_or(CraneliftLoweringError::MissingValue)?;
                let invalid = builder
                    .ins()
                    .icmp(IntCC::UnsignedGreaterThan, index, length);
                builder.ins().trapnz(invalid, TrapCode::unwrap_user(7));
                index
            } else {
                length
            };
            let old_data = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 16);
            let new_capacity = builder.ins().imul_imm(capacity, 2);
            let new_data_size = builder.ins().imul_imm(new_capacity, 8);
            let new_size = builder.ins().iadd_imm(new_data_size, 16);
            let malloc_ref = runtime.reference(builder.func, runtime.malloc, false);
            let allocation = builder.ins().call(malloc_ref, &[new_size]);
            let allocation = *builder
                .inst_results(allocation)
                .first()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.ins().trapz(allocation, TrapCode::unwrap_user(8));
            builder
                .ins()
                .store(MemFlagsData::new(), new_size, allocation, 0);
            let data = builder.ins().iadd_imm(allocation, 16);
            let copy_size = builder.ins().imul_imm(length, 8);
            let memcpy_ref = runtime.reference(builder.func, runtime.memcpy, true);
            builder.ins().call(memcpy_ref, &[data, old_data, copy_size]);
            let free_ref = runtime.reference(builder.func, runtime.free, false);
            let old_data_allocation = builder.ins().iadd_imm(old_data, -16);
            builder.ins().call(free_ref, &[old_data_allocation]);
            builder
                .ins()
                .store(MemFlagsData::new(), new_capacity, pointer, 8);
            builder.ins().store(MemFlagsData::new(), data, pointer, 16);
            if index.is_some() {
                for destination_index in (1_i64..4).rev() {
                    let source_index = destination_index - 1;
                    let source = builder.ins().load(
                        types::I64,
                        MemFlagsData::new(),
                        data,
                        i32::try_from(source_index * 8).unwrap_or(0),
                    );
                    let current = builder.ins().load(
                        types::I64,
                        MemFlagsData::new(),
                        data,
                        i32::try_from(destination_index * 8).unwrap_or(0),
                    );
                    let destination = values
                        .get(index.as_ref().expect("indexed add has index"))
                        .copied()
                        .ok_or(CraneliftLoweringError::MissingValue)?;
                    let index_in_range = builder.ins().icmp_imm(
                        IntCC::UnsignedGreaterThan,
                        destination,
                        source_index,
                    );
                    let length_in_range = builder.ins().icmp_imm(
                        IntCC::UnsignedLessThanOrEqual,
                        length,
                        destination_index,
                    );
                    let should_shift = builder.ins().band(index_in_range, length_in_range);
                    let selected = builder.ins().select(should_shift, source, current);
                    builder.ins().store(
                        MemFlagsData::new(),
                        selected,
                        data,
                        i32::try_from(destination_index * 8).unwrap_or(0),
                    );
                }
            }
            let offset = builder.ins().imul_imm(position, 8);
            let destination = builder.ins().iadd(data, offset);
            builder
                .ins()
                .store(MemFlagsData::new(), value, destination, 0);
            let new_length = builder.ins().iadd_imm(length, 1);
            builder
                .ins()
                .store(MemFlagsData::new(), new_length, pointer, 0);
            Ok(())
        }
        MirInstruction::DynamicArrayRemove { array, index, .. } => {
            let pointer = values
                .get(array)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let index = values
                .get(index)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let length = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 0);
            let invalid = builder
                .ins()
                .icmp(IntCC::UnsignedGreaterThanOrEqual, index, length);
            builder.ins().trapnz(invalid, TrapCode::unwrap_user(9));
            let data = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, 16);
            for destination_index in 0_i64..3 {
                let source = builder.ins().load(
                    types::I64,
                    MemFlagsData::new(),
                    data,
                    i32::try_from((destination_index + 1) * 8).unwrap_or(0),
                );
                let current = builder.ins().load(
                    types::I64,
                    MemFlagsData::new(),
                    data,
                    i32::try_from(destination_index * 8).unwrap_or(0),
                );
                let after_index = builder.ins().icmp_imm(
                    IntCC::UnsignedGreaterThanOrEqual,
                    index,
                    destination_index,
                );
                let before_last = builder.ins().icmp_imm(
                    IntCC::UnsignedGreaterThan,
                    length,
                    destination_index + 1,
                );
                let should_shift = builder.ins().band(after_index, before_last);
                let selected = builder.ins().select(should_shift, source, current);
                builder.ins().store(
                    MemFlagsData::new(),
                    selected,
                    data,
                    i32::try_from(destination_index * 8).unwrap_or(0),
                );
            }
            let new_length = builder.ins().iadd_imm(length, -1);
            builder
                .ins()
                .store(MemFlagsData::new(), new_length, pointer, 0);
            Ok(())
        }
        MirInstruction::FieldLoad {
            output,
            receiver,
            index,
            ..
        } => {
            let pointer = values
                .get(receiver)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let offset = i32::try_from(index * 8)
                .map_err(|_| CraneliftLoweringError::UnsupportedInstruction)?;
            let value = builder
                .ins()
                .load(types::I64, MemFlagsData::new(), pointer, offset);
            values.insert(*output, value);
            Ok(())
        }
        MirInstruction::FieldStore {
            receiver,
            index,
            value,
            ..
        } => {
            let pointer = values
                .get(receiver)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let value = values
                .get(value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let offset = i32::try_from(index * 8)
                .map_err(|_| CraneliftLoweringError::UnsupportedInstruction)?;
            builder
                .ins()
                .store(MemFlagsData::new(), value, pointer, offset);
            Ok(())
        }
        MirInstruction::ArrayAssign { local, value, .. } => {
            let source = aggregate_values
                .get(value)
                .cloned()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let (_, length) = context
                .function
                .locals()
                .iter()
                .find(|candidate| candidate.id() == *local)
                .and_then(|candidate| array_shape(candidate.ty(), context.type_arena))
                .ok_or(CraneliftLoweringError::UnsupportedRuntimeType)?;
            if source.len() != usize::try_from(length).unwrap_or(usize::MAX) {
                return Err(CraneliftLoweringError::MissingValue);
            }
            for (position, value) in source.into_iter().enumerate() {
                let variable = array_locals
                    .get(&(*local, position as u64))
                    .copied()
                    .ok_or(CraneliftLoweringError::MissingValue)?;
                builder.def_var(variable, value);
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
        MirInstruction::ArrayElementLoad {
            output,
            array,
            index,
            ..
        } => {
            let index_value = values
                .get(index)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let source = aggregate_values
                .get(array)
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let negative = builder
                .ins()
                .icmp_imm(IntCC::SignedLessThan, index_value, 0);
            builder.ins().trapnz(negative, TrapCode::unwrap_user(3));
            let mut result = None;
            for (position, value) in source.iter().copied().enumerate().rev() {
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
            if let Some(result) = task_result_type(ty, context.type_arena) {
                task_results.insert(*output, result);
                if let Some(origin) = task_locals.get(&local_id).copied() {
                    task_origins.insert(*output, origin);
                }
            }
            Ok(())
        }
        MirInstruction::StoreLocal {
            local,
            value: source_value,
            ..
        } => {
            let local_id = *local;
            let local = locals
                .get(local)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            let value = values
                .get(source_value)
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            builder.def_var(local, value);
            if let Some(origin) = task_origins.get(source_value).copied() {
                task_locals.insert(local_id, origin);
            } else {
                task_locals.remove(&local_id);
            }
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
    aggregate_values: &HashMap<MirValueId, Vec<Value>>,
) -> Result<(), CraneliftLoweringError> {
    match terminator {
        MirTerminator::Return { value, .. } => {
            if let Some(aggregate) = aggregate_values.get(&value) {
                builder.ins().return_(aggregate);
            } else {
                let value = values
                    .get(&value)
                    .copied()
                    .ok_or(CraneliftLoweringError::MissingValue)?;
                builder.ins().return_(&[value]);
            }
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
        MirTerminator::Trap { .. } => {
            builder.ins().trap(TrapCode::unwrap_user(6));
            Ok(())
        }
    }
}
