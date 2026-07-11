use crate::{
    module::{ModuleName, PackageNamespace},
    source::ByteSpan,
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
    Plus,
    Minus,
    BitwiseNot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HirBinaryOperator {
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
    pub fn local_read(id: HirExpressionId, span: ByteSpan, ty: TypeId, local: HirLocalId) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::LocalRead(local),
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
