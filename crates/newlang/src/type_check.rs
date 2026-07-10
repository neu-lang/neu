use crate::ast::AstNodeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AmbiguousTypeRule {
    LiteralTyping,
    PrimitiveScalarCatalog,
    AssignmentCompatibility,
    CallResolution,
    FunctionTypeApplication,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeCheckDiagnosticKind {
    AmbiguousTypeRule,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeCheckDiagnostic {
    kind: TypeCheckDiagnosticKind,
    rule: AmbiguousTypeRule,
    node: AstNodeId,
}

impl TypeCheckDiagnostic {
    pub fn ambiguous_type_rule(rule: AmbiguousTypeRule, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::AmbiguousTypeRule,
            rule,
            node,
        }
    }

    pub fn kind(&self) -> TypeCheckDiagnosticKind {
        self.kind
    }

    pub fn rule(&self) -> AmbiguousTypeRule {
        self.rule
    }

    pub fn node(&self) -> AstNodeId {
        self.node
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeCheckReport {
    diagnostics: Vec<TypeCheckDiagnostic>,
}

impl TypeCheckReport {
    pub fn blocked(diagnostics: Vec<TypeCheckDiagnostic>) -> Self {
        Self { diagnostics }
    }

    pub fn is_blocked(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    pub fn diagnostics(&self) -> &[TypeCheckDiagnostic] {
        &self.diagnostics
    }
}
