use crate::{
    hir::HirExpression,
    hir::{
        HirBinaryOperator, HirExpressionId, HirExpressionKind, HirFunction, HirModule,
        HirUnaryOperator,
    },
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
            | Self::ParameterRead { span, .. }
            | Self::CheckedArithmetic { span, .. }
            | Self::Unary { span, .. }
            | Self::LogicalNot { span, .. }
            | Self::Compare { span, .. }
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
                HirExpressionKind::ParameterRead(parameter) => {
                    instructions.push(MirInstruction::ParameterRead {
                        output,
                        parameter: MirParameterId::from_raw(parameter.index()),
                        span: expression.span(),
                    });
                }
                HirExpressionKind::Binary(binary) => {
                    let left = MirValueId::from_raw(binary.left().index());
                    let right = MirValueId::from_raw(binary.right().index());
                    if let Some(operation) = lower_comparison(binary.operator()) {
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
                    let operand = MirValueId::from_raw(unary.operand().index());
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
            function
                .parameters()
                .iter()
                .map(|parameter| (MirValueId::from_raw(parameter.id().index()), parameter.ty()))
                .collect(),
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
    next_block: usize,
    next_value: usize,
    lowered: Vec<HirExpressionId>,
}

impl<'a> ShortCircuitLowerer<'a> {
    fn new(function: &'a HirFunction, types: &'a TypeArena) -> Self {
        let next_value = function
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
        Self {
            function,
            types,
            blocks: vec![LowerBlock {
                id: MirBlockId::from_raw(0),
                instructions: Vec::new(),
                terminator: None,
            }],
            current: 0,
            locals: vec![MirLocal::new(
                MirLocalId::from_raw(0),
                bool_type,
                function.span(),
            )],
            next_block: 1,
            next_value,
            lowered: Vec::new(),
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
            .function
            .expressions()
            .iter()
            .find(|expression| expression.id() == id)
            .cloned()
            .ok_or(MirLoweringError::UnsupportedExpression)?;
        require_hir_expression_type(&expression, self.types)?;
        let output = MirValueId::from_raw(id.index());
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
                    local: MirLocalId::from_raw(0),
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
                    local: MirLocalId::from_raw(0),
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
                    local: MirLocalId::from_raw(0),
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
            _ => return Err(MirLoweringError::UnsupportedExpression),
        }
        self.lowered.push(id);
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
        Ok(MirFunction::new(
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
            MirCleanupBoundary::empty(),
        )
        .with_entry(self.function.is_entry()))
    }
}

fn lower_hir_function_with_short_circuit(
    function: &HirFunction,
    types: &TypeArena,
) -> Result<MirFunction, MirLoweringError> {
    ShortCircuitLowerer::new(function, types).finish()
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
