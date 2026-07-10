use std::collections::HashMap;

use crate::ast::AstNodeId;
use crate::source::ByteSpan;
use crate::symbol::SymbolId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedName {
    reference: AstNodeId,
    symbol: SymbolId,
}

impl ResolvedName {
    pub fn new(reference: AstNodeId, symbol: SymbolId) -> Self {
        Self { reference, symbol }
    }

    pub fn reference(self) -> AstNodeId {
        self.reference
    }

    pub fn symbol(self) -> SymbolId {
        self.symbol
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolutionInsert {
    Inserted(ResolvedName),
    Duplicate {
        existing: ResolvedName,
        attempted: ResolvedName,
    },
}

#[derive(Debug, Default)]
pub struct ResolutionTable {
    resolved_names: Vec<ResolvedName>,
    indices_by_reference: HashMap<AstNodeId, usize>,
}

impl ResolutionTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, resolved_name: ResolvedName) -> ResolutionInsert {
        if let Some(index) = self.indices_by_reference.get(&resolved_name.reference()) {
            return ResolutionInsert::Duplicate {
                existing: self.resolved_names[*index],
                attempted: resolved_name,
            };
        }

        let index = self.resolved_names.len();
        self.indices_by_reference
            .insert(resolved_name.reference(), index);
        self.resolved_names.push(resolved_name);
        ResolutionInsert::Inserted(resolved_name)
    }

    pub fn get(&self, reference: AstNodeId) -> Option<&ResolvedName> {
        self.indices_by_reference
            .get(&reference)
            .and_then(|index| self.resolved_names.get(*index))
    }

    pub fn resolved_names(&self) -> &[ResolvedName] {
        &self.resolved_names
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolutionDiagnosticKind {
    UnresolvedName,
    DuplicateName,
    AmbiguousName,
    UnsupportedImportResolution,
    UnsupportedCrossModuleLookup,
    UnsupportedMemberResolution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolutionDiagnostic {
    kind: ResolutionDiagnosticKind,
    primary_span: ByteSpan,
}

impl ResolutionDiagnostic {
    pub fn new(kind: ResolutionDiagnosticKind, primary_span: ByteSpan) -> Self {
        Self { kind, primary_span }
    }

    pub fn kind(self) -> ResolutionDiagnosticKind {
        self.kind
    }

    pub fn primary_span(self) -> ByteSpan {
        self.primary_span
    }
}
