use crate::ast::AstNodeId;

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
