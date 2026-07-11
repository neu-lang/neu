use crate::{
    hir::HirExpression,
    hir::{HirBinaryOperator, HirExpressionKind, HirModule, HirUnaryOperator},
    module::{FunctionSymbolIdentity, ModuleName},
    source::ByteSpan,
    types::{PrimitiveType, TypeArena, TypeId, TypeKind},
};

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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MirInstruction {
    IntConstant {
        output: MirValueId,
        value: i64,
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
    DirectCall {
        output: MirValueId,
        callee: MirFunctionId,
        arguments: Vec<MirValueId>,
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
            | Self::BoolConstant { span, .. }
            | Self::FloatConstant { span, .. }
            | Self::ByteConstant { span, .. }
            | Self::UnitConstant { span }
            | Self::CheckedArithmetic { span, .. }
            | Self::Unary { span, .. }
            | Self::LoadLocal { span, .. }
            | Self::StoreLocal { span, .. }
            | Self::DirectCall { span, .. } => *span,
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
    Return { value: MirValueId, span: ByteSpan },
    ReturnUnit { span: ByteSpan },
    Branch { target: MirBlockId, span: ByteSpan },
    Trap { reason: MirTrap, span: ByteSpan },
}
impl MirTerminator {
    pub fn return_value(value: MirValueId, span: ByteSpan) -> Self {
        Self::Return { value, span }
    }

    pub fn return_unit(span: ByteSpan) -> Self {
        Self::ReturnUnit { span }
    }
    pub fn span(&self) -> ByteSpan {
        match self {
            Self::Return { span, .. }
            | Self::ReturnUnit { span }
            | Self::Branch { span, .. }
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MirCleanupBoundary {
    reserved: bool,
}
impl MirCleanupBoundary {
    pub fn empty() -> Self {
        Self { reserved: false }
    }
    pub fn is_empty(&self) -> bool {
        !self.reserved
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
    pub fn cleanup_boundary(&self) -> MirCleanupBoundary {
        self.cleanup_boundary
    }
    pub fn with_symbol_identity(mut self, identity: FunctionSymbolIdentity) -> Self {
        self.symbol_identity = Some(identity);
        self
    }
    pub fn symbol_identity(&self) -> Option<&FunctionSymbolIdentity> {
        self.symbol_identity.as_ref()
    }
    pub fn with_entry(mut self, entry: bool) -> Self {
        self.entry = entry;
        self
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
        let mut instructions = Vec::new();
        for expression in function.expressions() {
            require_hir_expression_type(expression, types)?;
            let output = MirValueId::from_raw(expression.id().index());
            match expression.kind() {
                HirExpressionKind::IntLiteral(value) => instructions.push(
                    MirInstruction::int_constant(output, *value, expression.span()),
                ),
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
                HirExpressionKind::Binary(binary) => {
                    instructions.push(MirInstruction::CheckedArithmetic {
                        output,
                        operation: lower_binary(binary.operator())?,
                        left: MirValueId::from_raw(binary.left().index()),
                        right: MirValueId::from_raw(binary.right().index()),
                        span: expression.span(),
                    })
                }
                HirExpressionKind::Unary(unary) => instructions.push(MirInstruction::Unary {
                    output,
                    operation: lower_unary(unary.operator())?,
                    operand: MirValueId::from_raw(unary.operand().index()),
                    span: expression.span(),
                }),
                HirExpressionKind::DirectCall(call) => {
                    instructions.push(MirInstruction::DirectCall {
                        output,
                        callee: MirFunctionId::from_raw(call.callee().index()),
                        arguments: call
                            .arguments()
                            .iter()
                            .map(|argument| MirValueId::from_raw(argument.index()))
                            .collect(),
                        span: expression.span(),
                    })
                }
                _ => return Err(MirLoweringError::UnsupportedExpression),
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
                MirValueId::from_raw(returned.expression().index()),
                returned.span(),
            )
        };
        let mir_function = MirFunction::new(
            MirFunctionId::from_raw(function.id().index()),
            function.span(),
            vec![],
            function.return_type(),
            vec![],
            vec![MirBasicBlock::new(
                MirBlockId::from_raw(0),
                instructions,
                terminator,
            )],
            MirCleanupBoundary::empty(),
        )
        .with_entry(function.is_entry());
        functions.push(match function.symbol_identity() {
            Some(identity) => mir_function.with_symbol_identity(identity.clone()),
            None => mir_function,
        });
    }
    Ok(MirModule::new(hir.name().clone(), functions))
}

fn require_bootstrap_runtime_type(ty: TypeId, types: &TypeArena) -> Result<(), MirLoweringError> {
    matches!(
        types.get(ty).map(|record| record.kind()),
        Some(TypeKind::Primitive(
            PrimitiveType::Bool
                | PrimitiveType::Int
                | PrimitiveType::Float
                | PrimitiveType::Byte
                | PrimitiveType::Unit
        ))
    )
    .then_some(())
    .ok_or(MirLoweringError::UnsupportedRuntimeType)
}

fn require_hir_expression_type(
    expression: &HirExpression,
    types: &TypeArena,
) -> Result<(), MirLoweringError> {
    let expected = match expression.kind() {
        HirExpressionKind::IntLiteral(_) => PrimitiveType::Int,
        HirExpressionKind::BoolLiteral(_) => PrimitiveType::Bool,
        HirExpressionKind::FloatLiteral(_) => PrimitiveType::Float,
        HirExpressionKind::ByteLiteral(_) => PrimitiveType::Byte,
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
fn lower_unary(operator: HirUnaryOperator) -> Result<MirUnary, MirLoweringError> {
    Ok(match operator {
        HirUnaryOperator::Not => return Err(MirLoweringError::UnsupportedExpression),
        HirUnaryOperator::Plus => MirUnary::Plus,
        HirUnaryOperator::Minus => MirUnary::Negate,
        HirUnaryOperator::BitwiseNot => MirUnary::BitwiseNot,
    })
}
