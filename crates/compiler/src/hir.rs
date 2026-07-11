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
}
