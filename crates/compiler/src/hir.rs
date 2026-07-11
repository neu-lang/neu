use crate::{
    module::{FunctionSymbolIdentity, ModuleName, PackageNamespace},
    parser::{ParseOutput, ParsedBinaryOperator, ParsedLiteralKind},
    source::ByteSpan,
    type_check::{ExpressionType, FunctionSignature},
    types::TypeId,
};

macro_rules! hir_id {
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

hir_id!(HirFunctionId);
hir_id!(HirParameterId);
hir_id!(HirLocalId);
hir_id!(HirExpressionId);

pub struct CheckedHirSource<'a> {
    module: ModuleName,
    package: PackageNamespace,
    parsed: &'a ParseOutput,
    signatures: &'a [FunctionSignature],
    expression_types: &'a [ExpressionType],
    clean: bool,
    byte_type: Option<TypeId>,
}
impl<'a> CheckedHirSource<'a> {
    pub fn new(
        module: ModuleName,
        package: PackageNamespace,
        parsed: &'a ParseOutput,
        signatures: &'a [FunctionSignature],
        expression_types: &'a [ExpressionType],
        clean: bool,
    ) -> Self {
        Self {
            module,
            package,
            parsed,
            signatures,
            expression_types,
            clean,
            byte_type: None,
        }
    }

