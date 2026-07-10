use std::collections::HashMap;

use crate::ast::{AstArena, AstNodeId, AstNodeKind};
use crate::module::{ModuleMetadata, ModuleName, PackageNamespace};
use crate::parser::{ParsedDeclarationName, ParsedLocalBindingName};
use crate::source::ByteSpan;
use crate::symbol::{SymbolId, SymbolInterner};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeclarationKind {
    Function,
    Type,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DeclarationKey {
    module: ModuleName,
    package: PackageNamespace,
    name: SymbolId,
    kind: DeclarationKind,
}

impl DeclarationKey {
    pub fn new(
        module: ModuleName,
        package: PackageNamespace,
        name: SymbolId,
        kind: DeclarationKind,
    ) -> Self {
        Self {
            module,
            package,
            name,
            kind,
        }
    }

    pub fn module(&self) -> &ModuleName {
        &self.module
    }

    pub fn package(&self) -> &PackageNamespace {
        &self.package
    }

    pub fn name(&self) -> SymbolId {
        self.name
    }

    pub fn kind(&self) -> DeclarationKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclaredName {
    key: DeclarationKey,
    declaration: AstNodeId,
}

impl DeclaredName {
    pub fn new(key: DeclarationKey, declaration: AstNodeId) -> Self {
        Self { key, declaration }
    }

    pub fn key(&self) -> &DeclarationKey {
        &self.key
    }

    pub fn declaration(&self) -> AstNodeId {
        self.declaration
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarationInsert {
    Inserted(DeclaredName),
    Duplicate {
        existing: DeclaredName,
        attempted: DeclaredName,
    },
}

#[derive(Debug, Default)]
pub struct DeclarationIndex {
    declarations: Vec<DeclaredName>,
    indices_by_key: HashMap<DeclarationKey, usize>,
}

impl DeclarationIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, declared_name: DeclaredName) -> DeclarationInsert {
        if let Some(index) = self.indices_by_key.get(declared_name.key()) {
            return DeclarationInsert::Duplicate {
                existing: self.declarations[*index].clone(),
                attempted: declared_name,
            };
        }

        let index = self.declarations.len();
        self.indices_by_key
            .insert(declared_name.key().clone(), index);
        self.declarations.push(declared_name.clone());
        DeclarationInsert::Inserted(declared_name)
    }

    pub fn get(&self, key: &DeclarationKey) -> Option<&DeclaredName> {
        self.indices_by_key
            .get(key)
            .and_then(|index| self.declarations.get(*index))
    }

    pub fn lookup_top_level(&self, lookup: TopLevelLookup) -> TopLevelLookupResult {
        let key = DeclarationKey::new(lookup.module, lookup.package, lookup.name, lookup.kind);
        match self.get(&key) {
            Some(declaration) => TopLevelLookupResult::Found(declaration.clone()),
            None => TopLevelLookupResult::Unresolved(ResolutionDiagnostic::new(
                ResolutionDiagnosticKind::UnresolvedName,
                lookup.primary_span,
            )),
        }
    }

    pub fn declarations(&self) -> &[DeclaredName] {
        &self.declarations
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopLevelLookup {
    module: ModuleName,
    package: PackageNamespace,
    name: SymbolId,
    kind: DeclarationKind,
    primary_span: ByteSpan,
}

impl TopLevelLookup {
    pub fn new(
        module: ModuleName,
        package: PackageNamespace,
        name: SymbolId,
        kind: DeclarationKind,
        primary_span: ByteSpan,
    ) -> Self {
        Self {
            module,
            package,
            name,
            kind,
            primary_span,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TopLevelLookupResult {
    Found(DeclaredName),
    Unresolved(ResolutionDiagnostic),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocalScopeId(usize);

impl LocalScopeId {
    pub fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalScope {
    id: LocalScopeId,
    owner: AstNodeId,
    parent: Option<LocalScopeId>,
}

impl LocalScope {
    pub fn new(id: LocalScopeId, owner: AstNodeId, parent: Option<LocalScopeId>) -> Self {
        Self { id, owner, parent }
    }

    pub fn id(self) -> LocalScopeId {
        self.id
    }

    pub fn owner(self) -> AstNodeId {
        self.owner
    }

    pub fn parent(self) -> Option<LocalScopeId> {
        self.parent
    }
}

#[derive(Debug, Default)]
pub struct LocalScopeTree {
    scopes: Vec<LocalScope>,
}

impl LocalScopeTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_scope(&mut self, owner: AstNodeId, parent: Option<LocalScopeId>) -> LocalScopeId {
        let id = LocalScopeId::from_raw(self.scopes.len());
        self.scopes.push(LocalScope::new(id, owner, parent));
        id
    }

    pub fn get(&self, id: LocalScopeId) -> Option<&LocalScope> {
        self.scopes.get(id.index())
    }

    pub fn scopes(&self) -> &[LocalScope] {
        &self.scopes
    }
}

pub fn build_local_scope_tree(arena: &AstArena) -> LocalScopeTree {
    let mut scope_owners: Vec<_> = arena
        .nodes()
        .iter()
        .filter(|node| matches!(node.kind, AstNodeKind::Block | AstNodeKind::DeclarationBody))
        .collect();
    scope_owners.sort_by_key(|node| (node.span.start(), std::cmp::Reverse(node.span.end())));

    let mut tree = LocalScopeTree::new();
    let mut owner_scopes: Vec<(&crate::ast::AstNode, LocalScopeId)> = Vec::new();

    for owner in scope_owners {
        let parent = if owner.kind == AstNodeKind::Block {
            owner_scopes
                .iter()
                .rev()
                .find(|(candidate, _)| {
                    candidate.kind == AstNodeKind::Block
                        && candidate.span.start() < owner.span.start()
                        && owner.span.end() < candidate.span.end()
                })
                .map(|(_, scope)| *scope)
        } else {
            None
        };
        let scope = tree.add_scope(owner.id, parent);
        owner_scopes.push((owner, scope));
    }

    tree
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalBindingKind {
    Val,
    Var,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LocalBindingKey {
    scope: LocalScopeId,
    name: SymbolId,
}

impl LocalBindingKey {
    pub fn new(scope: LocalScopeId, name: SymbolId) -> Self {
        Self { scope, name }
    }

    pub fn scope(self) -> LocalScopeId {
        self.scope
    }

    pub fn name(self) -> SymbolId {
        self.name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalBinding {
    key: LocalBindingKey,
    binding: AstNodeId,
    kind: LocalBindingKind,
}

impl LocalBinding {
    pub fn new(key: LocalBindingKey, binding: AstNodeId, kind: LocalBindingKind) -> Self {
        Self { key, binding, kind }
    }

    pub fn key(&self) -> &LocalBindingKey {
        &self.key
    }

    pub fn binding(&self) -> AstNodeId {
        self.binding
    }

    pub fn kind(&self) -> LocalBindingKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalBindingInsert {
    Inserted(LocalBinding),
    Duplicate {
        existing: LocalBinding,
        attempted: LocalBinding,
    },
}

#[derive(Debug, Default)]
pub struct LocalBindingIndex {
    bindings: Vec<LocalBinding>,
    indices_by_key: HashMap<LocalBindingKey, usize>,
}

impl LocalBindingIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, binding: LocalBinding) -> LocalBindingInsert {
        if let Some(index) = self.indices_by_key.get(binding.key()) {
            return LocalBindingInsert::Duplicate {
                existing: self.bindings[*index].clone(),
                attempted: binding,
            };
        }

        let index = self.bindings.len();
        self.indices_by_key.insert(*binding.key(), index);
        self.bindings.push(binding.clone());
        LocalBindingInsert::Inserted(binding)
    }

    pub fn get(&self, key: &LocalBindingKey) -> Option<&LocalBinding> {
        self.indices_by_key
            .get(key)
            .and_then(|index| self.bindings.get(*index))
    }

    pub fn bindings(&self) -> &[LocalBinding] {
        &self.bindings
    }
}

#[derive(Debug)]
pub struct LocalBindingIndexBuild {
    index: LocalBindingIndex,
    inserts: Vec<LocalBindingInsert>,
    diagnostics: Vec<ResolutionDiagnostic>,
}

impl LocalBindingIndexBuild {
    pub fn index(&self) -> &LocalBindingIndex {
        &self.index
    }

    pub fn inserts(&self) -> &[LocalBindingInsert] {
        &self.inserts
    }

    pub fn diagnostics(&self) -> &[ResolutionDiagnostic] {
        &self.diagnostics
    }
}

pub fn build_local_binding_index(
    bindings: &[ParsedLocalBindingName],
    scope: LocalScopeId,
    interner: &mut SymbolInterner,
) -> LocalBindingIndexBuild {
    let mut index = LocalBindingIndex::new();
    let mut inserts = Vec::new();
    let mut diagnostics = Vec::new();

    for binding in bindings {
        let name = interner.intern(&binding.name);
        let key = LocalBindingKey::new(scope, name);
        let insert = index.insert(LocalBinding::new(key, binding.binding, binding.kind));
        if matches!(insert, LocalBindingInsert::Duplicate { .. }) {
            diagnostics.push(ResolutionDiagnostic::new(
                ResolutionDiagnosticKind::DuplicateName,
                binding.name_span,
            ));
        }
        inserts.push(insert);
    }

    LocalBindingIndexBuild {
        index,
        inserts,
        diagnostics,
    }
}

pub fn build_scoped_local_binding_index(
    arena: &AstArena,
    bindings: &[ParsedLocalBindingName],
    scopes: &LocalScopeTree,
    interner: &mut SymbolInterner,
) -> LocalBindingIndexBuild {
    let mut index = LocalBindingIndex::new();
    let mut inserts = Vec::new();
    let mut diagnostics = Vec::new();

    for binding in bindings {
        let Some(scope) = containing_block_scope(arena, scopes, binding.binding) else {
            continue;
        };
        let name = interner.intern(&binding.name);
        let key = LocalBindingKey::new(scope, name);
        let insert = index.insert(LocalBinding::new(key, binding.binding, binding.kind));
        if matches!(insert, LocalBindingInsert::Duplicate { .. }) {
            diagnostics.push(ResolutionDiagnostic::new(
                ResolutionDiagnosticKind::DuplicateName,
                binding.name_span,
            ));
        }
        inserts.push(insert);
    }

    LocalBindingIndexBuild {
        index,
        inserts,
        diagnostics,
    }
}

fn containing_block_scope(
    arena: &AstArena,
    scopes: &LocalScopeTree,
    binding: AstNodeId,
) -> Option<LocalScopeId> {
    let binding_span = arena.node(binding)?.span;
    scopes
        .scopes()
        .iter()
        .filter(|scope| {
            let Some(owner) = arena.node(scope.owner()) else {
                return false;
            };
            owner.kind == AstNodeKind::Block
                && owner.span.start() <= binding_span.start()
                && binding_span.end() <= owner.span.end()
        })
        .min_by_key(|scope| {
            let owner = arena
                .node(scope.owner())
                .expect("scope owner should exist after filter");
            owner.span.end() - owner.span.start()
        })
        .map(|scope| scope.id())
}

#[derive(Debug)]
pub struct DeclarationIndexBuild {
    index: DeclarationIndex,
    inserts: Vec<DeclarationInsert>,
    diagnostics: Vec<ResolutionDiagnostic>,
}

impl DeclarationIndexBuild {
    pub fn index(&self) -> &DeclarationIndex {
        &self.index
    }

    pub fn inserts(&self) -> &[DeclarationInsert] {
        &self.inserts
    }

    pub fn diagnostics(&self) -> &[ResolutionDiagnostic] {
        &self.diagnostics
    }
}

pub fn build_declaration_index(
    metadata: &ModuleMetadata,
    declarations: &[ParsedDeclarationName],
    interner: &mut SymbolInterner,
) -> DeclarationIndexBuild {
    let mut index = DeclarationIndex::new();
    let mut inserts = Vec::new();
    let mut diagnostics = Vec::new();

    for declaration in declarations {
        let package = metadata
            .packages()
            .iter()
            .find(|package| package.source_file() == declaration.name_span.file())
            .map(|package| package.namespace().clone())
            .unwrap_or_else(PackageNamespace::root);
        let name = interner.intern(&declaration.name);
        let key = DeclarationKey::new(metadata.name().clone(), package, name, declaration.kind);
        let insert = index.insert(DeclaredName::new(key, declaration.declaration));
        if matches!(insert, DeclarationInsert::Duplicate { .. }) {
            diagnostics.push(ResolutionDiagnostic::new(
                ResolutionDiagnosticKind::DuplicateName,
                declaration.name_span,
            ));
        }
        inserts.push(insert);
    }

    DeclarationIndexBuild {
        index,
        inserts,
        diagnostics,
    }
}

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
