use crate::{ast::AstNodeId, borrow::BorrowKind, name_resolution::LocalBinding};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChildTask {
    task: AstNodeId,
    containing_scope: AstNodeId,
    completion_scope: AstNodeId,
}

impl ChildTask {
    pub fn new(task: AstNodeId, containing_scope: AstNodeId, completion_scope: AstNodeId) -> Self {
        Self {
            task,
            containing_scope,
            completion_scope,
        }
    }

    pub fn task(&self) -> AstNodeId {
        self.task
    }

    pub fn containing_scope(&self) -> AstNodeId {
        self.containing_scope
    }

    pub fn completion_scope(&self) -> AstNodeId {
        self.completion_scope
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuredTaskScope {
    scope: AstNodeId,
    children: Vec<ChildTask>,
}

impl StructuredTaskScope {
    pub fn new(scope: AstNodeId, children: Vec<ChildTask>) -> Self {
        Self { scope, children }
    }

    pub fn scope(&self) -> AstNodeId {
        self.scope
    }

    pub fn children(&self) -> &[ChildTask] {
        &self.children
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoroutineDiagnosticKind {
    TaskScopeEscape,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoroutineDiagnostic {
    kind: CoroutineDiagnosticKind,
    task: AstNodeId,
    containing_scope: AstNodeId,
    completion_scope: AstNodeId,
}

impl CoroutineDiagnostic {
    pub fn task_scope_escape(
        task: AstNodeId,
        containing_scope: AstNodeId,
        completion_scope: AstNodeId,
    ) -> Self {
        Self {
            kind: CoroutineDiagnosticKind::TaskScopeEscape,
            task,
            containing_scope,
            completion_scope,
        }
    }

    pub fn kind(&self) -> CoroutineDiagnosticKind {
        self.kind
    }

    pub fn task(&self) -> AstNodeId {
        self.task
    }

    pub fn containing_scope(&self) -> AstNodeId {
        self.containing_scope
    }

    pub fn completion_scope(&self) -> AstNodeId {
        self.completion_scope
    }
}

pub fn analyze_structured_task_scopes(scopes: &[StructuredTaskScope]) -> Vec<CoroutineDiagnostic> {
    let mut diagnostics = Vec::new();

    for scope in scopes {
        for child in scope.children() {
            if child.completion_scope() != child.containing_scope()
                || child.containing_scope() != scope.scope()
            {
                diagnostics.push(CoroutineDiagnostic::task_scope_escape(
                    child.task(),
                    scope.scope(),
                    child.completion_scope(),
                ));
            }
        }
    }

    diagnostics
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuspendedBorrow {
    suspension: AstNodeId,
    binding: LocalBinding,
    borrow: AstNodeId,
    kind: BorrowKind,
    borrowed_value_scope: AstNodeId,
    frame_scope: AstNodeId,
    frame_concurrently_accessible: bool,
}

impl SuspendedBorrow {
    pub fn new(
        suspension: AstNodeId,
        binding: LocalBinding,
        borrow: AstNodeId,
        kind: BorrowKind,
        borrowed_value_scope: AstNodeId,
        frame_scope: AstNodeId,
        frame_concurrently_accessible: bool,
    ) -> Self {
        Self {
            suspension,
            binding,
            borrow,
            kind,
            borrowed_value_scope,
            frame_scope,
            frame_concurrently_accessible,
        }
    }

    pub fn suspension(&self) -> AstNodeId {
        self.suspension
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }

    pub fn borrow(&self) -> AstNodeId {
        self.borrow
    }

    pub fn kind(&self) -> BorrowKind {
        self.kind
    }

    pub fn borrowed_value_scope(&self) -> AstNodeId {
        self.borrowed_value_scope
    }

    pub fn frame_scope(&self) -> AstNodeId {
        self.frame_scope
    }

    pub fn frame_concurrently_accessible(&self) -> bool {
        self.frame_concurrently_accessible
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SuspensionRejection {
    concurrent_frame_access: bool,
    outlives_borrowed_value: bool,
}

impl SuspensionRejection {
    pub fn new(concurrent_frame_access: bool, outlives_borrowed_value: bool) -> Self {
        Self {
            concurrent_frame_access,
            outlives_borrowed_value,
        }
    }

    pub fn concurrent_frame_access() -> Self {
        Self::new(true, false)
    }

    pub fn outlives_borrowed_value() -> Self {
        Self::new(false, true)
    }

    pub fn both() -> Self {
        Self::new(true, true)
    }

    pub fn has_concurrent_frame_access(&self) -> bool {
        self.concurrent_frame_access
    }

    pub fn outlives_value(&self) -> bool {
        self.outlives_borrowed_value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SuspensionDiagnosticKind {
    BorrowAcrossSuspension,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuspensionDiagnostic {
    kind: SuspensionDiagnosticKind,
    suspension: AstNodeId,
    borrow: AstNodeId,
    binding: LocalBinding,
    borrow_kind: BorrowKind,
    borrowed_value_scope: AstNodeId,
    frame_scope: AstNodeId,
    rejection: SuspensionRejection,
}

impl SuspensionDiagnostic {
    pub fn borrow_across_suspension(
        suspension: AstNodeId,
        borrow: AstNodeId,
        binding: LocalBinding,
        borrow_kind: BorrowKind,
        borrowed_value_scope: AstNodeId,
        frame_scope: AstNodeId,
        rejection: SuspensionRejection,
    ) -> Self {
        Self {
            kind: SuspensionDiagnosticKind::BorrowAcrossSuspension,
            suspension,
            borrow,
            binding,
            borrow_kind,
            borrowed_value_scope,
            frame_scope,
            rejection,
        }
    }

    pub fn kind(&self) -> SuspensionDiagnosticKind {
        self.kind
    }

    pub fn suspension(&self) -> AstNodeId {
        self.suspension
    }

    pub fn borrow(&self) -> AstNodeId {
        self.borrow
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }

    pub fn borrow_kind(&self) -> BorrowKind {
        self.borrow_kind
    }

    pub fn borrowed_value_scope(&self) -> AstNodeId {
        self.borrowed_value_scope
    }

    pub fn frame_scope(&self) -> AstNodeId {
        self.frame_scope
    }

    pub fn rejection(&self) -> SuspensionRejection {
        self.rejection
    }
}

pub fn analyze_suspended_borrows(records: &[SuspendedBorrow]) -> Vec<SuspensionDiagnostic> {
    let mut diagnostics = Vec::new();

    for record in records {
        let rejection = SuspensionRejection::new(
            record.frame_concurrently_accessible(),
            record.frame_scope() != record.borrowed_value_scope(),
        );
        if !rejection.has_concurrent_frame_access() && !rejection.outlives_value() {
            continue;
        }
        diagnostics.push(SuspensionDiagnostic::borrow_across_suspension(
            record.suspension(),
            record.borrow(),
            record.binding().clone(),
            record.kind(),
            record.borrowed_value_scope(),
            record.frame_scope(),
            rejection,
        ));
    }

    diagnostics
}
