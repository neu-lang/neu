use crate::{
    hir::{
        HirBinaryOperator, HirCapture, HirDirectCall, HirDispatchKind, HirExpressionId,
        HirExpressionKind, HirFunction, HirLocalId, HirModule, HirUnaryOperator,
    },
    hir::{HirControlFlow, HirExpression},
    module::{FunctionSymbolIdentity, ModuleName},
    ownership_effects::OwnershipEffectContract,
    source::ByteSpan,
    types::{GenericSpecializationIdentity, PrimitiveType, TypeArena, TypeId, TypeKind},
};
use std::collections::{HashMap, HashSet};

macro_rules! mir_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(usize);
        impl $name {
            pub fn from_raw(raw: usize) -> Self {
                Self(raw)
            }
            pub fn index(self) -> usize {
                self.0
            }
        }
    };
}
mir_id!(MirFunctionId);
mir_id!(MirValueId);
mir_id!(MirParameterId);
mir_id!(MirLocalId);
mir_id!(MirBlockId);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MirModule {
    name: ModuleName,
    functions: Vec<MirFunction>,
}
impl MirModule {
    pub fn new(name: ModuleName, functions: Vec<MirFunction>) -> Self {
        Self { name, functions }
    }
    pub fn name(&self) -> &ModuleName {
        &self.name
    }
    pub fn functions(&self) -> &[MirFunction] {
        &self.functions
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MirLocal {
    id: MirLocalId,
    ty: TypeId,
    span: ByteSpan,
}
impl MirLocal {
    pub fn new(id: MirLocalId, ty: TypeId, span: ByteSpan) -> Self {
        Self { id, ty, span }
    }
    pub fn id(&self) -> MirLocalId {
        self.id
    }
    pub fn ty(&self) -> TypeId {
        self.ty
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MirArithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Exponent,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MirUnary {
    Plus,
    Negate,
    BitwiseNot,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MirComparison {
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MirDispatchKind {
    Direct,
    VirtualClass,
    Interface,
    StaticSuper,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MirDispatchTarget {
    receiver_type: TypeId,
    callee: MirFunctionId,
}

fn mir_call_instruction(
    call: &HirDirectCall,
    output: MirValueId,
    arguments: Vec<MirValueId>,
    span: ByteSpan,
) -> MirInstruction {
    let targets = call
        .targets()
        .iter()
        .map(|target| {
            MirDispatchTarget::new(
                target.receiver_type(),
                MirFunctionId::from_raw(target.callee().index()),
            )
        })
        .collect();
    match call.dispatch() {
        HirDispatchKind::Direct => MirInstruction::DirectCall {
            output,
            callee: MirFunctionId::from_raw(call.callee().index()),
            arguments,
            span,
        },
        HirDispatchKind::VirtualClass => MirInstruction::VirtualCall {
            output,
            arguments,
            targets,
            span,
        },
        HirDispatchKind::Interface => MirInstruction::InterfaceCall {
            output,
            arguments,
            targets,
            span,
        },
        HirDispatchKind::StaticSuper => MirInstruction::StaticSuperCall {
            output,
            callee: MirFunctionId::from_raw(call.callee().index()),
            arguments,
            span,
        },
    }
}

impl MirDispatchTarget {
    pub fn new(receiver_type: TypeId, callee: MirFunctionId) -> Self {
        Self {
            receiver_type,
            callee,
        }
    }
    pub fn receiver_type(self) -> TypeId {
        self.receiver_type
    }
    pub fn callee(self) -> MirFunctionId {
        self.callee
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MirInstruction {
    IntConstant {
        output: MirValueId,
        value: i64,
        span: ByteSpan,
    },
    EnumConstruct {
        output: MirValueId,
        tag: i64,
        payload: MirValueId,
        span: ByteSpan,
    },
    EnumPayload {
        output: MirValueId,
        value: MirValueId,
        index: usize,
        span: ByteSpan,
    },
    EnumTag {
        output: MirValueId,
        value: MirValueId,
        span: ByteSpan,
    },
    ChannelResultTag {
        output: MirValueId,
        value: MirValueId,
        span: ByteSpan,
    },
    ChannelResultPayload {
        output: MirValueId,
        value: MirValueId,
        element_type: TypeId,
        span: ByteSpan,
    },
    BoolConstant {
        output: MirValueId,
        value: bool,
        span: ByteSpan,
    },
    FloatConstant {
        output: MirValueId,
        bits: u64,
        span: ByteSpan,
    },
    ByteConstant {
        output: MirValueId,
        value: u8,
        span: ByteSpan,
    },
    UnitConstant {
        span: ByteSpan,
    },
    ParameterRead {
        output: MirValueId,
        parameter: MirParameterId,
        span: ByteSpan,
    },
    CheckedArithmetic {
        output: MirValueId,
        operation: MirArithmetic,
        left: MirValueId,
        right: MirValueId,
        span: ByteSpan,
    },
    Unary {
        output: MirValueId,
        operation: MirUnary,
        operand: MirValueId,
        span: ByteSpan,
    },
    LogicalNot {
        output: MirValueId,
        operand: MirValueId,
        span: ByteSpan,
    },
    Compare {
        output: MirValueId,
        operation: MirComparison,
        left: MirValueId,
        right: MirValueId,
        span: ByteSpan,
    },
    LoadLocal {
        output: MirValueId,
        local: MirLocalId,
        span: ByteSpan,
    },
    StoreLocal {
        local: MirLocalId,
        value: MirValueId,
        span: ByteSpan,
    },
    ScopeEnter {
        span: ByteSpan,
    },
    ScopeExit {
        span: ByteSpan,
    },
    Suspend {
        span: ByteSpan,
    },
    Resume {
        span: ByteSpan,
    },
    TaskSpawn {
        output: MirValueId,
        callee: MirFunctionId,
        callable: MirValueId,
        captures: Vec<MirValueId>,
        span: ByteSpan,
    },
    TaskAwait {
        output: MirValueId,
        task: MirValueId,
        span: ByteSpan,
    },
    TaskCancel {
        task: MirValueId,
        span: ByteSpan,
    },
    ChannelCreate {
        output: MirValueId,
        capacity: MirValueId,
        span: ByteSpan,
    },
    ChannelSend {
        channel: MirValueId,
        value: MirValueId,
        element_type: TypeId,
        span: ByteSpan,
    },
    ChannelReceive {
        output: MirValueId,
        channel: MirValueId,
        element_type: TypeId,
        span: ByteSpan,
    },
    ChannelClose {
        channel: MirValueId,
        span: ByteSpan,
    },
    DirectCall {
        output: MirValueId,
        callee: MirFunctionId,
        arguments: Vec<MirValueId>,
        span: ByteSpan,
    },
    TestAssert {
        condition: MirValueId,
        message: MirValueId,
        span: ByteSpan,
    },
    TestFail {
        message: MirValueId,
        span: ByteSpan,
    },
    FunctionReference {
        output: MirValueId,
        callee: MirFunctionId,
        span: ByteSpan,
    },
    IndirectCall {
        output: MirValueId,
        callee: MirValueId,
        function_type: TypeId,
        arguments: Vec<MirValueId>,
        span: ByteSpan,
    },
    VirtualCall {
        output: MirValueId,
        arguments: Vec<MirValueId>,
        targets: Vec<MirDispatchTarget>,
        span: ByteSpan,
    },
    InterfaceCall {
        output: MirValueId,
        arguments: Vec<MirValueId>,
        targets: Vec<MirDispatchTarget>,
        span: ByteSpan,
    },
    StaticSuperCall {
        output: MirValueId,
        callee: MirFunctionId,
        arguments: Vec<MirValueId>,
        span: ByteSpan,
    },
    ArrayInit {
        local: MirLocalId,
        elements: Vec<MirValueId>,
        span: ByteSpan,
    },
    ArrayValue {
        output: MirValueId,
        local: MirLocalId,
        span: ByteSpan,
    },
    ArrayAssign {
        local: MirLocalId,
        value: MirValueId,
        span: ByteSpan,
    },
    ArrayLoad {
        output: MirValueId,
        local: MirLocalId,
        index: MirValueId,
        span: ByteSpan,
    },
    ArrayElementLoad {
        output: MirValueId,
        array: MirValueId,
        index: MirValueId,
        span: ByteSpan,
    },
    ArrayStore {
        local: MirLocalId,
        index: MirValueId,
        value: MirValueId,
        span: ByteSpan,
    },
    StringConstant {
        output: MirValueId,
        bytes: Vec<u8>,
        span: ByteSpan,
    },
    StringLength {
        output: MirValueId,
        value: MirValueId,
        span: ByteSpan,
    },
    StringIndex {
        output: MirValueId,
        value: MirValueId,
        index: MirValueId,
        span: ByteSpan,
    },
    StringClone {
        output: MirValueId,
        value: MirValueId,
        span: ByteSpan,
    },
    StringConcat {
        output: MirValueId,
        left: MirValueId,
        right: MirValueId,
        span: ByteSpan,
    },
    StringCompare {
        output: MirValueId,
        left: MirValueId,
        right: MirValueId,
        negate: bool,
        span: ByteSpan,
    },
    NewObject {
        output: MirValueId,
        type_id: TypeId,
        arguments: Vec<MirValueId>,
        span: ByteSpan,
    },
    DynamicArrayNew {
        output: MirValueId,
        type_id: TypeId,
        span: ByteSpan,
    },
    DynamicArraySize {
        output: MirValueId,
        array: MirValueId,
        span: ByteSpan,
    },
    DynamicArrayLoad {
        output: MirValueId,
        array: MirValueId,
        index: MirValueId,
        span: ByteSpan,
    },
    DynamicArrayAdd {
        element_type: TypeId,
        array: MirValueId,
        value: MirValueId,
        index: Option<MirValueId>,
        span: ByteSpan,
    },
    DynamicArrayRemove {
        element_type: TypeId,
        array: MirValueId,
        index: MirValueId,
        span: ByteSpan,
    },
    FieldLoad {
        output: MirValueId,
        receiver: MirValueId,
        index: usize,
        span: ByteSpan,
    },
    FieldStore {
        receiver: MirValueId,
        index: usize,
        value: MirValueId,
        span: ByteSpan,
    },
    DestroyObject {
        value: MirValueId,
        span: ByteSpan,
    },
    DestroyDynamicArray {
        value: MirValueId,
        span: ByteSpan,
    },
    DestroyChannel {
        value: MirValueId,
        span: ByteSpan,
    },
}
impl MirInstruction {
    pub fn int_constant(output: MirValueId, value: i64, span: ByteSpan) -> Self {
        Self::IntConstant {
            output,
            value,
            span,
        }
    }

    pub fn bool_constant(output: MirValueId, value: bool, span: ByteSpan) -> Self {
        Self::BoolConstant {
            output,
            value,
            span,
        }
    }

    pub fn float_constant(output: MirValueId, bits: u64, span: ByteSpan) -> Self {
        Self::FloatConstant { output, bits, span }
    }

    pub fn byte_constant(output: MirValueId, value: u8, span: ByteSpan) -> Self {
        Self::ByteConstant {
            output,
            value,
            span,
        }
    }

    pub fn unit_constant(span: ByteSpan) -> Self {
        Self::UnitConstant { span }
    }
    pub fn checked_add(
        output: MirValueId,
        left: MirValueId,
        right: MirValueId,
        span: ByteSpan,
    ) -> Self {
        Self::CheckedArithmetic {
            output,
            operation: MirArithmetic::Add,
            left,
            right,
            span,
        }
    }
    pub fn span(&self) -> ByteSpan {
        match self {
            Self::IntConstant { span, .. }
            | Self::EnumConstruct { span, .. }
            | Self::EnumPayload { span, .. }
            | Self::EnumTag { span, .. }
            | Self::ChannelResultTag { span, .. }
            | Self::ChannelResultPayload { span, .. }
            | Self::BoolConstant { span, .. }
            | Self::FloatConstant { span, .. }
            | Self::ByteConstant { span, .. }
            | Self::UnitConstant { span }
            | Self::ParameterRead { span, .. }
            | Self::CheckedArithmetic { span, .. }
            | Self::Unary { span, .. }
            | Self::LogicalNot { span, .. }
            | Self::Compare { span, .. }
            | Self::LoadLocal { span, .. }
            | Self::StoreLocal { span, .. }
            | Self::ScopeEnter { span }
            | Self::ScopeExit { span }
            | Self::Suspend { span }
            | Self::Resume { span }
            | Self::TaskSpawn { span, .. }
            | Self::TaskAwait { span, .. }
            | Self::TaskCancel { span, .. }
            | Self::ChannelCreate { span, .. }
            | Self::ChannelSend { span, .. }
            | Self::ChannelReceive { span, .. }
            | Self::ChannelClose { span, .. }
            | Self::DirectCall { span, .. }
            | Self::TestAssert { span, .. }
            | Self::TestFail { span, .. }
            | Self::FunctionReference { span, .. }
            | Self::IndirectCall { span, .. }
            | Self::VirtualCall { span, .. }
            | Self::InterfaceCall { span, .. }
            | Self::StaticSuperCall { span, .. }
            | Self::ArrayInit { span, .. }
            | Self::ArrayValue { span, .. }
            | Self::ArrayAssign { span, .. }
            | Self::ArrayLoad { span, .. }
            | Self::ArrayStore { span, .. } => *span,
            Self::StringConstant { span, .. }
            | Self::StringLength { span, .. }
            | Self::StringIndex { span, .. }
            | Self::ArrayElementLoad { span, .. }
            | Self::StringClone { span, .. }
            | Self::StringConcat { span, .. }
            | Self::StringCompare { span, .. } => *span,
            Self::NewObject { span, .. } => *span,
            Self::DynamicArrayNew { span, .. } => *span,
            Self::DynamicArraySize { span, .. }
            | Self::DynamicArrayLoad { span, .. }
            | Self::DynamicArrayAdd { span, .. }
            | Self::DynamicArrayRemove { span, .. } => *span,
            Self::FieldLoad { span, .. } => *span,
            Self::FieldStore { span, .. } => *span,
            Self::DestroyObject { span, .. } => *span,
            Self::DestroyDynamicArray { span, .. } => *span,
            Self::DestroyChannel { span, .. } => *span,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MirTrap {
    IntegerOverflow,
    DivisionByZero,
    NegativeExponent,
    InvalidShiftCount,
    UnsupportedRuntime,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MirTerminator {
    Return {
        value: MirValueId,
        span: ByteSpan,
    },
    ReturnUnit {
        span: ByteSpan,
    },
    Branch {
        target: MirBlockId,
        span: ByteSpan,
    },
    BranchIf {
        condition: MirValueId,
        then_target: MirBlockId,
        else_target: MirBlockId,
        span: ByteSpan,
    },
    Trap {
        reason: MirTrap,
        span: ByteSpan,
    },
}
impl MirTerminator {
    pub fn return_value(value: MirValueId, span: ByteSpan) -> Self {
        Self::Return { value, span }
    }

    pub fn return_unit(span: ByteSpan) -> Self {
        Self::ReturnUnit { span }
    }
    pub fn branch_if(
        condition: MirValueId,
        then_target: MirBlockId,
        else_target: MirBlockId,
        span: ByteSpan,
    ) -> Self {
        Self::BranchIf {
            condition,
            then_target,
            else_target,
            span,
        }
    }
    pub fn span(&self) -> ByteSpan {
        match self {
            Self::Return { span, .. }
            | Self::ReturnUnit { span }
            | Self::Branch { span, .. }
            | Self::BranchIf { span, .. }
            | Self::Trap { span, .. } => *span,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MirBasicBlock {
    id: MirBlockId,
    instructions: Vec<MirInstruction>,
    terminator: MirTerminator,
}
impl MirBasicBlock {
    pub fn new(
        id: MirBlockId,
        instructions: Vec<MirInstruction>,
        terminator: MirTerminator,
    ) -> Self {
        Self {
            id,
            instructions,
            terminator,
        }
    }
    pub fn id(&self) -> MirBlockId {
        self.id
    }
    pub fn instructions(&self) -> &[MirInstruction] {
        &self.instructions
    }
    pub fn terminator(&self) -> MirTerminator {
        self.terminator
    }
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MirCleanupBoundary {
    owned_locals: Vec<MirLocalId>,
    owned_objects: Vec<MirLocalId>,
    owned_dynamic_arrays: Vec<MirLocalId>,
    owned_tasks: Vec<MirLocalId>,
    owned_channels: Vec<MirLocalId>,
    owned_parameters: Vec<MirValueId>,
    owned_task_parameters: Vec<MirValueId>,
    owned_channel_parameters: Vec<MirValueId>,
    returns_owned: bool,
}
impl MirCleanupBoundary {
    pub fn empty() -> Self {
        Self {
            owned_locals: Vec::new(),
            owned_objects: Vec::new(),
            owned_dynamic_arrays: Vec::new(),
            owned_tasks: Vec::new(),
            owned_channels: Vec::new(),
            owned_parameters: Vec::new(),
            owned_task_parameters: Vec::new(),
            owned_channel_parameters: Vec::new(),
            returns_owned: false,
        }
    }
    pub fn for_function(function: &HirFunction, types: &TypeArena) -> Self {
        let is_string = |ty| {
            types
                .get(ty)
                .is_some_and(|record| record.kind() == &TypeKind::Primitive(PrimitiveType::String))
        };
        Self {
            owned_locals: function
                .locals()
                .iter()
                .filter(|local| is_string(local.ty()))
                .map(|local| MirLocalId::from_raw(local.id().index()))
                .collect(),
            owned_objects: function
                .locals()
                .iter()
                .filter(|local| {
                    types.get(local.ty()).is_some_and(|record| {
                        matches!(
                            record.kind(),
                            TypeKind::Nominal(_) | TypeKind::GenericInstance(_)
                        )
                    })
                })
                .map(|local| MirLocalId::from_raw(local.id().index()))
                .collect(),
            owned_dynamic_arrays: function
                .locals()
                .iter()
                .filter(|local| {
                    types
                        .get(local.ty())
                        .is_some_and(|record| matches!(record.kind(), TypeKind::DynamicArray(_)))
                })
                .map(|local| MirLocalId::from_raw(local.id().index()))
                .collect(),
            owned_tasks: function
                .locals()
                .iter()
                .filter(|local| {
                    types
                        .get(local.ty())
                        .is_some_and(|record| matches!(record.kind(), TypeKind::Task(_)))
                })
                .map(|local| MirLocalId::from_raw(local.id().index()))
                .collect(),
            owned_channels: function
                .locals()
                .iter()
                .filter(|local| {
                    types
                        .get(local.ty())
                        .is_some_and(|record| matches!(record.kind(), TypeKind::Channel(_)))
                })
                .map(|local| MirLocalId::from_raw(local.id().index()))
                .collect(),
            owned_parameters: function
                .parameters()
                .iter()
                .filter(|parameter| is_string(parameter.ty()))
                .map(|parameter| MirValueId::from_raw(parameter.id().index()))
                .collect(),
            owned_task_parameters: function
                .parameters()
                .iter()
                .filter(|parameter| {
                    types
                        .get(parameter.ty())
                        .is_some_and(|record| matches!(record.kind(), TypeKind::Task(_)))
                })
                .map(|parameter| MirValueId::from_raw(parameter.id().index()))
                .collect(),
            owned_channel_parameters: function
                .parameters()
                .iter()
                .filter(|parameter| {
                    types
                        .get(parameter.ty())
                        .is_some_and(|record| matches!(record.kind(), TypeKind::Channel(_)))
                })
                .map(|parameter| MirValueId::from_raw(parameter.id().index()))
                .collect(),
            returns_owned: is_string(function.return_type()),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.owned_locals.is_empty()
            && self.owned_objects.is_empty()
            && self.owned_dynamic_arrays.is_empty()
            && self.owned_tasks.is_empty()
            && self.owned_channels.is_empty()
            && self.owned_parameters.is_empty()
            && self.owned_task_parameters.is_empty()
            && self.owned_channel_parameters.is_empty()
            && !self.returns_owned
    }
    pub fn owned_locals(&self) -> &[MirLocalId] {
        &self.owned_locals
    }
    pub fn owned_parameters(&self) -> &[MirValueId] {
        &self.owned_parameters
    }
    pub fn owned_objects(&self) -> &[MirLocalId] {
        &self.owned_objects
    }
    pub fn owned_dynamic_arrays(&self) -> &[MirLocalId] {
        &self.owned_dynamic_arrays
    }
    pub fn owned_tasks(&self) -> &[MirLocalId] {
        &self.owned_tasks
    }
    pub fn owned_channels(&self) -> &[MirLocalId] {
        &self.owned_channels
    }
    pub fn owned_task_parameters(&self) -> &[MirValueId] {
        &self.owned_task_parameters
    }
    pub fn owned_channel_parameters(&self) -> &[MirValueId] {
        &self.owned_channel_parameters
    }
    pub fn returns_owned(&self) -> bool {
        self.returns_owned
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MirFunction {
    id: MirFunctionId,
    span: ByteSpan,
    parameters: Vec<(MirValueId, TypeId)>,
    return_type: TypeId,
    locals: Vec<MirLocal>,
    blocks: Vec<MirBasicBlock>,
    cleanup_boundary: MirCleanupBoundary,
    symbol_identity: Option<FunctionSymbolIdentity>,
    entry: bool,
    suspend: bool,
    effect_contract: Option<OwnershipEffectContract>,
    specialization_identity: Option<GenericSpecializationIdentity>,
    captures: Vec<HirCapture>,
}
impl MirFunction {
    pub fn new(
        id: MirFunctionId,
        span: ByteSpan,
        parameters: Vec<(MirValueId, TypeId)>,
        return_type: TypeId,
        locals: Vec<MirLocal>,
        blocks: Vec<MirBasicBlock>,
        cleanup_boundary: MirCleanupBoundary,
    ) -> Self {
        Self {
            id,
            span,
            parameters,
            return_type,
            locals,
            blocks,
            cleanup_boundary,
            symbol_identity: None,
            entry: false,
            suspend: false,
            effect_contract: None,
            specialization_identity: None,
            captures: Vec::new(),
        }
    }
    pub fn id(&self) -> MirFunctionId {
        self.id
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn parameters(&self) -> &[(MirValueId, TypeId)] {
        &self.parameters
    }
    pub fn return_type(&self) -> TypeId {
        self.return_type
    }
    pub fn locals(&self) -> &[MirLocal] {
        &self.locals
    }
    pub fn blocks(&self) -> &[MirBasicBlock] {
        &self.blocks
    }
    pub fn cleanup_boundary(&self) -> &MirCleanupBoundary {
        &self.cleanup_boundary
    }
    pub fn captures(&self) -> &[HirCapture] {
        &self.captures
    }
    pub fn with_captures(mut self, captures: Vec<HirCapture>) -> Self {
        self.captures = captures;
        self
    }
    pub fn with_symbol_identity(mut self, identity: FunctionSymbolIdentity) -> Self {
        self.symbol_identity = Some(identity);
        self
    }
    pub fn symbol_identity(&self) -> Option<&FunctionSymbolIdentity> {
        self.symbol_identity.as_ref()
    }
    pub fn with_specialization_identity(mut self, identity: GenericSpecializationIdentity) -> Self {
        self.specialization_identity = Some(identity);
        self
    }
    pub fn specialization_identity(&self) -> Option<&GenericSpecializationIdentity> {
        self.specialization_identity.as_ref()
    }
    pub fn with_entry(mut self, entry: bool) -> Self {
        self.entry = entry;
        self
    }
    pub fn with_suspend(mut self, suspend: bool) -> Self {
        self.suspend = suspend;
        self
    }
    pub fn is_suspend(&self) -> bool {
        self.suspend
    }
    pub fn with_effect_contract(mut self, contract: OwnershipEffectContract) -> Self {
        self.effect_contract = Some(contract);
        self
    }
    pub fn effect_contract(&self) -> Option<&OwnershipEffectContract> {
        self.effect_contract.as_ref()
    }
    pub fn is_entry(&self) -> bool {
        self.entry
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MirLoweringError {
    UnsupportedExpression,
    MissingReturn,
    UnsupportedRuntimeType,
}
pub fn lower_hir_to_mir(hir: &HirModule, types: &TypeArena) -> Result<MirModule, MirLoweringError> {
    let mut functions = Vec::new();
    for function in hir.functions() {
        require_bootstrap_runtime_type(function.return_type(), types)?;
        if !function.control_flow().is_empty()
            || function.expressions().iter().any(|expression| {
                matches!(
                    expression.kind(),
                    HirExpressionKind::Conditional { .. } | HirExpressionKind::When { .. }
                )
            })
        {
            functions.push(lower_hir_function_with_control_flow(function, types)?);
            continue;
        }
        if function.expressions().iter().any(|expression| {
            matches!(
                expression.kind(),
                HirExpressionKind::Binary(binary)
                    if matches!(
                        binary.operator(),
                        HirBinaryOperator::LogicalAnd | HirBinaryOperator::LogicalOr
                    )
            )
        }) {
            functions.push(lower_hir_function_with_short_circuit(function, types)?);
            continue;
        }
        let mut instructions = Vec::new();
        for expression in function.expressions() {
            require_hir_expression_type(expression, types)?;
            let output = mir_expression_value_id(function, expression.id());
            match expression.kind() {
                HirExpressionKind::IntLiteral(value) => instructions.push(
                    MirInstruction::int_constant(output, *value, expression.span()),
                ),
                HirExpressionKind::EnumVariant(value) => instructions.push(
                    MirInstruction::int_constant(output, *value, expression.span()),
                ),
                HirExpressionKind::EnumConstruct { tag, payloads } => {
                    let payload = payloads
                        .first()
                        .copied()
                        .ok_or(MirLoweringError::UnsupportedExpression)?;
                    instructions.push(MirInstruction::EnumConstruct {
                        output,
                        tag: *tag,
                        payload: mir_expression_value_id(function, payload),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::EnumPayload { subject, index } => {
                    let value = mir_expression_value_id(function, *subject);
                    let is_channel_result = function
                        .expressions()
                        .iter()
                        .find(|candidate| candidate.id() == *subject)
                        .and_then(|candidate| types.get(candidate.ty()))
                        .is_some_and(|record| matches!(record.kind(), TypeKind::ChannelResult(_)));
                    if is_channel_result {
                        let element_type = types
                            .get(expression.ty())
                            .map(|record| record.id())
                            .ok_or(MirLoweringError::UnsupportedRuntimeType)?;
                        instructions.push(MirInstruction::ChannelResultPayload {
                            output,
                            value,
                            element_type,
                            span: expression.span(),
                        });
                    } else {
                        instructions.push(MirInstruction::EnumPayload {
                            output,
                            value,
                            index: *index,
                            span: expression.span(),
                        });
                    }
                }
                HirExpressionKind::BoolLiteral(value) => instructions.push(
                    MirInstruction::bool_constant(output, *value, expression.span()),
                ),
                HirExpressionKind::FloatLiteral(bits) => instructions.push(
                    MirInstruction::float_constant(output, *bits, expression.span()),
                ),
                HirExpressionKind::ByteLiteral(value) => instructions.push(
                    MirInstruction::byte_constant(output, *value, expression.span()),
                ),
                HirExpressionKind::UnitLiteral => {
                    instructions.push(MirInstruction::unit_constant(expression.span()))
                }
                HirExpressionKind::ParameterRead(parameter) => {
                    instructions.push(MirInstruction::ParameterRead {
                        output,
                        parameter: MirParameterId::from_raw(parameter.index()),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::LocalRead(local) => {
                    if matches!(
                        types.get(expression.ty()).map(|record| record.kind()),
                        Some(TypeKind::Array(_))
                    ) {
                        instructions.push(MirInstruction::ArrayValue {
                            output,
                            local: MirLocalId::from_raw(local.index()),
                            span: expression.span(),
                        });
                    } else if !matches!(
                        types.get(expression.ty()).map(|record| record.kind()),
                        Some(TypeKind::Primitive(PrimitiveType::Unit) | TypeKind::Array(_))
                    ) {
                        instructions.push(MirInstruction::LoadLocal {
                            output,
                            local: MirLocalId::from_raw(local.index()),
                            span: expression.span(),
                        });
                    }
                }
                HirExpressionKind::Binary(binary) => {
                    let left = mir_expression_value_id(function, binary.left());
                    let right = mir_expression_value_id(function, binary.right());
                    let left_is_string = function
                        .expressions()
                        .get(binary.left().index())
                        .is_some_and(|candidate| {
                            types.get(candidate.ty()).is_some_and(|record| {
                                record.kind() == &TypeKind::Primitive(PrimitiveType::String)
                            })
                        });
                    if left_is_string && binary.operator() == HirBinaryOperator::Plus {
                        instructions.push(MirInstruction::StringConcat {
                            output,
                            left,
                            right,
                            span: expression.span(),
                        });
                    } else if left_is_string
                        && matches!(
                            binary.operator(),
                            HirBinaryOperator::Equal | HirBinaryOperator::NotEqual
                        )
                    {
                        instructions.push(MirInstruction::StringCompare {
                            output,
                            left,
                            right,
                            negate: binary.operator() == HirBinaryOperator::NotEqual,
                            span: expression.span(),
                        });
                    } else if let Some(operation) = lower_comparison(binary.operator()) {
                        instructions.push(MirInstruction::Compare {
                            output,
                            operation,
                            left,
                            right,
                            span: expression.span(),
                        });
                    } else {
                        instructions.push(MirInstruction::CheckedArithmetic {
                            output,
                            operation: lower_binary(binary.operator())?,
                            left,
                            right,
                            span: expression.span(),
                        });
                    }
                }
                HirExpressionKind::Unary(unary) => {
                    let operand = mir_expression_value_id(function, unary.operand());
                    if matches!(unary.operator(), HirUnaryOperator::Not) {
                        instructions.push(MirInstruction::LogicalNot {
                            output,
                            operand,
                            span: expression.span(),
                        });
                    } else {
                        instructions.push(MirInstruction::Unary {
                            output,
                            operation: lower_unary(unary.operator())?,
                            operand,
                            span: expression.span(),
                        });
                    }
                }
                HirExpressionKind::DirectCall(call) => {
                    instructions.push(mir_call_instruction(
                        call,
                        output,
                        call.arguments()
                            .iter()
                            .map(|argument| mir_expression_value_id(function, *argument))
                            .collect(),
                        expression.span(),
                    ));
                }
                HirExpressionKind::TestAssert { condition, message } => {
                    instructions.push(MirInstruction::TestAssert {
                        condition: mir_expression_value_id(function, *condition),
                        message: mir_expression_value_id(function, *message),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::TestFail { message } => {
                    instructions.push(MirInstruction::TestFail {
                        message: mir_expression_value_id(function, *message),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::FunctionReference(callee) => {
                    instructions.push(MirInstruction::FunctionReference {
                        output,
                        callee: MirFunctionId::from_raw(callee.index()),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::IndirectCall { callee, arguments } => {
                    instructions.push(MirInstruction::IndirectCall {
                        output,
                        callee: mir_expression_value_id(function, *callee),
                        function_type: function
                            .expressions()
                            .iter()
                            .find(|candidate| candidate.id() == *callee)
                            .map(|candidate| candidate.ty())
                            .ok_or(MirLoweringError::UnsupportedExpression)?,
                        arguments: arguments
                            .iter()
                            .map(|argument| mir_expression_value_id(function, *argument))
                            .collect(),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::ArrayLiteral(elements) => {
                    let local = function
                        .locals()
                        .iter()
                        .find(|local| local.initializer() == Some(expression.id()))
                        .map(|local| local.id())
                        .ok_or(MirLoweringError::UnsupportedExpression)?;
                    instructions.push(MirInstruction::ArrayInit {
                        local: MirLocalId::from_raw(local.index()),
                        elements: elements
                            .iter()
                            .map(|element| mir_expression_value_id(function, *element))
                            .collect(),
                        span: expression.span(),
                    });
                    instructions.push(MirInstruction::ArrayValue {
                        output,
                        local: MirLocalId::from_raw(local.index()),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::StringLiteral(bytes) => {
                    instructions.push(MirInstruction::StringConstant {
                        output,
                        bytes: bytes.clone(),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::StringLength(value) => {
                    instructions.push(MirInstruction::StringLength {
                        output,
                        value: mir_expression_value_id(function, *value),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::StringClone(value) => {
                    instructions.push(MirInstruction::StringClone {
                        output,
                        value: mir_expression_value_id(function, *value),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::Index { array, index } => {
                    let array_expression = function
                        .expressions()
                        .iter()
                        .find(|candidate| candidate.id() == *array)
                        .ok_or(MirLoweringError::UnsupportedExpression)?;
                    if types.get(array_expression.ty()).is_some_and(|record| {
                        record.kind() == &TypeKind::Primitive(PrimitiveType::String)
                    }) {
                        instructions.push(MirInstruction::StringIndex {
                            output,
                            value: mir_expression_value_id(function, *array),
                            index: mir_expression_value_id(function, *index),
                            span: expression.span(),
                        });
                    } else if types
                        .get(array_expression.ty())
                        .is_some_and(|record| matches!(record.kind(), TypeKind::DynamicArray(_)))
                    {
                        instructions.push(MirInstruction::DynamicArrayLoad {
                            output,
                            array: mir_expression_value_id(function, *array),
                            index: mir_expression_value_id(function, *index),
                            span: expression.span(),
                        });
                    } else {
                        let local = match array_expression.kind() {
                            HirExpressionKind::LocalRead(local) => *local,
                            _ => return Err(MirLoweringError::UnsupportedExpression),
                        };
                        instructions.push(MirInstruction::ArrayLoad {
                            output,
                            local: MirLocalId::from_raw(local.index()),
                            index: mir_expression_value_id(function, *index),
                            span: expression.span(),
                        });
                    }
                }
                HirExpressionKind::FieldAccess {
                    receiver, index, ..
                } => {
                    instructions.push(MirInstruction::FieldLoad {
                        output,
                        receiver: mir_expression_value_id(function, *receiver),
                        index: *index,
                        span: expression.span(),
                    });
                }
                HirExpressionKind::NewObject { arguments, .. } => {
                    instructions.push(MirInstruction::NewObject {
                        output,
                        type_id: expression.ty(),
                        arguments: arguments
                            .iter()
                            .map(|argument| mir_expression_value_id(function, *argument))
                            .collect(),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::DynamicArrayNew => {
                    instructions.push(MirInstruction::DynamicArrayNew {
                        output,
                        type_id: expression.ty(),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::DynamicArraySize(array) => {
                    instructions.push(MirInstruction::DynamicArraySize {
                        output,
                        array: mir_expression_value_id(function, *array),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::DynamicArrayAdd {
                    array,
                    value,
                    index,
                } => {
                    instructions.push(MirInstruction::DynamicArrayAdd {
                        element_type: types
                            .get(
                                function
                                    .expressions()
                                    .iter()
                                    .find(|candidate| candidate.id() == *array)
                                    .map(|array| array.ty())
                                    .ok_or(MirLoweringError::UnsupportedExpression)?,
                            )
                            .and_then(|record| match record.kind() {
                                TypeKind::DynamicArray(array) => Some(array.element()),
                                _ => None,
                            })
                            .ok_or(MirLoweringError::UnsupportedExpression)?,
                        array: mir_expression_value_id(function, *array),
                        value: mir_expression_value_id(function, *value),
                        index: index.map(|index| mir_expression_value_id(function, index)),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::DynamicArrayRemove { array, index } => {
                    instructions.push(MirInstruction::DynamicArrayRemove {
                        element_type: types
                            .get(
                                function
                                    .expressions()
                                    .iter()
                                    .find(|candidate| candidate.id() == *array)
                                    .map(|array| array.ty())
                                    .ok_or(MirLoweringError::UnsupportedExpression)?,
                            )
                            .and_then(|record| match record.kind() {
                                TypeKind::DynamicArray(array) => Some(array.element()),
                                _ => None,
                            })
                            .ok_or(MirLoweringError::UnsupportedExpression)?,
                        array: mir_expression_value_id(function, *array),
                        index: mir_expression_value_id(function, *index),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::TaskSpawn { callable, captures } => {
                    let callee = mir_task_callee(function, *callable)?;
                    instructions.push(MirInstruction::TaskSpawn {
                        output,
                        callee,
                        callable: mir_expression_value_id(function, *callable),
                        captures: captures
                            .iter()
                            .map(|capture| mir_expression_value_id(function, *capture))
                            .collect(),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::TaskAwait(task) => {
                    instructions.push(MirInstruction::Suspend {
                        span: expression.span(),
                    });
                    instructions.push(MirInstruction::TaskAwait {
                        output,
                        task: mir_expression_value_id(function, *task),
                        span: expression.span(),
                    });
                    instructions.push(MirInstruction::Resume {
                        span: expression.span(),
                    });
                }
                HirExpressionKind::TaskCancel(task) => {
                    instructions.push(MirInstruction::TaskCancel {
                        task: mir_expression_value_id(function, *task),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::ChannelCreate(capacity) => {
                    instructions.push(MirInstruction::ChannelCreate {
                        output,
                        capacity: mir_expression_value_id(function, *capacity),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::ChannelSend { channel, value } => {
                    let element_type = types
                        .get(
                            function
                                .expressions()
                                .iter()
                                .find(|candidate| candidate.id() == *channel)
                                .map(|candidate| candidate.ty())
                                .ok_or(MirLoweringError::UnsupportedExpression)?,
                        )
                        .and_then(|record| match record.kind() {
                            TypeKind::Channel(channel) => Some(channel.element()),
                            _ => None,
                        })
                        .ok_or(MirLoweringError::UnsupportedExpression)?;
                    instructions.push(MirInstruction::ChannelSend {
                        channel: mir_expression_value_id(function, *channel),
                        value: mir_expression_value_id(function, *value),
                        element_type,
                        span: expression.span(),
                    });
                }
                HirExpressionKind::ChannelReceive(channel) => {
                    let element_type = types
                        .get(
                            function
                                .expressions()
                                .iter()
                                .find(|candidate| candidate.id() == *channel)
                                .map(|candidate| candidate.ty())
                                .ok_or(MirLoweringError::UnsupportedExpression)?,
                        )
                        .and_then(|record| match record.kind() {
                            TypeKind::Channel(channel) => Some(channel.element()),
                            _ => None,
                        })
                        .ok_or(MirLoweringError::UnsupportedExpression)?;
                    instructions.push(MirInstruction::Suspend {
                        span: expression.span(),
                    });
                    instructions.push(MirInstruction::ChannelReceive {
                        output,
                        channel: mir_expression_value_id(function, *channel),
                        element_type,
                        span: expression.span(),
                    });
                    instructions.push(MirInstruction::Resume {
                        span: expression.span(),
                    });
                }
                HirExpressionKind::ChannelClose(channel) => {
                    instructions.push(MirInstruction::ChannelClose {
                        channel: mir_expression_value_id(function, *channel),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::Conditional { .. } => {
                    return Err(MirLoweringError::UnsupportedExpression);
                }
                HirExpressionKind::When { .. } => {
                    return Err(MirLoweringError::UnsupportedExpression);
                }
            }
            for local in function.locals() {
                if local.initializer() == Some(expression.id()) {
                    if matches!(
                        types.get(local.ty()).map(|record| record.kind()),
                        Some(TypeKind::Array(_))
                    ) {
                        instructions.push(MirInstruction::ArrayAssign {
                            local: MirLocalId::from_raw(local.id().index()),
                            value: output,
                            span: expression.span(),
                        });
                    } else if !matches!(
                        types.get(local.ty()).map(|record| record.kind()),
                        Some(TypeKind::Primitive(PrimitiveType::Unit) | TypeKind::Array(_))
                    ) {
                        instructions.push(MirInstruction::StoreLocal {
                            local: MirLocalId::from_raw(local.id().index()),
                            value: output,
                            span: expression.span(),
                        });
                    }
                }
            }
            for assignment in function.assignments() {
                if assignment.value().index() == expression.id().index()
                    && let Some((receiver, index)) = assignment.field()
                {
                    instructions.push(MirInstruction::FieldStore {
                        receiver: mir_expression_value_id(function, receiver),
                        index,
                        value: output,
                        span: assignment.span(),
                    });
                    continue;
                }
                if assignment.value().index() == expression.id().index()
                    && !matches!(
                        function
                            .locals()
                            .iter()
                            .find(|local| local.id() == assignment.target())
                            .map(|local| types.get(local.ty()).map(|record| record.kind())),
                        Some(Some(TypeKind::Primitive(PrimitiveType::Unit)))
                    )
                {
                    if let Some(index) = assignment.index() {
                        instructions.push(MirInstruction::ArrayStore {
                            local: MirLocalId::from_raw(assignment.target().index()),
                            index: mir_expression_value_id(function, index),
                            value: output,
                            span: assignment.span(),
                        });
                    } else {
                        instructions.push(MirInstruction::StoreLocal {
                            local: MirLocalId::from_raw(assignment.target().index()),
                            value: output,
                            span: assignment.span(),
                        });
                    }
                }
            }
        }
        if !matches!(
            types
                .get(function.return_type())
                .map(|record| record.kind()),
            Some(TypeKind::Nominal(_) | TypeKind::GenericInstance(_) | TypeKind::DynamicArray(_),)
        ) {
            let cleanup_value =
                MirValueId::from_raw(function.parameters().len() + function.expressions().len());
            for local in function.locals().iter().filter(|local| {
                matches!(
                    types.get(local.ty()).map(|record| record.kind()),
                    Some(TypeKind::Nominal(_) | TypeKind::GenericInstance(_),)
                )
            }) {
                instructions.push(MirInstruction::LoadLocal {
                    output: cleanup_value,
                    local: MirLocalId::from_raw(local.id().index()),
                    span: local.span(),
                });
                instructions.push(MirInstruction::DestroyObject {
                    value: cleanup_value,
                    span: local.span(),
                });
            }
            for local in function.locals().iter().filter(|local| {
                matches!(
                    types.get(local.ty()).map(|record| record.kind()),
                    Some(TypeKind::DynamicArray(_))
                )
            }) {
                instructions.push(MirInstruction::LoadLocal {
                    output: cleanup_value,
                    local: MirLocalId::from_raw(local.id().index()),
                    span: local.span(),
                });
                instructions.push(MirInstruction::DestroyDynamicArray {
                    value: cleanup_value,
                    span: local.span(),
                });
            }
            for local in function.locals().iter().filter(|local| {
                matches!(
                    types.get(local.ty()).map(|record| record.kind()),
                    Some(TypeKind::Channel(_))
                )
            }) {
                instructions.push(MirInstruction::LoadLocal {
                    output: cleanup_value,
                    local: MirLocalId::from_raw(local.id().index()),
                    span: local.span(),
                });
                instructions.push(MirInstruction::DestroyChannel {
                    value: cleanup_value,
                    span: local.span(),
                });
            }
        }
        let returned = function
            .returns()
            .first()
            .ok_or(MirLoweringError::MissingReturn)?;
        let terminator = if matches!(
            types
                .get(function.return_type())
                .map(|record| record.kind()),
            Some(TypeKind::Primitive(PrimitiveType::Unit))
        ) {
            MirTerminator::return_unit(returned.span())
        } else {
            MirTerminator::return_value(
                mir_expression_value_id(function, returned.expression()),
                returned.span(),
            )
        };
        let mut mir_function = MirFunction::new(
            MirFunctionId::from_raw(function.id().index()),
            function.span(),
            function
                .parameters()
                .iter()
                .map(|parameter| (MirValueId::from_raw(parameter.id().index()), parameter.ty()))
                .collect(),
            function.return_type(),
            function
                .locals()
                .iter()
                .map(|local| {
                    MirLocal::new(
                        MirLocalId::from_raw(local.id().index()),
                        local.ty(),
                        local.span(),
                    )
                })
                .collect(),
            vec![MirBasicBlock::new(
                MirBlockId::from_raw(0),
                instructions,
                terminator,
            )],
            MirCleanupBoundary::for_function(function, types),
        )
        .with_entry(function.is_entry())
        .with_suspend(function.is_suspend())
        .with_captures(function.captures().to_vec());
        if let Some(contract) = function.effect_contract() {
            mir_function = mir_function.with_effect_contract(contract.clone());
        }
        if let Some(identity) = function.specialization_identity() {
            mir_function = mir_function.with_specialization_identity(identity.clone());
        }
        functions.push(match function.symbol_identity() {
            Some(identity) => mir_function.with_symbol_identity(identity.clone()),
            None => mir_function,
        });
    }
    Ok(MirModule::new(hir.name().clone(), functions))
}

fn mir_expression_value_id(function: &HirFunction, expression: HirExpressionId) -> MirValueId {
    MirValueId::from_raw(function.parameters().len() + expression.index())
}

fn mir_task_callee(
    function: &HirFunction,
    callable: HirExpressionId,
) -> Result<MirFunctionId, MirLoweringError> {
    match function
        .expressions()
        .iter()
        .find(|expression| expression.id() == callable)
        .map(|expression| expression.kind())
    {
        Some(HirExpressionKind::FunctionReference(callee)) => {
            Ok(MirFunctionId::from_raw(callee.index()))
        }
        _ => Err(MirLoweringError::UnsupportedExpression),
    }
}

struct LowerBlock {
    id: MirBlockId,
    instructions: Vec<MirInstruction>,
    terminator: Option<MirTerminator>,
}

struct ShortCircuitLowerer<'a> {
    function: &'a HirFunction,
    types: &'a TypeArena,
    blocks: Vec<LowerBlock>,
    current: usize,
    locals: Vec<MirLocal>,
    scratch_local: MirLocalId,
    next_block: usize,
    next_value: usize,
    expression_index: HashMap<HirExpressionId, HirExpression>,
    lowered: HashSet<HirExpressionId>,
}

impl<'a> ShortCircuitLowerer<'a> {
    fn new(function: &'a HirFunction, types: &'a TypeArena) -> Self {
        let next_value = function.parameters().len()
            + function
                .expressions()
                .iter()
                .map(|expression| expression.id().index())
                .max()
                .map_or(0, |value| value + 1);
        let bool_type = function
            .expressions()
            .iter()
            .find_map(|expression| {
                matches!(
                    expression.kind(),
                    HirExpressionKind::Binary(binary)
                        if matches!(
                            binary.operator(),
                            HirBinaryOperator::LogicalAnd | HirBinaryOperator::LogicalOr
                        )
                )
                .then_some(expression.ty())
            })
            .unwrap_or(TypeId::from_raw(0));
        let mut locals = function
            .locals()
            .iter()
            .map(|local| {
                MirLocal::new(
                    MirLocalId::from_raw(local.id().index()),
                    local.ty(),
                    local.span(),
                )
            })
            .collect::<Vec<_>>();
        let scratch_local = MirLocalId::from_raw(locals.len());
        locals.push(MirLocal::new(scratch_local, bool_type, function.span()));
        Self {
            function,
            types,
            blocks: vec![LowerBlock {
                id: MirBlockId::from_raw(0),
                instructions: Vec::new(),
                terminator: None,
            }],
            current: 0,
            locals,
            scratch_local,
            next_block: 1,
            next_value,
            expression_index: function
                .expressions()
                .iter()
                .cloned()
                .map(|expression| (expression.id(), expression))
                .collect(),
            lowered: HashSet::new(),
        }
    }

    fn new_block(&mut self) -> usize {
        let index = self.blocks.len();
        self.blocks.push(LowerBlock {
            id: MirBlockId::from_raw(self.next_block),
            instructions: Vec::new(),
            terminator: None,
        });
        self.next_block += 1;
        index
    }

    fn fresh_value(&mut self) -> MirValueId {
        let value = MirValueId::from_raw(self.next_value);
        self.next_value += 1;
        value
    }

    fn set_terminator(&mut self, terminator: MirTerminator) {
        self.blocks[self.current].terminator = Some(terminator);
    }

    fn push(&mut self, instruction: MirInstruction) {
        self.blocks[self.current].instructions.push(instruction);
    }

    fn lower_expression(&mut self, id: HirExpressionId) -> Result<MirValueId, MirLoweringError> {
        let expression = self
            .expression_index
            .get(&id)
            .cloned()
            .ok_or(MirLoweringError::UnsupportedExpression)?;
        require_hir_expression_type(&expression, self.types)?;
        let output = mir_expression_value_id(self.function, id);
        if self.lowered.contains(&id) {
            return Ok(output);
        }

        match expression.kind() {
            HirExpressionKind::IntLiteral(value) => {
                self.push(MirInstruction::int_constant(
                    output,
                    *value,
                    expression.span(),
                ));
            }
            HirExpressionKind::EnumVariant(value) => {
                self.push(MirInstruction::int_constant(
                    output,
                    *value,
                    expression.span(),
                ));
            }
            HirExpressionKind::EnumConstruct { tag, payloads } => {
                let payload = payloads
                    .first()
                    .copied()
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                let payload = self.lower_expression(payload)?;
                self.push(MirInstruction::EnumConstruct {
                    output,
                    tag: *tag,
                    payload,
                    span: expression.span(),
                });
            }
            HirExpressionKind::EnumPayload { subject, index } => {
                let value = self.lower_expression(*subject)?;
                let is_channel_result = self
                    .function
                    .expressions()
                    .iter()
                    .find(|candidate| candidate.id() == *subject)
                    .and_then(|candidate| self.types.get(candidate.ty()))
                    .is_some_and(|record| matches!(record.kind(), TypeKind::ChannelResult(_)));
                if is_channel_result {
                    self.push(MirInstruction::ChannelResultPayload {
                        output,
                        value,
                        element_type: expression.ty(),
                        span: expression.span(),
                    });
                } else {
                    self.push(MirInstruction::EnumPayload {
                        output,
                        value,
                        index: *index,
                        span: expression.span(),
                    });
                }
            }
            HirExpressionKind::BoolLiteral(value) => {
                self.push(MirInstruction::bool_constant(
                    output,
                    *value,
                    expression.span(),
                ));
            }
            HirExpressionKind::FloatLiteral(bits) => {
                self.push(MirInstruction::float_constant(
                    output,
                    *bits,
                    expression.span(),
                ));
            }
            HirExpressionKind::ByteLiteral(value) => {
                self.push(MirInstruction::byte_constant(
                    output,
                    *value,
                    expression.span(),
                ));
            }
            HirExpressionKind::ParameterRead(parameter) => {
                self.push(MirInstruction::ParameterRead {
                    output,
                    parameter: MirParameterId::from_raw(parameter.index()),
                    span: expression.span(),
                });
            }
            HirExpressionKind::UnitLiteral => {
                self.push(MirInstruction::unit_constant(expression.span()));
            }
            HirExpressionKind::LocalRead(local) => {
                if matches!(
                    self.types.get(expression.ty()).map(|record| record.kind()),
                    Some(TypeKind::Array(_))
                ) {
                    self.push(MirInstruction::ArrayValue {
                        output,
                        local: MirLocalId::from_raw(local.index()),
                        span: expression.span(),
                    });
                } else if !matches!(
                    self.types.get(expression.ty()).map(|record| record.kind()),
                    Some(TypeKind::Primitive(PrimitiveType::Unit))
                ) {
                    self.push(MirInstruction::LoadLocal {
                        output,
                        local: MirLocalId::from_raw(local.index()),
                        span: expression.span(),
                    });
                }
            }
            HirExpressionKind::Binary(binary)
                if matches!(
                    binary.operator(),
                    HirBinaryOperator::LogicalAnd | HirBinaryOperator::LogicalOr
                ) =>
            {
                let left = self.lower_expression(binary.left())?;
                let left_block = self.current;
                let rhs_block = self.new_block();
                let short_block = self.new_block();
                let merge_block = self.new_block();
                let rhs_target = self.blocks[rhs_block].id;
                let short_target = self.blocks[short_block].id;
                let merge_target = self.blocks[merge_block].id;
                let (then_target, else_target) = match binary.operator() {
                    HirBinaryOperator::LogicalAnd => (rhs_target, short_target),
                    HirBinaryOperator::LogicalOr => (short_target, rhs_target),
                    _ => unreachable!(),
                };
                self.blocks[left_block].terminator = Some(MirTerminator::branch_if(
                    left,
                    then_target,
                    else_target,
                    expression.span(),
                ));

                self.current = rhs_block;
                let right = self.lower_expression(binary.right())?;
                self.push(MirInstruction::StoreLocal {
                    local: self.scratch_local,
                    value: right,
                    span: expression.span(),
                });
                self.set_terminator(MirTerminator::Branch {
                    target: merge_target,
                    span: expression.span(),
                });

                self.current = short_block;
                let short_value = self.fresh_value();
                self.push(MirInstruction::bool_constant(
                    short_value,
                    matches!(binary.operator(), HirBinaryOperator::LogicalOr),
                    expression.span(),
                ));
                self.push(MirInstruction::StoreLocal {
                    local: self.scratch_local,
                    value: short_value,
                    span: expression.span(),
                });
                self.set_terminator(MirTerminator::Branch {
                    target: merge_target,
                    span: expression.span(),
                });

                self.current = merge_block;
                self.push(MirInstruction::LoadLocal {
                    output,
                    local: self.scratch_local,
                    span: expression.span(),
                });
            }
            HirExpressionKind::Binary(binary) => {
                let left = self.lower_expression(binary.left())?;
                let right = self.lower_expression(binary.right())?;
                if let Some(operation) = lower_comparison(binary.operator()) {
                    self.push(MirInstruction::Compare {
                        output,
                        operation,
                        left,
                        right,
                        span: expression.span(),
                    });
                } else {
                    self.push(MirInstruction::CheckedArithmetic {
                        output,
                        operation: lower_binary(binary.operator())?,
                        left,
                        right,
                        span: expression.span(),
                    });
                }
            }
            HirExpressionKind::Unary(unary) => {
                let operand = self.lower_expression(unary.operand())?;
                if matches!(unary.operator(), HirUnaryOperator::Not) {
                    self.push(MirInstruction::LogicalNot {
                        output,
                        operand,
                        span: expression.span(),
                    });
                } else {
                    self.push(MirInstruction::Unary {
                        output,
                        operation: lower_unary(unary.operator())?,
                        operand,
                        span: expression.span(),
                    });
                }
            }
            HirExpressionKind::DirectCall(call) => {
                self.push(mir_call_instruction(
                    call,
                    output,
                    call.arguments()
                        .iter()
                        .map(|argument| mir_expression_value_id(self.function, *argument))
                        .collect(),
                    expression.span(),
                ));
            }
            HirExpressionKind::TestAssert { condition, message } => {
                self.push(MirInstruction::TestAssert {
                    condition: mir_expression_value_id(self.function, *condition),
                    message: mir_expression_value_id(self.function, *message),
                    span: expression.span(),
                });
            }
            HirExpressionKind::TestFail { message } => {
                self.push(MirInstruction::TestFail {
                    message: mir_expression_value_id(self.function, *message),
                    span: expression.span(),
                });
            }
            HirExpressionKind::FunctionReference(callee) => {
                self.push(MirInstruction::FunctionReference {
                    output,
                    callee: MirFunctionId::from_raw(callee.index()),
                    span: expression.span(),
                });
            }
            HirExpressionKind::IndirectCall { callee, arguments } => {
                let callee_expression = *callee;
                let callee = self.lower_expression(callee_expression)?;
                let arguments = arguments
                    .iter()
                    .map(|argument| self.lower_expression(*argument))
                    .collect::<Result<Vec<_>, _>>()?;
                self.push(MirInstruction::IndirectCall {
                    output,
                    callee,
                    function_type: self
                        .function
                        .expressions()
                        .iter()
                        .find(|candidate| candidate.id() == callee_expression)
                        .map(|candidate| candidate.ty())
                        .ok_or(MirLoweringError::UnsupportedExpression)?,
                    arguments,
                    span: expression.span(),
                });
            }
            HirExpressionKind::Conditional { .. } | HirExpressionKind::When { .. } => {
                return Err(MirLoweringError::UnsupportedExpression);
            }
            HirExpressionKind::DynamicArrayNew => {
                self.push(MirInstruction::DynamicArrayNew {
                    output,
                    type_id: expression.ty(),
                    span: expression.span(),
                });
            }
            HirExpressionKind::DynamicArraySize(array) => {
                let array = self.lower_expression(*array)?;
                self.push(MirInstruction::DynamicArraySize {
                    output,
                    array,
                    span: expression.span(),
                });
            }
            HirExpressionKind::DynamicArrayAdd {
                array,
                value,
                index,
            } => {
                let array_value = self.lower_expression(*array)?;
                let value = self.lower_expression(*value)?;
                let element_type = self
                    .types
                    .get(
                        self.function
                            .expressions()
                            .iter()
                            .find(|candidate| candidate.id() == *array)
                            .map(|candidate| candidate.ty())
                            .ok_or(MirLoweringError::UnsupportedExpression)?,
                    )
                    .and_then(|record| match record.kind() {
                        TypeKind::DynamicArray(array) => Some(array.element()),
                        _ => None,
                    })
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                let index = index
                    .map(|index| self.lower_expression(index))
                    .transpose()?;
                self.push(MirInstruction::DynamicArrayAdd {
                    element_type,
                    array: array_value,
                    value,
                    index,
                    span: expression.span(),
                });
            }
            HirExpressionKind::DynamicArrayRemove { array, index } => {
                let array_id = *array;
                let array = self.lower_expression(array_id)?;
                let index = self.lower_expression(*index)?;
                let element_type = self
                    .function
                    .expressions()
                    .iter()
                    .find(|candidate| candidate.id() == array_id)
                    .and_then(|candidate| self.types.get(candidate.ty()))
                    .and_then(|record| match record.kind() {
                        TypeKind::DynamicArray(array) => Some(array.element()),
                        _ => None,
                    })
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                self.push(MirInstruction::DynamicArrayRemove {
                    element_type,
                    array,
                    index,
                    span: expression.span(),
                });
            }
            HirExpressionKind::ArrayLiteral(elements) => {
                let local = self
                    .function
                    .locals()
                    .iter()
                    .find(|local| local.initializer() == Some(id))
                    .map(|local| local.id())
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                let element_values = elements
                    .iter()
                    .map(|element| self.lower_expression(*element))
                    .collect::<Result<Vec<_>, _>>()?;
                self.push(MirInstruction::ArrayInit {
                    local: MirLocalId::from_raw(local.index()),
                    elements: element_values,
                    span: expression.span(),
                });
                self.push(MirInstruction::ArrayValue {
                    output,
                    local: MirLocalId::from_raw(local.index()),
                    span: expression.span(),
                });
            }
            HirExpressionKind::TaskSpawn { callable, captures } => {
                let callable_id = *callable;
                let callable = self.lower_expression(callable_id)?;
                let callee = mir_task_callee(self.function, callable_id)?;
                let captures = captures
                    .iter()
                    .map(|capture| self.lower_expression(*capture))
                    .collect::<Result<Vec<_>, _>>()?;
                self.push(MirInstruction::TaskSpawn {
                    output,
                    callee,
                    callable,
                    captures,
                    span: expression.span(),
                });
            }
            HirExpressionKind::TaskAwait(task) => {
                let task = self.lower_expression(*task)?;
                self.push(MirInstruction::Suspend {
                    span: expression.span(),
                });
                self.push(MirInstruction::TaskAwait {
                    output,
                    task,
                    span: expression.span(),
                });
                self.push(MirInstruction::Resume {
                    span: expression.span(),
                });
            }
            HirExpressionKind::TaskCancel(task) => {
                let task = self.lower_expression(*task)?;
                self.push(MirInstruction::TaskCancel {
                    task,
                    span: expression.span(),
                });
            }
            HirExpressionKind::ChannelCreate(capacity) => {
                let capacity = self.lower_expression(*capacity)?;
                self.push(MirInstruction::ChannelCreate {
                    output,
                    capacity,
                    span: expression.span(),
                });
            }
            HirExpressionKind::ChannelSend { channel, value } => {
                let channel_id = *channel;
                let channel_value = self.lower_expression(channel_id)?;
                let value = self.lower_expression(*value)?;
                let element_type = self
                    .function
                    .expressions()
                    .iter()
                    .find(|candidate| candidate.id() == channel_id)
                    .and_then(|candidate| self.types.get(candidate.ty()))
                    .and_then(|record| match record.kind() {
                        TypeKind::Channel(channel) => Some(channel.element()),
                        _ => None,
                    })
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                self.push(MirInstruction::ChannelSend {
                    channel: channel_value,
                    value,
                    element_type,
                    span: expression.span(),
                });
            }
            HirExpressionKind::ChannelReceive(channel) => {
                let channel_id = *channel;
                let channel = self.lower_expression(channel_id)?;
                let element_type = self
                    .function
                    .expressions()
                    .iter()
                    .find(|candidate| candidate.id() == channel_id)
                    .and_then(|candidate| self.types.get(candidate.ty()))
                    .and_then(|record| match record.kind() {
                        TypeKind::Channel(channel) => Some(channel.element()),
                        _ => None,
                    })
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                self.push(MirInstruction::Suspend {
                    span: expression.span(),
                });
                self.push(MirInstruction::ChannelReceive {
                    output,
                    channel,
                    element_type,
                    span: expression.span(),
                });
                self.push(MirInstruction::Resume {
                    span: expression.span(),
                });
            }
            HirExpressionKind::ChannelClose(channel) => {
                let channel = self.lower_expression(*channel)?;
                self.push(MirInstruction::ChannelClose {
                    channel,
                    span: expression.span(),
                });
            }
            HirExpressionKind::Index { .. }
            | HirExpressionKind::StringLiteral(_)
            | HirExpressionKind::StringLength(_)
            | HirExpressionKind::StringClone(_)
            | HirExpressionKind::FieldAccess { .. }
            | HirExpressionKind::NewObject { .. } => {
                return Err(MirLoweringError::UnsupportedExpression);
            }
        }
        for local in self.function.locals() {
            if local.initializer() == Some(expression.id())
                && !matches!(
                    self.types.get(local.ty()).map(|record| record.kind()),
                    Some(TypeKind::Primitive(PrimitiveType::Unit))
                )
            {
                self.push(MirInstruction::StoreLocal {
                    local: MirLocalId::from_raw(local.id().index()),
                    value: output,
                    span: expression.span(),
                });
            }
        }
        for assignment in self.function.assignments() {
            if assignment.value().index() == expression.id().index()
                && !matches!(
                    self.function
                        .locals()
                        .iter()
                        .find(|local| local.id() == assignment.target())
                        .map(|local| self.types.get(local.ty()).map(|record| record.kind())),
                    Some(Some(TypeKind::Primitive(PrimitiveType::Unit)))
                )
            {
                self.push(MirInstruction::StoreLocal {
                    local: MirLocalId::from_raw(assignment.target().index()),
                    value: output,
                    span: assignment.span(),
                });
            }
        }
        self.lowered.insert(id);
        Ok(output)
    }

    fn finish(mut self) -> Result<MirFunction, MirLoweringError> {
        let returned = self
            .function
            .returns()
            .first()
            .ok_or(MirLoweringError::MissingReturn)?;
        let value = self.lower_expression(returned.expression())?;
        let return_is_unit = matches!(
            self.types
                .get(self.function.return_type())
                .map(|record| record.kind()),
            Some(TypeKind::Primitive(PrimitiveType::Unit))
        );
        if self.blocks[self.current].terminator.is_none() {
            self.blocks[self.current].terminator = Some(if return_is_unit {
                MirTerminator::return_unit(returned.span())
            } else {
                MirTerminator::return_value(value, returned.span())
            });
        }
        let blocks = self
            .blocks
            .into_iter()
            .map(|block| {
                Ok(MirBasicBlock::new(
                    block.id,
                    block.instructions,
                    block.terminator.ok_or(MirLoweringError::MissingReturn)?,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut mir = MirFunction::new(
            MirFunctionId::from_raw(self.function.id().index()),
            self.function.span(),
            self.function
                .parameters()
                .iter()
                .map(|parameter| (MirValueId::from_raw(parameter.id().index()), parameter.ty()))
                .collect(),
            self.function.return_type(),
            self.locals,
            blocks,
            MirCleanupBoundary::for_function(self.function, self.types),
        )
        .with_entry(self.function.is_entry())
        .with_suspend(self.function.is_suspend());
        if let Some(contract) = self.function.effect_contract() {
            mir = mir.with_effect_contract(contract.clone());
        }
        Ok(mir)
    }
}

fn lower_hir_function_with_short_circuit(
    function: &HirFunction,
    types: &TypeArena,
) -> Result<MirFunction, MirLoweringError> {
    ShortCircuitLowerer::new(function, types).finish()
}

struct ControlFlowLowerer<'a> {
    function: &'a HirFunction,
    types: &'a TypeArena,
    blocks: Vec<LowerBlock>,
    current: usize,
    locals: Vec<MirLocal>,
    next_local: usize,
    next_block: usize,
    next_value: usize,
    expression_index: HashMap<HirExpressionId, HirExpression>,
    lowered: HashSet<HirExpressionId>,
    loops: Vec<(MirBlockId, MirBlockId)>,
}

impl<'a> ControlFlowLowerer<'a> {
    fn new(function: &'a HirFunction, types: &'a TypeArena) -> Self {
        let next_value = function.parameters().len()
            + function
                .expressions()
                .iter()
                .map(|expression| expression.id().index())
                .max()
                .map_or(0, |value| value + 1);
        Self {
            function,
            types,
            blocks: vec![LowerBlock {
                id: MirBlockId::from_raw(0),
                instructions: Vec::new(),
                terminator: None,
            }],
            current: 0,
            locals: function
                .locals()
                .iter()
                .map(|local| {
                    MirLocal::new(
                        MirLocalId::from_raw(local.id().index()),
                        local.ty(),
                        local.span(),
                    )
                })
                .collect(),
            next_local: function.locals().len(),
            next_block: 1,
            next_value,
            expression_index: function
                .expressions()
                .iter()
                .cloned()
                .map(|expression| (expression.id(), expression))
                .collect(),
            lowered: HashSet::new(),
            loops: Vec::new(),
        }
    }

    fn new_block(&mut self) -> usize {
        let index = self.blocks.len();
        self.blocks.push(LowerBlock {
            id: MirBlockId::from_raw(self.next_block),
            instructions: Vec::new(),
            terminator: None,
        });
        self.next_block += 1;
        index
    }

    fn fresh_value(&mut self) -> MirValueId {
        let value = MirValueId::from_raw(self.next_value);
        self.next_value += 1;
        value
    }

    fn fresh_local(&mut self, ty: TypeId, span: crate::source::ByteSpan) -> MirLocalId {
        let local = MirLocalId::from_raw(self.next_local);
        self.next_local += 1;
        self.locals.push(MirLocal::new(local, ty, span));
        local
    }

    fn push(&mut self, instruction: MirInstruction) {
        self.blocks[self.current].instructions.push(instruction);
    }

    fn terminate(&mut self, terminator: MirTerminator) {
        self.blocks[self.current].terminator = Some(terminator);
    }

    fn is_terminated(&self) -> bool {
        self.blocks[self.current].terminator.is_some()
    }

    fn lower_expression(&mut self, id: HirExpressionId) -> Result<MirValueId, MirLoweringError> {
        let expression = self
            .expression_index
            .get(&id)
            .cloned()
            .ok_or(MirLoweringError::UnsupportedExpression)?;
        require_hir_expression_type(&expression, self.types)?;
        let output = mir_expression_value_id(self.function, id);
        if self.lowered.contains(&id) {
            return Ok(output);
        }
        match expression.kind() {
            HirExpressionKind::IntLiteral(value) => {
                self.push(MirInstruction::int_constant(
                    output,
                    *value,
                    expression.span(),
                ));
            }
            HirExpressionKind::EnumVariant(value) => {
                self.push(MirInstruction::int_constant(
                    output,
                    *value,
                    expression.span(),
                ));
            }
            HirExpressionKind::EnumConstruct { tag, payloads } => {
                let payload = payloads
                    .first()
                    .copied()
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                let payload = self.lower_expression(payload)?;
                self.push(MirInstruction::EnumConstruct {
                    output,
                    tag: *tag,
                    payload,
                    span: expression.span(),
                });
            }
            HirExpressionKind::EnumPayload { subject, index } => {
                let value = self.lower_expression(*subject)?;
                let is_channel_result = self
                    .function
                    .expressions()
                    .iter()
                    .find(|candidate| candidate.id() == *subject)
                    .and_then(|candidate| self.types.get(candidate.ty()))
                    .is_some_and(|record| matches!(record.kind(), TypeKind::ChannelResult(_)));
                if is_channel_result {
                    self.push(MirInstruction::ChannelResultPayload {
                        output,
                        value,
                        element_type: expression.ty(),
                        span: expression.span(),
                    });
                } else {
                    self.push(MirInstruction::EnumPayload {
                        output,
                        value,
                        index: *index,
                        span: expression.span(),
                    });
                }
            }
            HirExpressionKind::BoolLiteral(value) => {
                self.push(MirInstruction::bool_constant(
                    output,
                    *value,
                    expression.span(),
                ));
            }
            HirExpressionKind::FloatLiteral(bits) => {
                self.push(MirInstruction::float_constant(
                    output,
                    *bits,
                    expression.span(),
                ));
            }
            HirExpressionKind::ByteLiteral(value) => {
                self.push(MirInstruction::byte_constant(
                    output,
                    *value,
                    expression.span(),
                ));
            }
            HirExpressionKind::UnitLiteral => {
                self.push(MirInstruction::unit_constant(expression.span()));
            }
            HirExpressionKind::ParameterRead(parameter) => {
                self.push(MirInstruction::ParameterRead {
                    output,
                    parameter: MirParameterId::from_raw(parameter.index()),
                    span: expression.span(),
                });
            }
            HirExpressionKind::LocalRead(local) => {
                if matches!(
                    self.types.get(expression.ty()).map(|record| record.kind()),
                    Some(TypeKind::Array(_))
                ) {
                    self.push(MirInstruction::ArrayValue {
                        output,
                        local: MirLocalId::from_raw(local.index()),
                        span: expression.span(),
                    });
                } else if !matches!(
                    self.types.get(expression.ty()).map(|record| record.kind()),
                    Some(TypeKind::Primitive(PrimitiveType::Unit))
                ) {
                    self.push(MirInstruction::LoadLocal {
                        output,
                        local: MirLocalId::from_raw(local.index()),
                        span: expression.span(),
                    });
                }
            }
            HirExpressionKind::Binary(binary) => {
                let left = self.lower_expression(binary.left())?;
                let right = self.lower_expression(binary.right())?;
                if let Some(operation) = lower_comparison(binary.operator()) {
                    self.push(MirInstruction::Compare {
                        output,
                        operation,
                        left,
                        right,
                        span: expression.span(),
                    });
                } else {
                    self.push(MirInstruction::CheckedArithmetic {
                        output,
                        operation: lower_binary(binary.operator())?,
                        left,
                        right,
                        span: expression.span(),
                    });
                }
            }
            HirExpressionKind::Unary(unary) => {
                let operand = self.lower_expression(unary.operand())?;
                if matches!(unary.operator(), HirUnaryOperator::Not) {
                    self.push(MirInstruction::LogicalNot {
                        output,
                        operand,
                        span: expression.span(),
                    });
                } else {
                    self.push(MirInstruction::Unary {
                        output,
                        operation: lower_unary(unary.operator())?,
                        operand,
                        span: expression.span(),
                    });
                }
            }
            HirExpressionKind::DirectCall(call) => {
                self.push(mir_call_instruction(
                    call,
                    output,
                    call.arguments()
                        .iter()
                        .map(|argument| mir_expression_value_id(self.function, *argument))
                        .collect(),
                    expression.span(),
                ));
            }
            HirExpressionKind::TestAssert { condition, message } => {
                self.push(MirInstruction::TestAssert {
                    condition: mir_expression_value_id(self.function, *condition),
                    message: mir_expression_value_id(self.function, *message),
                    span: expression.span(),
                });
            }
            HirExpressionKind::TestFail { message } => {
                self.push(MirInstruction::TestFail {
                    message: mir_expression_value_id(self.function, *message),
                    span: expression.span(),
                });
            }
            HirExpressionKind::FunctionReference(callee) => {
                self.push(MirInstruction::FunctionReference {
                    output,
                    callee: MirFunctionId::from_raw(callee.index()),
                    span: expression.span(),
                });
            }
            HirExpressionKind::IndirectCall { callee, arguments } => {
                let callee_expression = *callee;
                let callee = self.lower_expression(callee_expression)?;
                let arguments = arguments
                    .iter()
                    .map(|argument| self.lower_expression(*argument))
                    .collect::<Result<Vec<_>, _>>()?;
                self.push(MirInstruction::IndirectCall {
                    output,
                    callee,
                    function_type: self
                        .function
                        .expressions()
                        .iter()
                        .find(|candidate| candidate.id() == callee_expression)
                        .map(|candidate| candidate.ty())
                        .ok_or(MirLoweringError::UnsupportedExpression)?,
                    arguments,
                    span: expression.span(),
                });
            }
            HirExpressionKind::Conditional {
                condition,
                then_value,
                else_value,
            } => {
                let condition = self.lower_expression(*condition)?;
                let result_local = self.fresh_local(expression.ty(), expression.span());
                let then_block = self.new_block();
                let else_block = self.new_block();
                let merge_block = self.new_block();
                self.terminate(MirTerminator::branch_if(
                    condition,
                    self.blocks[then_block].id,
                    self.blocks[else_block].id,
                    expression.span(),
                ));
                self.current = then_block;
                let then_value = self.lower_expression(*then_value)?;
                self.push(MirInstruction::StoreLocal {
                    local: result_local,
                    value: then_value,
                    span: expression.span(),
                });
                self.terminate(MirTerminator::Branch {
                    target: self.blocks[merge_block].id,
                    span: expression.span(),
                });
                self.current = else_block;
                let else_value = self.lower_expression(*else_value)?;
                self.push(MirInstruction::StoreLocal {
                    local: result_local,
                    value: else_value,
                    span: expression.span(),
                });
                self.terminate(MirTerminator::Branch {
                    target: self.blocks[merge_block].id,
                    span: expression.span(),
                });
                self.current = merge_block;
                self.push(MirInstruction::LoadLocal {
                    output,
                    local: result_local,
                    span: expression.span(),
                });
            }
            HirExpressionKind::When { subject, arms } => {
                let subject_value = self.lower_expression(*subject)?;
                let subject_expression = *subject;
                let subject = self.fresh_value();
                let is_channel_result = self
                    .function
                    .expressions()
                    .iter()
                    .find(|candidate| candidate.id() == subject_expression)
                    .and_then(|candidate| self.types.get(candidate.ty()))
                    .is_some_and(|record| matches!(record.kind(), TypeKind::ChannelResult(_)));
                if is_channel_result {
                    self.push(MirInstruction::ChannelResultTag {
                        output: subject,
                        value: subject_value,
                        span: expression.span(),
                    });
                } else {
                    self.push(MirInstruction::EnumTag {
                        output: subject,
                        value: subject_value,
                        span: expression.span(),
                    });
                }
                let result_local = self.fresh_local(expression.ty(), expression.span());
                let merge_block = self.new_block();
                let mut test_block = self.current;
                let mut wildcard = None;
                for (tag, value) in arms {
                    let Some(tag) = tag else {
                        wildcard = Some(*value);
                        continue;
                    };
                    let matched_block = self.new_block();
                    let next_block = self.new_block();
                    self.current = test_block;
                    let tag_value = self.fresh_value();
                    self.push(MirInstruction::int_constant(
                        tag_value,
                        *tag,
                        expression.span(),
                    ));
                    let condition = self.fresh_value();
                    self.push(MirInstruction::Compare {
                        output: condition,
                        operation: MirComparison::Equal,
                        left: subject,
                        right: tag_value,
                        span: expression.span(),
                    });
                    self.terminate(MirTerminator::branch_if(
                        condition,
                        self.blocks[matched_block].id,
                        self.blocks[next_block].id,
                        expression.span(),
                    ));
                    self.current = matched_block;
                    let value = self.lower_expression(*value)?;
                    self.push(MirInstruction::StoreLocal {
                        local: result_local,
                        value,
                        span: expression.span(),
                    });
                    self.terminate(MirTerminator::Branch {
                        target: self.blocks[merge_block].id,
                        span: expression.span(),
                    });
                    test_block = next_block;
                }
                self.current = test_block;
                if let Some(value) = wildcard {
                    let value = self.lower_expression(value)?;
                    self.push(MirInstruction::StoreLocal {
                        local: result_local,
                        value,
                        span: expression.span(),
                    });
                    self.terminate(MirTerminator::Branch {
                        target: self.blocks[merge_block].id,
                        span: expression.span(),
                    });
                } else {
                    self.terminate(MirTerminator::Trap {
                        reason: MirTrap::UnsupportedRuntime,
                        span: expression.span(),
                    });
                }
                self.current = merge_block;
                self.push(MirInstruction::LoadLocal {
                    output,
                    local: result_local,
                    span: expression.span(),
                });
            }
            HirExpressionKind::ArrayLiteral(elements) => {
                let local = self
                    .function
                    .locals()
                    .iter()
                    .find(|local| local.initializer() == Some(id))
                    .map(|local| local.id())
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                let element_values = elements
                    .iter()
                    .map(|element| self.lower_expression(*element))
                    .collect::<Result<Vec<_>, _>>()?;
                self.push(MirInstruction::ArrayInit {
                    local: MirLocalId::from_raw(local.index()),
                    elements: element_values,
                    span: expression.span(),
                });
                self.push(MirInstruction::ArrayValue {
                    output,
                    local: MirLocalId::from_raw(local.index()),
                    span: expression.span(),
                });
            }
            HirExpressionKind::TaskSpawn { callable, captures } => {
                let callable_id = *callable;
                let callable = self.lower_expression(callable_id)?;
                let callee = mir_task_callee(self.function, callable_id)?;
                let captures = captures
                    .iter()
                    .map(|capture| self.lower_expression(*capture))
                    .collect::<Result<Vec<_>, _>>()?;
                self.push(MirInstruction::TaskSpawn {
                    output,
                    callee,
                    callable,
                    captures,
                    span: expression.span(),
                });
            }
            HirExpressionKind::TaskAwait(task) => {
                let task = self.lower_expression(*task)?;
                self.push(MirInstruction::Suspend {
                    span: expression.span(),
                });
                self.push(MirInstruction::TaskAwait {
                    output,
                    task,
                    span: expression.span(),
                });
                self.push(MirInstruction::Resume {
                    span: expression.span(),
                });
            }
            HirExpressionKind::TaskCancel(task) => {
                let task = self.lower_expression(*task)?;
                self.push(MirInstruction::TaskCancel {
                    task,
                    span: expression.span(),
                });
            }
            HirExpressionKind::ChannelCreate(capacity) => {
                let capacity = self.lower_expression(*capacity)?;
                self.push(MirInstruction::ChannelCreate {
                    output,
                    capacity,
                    span: expression.span(),
                });
            }
            HirExpressionKind::ChannelSend { channel, value } => {
                let channel_id = *channel;
                let channel_value = self.lower_expression(channel_id)?;
                let value = self.lower_expression(*value)?;
                let element_type = self
                    .function
                    .expressions()
                    .iter()
                    .find(|candidate| candidate.id() == channel_id)
                    .and_then(|candidate| self.types.get(candidate.ty()))
                    .and_then(|record| match record.kind() {
                        TypeKind::Channel(channel) => Some(channel.element()),
                        _ => None,
                    })
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                self.push(MirInstruction::ChannelSend {
                    channel: channel_value,
                    value,
                    element_type,
                    span: expression.span(),
                });
            }
            HirExpressionKind::ChannelReceive(channel) => {
                let channel_id = *channel;
                let channel = self.lower_expression(channel_id)?;
                let element_type = self
                    .function
                    .expressions()
                    .iter()
                    .find(|candidate| candidate.id() == channel_id)
                    .and_then(|candidate| self.types.get(candidate.ty()))
                    .and_then(|record| match record.kind() {
                        TypeKind::Channel(channel) => Some(channel.element()),
                        _ => None,
                    })
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                self.push(MirInstruction::Suspend {
                    span: expression.span(),
                });
                self.push(MirInstruction::ChannelReceive {
                    output,
                    channel,
                    element_type,
                    span: expression.span(),
                });
                self.push(MirInstruction::Resume {
                    span: expression.span(),
                });
            }
            HirExpressionKind::ChannelClose(channel) => {
                let channel = self.lower_expression(*channel)?;
                self.push(MirInstruction::ChannelClose {
                    channel,
                    span: expression.span(),
                });
            }
            HirExpressionKind::DynamicArrayNew => {
                self.push(MirInstruction::DynamicArrayNew {
                    output,
                    type_id: expression.ty(),
                    span: expression.span(),
                });
            }
            HirExpressionKind::DynamicArraySize(array) => {
                let array = self.lower_expression(*array)?;
                self.push(MirInstruction::DynamicArraySize {
                    output,
                    array,
                    span: expression.span(),
                });
            }
            HirExpressionKind::DynamicArrayAdd {
                array,
                value,
                index,
            } => {
                let array_id = *array;
                let array = self.lower_expression(array_id)?;
                let value = self.lower_expression(*value)?;
                let element_type = self
                    .function
                    .expressions()
                    .iter()
                    .find(|candidate| candidate.id() == array_id)
                    .and_then(|candidate| self.types.get(candidate.ty()))
                    .and_then(|record| match record.kind() {
                        TypeKind::DynamicArray(array) => Some(array.element()),
                        _ => None,
                    })
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                let index = index
                    .map(|index| self.lower_expression(index))
                    .transpose()?;
                self.push(MirInstruction::DynamicArrayAdd {
                    element_type,
                    array,
                    value,
                    index,
                    span: expression.span(),
                });
            }
            HirExpressionKind::DynamicArrayRemove { array, index } => {
                let array_id = *array;
                let array = self.lower_expression(array_id)?;
                let index = self.lower_expression(*index)?;
                let element_type = self
                    .function
                    .expressions()
                    .iter()
                    .find(|candidate| candidate.id() == array_id)
                    .and_then(|candidate| self.types.get(candidate.ty()))
                    .and_then(|record| match record.kind() {
                        TypeKind::DynamicArray(array) => Some(array.element()),
                        _ => None,
                    })
                    .ok_or(MirLoweringError::UnsupportedExpression)?;
                self.push(MirInstruction::DynamicArrayRemove {
                    element_type,
                    array,
                    index,
                    span: expression.span(),
                });
            }
            HirExpressionKind::Index { .. }
            | HirExpressionKind::StringLiteral(_)
            | HirExpressionKind::StringLength(_)
            | HirExpressionKind::StringClone(_)
            | HirExpressionKind::FieldAccess { .. }
            | HirExpressionKind::NewObject { .. } => {
                return Err(MirLoweringError::UnsupportedExpression);
            }
        }
        self.lowered.insert(id);
        Ok(output)
    }

    fn lower_sequence(&mut self, statements: &[HirControlFlow]) -> Result<(), MirLoweringError> {
        for statement in statements {
            if self.is_terminated() {
                break;
            }
            match statement {
                HirControlFlow::LocalInitializer { local, value, span } => {
                    let value = self.lower_expression(*value)?;
                    if !matches!(
                        self.types
                            .get(self.local_type(*local)?)
                            .map(|record| record.kind()),
                        Some(TypeKind::Primitive(PrimitiveType::Unit) | TypeKind::Array(_))
                    ) {
                        self.push(MirInstruction::StoreLocal {
                            local: MirLocalId::from_raw(local.index()),
                            value,
                            span: *span,
                        });
                    }
                }
                HirControlFlow::Expression { value, .. } => {
                    let _ = self.lower_expression(*value)?;
                }
                HirControlFlow::Scope { body, span } => {
                    self.push(MirInstruction::ScopeEnter { span: *span });
                    self.lower_sequence(body)?;
                    if !self.is_terminated() {
                        self.push(MirInstruction::ScopeExit { span: *span });
                    }
                }
                HirControlFlow::Assignment(assignment) => {
                    let value = self.lower_expression(assignment.value())?;
                    self.push(MirInstruction::StoreLocal {
                        local: MirLocalId::from_raw(assignment.target().index()),
                        value,
                        span: assignment.span(),
                    });
                }
                HirControlFlow::Return(returned) => {
                    let value = self.lower_expression(returned.expression())?;
                    if self.is_unit(self.function.return_type()) {
                        self.terminate(MirTerminator::return_unit(returned.span()));
                    } else {
                        self.terminate(MirTerminator::return_value(value, returned.span()));
                    }
                }
                HirControlFlow::If {
                    condition,
                    then_body,
                    else_body,
                    span,
                } => {
                    let condition = self.lower_expression(*condition)?;
                    let then_block = self.new_block();
                    let else_block = self.new_block();
                    let merge_block = self.new_block();
                    self.terminate(MirTerminator::branch_if(
                        condition,
                        self.blocks[then_block].id,
                        self.blocks[else_block].id,
                        *span,
                    ));
                    self.current = then_block;
                    self.lower_sequence(then_body)?;
                    if !self.is_terminated() {
                        self.terminate(MirTerminator::Branch {
                            target: self.blocks[merge_block].id,
                            span: *span,
                        });
                    }
                    self.current = else_block;
                    self.lower_sequence(else_body)?;
                    if !self.is_terminated() {
                        self.terminate(MirTerminator::Branch {
                            target: self.blocks[merge_block].id,
                            span: *span,
                        });
                    }
                    self.current = merge_block;
                }
                HirControlFlow::For {
                    binding,
                    start,
                    end,
                    body,
                    span,
                } => {
                    let start_value = self.lower_expression(*start)?;
                    let end_value = self.lower_expression(*end)?;
                    let binding_id = MirLocalId::from_raw(binding.index());
                    self.push(MirInstruction::StoreLocal {
                        local: binding_id,
                        value: start_value,
                        span: *span,
                    });
                    let header = self.new_block();
                    let body_block = self.new_block();
                    let update = self.new_block();
                    let exit = self.new_block();
                    self.terminate(MirTerminator::Branch {
                        target: self.blocks[header].id,
                        span: *span,
                    });
                    self.current = header;
                    let current_value = self.fresh_value();
                    self.push(MirInstruction::LoadLocal {
                        output: current_value,
                        local: binding_id,
                        span: *span,
                    });
                    let condition = self.fresh_value();
                    self.push(MirInstruction::Compare {
                        output: condition,
                        operation: MirComparison::LessEqual,
                        left: current_value,
                        right: end_value,
                        span: *span,
                    });
                    self.terminate(MirTerminator::branch_if(
                        condition,
                        self.blocks[body_block].id,
                        self.blocks[exit].id,
                        *span,
                    ));
                    self.loops
                        .push((self.blocks[exit].id, self.blocks[update].id));
                    self.current = body_block;
                    self.lower_sequence(body)?;
                    if !self.is_terminated() {
                        self.terminate(MirTerminator::Branch {
                            target: self.blocks[update].id,
                            span: *span,
                        });
                    }
                    self.loops.pop();
                    self.current = update;
                    let current_value = self.fresh_value();
                    self.push(MirInstruction::LoadLocal {
                        output: current_value,
                        local: binding_id,
                        span: *span,
                    });
                    let one = self.fresh_value();
                    self.push(MirInstruction::int_constant(one, 1, *span));
                    let next = self.fresh_value();
                    self.push(MirInstruction::CheckedArithmetic {
                        output: next,
                        operation: MirArithmetic::Add,
                        left: current_value,
                        right: one,
                        span: *span,
                    });
                    self.push(MirInstruction::StoreLocal {
                        local: binding_id,
                        value: next,
                        span: *span,
                    });
                    self.terminate(MirTerminator::Branch {
                        target: self.blocks[header].id,
                        span: *span,
                    });
                    self.current = exit;
                }
                HirControlFlow::ForEach {
                    binding,
                    array,
                    body,
                    span,
                } => {
                    let array_value = self.lower_expression(*array)?;
                    let array_type = self
                        .function
                        .expressions()
                        .iter()
                        .find(|expression| expression.id() == *array)
                        .map(|expression| expression.ty())
                        .ok_or(MirLoweringError::UnsupportedExpression)?;
                    let is_dynamic = matches!(
                        self.types.get(array_type).map(|record| record.kind()),
                        Some(TypeKind::DynamicArray(_))
                    );
                    let fixed_length = match self.types.get(array_type).map(|record| record.kind())
                    {
                        Some(TypeKind::Array(array)) => Some(array.length()),
                        _ => None,
                    };
                    let index_local = MirLocalId::from_raw(self.locals.len());
                    self.locals
                        .push(MirLocal::new(index_local, self.int_type()?, *span));
                    let zero = self.fresh_value();
                    self.push(MirInstruction::int_constant(zero, 0, *span));
                    self.push(MirInstruction::StoreLocal {
                        local: index_local,
                        value: zero,
                        span: *span,
                    });
                    let header = self.new_block();
                    let body_block = self.new_block();
                    let update = self.new_block();
                    let exit = self.new_block();
                    self.terminate(MirTerminator::Branch {
                        target: self.blocks[header].id,
                        span: *span,
                    });
                    self.current = header;
                    let index = self.fresh_value();
                    self.push(MirInstruction::LoadLocal {
                        output: index,
                        local: index_local,
                        span: *span,
                    });
                    let length = self.fresh_value();
                    if is_dynamic {
                        self.push(MirInstruction::DynamicArraySize {
                            output: length,
                            array: array_value,
                            span: *span,
                        });
                    } else {
                        self.push(MirInstruction::int_constant(
                            length,
                            i64::try_from(fixed_length.unwrap_or(0)).unwrap_or(0),
                            *span,
                        ));
                    }
                    let condition = self.fresh_value();
                    self.push(MirInstruction::Compare {
                        output: condition,
                        operation: MirComparison::Less,
                        left: index,
                        right: length,
                        span: *span,
                    });
                    self.terminate(MirTerminator::branch_if(
                        condition,
                        self.blocks[body_block].id,
                        self.blocks[exit].id,
                        *span,
                    ));
                    self.loops
                        .push((self.blocks[exit].id, self.blocks[update].id));
                    self.current = body_block;
                    let element = self.fresh_value();
                    if is_dynamic {
                        self.push(MirInstruction::DynamicArrayLoad {
                            output: element,
                            array: array_value,
                            index,
                            span: *span,
                        });
                    } else {
                        self.push(MirInstruction::ArrayElementLoad {
                            output: element,
                            array: array_value,
                            index,
                            span: *span,
                        });
                    }
                    self.push(MirInstruction::StoreLocal {
                        local: MirLocalId::from_raw(binding.index()),
                        value: element,
                        span: *span,
                    });
                    self.lower_sequence(body)?;
                    if !self.is_terminated() {
                        self.terminate(MirTerminator::Branch {
                            target: self.blocks[update].id,
                            span: *span,
                        });
                    }
                    self.loops.pop();
                    self.current = update;
                    let current = self.fresh_value();
                    self.push(MirInstruction::LoadLocal {
                        output: current,
                        local: index_local,
                        span: *span,
                    });
                    let one = self.fresh_value();
                    self.push(MirInstruction::int_constant(one, 1, *span));
                    let next = self.fresh_value();
                    self.push(MirInstruction::CheckedArithmetic {
                        output: next,
                        operation: MirArithmetic::Add,
                        left: current,
                        right: one,
                        span: *span,
                    });
                    self.push(MirInstruction::StoreLocal {
                        local: index_local,
                        value: next,
                        span: *span,
                    });
                    self.terminate(MirTerminator::Branch {
                        target: self.blocks[header].id,
                        span: *span,
                    });
                    self.current = exit;
                }
                HirControlFlow::Break { span } => {
                    let (target, _) = self
                        .loops
                        .last()
                        .copied()
                        .ok_or(MirLoweringError::UnsupportedExpression)?;
                    self.terminate(MirTerminator::Branch {
                        target,
                        span: *span,
                    });
                }
                HirControlFlow::Continue { span } => {
                    let (_, target) = self
                        .loops
                        .last()
                        .copied()
                        .ok_or(MirLoweringError::UnsupportedExpression)?;
                    self.terminate(MirTerminator::Branch {
                        target,
                        span: *span,
                    });
                }
            }
        }
        Ok(())
    }

    fn local_type(&self, local: HirLocalId) -> Result<TypeId, MirLoweringError> {
        self.function
            .locals()
            .iter()
            .find(|candidate| candidate.id() == local)
            .map(|candidate| candidate.ty())
            .ok_or(MirLoweringError::UnsupportedExpression)
    }

    fn int_type(&self) -> Result<TypeId, MirLoweringError> {
        self.types
            .records()
            .iter()
            .find(|record| record.kind() == &TypeKind::Primitive(PrimitiveType::Int))
            .map(|record| record.id())
            .ok_or(MirLoweringError::UnsupportedRuntimeType)
    }

    fn is_unit(&self, ty: TypeId) -> bool {
        matches!(
            self.types.get(ty).map(|record| record.kind()),
            Some(TypeKind::Primitive(PrimitiveType::Unit))
        )
    }
}

fn lower_hir_function_with_control_flow(
    function: &HirFunction,
    types: &TypeArena,
) -> Result<MirFunction, MirLoweringError> {
    let mut lowerer = ControlFlowLowerer::new(function, types);
    if function.control_flow().is_empty() {
        for local in function.locals() {
            if let Some(initializer) = local.initializer() {
                let value = lowerer.lower_expression(initializer)?;
                if !lowerer.is_unit(local.ty()) {
                    lowerer.push(MirInstruction::StoreLocal {
                        local: MirLocalId::from_raw(local.id().index()),
                        value,
                        span: local.span(),
                    });
                }
            }
        }
        let returned = function
            .returns()
            .first()
            .ok_or(MirLoweringError::MissingReturn)?;
        let value = lowerer.lower_expression(returned.expression())?;
        if lowerer.is_unit(function.return_type()) {
            lowerer.terminate(MirTerminator::return_unit(returned.span()));
        } else {
            lowerer.terminate(MirTerminator::return_value(value, returned.span()));
        }
    } else {
        lowerer.lower_sequence(function.control_flow())?;
    }
    let ControlFlowLowerer { blocks, locals, .. } = lowerer;
    let blocks = blocks
        .into_iter()
        .map(|block| {
            Ok(MirBasicBlock::new(
                block.id,
                block.instructions,
                block.terminator.ok_or(MirLoweringError::MissingReturn)?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut mir = MirFunction::new(
        MirFunctionId::from_raw(function.id().index()),
        function.span(),
        function
            .parameters()
            .iter()
            .map(|parameter| (MirValueId::from_raw(parameter.id().index()), parameter.ty()))
            .collect(),
        function.return_type(),
        locals,
        blocks,
        MirCleanupBoundary::for_function(function, types),
    )
    .with_entry(function.is_entry())
    .with_suspend(function.is_suspend());
    if let Some(contract) = function.effect_contract() {
        mir = mir.with_effect_contract(contract.clone());
    }
    Ok(match function.symbol_identity() {
        Some(identity) => mir.with_symbol_identity(identity.clone()),
        None => mir,
    })
}

fn require_bootstrap_runtime_type(ty: TypeId, types: &TypeArena) -> Result<(), MirLoweringError> {
    match types.get(ty).map(|record| record.kind()) {
        Some(TypeKind::Primitive(
            PrimitiveType::Bool
            | PrimitiveType::Int
            | PrimitiveType::Float
            | PrimitiveType::Byte
            | PrimitiveType::String
            | PrimitiveType::Unit,
        )) => Ok(()),
        Some(TypeKind::Array(array)) => require_bootstrap_runtime_type(array.element(), types),
        Some(TypeKind::DynamicArray(_)) => Ok(()),
        Some(TypeKind::Task(task)) => require_bootstrap_runtime_type(task.result(), types),
        Some(TypeKind::Channel(_)) => Ok(()),
        Some(TypeKind::ChannelResult(result)) => {
            require_bootstrap_runtime_type(result.element(), types)
        }
        Some(
            TypeKind::Nominal(_)
            | TypeKind::GenericInstance(_)
            | TypeKind::GenericParameter(_)
            | TypeKind::Function(_),
        ) => Ok(()),
        _ => Err(MirLoweringError::UnsupportedRuntimeType),
    }
}

fn require_hir_expression_type(
    expression: &HirExpression,
    types: &TypeArena,
) -> Result<(), MirLoweringError> {
    let expected = match expression.kind() {
        HirExpressionKind::IntLiteral(_) => PrimitiveType::Int,
        HirExpressionKind::EnumVariant(_) => {
            return require_bootstrap_runtime_type(expression.ty(), types);
        }
        HirExpressionKind::BoolLiteral(_) => PrimitiveType::Bool,
        HirExpressionKind::FloatLiteral(_) => PrimitiveType::Float,
        HirExpressionKind::ByteLiteral(_) => PrimitiveType::Byte,
        HirExpressionKind::StringLiteral(_) => PrimitiveType::String,
        HirExpressionKind::UnitLiteral => PrimitiveType::Unit,
        _ => return require_bootstrap_runtime_type(expression.ty(), types),
    };
    matches!(
        types.get(expression.ty()).map(|record| record.kind()),
        Some(TypeKind::Primitive(actual)) if *actual == expected
    )
    .then_some(())
    .ok_or(MirLoweringError::UnsupportedRuntimeType)
}

fn lower_binary(operator: HirBinaryOperator) -> Result<MirArithmetic, MirLoweringError> {
    Ok(match operator {
        HirBinaryOperator::LogicalOr
        | HirBinaryOperator::LogicalAnd
        | HirBinaryOperator::Equal
        | HirBinaryOperator::NotEqual
        | HirBinaryOperator::Less
        | HirBinaryOperator::Greater
        | HirBinaryOperator::LessEqual
        | HirBinaryOperator::GreaterEqual => return Err(MirLoweringError::UnsupportedExpression),
        HirBinaryOperator::Plus => MirArithmetic::Add,
        HirBinaryOperator::Minus => MirArithmetic::Subtract,
        HirBinaryOperator::Multiply => MirArithmetic::Multiply,
        HirBinaryOperator::Divide => MirArithmetic::Divide,
        HirBinaryOperator::Remainder => MirArithmetic::Remainder,
        HirBinaryOperator::Exponent => MirArithmetic::Exponent,
        HirBinaryOperator::BitwiseAnd => MirArithmetic::BitwiseAnd,
        HirBinaryOperator::BitwiseOr => MirArithmetic::BitwiseOr,
        HirBinaryOperator::BitwiseXor => MirArithmetic::BitwiseXor,
        HirBinaryOperator::ShiftLeft => MirArithmetic::ShiftLeft,
        HirBinaryOperator::ShiftRight => MirArithmetic::ShiftRight,
    })
}

fn lower_comparison(operator: HirBinaryOperator) -> Option<MirComparison> {
    Some(match operator {
        HirBinaryOperator::Equal => MirComparison::Equal,
        HirBinaryOperator::NotEqual => MirComparison::NotEqual,
        HirBinaryOperator::Less => MirComparison::Less,
        HirBinaryOperator::Greater => MirComparison::Greater,
        HirBinaryOperator::LessEqual => MirComparison::LessEqual,
        HirBinaryOperator::GreaterEqual => MirComparison::GreaterEqual,
        _ => return None,
    })
}
fn lower_unary(operator: HirUnaryOperator) -> Result<MirUnary, MirLoweringError> {
    Ok(match operator {
        HirUnaryOperator::Not => return Err(MirLoweringError::UnsupportedExpression),
        HirUnaryOperator::Plus => MirUnary::Plus,
        HirUnaryOperator::Minus => MirUnary::Negate,
        HirUnaryOperator::BitwiseNot => MirUnary::BitwiseNot,
    })
}