    pub fn with_byte_type(mut self, byte_type: TypeId) -> Self {
        self.byte_type = Some(byte_type);
        self
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HirLoweringError {
    FrontendNotClean,
    MissingType,
    UnsupportedExpression,
}

pub fn lower_checked_hir_source(
    source: CheckedHirSource<'_>,
) -> Result<HirModule, HirLoweringError> {
    if !source.clean {
        return Err(HirLoweringError::FrontendNotClean);
    }
    let mut functions = Vec::new();
    for function in &source.parsed.function_declarations {
        let Some(signature) = source
            .signatures
            .iter()
            .find(|signature| signature.declaration() == function.declaration)
        else {
            continue;
        };
        let span = source
            .parsed
            .arena
            .node(function.declaration)
            .ok_or(HirLoweringError::UnsupportedExpression)?
            .span;
        let symbol_identity = source
            .parsed
            .declaration_names
            .iter()
            .find(|name| name.declaration == function.declaration)
            .map(|name| {
                FunctionSymbolIdentity::new(
                    source.module.clone(),
                    source.package.clone(),
                    name.name.clone(),
                )
            })
            .ok_or(HirLoweringError::UnsupportedExpression)?;
        let id = HirFunctionId::from_raw(functions.len());
        let mut expressions = Vec::new();
        let mut returns = Vec::new();
        let parameters = source
            .parsed
            .function_parameters
            .iter()
            .filter(|parameter| parameter.function == function.declaration)
            .enumerate()
            .map(|(index, parameter)| {
                let parameter_span = source
                    .parsed
                    .arena
                    .node(parameter.parameter)
                    .ok_or(HirLoweringError::UnsupportedExpression)?
                    .span;
                let ty = *signature
                    .parameter_types()
                    .get(index)
                    .ok_or(HirLoweringError::UnsupportedExpression)?;
                Ok(HirParameter::new(
                    HirParameterId::from_raw(index),
                    parameter_span,
                    ty,
                ))
            })
            .collect::<Result<Vec<_>, HirLoweringError>>()?;
        for returned in source
            .parsed
            .return_statements
            .iter()
            .filter(|returned| returned.function == function.declaration)
        {
            let value = returned
                .value
                .ok_or(HirLoweringError::UnsupportedExpression)?;
            let expression =
                lower_expression(&source, function.declaration, value, &mut expressions)?;
            let return_span = source
                .parsed
                .arena
                .node(returned.statement)
                .ok_or(HirLoweringError::UnsupportedExpression)?
                .span;
            returns.push(HirReturn::new(return_span, expression));
        }
        functions.push(
            HirFunction::new(
                id,
                source.module.clone(),
                source.package.clone(),
                span,
                declaration_is_main(source.parsed, function.declaration),
                signature.return_type(),
                parameters,
                vec![],
                expressions,
                returns,
                HirSafetyFacts::executable_subset_checked(),
                vec![],
            )
            .with_symbol_identity(symbol_identity),
        );
    }
    Ok(HirModule::new(source.module, functions))
}

fn lower_expression(
    source: &CheckedHirSource<'_>,
    function_declaration: crate::ast::AstNodeId,
    expression: crate::ast::AstNodeId,
    output: &mut Vec<HirExpression>,
) -> Result<HirExpressionId, HirLoweringError> {
    let ty = source
        .expression_types
        .iter()
        .find(|typed| typed.expression() == expression)
        .map(|typed| typed.ty())
        .ok_or(HirLoweringError::MissingType)?;
    let span = source
        .parsed
        .arena
        .node(expression)
        .ok_or(HirLoweringError::UnsupportedExpression)?
        .span;
    let id = HirExpressionId::from_raw(output.len());
    if let Some(literal) = source
        .parsed
        .literal_expressions
        .iter()
        .find(|literal| literal.expression == expression)
    {
        match literal.kind {
            ParsedLiteralKind::BoolTrue | ParsedLiteralKind::BoolFalse => {
                output.push(HirExpression::bool_literal(
                    id,
                    span,
                    ty,
                    literal.kind == ParsedLiteralKind::BoolTrue,
                ));
            }
            ParsedLiteralKind::Unit => output.push(HirExpression::unit_literal(id, span, ty)),
            ParsedLiteralKind::Float => {
                let bits = source
                    .parsed
                    .float_literals
                    .iter()
                    .find(|float| float.expression == expression)
                    .and_then(|float| float.bits)
                    .ok_or(HirLoweringError::UnsupportedExpression)?;
                output.push(HirExpression::float_literal(
                    id,
                    span,
                    ty,
                    f64::from_bits(bits),
                ));
            }
            ParsedLiteralKind::AcceptedInteger => {
                let value = source
                    .parsed
                    .integer_literals
                    .iter()
                    .find(|integer| integer.expression == expression)
                    .and_then(|integer| integer.value)
                    .and_then(|value| i64::try_from(value).ok())
                    .ok_or(HirLoweringError::UnsupportedExpression)?;
                if source.byte_type == Some(ty) {
                    let value =
                        u8::try_from(value).map_err(|_| HirLoweringError::UnsupportedExpression)?;
                    output.push(HirExpression::byte_literal(id, span, ty, value));
                } else {
                    output.push(HirExpression::int_literal(id, span, ty, value));
                }
            }
            ParsedLiteralKind::AcceptedString | ParsedLiteralKind::Null => {
                return Err(HirLoweringError::UnsupportedExpression);
            }
        }
        return Ok(id);
    }
    if let Some(binary) = source
        .parsed
        .binary_expressions
        .iter()
        .find(|binary| binary.expression == expression)
    {
        let left = lower_expression(source, function_declaration, binary.left, output)?;
        let right = lower_expression(source, function_declaration, binary.right, output)?;
        output.push(HirExpression::binary(
            id,
            span,
            ty,
            lower_binary_operator(binary.operator)?,
            left,
            right,
        ));
        return Ok(id);
    }
    if let Some(unary) = source
        .parsed
        .unary_expressions
        .iter()
        .find(|unary| unary.expression == expression)
    {
        let operand = lower_expression(source, function_declaration, unary.operand, output)?;
        output.push(HirExpression::unary(
            id,
            span,
            ty,
            lower_unary_operator(unary.operator)?,
            operand,
        ));
        return Ok(id);
    }
    if let Some(name) = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == expression)
        && let Some(parameter_index) = source
            .parsed
            .function_parameters
            .iter()
            .filter(|parameter| parameter.function == function_declaration)
            .position(|parameter| parameter.name == name.name)
    {
        output.push(HirExpression::parameter_read(
            id,
            span,
            ty,
            HirParameterId::from_raw(parameter_index),
        ));
        return Ok(id);
    }
    if let Some(call) = source
        .parsed
        .call_expressions
        .iter()
        .find(|call| call.expression == expression)
    {
        let name = source
            .parsed
            .name_references
            .iter()
            .find(|name| name.reference == call.callee)
            .ok_or(HirLoweringError::UnsupportedExpression)?;
        let declaration = source
            .parsed
            .declaration_names
            .iter()
            .position(|declaration| declaration.name == name.name)
            .ok_or(HirLoweringError::UnsupportedExpression)?;
        let arguments = call
            .arguments
            .iter()
            .map(|argument| lower_expression(source, function_declaration, *argument, output))
            .collect::<Result<Vec<_>, _>>()?;
        output.push(HirExpression::direct_call(
            id,
            span,
            ty,
            HirDirectCall::new(HirFunctionId::from_raw(declaration), arguments),
        ));
        return Ok(id);
    }
    Err(HirLoweringError::UnsupportedExpression)
}

fn declaration_is_main(parsed: &ParseOutput, declaration: crate::ast::AstNodeId) -> bool {
    parsed
        .declaration_names
        .iter()
        .any(|name| name.declaration == declaration && name.name == "main")
}
fn lower_binary_operator(
    operator: ParsedBinaryOperator,
) -> Result<HirBinaryOperator, HirLoweringError> {
    Ok(match operator {
        ParsedBinaryOperator::LogicalOr => HirBinaryOperator::LogicalOr,
        ParsedBinaryOperator::LogicalAnd => HirBinaryOperator::LogicalAnd,
        ParsedBinaryOperator::Equal => HirBinaryOperator::Equal,
        ParsedBinaryOperator::NotEqual => HirBinaryOperator::NotEqual,
        ParsedBinaryOperator::Less => HirBinaryOperator::Less,
        ParsedBinaryOperator::Greater => HirBinaryOperator::Greater,
        ParsedBinaryOperator::LessEqual => HirBinaryOperator::LessEqual,
        ParsedBinaryOperator::GreaterEqual => HirBinaryOperator::GreaterEqual,
        ParsedBinaryOperator::Plus => HirBinaryOperator::Plus,
        ParsedBinaryOperator::Minus => HirBinaryOperator::Minus,
        ParsedBinaryOperator::Star => HirBinaryOperator::Multiply,
        ParsedBinaryOperator::Slash => HirBinaryOperator::Divide,
        ParsedBinaryOperator::Percent => HirBinaryOperator::Remainder,
        ParsedBinaryOperator::Exponent => HirBinaryOperator::Exponent,
        ParsedBinaryOperator::BitwiseAnd => HirBinaryOperator::BitwiseAnd,
        ParsedBinaryOperator::BitwiseOr => HirBinaryOperator::BitwiseOr,
        ParsedBinaryOperator::BitwiseXor => HirBinaryOperator::BitwiseXor,
        ParsedBinaryOperator::ShiftLeft => HirBinaryOperator::ShiftLeft,
        ParsedBinaryOperator::ShiftRight => HirBinaryOperator::ShiftRight,
    })
}

fn lower_unary_operator(
    operator: crate::parser::ParsedUnaryOperator,
) -> Result<HirUnaryOperator, HirLoweringError> {
    Ok(match operator {
        crate::parser::ParsedUnaryOperator::Not => HirUnaryOperator::Not,
        crate::parser::ParsedUnaryOperator::Plus => HirUnaryOperator::Plus,
        crate::parser::ParsedUnaryOperator::Minus => HirUnaryOperator::Minus,
        crate::parser::ParsedUnaryOperator::BitwiseNot => HirUnaryOperator::BitwiseNot,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirModule {
    name: ModuleName,
    functions: Vec<HirFunction>,
}

impl HirModule {
    pub fn new(name: ModuleName, functions: Vec<HirFunction>) -> Self {
        Self { name, functions }
    }
    pub fn name(&self) -> &ModuleName {
        &self.name
    }
    pub fn functions(&self) -> &[HirFunction] {
        &self.functions
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirParameter {
    id: HirParameterId,
    span: ByteSpan,
    ty: TypeId,
}
impl HirParameter {
    pub fn new(id: HirParameterId, span: ByteSpan, ty: TypeId) -> Self {
        Self { id, span, ty }
    }
    pub fn id(&self) -> HirParameterId {
        self.id
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn ty(&self) -> TypeId {
        self.ty
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirLocal {
    id: HirLocalId,
    span: ByteSpan,
    ty: TypeId,
    mutable: bool,
}
impl HirLocal {
    pub fn new(id: HirLocalId, span: ByteSpan, ty: TypeId, mutable: bool) -> Self {
        Self {
            id,
            span,
            ty,
            mutable,
        }
    }
    pub fn id(&self) -> HirLocalId {
        self.id
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn ty(&self) -> TypeId {
        self.ty
    }
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirDirectCall {
    callee: HirFunctionId,
    arguments: Vec<HirExpressionId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HirUnaryOperator {
    Not,
    Plus,
    Minus,
    BitwiseNot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HirBinaryOperator {
    LogicalOr,
    LogicalAnd,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Plus,
    Minus,
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
pub struct HirUnary {
    operator: HirUnaryOperator,
    operand: HirExpressionId,
}
impl HirUnary {
    pub fn new(operator: HirUnaryOperator, operand: HirExpressionId) -> Self {
        Self { operator, operand }
    }
    pub fn operator(&self) -> HirUnaryOperator {
        self.operator
    }
    pub fn operand(&self) -> HirExpressionId {
        self.operand
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirBinary {
    operator: HirBinaryOperator,
    left: HirExpressionId,
    right: HirExpressionId,
}
impl HirBinary {
    pub fn new(operator: HirBinaryOperator, left: HirExpressionId, right: HirExpressionId) -> Self {
        Self {
            operator,
            left,
            right,
        }
    }
    pub fn operator(&self) -> HirBinaryOperator {
        self.operator
    }
    pub fn left(&self) -> HirExpressionId {
        self.left
    }
    pub fn right(&self) -> HirExpressionId {
        self.right
    }
}
impl HirDirectCall {
    pub fn new(callee: HirFunctionId, arguments: Vec<HirExpressionId>) -> Self {
        Self { callee, arguments }
    }
    pub fn callee(&self) -> HirFunctionId {
        self.callee
    }
    pub fn arguments(&self) -> &[HirExpressionId] {
        &self.arguments
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HirExpressionKind {
    IntLiteral(i64),
    BoolLiteral(bool),
    UnitLiteral,
    FloatLiteral(u64),
    ByteLiteral(u8),
    ParameterRead(HirParameterId),
    LocalRead(HirLocalId),
    Unary(HirUnary),
    Binary(HirBinary),
    DirectCall(HirDirectCall),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirExpression {
    id: HirExpressionId,
    span: ByteSpan,
    ty: TypeId,
    kind: HirExpressionKind,
}
impl HirExpression {
    pub fn int_literal(id: HirExpressionId, span: ByteSpan, ty: TypeId, value: i64) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::IntLiteral(value),
        }
    }

    pub fn bool_literal(id: HirExpressionId, span: ByteSpan, ty: TypeId, value: bool) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::BoolLiteral(value),
        }
    }

    pub fn unit_literal(id: HirExpressionId, span: ByteSpan, ty: TypeId) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::UnitLiteral,
        }
    }

    pub fn float_literal(id: HirExpressionId, span: ByteSpan, ty: TypeId, value: f64) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::FloatLiteral(value.to_bits()),
        }
    }

    pub fn byte_literal(id: HirExpressionId, span: ByteSpan, ty: TypeId, value: u8) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::ByteLiteral(value),
        }
    }
    pub fn local_read(id: HirExpressionId, span: ByteSpan, ty: TypeId, local: HirLocalId) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::LocalRead(local),
        }
    }
    pub fn parameter_read(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        parameter: HirParameterId,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::ParameterRead(parameter),
        }
    }
    pub fn unary(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        operator: HirUnaryOperator,
        operand: HirExpressionId,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::Unary(HirUnary::new(operator, operand)),
        }
    }
    pub fn binary(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        operator: HirBinaryOperator,
        left: HirExpressionId,
        right: HirExpressionId,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::Binary(HirBinary::new(operator, left, right)),
        }
    }
    pub fn direct_call(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        call: HirDirectCall,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::DirectCall(call),
        }
    }
    pub fn id(&self) -> HirExpressionId {
        self.id
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn ty(&self) -> TypeId {
        self.ty
    }
    pub fn kind(&self) -> &HirExpressionKind {
        &self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirReturn {
    span: ByteSpan,
    expression: HirExpressionId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirAssignment {
    span: ByteSpan,
    target: HirLocalId,
    value: HirExpressionId,
}
impl HirAssignment {
    pub fn new(span: ByteSpan, target: HirLocalId, value: HirExpressionId) -> Self {
        Self {
            span,
            target,
            value,
        }
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn target(&self) -> HirLocalId {
        self.target
    }
    pub fn value(&self) -> HirExpressionId {
        self.value
    }
}
impl HirReturn {
    pub fn new(span: ByteSpan, expression: HirExpressionId) -> Self {
        Self { span, expression }
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn expression(&self) -> HirExpressionId {
        self.expression
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirSafetyFacts {
    executable_subset_checked: bool,
}
impl HirSafetyFacts {
    pub fn executable_subset_checked() -> Self {
        Self {
            executable_subset_checked: true,
        }
    }
    pub fn is_executable_subset_checked(&self) -> bool {
        self.executable_subset_checked
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirUnsupportedForm {
    span: ByteSpan,
}
impl HirUnsupportedForm {
    pub fn new(span: ByteSpan) -> Self {
        Self { span }
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirFunction {
    id: HirFunctionId,
    module: ModuleName,
    package: PackageNamespace,
    span: ByteSpan,
    entry: bool,
    return_type: TypeId,
    parameters: Vec<HirParameter>,
    locals: Vec<HirLocal>,
    expressions: Vec<HirExpression>,
    returns: Vec<HirReturn>,
    assignments: Vec<HirAssignment>,
    safety_facts: HirSafetyFacts,
    unsupported_forms: Vec<HirUnsupportedForm>,
    symbol_identity: Option<FunctionSymbolIdentity>,
}
impl HirFunction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: HirFunctionId,
        module: ModuleName,
        package: PackageNamespace,
        span: ByteSpan,
        entry: bool,
        return_type: TypeId,
        parameters: Vec<HirParameter>,
        locals: Vec<HirLocal>,
        expressions: Vec<HirExpression>,
        returns: Vec<HirReturn>,
        safety_facts: HirSafetyFacts,
        unsupported_forms: Vec<HirUnsupportedForm>,
    ) -> Self {
        Self {
            id,
            module,
            package,
            span,
            entry,
            return_type,
            parameters,
            locals,
            expressions,
            returns,
            assignments: Vec::new(),
            safety_facts,
            unsupported_forms,
            symbol_identity: None,
        }
    }
    pub fn id(&self) -> HirFunctionId {
        self.id
    }
    pub fn module(&self) -> &ModuleName {
        &self.module
    }
    pub fn package(&self) -> &PackageNamespace {
        &self.package
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn is_entry(&self) -> bool {
        self.entry
    }
    pub fn return_type(&self) -> TypeId {
        self.return_type
    }
    pub fn parameters(&self) -> &[HirParameter] {
        &self.parameters
    }
    pub fn locals(&self) -> &[HirLocal] {
        &self.locals
    }
    pub fn expressions(&self) -> &[HirExpression] {
        &self.expressions
    }
    pub fn returns(&self) -> &[HirReturn] {
        &self.returns
    }
    pub fn with_assignments(mut self, assignments: Vec<HirAssignment>) -> Self {
        self.assignments = assignments;
        self
    }
    pub fn assignments(&self) -> &[HirAssignment] {
        &self.assignments
    }
    pub fn safety_facts(&self) -> &HirSafetyFacts {
        &self.safety_facts
    }
    pub fn unsupported_forms(&self) -> &[HirUnsupportedForm] {
        &self.unsupported_forms
    }
    pub fn with_symbol_identity(mut self, identity: FunctionSymbolIdentity) -> Self {
        self.symbol_identity = Some(identity);
        self
    }
    pub fn symbol_identity(&self) -> Option<&FunctionSymbolIdentity> {
        self.symbol_identity.as_ref()
    }
    pub fn direct_call(&self, id: HirExpressionId) -> Option<&HirDirectCall> {
        self.expressions
            .iter()
            .find(|expression| expression.id == id)
            .and_then(|expression| match &expression.kind {
                HirExpressionKind::DirectCall(call) => Some(call),
                _ => None,
            })
    }
    pub fn local_read(&self, id: HirExpressionId) -> Option<HirLocalId> {
        self.expressions
            .iter()
            .find(|expression| expression.id == id)
            .and_then(|expression| match expression.kind {
                HirExpressionKind::LocalRead(local) => Some(local),
                _ => None,
            })
    }
    pub fn parameter_read(&self, id: HirExpressionId) -> Option<HirParameterId> {
        self.expressions
            .iter()
            .find(|expression| expression.id == id)
            .and_then(|expression| match expression.kind {
                HirExpressionKind::ParameterRead(parameter) => Some(parameter),
                _ => None,
            })
    }
    pub fn unary(&self, id: HirExpressionId) -> Option<&HirUnary> {
        self.expressions
            .iter()
            .find(|expression| expression.id == id)
            .and_then(|expression| match &expression.kind {
                HirExpressionKind::Unary(unary) => Some(unary),
                _ => None,
            })
    }
    pub fn binary(&self, id: HirExpressionId) -> Option<&HirBinary> {
        self.expressions
            .iter()
            .find(|expression| expression.id == id)
            .and_then(|expression| match &expression.kind {
                HirExpressionKind::Binary(binary) => Some(binary),
                _ => None,
            })
    }
}
