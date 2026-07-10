use crate::ast::AstNodeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SafetyBasis {
    ProvenSafe,
    TrustedUnsafe,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsafeContextKind {
    Block,
    Function,
    ModuleAudit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsafeContext {
    node: AstNodeId,
    kind: UnsafeContextKind,
}

impl UnsafeContext {
    pub fn new(node: AstNodeId, kind: UnsafeContextKind) -> Self {
        Self { node, kind }
    }

    pub fn node(&self) -> AstNodeId {
        self.node
    }

    pub fn kind(&self) -> UnsafeContextKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsafeOperationKind {
    RawPointerDereference,
    ForeignCall,
    UnsafeCapabilityAssertion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsafeOperation {
    node: AstNodeId,
    kind: UnsafeOperationKind,
    safety_basis: SafetyBasis,
    context: Option<AstNodeId>,
}

impl UnsafeOperation {
    pub fn new(
        node: AstNodeId,
        kind: UnsafeOperationKind,
        safety_basis: SafetyBasis,
        context: Option<AstNodeId>,
    ) -> Self {
        Self {
            node,
            kind,
            safety_basis,
            context,
        }
    }

    pub fn node(&self) -> AstNodeId {
        self.node
    }

    pub fn kind(&self) -> UnsafeOperationKind {
        self.kind
    }

    pub fn safety_basis(&self) -> SafetyBasis {
        self.safety_basis
    }

    pub fn context(&self) -> Option<AstNodeId> {
        self.context
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsafeDiagnosticKind {
    UnsafeOperationOutsideContext,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsafeDiagnostic {
    kind: UnsafeDiagnosticKind,
    operation: AstNodeId,
    operation_kind: UnsafeOperationKind,
    safety_basis: SafetyBasis,
    context: Option<AstNodeId>,
}

impl UnsafeDiagnostic {
    pub fn unsafe_operation_outside_context(
        operation: AstNodeId,
        operation_kind: UnsafeOperationKind,
        safety_basis: SafetyBasis,
        context: Option<AstNodeId>,
    ) -> Self {
        Self {
            kind: UnsafeDiagnosticKind::UnsafeOperationOutsideContext,
            operation,
            operation_kind,
            safety_basis,
            context,
        }
    }

    pub fn kind(&self) -> UnsafeDiagnosticKind {
        self.kind
    }

    pub fn operation(&self) -> AstNodeId {
        self.operation
    }

    pub fn operation_kind(&self) -> UnsafeOperationKind {
        self.operation_kind
    }

    pub fn safety_basis(&self) -> SafetyBasis {
        self.safety_basis
    }

    pub fn context(&self) -> Option<AstNodeId> {
        self.context
    }
}

pub fn analyze_unsafe_operations(
    contexts: &[UnsafeContext],
    operations: &[UnsafeOperation],
) -> Vec<UnsafeDiagnostic> {
    let mut diagnostics = Vec::new();

    for operation in operations {
        if operation.safety_basis() == SafetyBasis::ProvenSafe {
            continue;
        }
        if operation
            .context()
            .is_some_and(|context| contexts.iter().any(|candidate| candidate.node() == context))
        {
            continue;
        }
        diagnostics.push(UnsafeDiagnostic::unsafe_operation_outside_context(
            operation.node(),
            operation.kind(),
            operation.safety_basis(),
            operation.context(),
        ));
    }

    diagnostics
}
