use crate::{ast::AstNodeId, name_resolution::LocalBinding};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BorrowKind {
    Shared,
    Exclusive,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BorrowRecord {
    node: AstNodeId,
    binding: LocalBinding,
    kind: BorrowKind,
    region: AstNodeId,
}

impl BorrowRecord {
    pub fn new(
        node: AstNodeId,
        binding: LocalBinding,
        kind: BorrowKind,
        region: AstNodeId,
    ) -> Self {
        Self {
            node,
            binding,
            kind,
            region,
        }
    }

    pub fn node(&self) -> AstNodeId {
        self.node
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }

    pub fn kind(&self) -> BorrowKind {
        self.kind
    }

    pub fn region(&self) -> AstNodeId {
        self.region
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BorrowDiagnosticKind {
    BorrowConflict,
    LifetimeEscape,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BorrowDiagnostic {
    kind: BorrowDiagnosticKind,
    node: AstNodeId,
    conflict_origin: AstNodeId,
}

impl BorrowDiagnostic {
    pub fn borrow_conflict(node: AstNodeId, conflict_origin: AstNodeId) -> Self {
        Self {
            kind: BorrowDiagnosticKind::BorrowConflict,
            node,
            conflict_origin,
        }
    }

    pub fn lifetime_escape(node: AstNodeId, conflict_origin: AstNodeId) -> Self {
        Self {
            kind: BorrowDiagnosticKind::LifetimeEscape,
            node,
            conflict_origin,
        }
    }

    pub fn kind(&self) -> BorrowDiagnosticKind {
        self.kind
    }

    pub fn node(&self) -> AstNodeId {
        self.node
    }

    pub fn conflict_origin(&self) -> AstNodeId {
        self.conflict_origin
    }
}

pub fn analyze_borrow_conflicts(borrows: &[BorrowRecord]) -> Vec<BorrowDiagnostic> {
    let mut diagnostics = Vec::new();

    for (index, borrow) in borrows.iter().enumerate() {
        let Some(conflict) = borrows[..index]
            .iter()
            .rev()
            .find(|previous| borrows_conflict(previous, borrow))
        else {
            continue;
        };
        diagnostics.push(BorrowDiagnostic::borrow_conflict(
            borrow.node(),
            conflict.node(),
        ));
    }

    diagnostics
}

fn borrows_conflict(previous: &BorrowRecord, later: &BorrowRecord) -> bool {
    previous.binding() == later.binding()
        && previous.region() == later.region()
        && (previous.kind() == BorrowKind::Exclusive || later.kind() == BorrowKind::Exclusive)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifetimeEscapeRecord {
    escape_node: AstNodeId,
    binding: LocalBinding,
    borrow_node: AstNodeId,
    borrow_region: AstNodeId,
    use_region: AstNodeId,
}

impl LifetimeEscapeRecord {
    pub fn new(
        escape_node: AstNodeId,
        binding: LocalBinding,
        borrow_node: AstNodeId,
        borrow_region: AstNodeId,
        use_region: AstNodeId,
    ) -> Self {
        Self {
            escape_node,
            binding,
            borrow_node,
            borrow_region,
            use_region,
        }
    }

    pub fn escape_node(&self) -> AstNodeId {
        self.escape_node
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }

    pub fn borrow_node(&self) -> AstNodeId {
        self.borrow_node
    }

    pub fn borrow_region(&self) -> AstNodeId {
        self.borrow_region
    }

    pub fn use_region(&self) -> AstNodeId {
        self.use_region
    }
}

pub fn analyze_lifetime_escapes(records: &[LifetimeEscapeRecord]) -> Vec<BorrowDiagnostic> {
    let mut diagnostics = Vec::new();

    for record in records {
        if record.borrow_region() == record.use_region() {
            continue;
        }
        diagnostics.push(BorrowDiagnostic::lifetime_escape(
            record.escape_node(),
            record.borrow_node(),
        ));
    }

    diagnostics
}
