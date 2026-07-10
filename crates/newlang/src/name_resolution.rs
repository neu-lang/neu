use std::collections::{hash_map::Entry, HashMap};

use crate::ast::{AstArena, AstNodeId, AstNodeKind};
use crate::module::{ModuleMetadata, ModuleName, PackageNamespace};
use crate::parser::{
    ParsedDeclarationName, ParsedEnumVariant, ParsedFunctionParameter, ParsedLocalBindingName,
    ParsedMatchArm, ParsedNameReference, ParsedQualifiedCasePattern, ParsedTypeNameReference,
    ParsedWhenExpression,
};
use crate::source::ByteSpan;
use crate::symbol::{SymbolId, SymbolInterner};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeclarationKind {
    Function,
    Type,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumVariantIdentity {
    module: ModuleName,
    package: PackageNamespace,
    enum_declaration: AstNodeId,
    enum_name: SymbolId,
    variant: AstNodeId,
    variant_name: SymbolId,
}

impl EnumVariantIdentity {
    fn new(
        module: ModuleName,
        package: PackageNamespace,
        enum_declaration: AstNodeId,
        enum_name: SymbolId,
        variant: AstNodeId,
        variant_name: SymbolId,
    ) -> Self {
        Self {
            module,
            package,
            enum_declaration,
            enum_name,
            variant,
            variant_name,
        }
    }

    pub fn module(&self) -> &ModuleName {
        &self.module
    }

    pub fn package(&self) -> &PackageNamespace {
        &self.package
    }

    pub fn enum_declaration(&self) -> AstNodeId {
        self.enum_declaration
    }

    pub fn enum_name(&self) -> SymbolId {
        self.enum_name
    }

    pub fn variant(&self) -> AstNodeId {
        self.variant
    }

    pub fn variant_name(&self) -> SymbolId {
        self.variant_name
    }
}

#[derive(Debug, Default)]
pub struct EnumVariantIndex {
    variants: Vec<EnumVariantIdentity>,
}

impl EnumVariantIndex {
    pub fn variants(&self) -> &[EnumVariantIdentity] {
        &self.variants
    }
}

pub fn build_enum_variant_index(
    metadata: &ModuleMetadata,
    variants: &[ParsedEnumVariant],
    declarations: &[ParsedDeclarationName],
    interner: &mut SymbolInterner,
) -> EnumVariantIndex {
    let mut index = EnumVariantIndex::default();

    for variant in variants {
        let Some(enum_declaration) = declarations
            .iter()
            .find(|declaration| declaration.declaration == variant.enum_declaration)
        else {
            continue;
        };
        let package = metadata
            .packages()
            .iter()
            .find(|package| package.source_file() == enum_declaration.name_span.file())
            .map(|package| package.namespace().clone())
            .unwrap_or_else(PackageNamespace::root);
        index.variants.push(EnumVariantIdentity::new(
            metadata.name().clone(),
            package,
            variant.enum_declaration,
            interner.intern(&enum_declaration.name),
            variant.variant,
            interner.intern(&variant.name),
        ));
    }

    index
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumParameterTypeRecord {
    parameter: AstNodeId,
    enum_declaration: AstNodeId,
}

impl EnumParameterTypeRecord {
    pub fn parameter(&self) -> AstNodeId {
        self.parameter
    }

    pub fn enum_declaration(&self) -> AstNodeId {
        self.enum_declaration
    }
}

#[derive(Debug, Default)]
pub struct EnumParameterTypeResolution {
    records: Vec<EnumParameterTypeRecord>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchDiagnosticKind {
    InvalidMatchSubject,
    UnknownMatchVariant,
    DuplicateEnumVariant,
    DuplicateMatchVariant,
    DuplicateMatchWildcard,
    NonExhaustiveMatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MatchDiagnostic {
    kind: MatchDiagnosticKind,
    node: AstNodeId,
}

impl MatchDiagnostic {
    pub fn kind(self) -> MatchDiagnosticKind {
        self.kind
    }
    pub fn node(self) -> AstNodeId {
        self.node
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedWhenSubject {
    expression: AstNodeId,
    enum_declaration: AstNodeId,
}

impl ResolvedWhenSubject {
    pub fn expression(self) -> AstNodeId {
        self.expression
    }
    pub fn enum_declaration(self) -> AstNodeId {
        self.enum_declaration
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedVariantArm {
    arm: AstNodeId,
    variant: AstNodeId,
}
impl ResolvedVariantArm {
    pub fn variant(self) -> AstNodeId {
        self.variant
    }
}

#[derive(Debug, Default)]
pub struct MatchAnalysisReport {
    subjects: Vec<ResolvedWhenSubject>,
    arms: Vec<ResolvedVariantArm>,
    diagnostics: Vec<MatchDiagnostic>,
}

impl MatchAnalysisReport {
    pub fn subjects(&self) -> &[ResolvedWhenSubject] {
        &self.subjects
    }
    pub fn arms(&self) -> &[ResolvedVariantArm] {
        &self.arms
    }
    pub fn diagnostics(&self) -> &[MatchDiagnostic] {
        &self.diagnostics
    }
}

pub fn analyze_duplicate_enum_variants(
    variants: &[ParsedEnumVariant],
    interner: &mut SymbolInterner,
) -> Vec<MatchDiagnostic> {
    let mut seen = HashMap::new();
    let mut diagnostics = Vec::new();
    for variant in variants {
        let key = (variant.enum_declaration, interner.intern(&variant.name));
        match seen.entry(key) {
            Entry::Occupied(_) => diagnostics.push(MatchDiagnostic {
                kind: MatchDiagnosticKind::DuplicateEnumVariant,
                node: variant.variant,
            }),
            Entry::Vacant(entry) => {
                entry.insert(variant.variant);
            }
        }
    }
    diagnostics
}

pub fn analyze_duplicate_match_arms(
    expressions: &[ParsedWhenExpression],
    arms: &[ParsedMatchArm],
    resolved_arms: &[ResolvedVariantArm],
) -> Vec<MatchDiagnostic> {
    let mut diagnostics = Vec::new();
    for expression in expressions {
        let mut seen_variants = HashMap::new();
        let mut seen_wildcard = false;
        for arm_id in &expression.arms {
            let Some(arm) = arms.iter().find(|arm| arm.arm == *arm_id) else {
                continue;
            };
            if arm.pattern_kind == AstNodeKind::WildcardPattern {
                if std::mem::replace(&mut seen_wildcard, true) {
                    diagnostics.push(MatchDiagnostic {
                        kind: MatchDiagnosticKind::DuplicateMatchWildcard,
                        node: arm.pattern,
                    });
                }
                continue;
            }

            let Some(resolved) = resolved_arms
                .iter()
                .find(|resolved| resolved.arm == arm.arm)
            else {
                continue;
            };
            match seen_variants.entry(resolved.variant) {
                Entry::Occupied(_) => diagnostics.push(MatchDiagnostic {
                    kind: MatchDiagnosticKind::DuplicateMatchVariant,
                    node: arm.pattern,
                }),
                Entry::Vacant(entry) => {
                    entry.insert(arm.arm);
                }
            }
        }
    }
    diagnostics
}

pub fn analyze_match_exhaustiveness(
    expressions: &[ParsedWhenExpression],
    arms: &[ParsedMatchArm],
    subjects: &[ResolvedWhenSubject],
    resolved_arms: &[ResolvedVariantArm],
    variants: &EnumVariantIndex,
    resolution_diagnostics: &[MatchDiagnostic],
    duplicate_diagnostics: &[MatchDiagnostic],
) -> Vec<MatchDiagnostic> {
    let mut diagnostics = Vec::new();
    for expression in expressions {
        let Some(subject) = subjects
            .iter()
            .find(|subject| subject.expression == expression.expression)
        else {
            continue;
        };
        let expression_arms: Vec<_> = expression
            .arms
            .iter()
            .filter_map(|arm_id| arms.iter().find(|arm| arm.arm == *arm_id))
            .collect();
        let has_prior_diagnostic = resolution_diagnostics
            .iter()
            .chain(duplicate_diagnostics)
            .any(|diagnostic| {
                diagnostic.node == expression.subject
                    || expression_arms
                        .iter()
                        .any(|arm| arm.pattern == diagnostic.node)
            });
        if has_prior_diagnostic
            || expression_arms
                .iter()
                .any(|arm| arm.pattern_kind == AstNodeKind::WildcardPattern)
        {
            continue;
        }

        let is_missing_variant = variants
            .variants()
            .iter()
            .filter(|variant| variant.enum_declaration() == subject.enum_declaration)
            .any(|variant| {
                !expression_arms.iter().any(|arm| {
                    resolved_arms.iter().any(|resolved| {
                        resolved.arm == arm.arm && resolved.variant == variant.variant()
                    })
                })
            });
        if is_missing_variant {
            diagnostics.push(MatchDiagnostic {
                kind: MatchDiagnosticKind::NonExhaustiveMatch,
                node: expression.subject,
            });
        }
    }
    diagnostics
}

pub fn resolve_qualified_variant_arms(
    expressions: &[ParsedWhenExpression],
    arms: &[ParsedMatchArm],
    patterns: &[ParsedQualifiedCasePattern],
    subjects: &[ResolvedWhenSubject],
    variants: &EnumVariantIndex,
    interner: &mut SymbolInterner,
) -> MatchAnalysisReport {
    let mut report = MatchAnalysisReport::default();
    for expression in expressions {
        let Some(subject) = subjects
            .iter()
            .find(|subject| subject.expression() == expression.expression)
        else {
            continue;
        };
        for arm_id in &expression.arms {
            let Some(arm) = arms.iter().find(|arm| arm.arm == *arm_id) else {
                continue;
            };
            let Some(pattern) = patterns
                .iter()
                .find(|pattern| pattern.pattern == arm.pattern)
            else {
                continue;
            };
            let enum_name = interner.intern(&pattern.enum_name);
            let variant_name = interner.intern(&pattern.variant_name);
            let variant = variants.variants().iter().find(|variant| {
                variant.enum_declaration() == subject.enum_declaration()
                    && variant.enum_name() == enum_name
                    && variant.variant_name() == variant_name
            });
            if let Some(variant) = variant {
                report.arms.push(ResolvedVariantArm {
                    arm: arm.arm,
                    variant: variant.variant(),
                });
            } else {
                report.diagnostics.push(MatchDiagnostic {
                    kind: MatchDiagnosticKind::UnknownMatchVariant,
                    node: arm.pattern,
                });
            }
        }
    }
    report
}

pub fn analyze_when_subjects(
    expressions: &[crate::parser::ParsedWhenExpression],
    resolved: &[ResolvedLocalBinding],
    enum_parameters: &EnumParameterTypeResolution,
) -> MatchAnalysisReport {
    let mut report = MatchAnalysisReport::default();
    for expression in expressions {
        let enum_declaration = resolved
            .iter()
            .find(|entry| entry.reference() == expression.subject)
            .and_then(|entry| {
                enum_parameters
                    .records()
                    .iter()
                    .find(|record| record.parameter() == entry.binding().binding())
            });
        if let Some(enum_declaration) = enum_declaration {
            report.subjects.push(ResolvedWhenSubject {
                expression: expression.expression,
                enum_declaration: enum_declaration.enum_declaration,
            });
        } else {
            report.diagnostics.push(MatchDiagnostic {
                kind: MatchDiagnosticKind::InvalidMatchSubject,
                node: expression.subject,
            });
        }
    }
    report
}

impl EnumParameterTypeResolution {
    pub fn records(&self) -> &[EnumParameterTypeRecord] {
        &self.records
    }
}

pub fn resolve_enum_parameter_types(
    arena: &AstArena,
    metadata: &ModuleMetadata,
    parameters: &[ParsedFunctionParameter],
    type_references: &[ParsedTypeNameReference],
    declarations: &[ParsedDeclarationName],
    interner: &mut SymbolInterner,
) -> EnumParameterTypeResolution {
    let mut resolution = EnumParameterTypeResolution::default();

    for parameter in parameters {
        let Some(annotation) = type_references
            .iter()
            .find(|reference| reference.reference == parameter.annotation)
        else {
            continue;
        };
        let annotation_package = package_for_file(metadata, annotation.name_span.file());
        let annotation_name = interner.intern(&annotation.name);
        let enum_declaration = declarations.iter().find(|declaration| {
            package_for_file(metadata, declaration.name_span.file()) == annotation_package
                && interner.intern(&declaration.name) == annotation_name
                && arena
                    .node(declaration.declaration)
                    .is_some_and(|node| node.kind == AstNodeKind::EnumDeclaration)
        });
        if let Some(enum_declaration) = enum_declaration {
            resolution.records.push(EnumParameterTypeRecord {
                parameter: parameter.parameter,
                enum_declaration: enum_declaration.declaration,
            });
        }
    }

    resolution
}

fn package_for_file(
    metadata: &ModuleMetadata,
    file: crate::source::SourceFileId,
) -> PackageNamespace {
    metadata
        .packages()
        .iter()
        .find(|package| package.source_file() == file)
        .map(|package| package.namespace().clone())
        .unwrap_or_else(PackageNamespace::root)
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
    Immutable,
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

    pub fn lookup_local(
        &self,
        scopes: &LocalScopeTree,
        arena: &AstArena,
        lookup: LocalNameLookup,
    ) -> LocalNameLookupResult {
        let mut current_scope = Some(lookup.start_scope);
        while let Some(scope) = current_scope {
            let key = LocalBindingKey::new(scope, lookup.name);
            if let Some(binding) = self.get(&key) {
                if local_binding_is_visible(arena, binding, lookup.reference_span) {
                    return LocalNameLookupResult::Found(binding.clone());
                }
            }
            current_scope = scopes.get(scope).and_then(|scope| scope.parent());
        }

        LocalNameLookupResult::Unresolved(ResolutionDiagnostic::new(
            ResolutionDiagnosticKind::UnresolvedName,
            lookup.reference_span,
        ))
    }

    pub fn bindings(&self) -> &[LocalBinding] {
        &self.bindings
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalNameLookup {
    start_scope: LocalScopeId,
    name: SymbolId,
    reference_span: ByteSpan,
}

impl LocalNameLookup {
    pub fn new(start_scope: LocalScopeId, name: SymbolId, reference_span: ByteSpan) -> Self {
        Self {
            start_scope,
            name,
            reference_span,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalNameLookupResult {
    Found(LocalBinding),
    Unresolved(ResolutionDiagnostic),
}

#[derive(Debug)]
pub struct LocalReferenceBind {
    table: ResolutionTable,
    inserts: Vec<ResolutionInsert>,
    resolved_local_bindings: Vec<ResolvedLocalBinding>,
    diagnostics: Vec<ResolutionDiagnostic>,
}

impl LocalReferenceBind {
    pub fn table(&self) -> &ResolutionTable {
        &self.table
    }

    pub fn inserts(&self) -> &[ResolutionInsert] {
        &self.inserts
    }

    pub fn resolved_local_bindings(&self) -> &[ResolvedLocalBinding] {
        &self.resolved_local_bindings
    }

    pub fn diagnostics(&self) -> &[ResolutionDiagnostic] {
        &self.diagnostics
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedLocalBinding {
    reference: AstNodeId,
    binding: LocalBinding,
}

impl ResolvedLocalBinding {
    pub fn new(reference: AstNodeId, binding: LocalBinding) -> Self {
        Self { reference, binding }
    }

    pub fn reference(&self) -> AstNodeId {
        self.reference
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }
}

pub fn bind_local_name_references(
    arena: &AstArena,
    references: &[ParsedNameReference],
    scopes: &LocalScopeTree,
    local_bindings: &LocalBindingIndex,
    interner: &mut SymbolInterner,
) -> LocalReferenceBind {
    let mut table = ResolutionTable::new();
    let mut inserts = Vec::new();
    let mut resolved_local_bindings = Vec::new();
    let mut diagnostics = Vec::new();

    for reference in references {
        let name = interner.intern(&reference.name);
        let Some(scope) = containing_block_scope(arena, scopes, reference.reference) else {
            diagnostics.push(ResolutionDiagnostic::new(
                ResolutionDiagnosticKind::UnresolvedName,
                reference.name_span,
            ));
            continue;
        };

        match local_bindings.lookup_local(
            scopes,
            arena,
            LocalNameLookup::new(scope, name, reference.name_span),
        ) {
            LocalNameLookupResult::Found(binding) => {
                resolved_local_bindings.push(ResolvedLocalBinding::new(
                    reference.reference,
                    binding.clone(),
                ));
                let insert =
                    table.insert(ResolvedName::new(reference.reference, binding.key().name()));
                inserts.push(insert);
            }
            LocalNameLookupResult::Unresolved(diagnostic) => diagnostics.push(diagnostic),
        }
    }

    LocalReferenceBind {
        table,
        inserts,
        resolved_local_bindings,
        diagnostics,
    }
}

#[derive(Debug)]
pub struct FunctionReferenceBind {
    table: ResolutionTable,
    inserts: Vec<ResolutionInsert>,
    diagnostics: Vec<ResolutionDiagnostic>,
}

impl FunctionReferenceBind {
    pub fn table(&self) -> &ResolutionTable {
        &self.table
    }

    pub fn inserts(&self) -> &[ResolutionInsert] {
        &self.inserts
    }

    pub fn diagnostics(&self) -> &[ResolutionDiagnostic] {
        &self.diagnostics
    }
}

pub fn bind_unqualified_function_references(
    metadata: &ModuleMetadata,
    arena: &AstArena,
    references: &[ParsedNameReference],
    scopes: &LocalScopeTree,
    local_bindings: &LocalBindingIndex,
    declarations: &DeclarationIndex,
    interner: &mut SymbolInterner,
) -> FunctionReferenceBind {
    let mut table = ResolutionTable::new();
    let mut inserts = Vec::new();
    let mut diagnostics = Vec::new();

    for reference in references {
        let name = interner.intern(&reference.name);
        let local_result = containing_block_scope(arena, scopes, reference.reference)
            .map(|scope| {
                local_bindings.lookup_local(
                    scopes,
                    arena,
                    LocalNameLookup::new(scope, name, reference.name_span),
                )
            })
            .unwrap_or_else(|| {
                LocalNameLookupResult::Unresolved(ResolutionDiagnostic::new(
                    ResolutionDiagnosticKind::UnresolvedName,
                    reference.name_span,
                ))
            });

        if let LocalNameLookupResult::Found(binding) = local_result {
            let insert = table.insert(ResolvedName::new(reference.reference, binding.key().name()));
            inserts.push(insert);
            continue;
        }

        let package = metadata
            .packages()
            .iter()
            .find(|package| package.source_file() == reference.name_span.file())
            .map(|package| package.namespace().clone())
            .unwrap_or_else(PackageNamespace::root);

        match declarations.lookup_top_level(TopLevelLookup::new(
            metadata.name().clone(),
            package,
            name,
            DeclarationKind::Function,
            reference.name_span,
        )) {
            TopLevelLookupResult::Found(declaration) => {
                let insert = table.insert(ResolvedName::new(
                    reference.reference,
                    declaration.key().name(),
                ));
                inserts.push(insert);
            }
            TopLevelLookupResult::Unresolved(diagnostic) => diagnostics.push(diagnostic),
        }
    }

    FunctionReferenceBind {
        table,
        inserts,
        diagnostics,
    }
}

#[derive(Debug)]
pub struct TypeReferenceBind {
    table: ResolutionTable,
    inserts: Vec<ResolutionInsert>,
    diagnostics: Vec<ResolutionDiagnostic>,
}

impl TypeReferenceBind {
    pub fn table(&self) -> &ResolutionTable {
        &self.table
    }

    pub fn inserts(&self) -> &[ResolutionInsert] {
        &self.inserts
    }

    pub fn diagnostics(&self) -> &[ResolutionDiagnostic] {
        &self.diagnostics
    }
}

pub fn bind_unqualified_type_references(
    metadata: &ModuleMetadata,
    arena: &AstArena,
    references: &[ParsedTypeNameReference],
    scopes: &LocalScopeTree,
    local_bindings: &LocalBindingIndex,
    declarations: &DeclarationIndex,
    interner: &mut SymbolInterner,
) -> TypeReferenceBind {
    let mut table = ResolutionTable::new();
    let mut inserts = Vec::new();
    let mut diagnostics = Vec::new();

    for reference in references {
        let name = interner.intern(&reference.name);
        let local_result = containing_block_scope(arena, scopes, reference.reference)
            .map(|scope| {
                local_bindings.lookup_local(
                    scopes,
                    arena,
                    LocalNameLookup::new(scope, name, reference.name_span),
                )
            })
            .unwrap_or_else(|| {
                LocalNameLookupResult::Unresolved(ResolutionDiagnostic::new(
                    ResolutionDiagnosticKind::UnresolvedName,
                    reference.name_span,
                ))
            });

        if let LocalNameLookupResult::Found(binding) = local_result {
            let insert = table.insert(ResolvedName::new(reference.reference, binding.key().name()));
            inserts.push(insert);
            continue;
        }

        let package = metadata
            .packages()
            .iter()
            .find(|package| package.source_file() == reference.name_span.file())
            .map(|package| package.namespace().clone())
            .unwrap_or_else(PackageNamespace::root);

        match declarations.lookup_top_level(TopLevelLookup::new(
            metadata.name().clone(),
            package,
            name,
            DeclarationKind::Type,
            reference.name_span,
        )) {
            TopLevelLookupResult::Found(declaration) => {
                let insert = table.insert(ResolvedName::new(
                    reference.reference,
                    declaration.key().name(),
                ));
                inserts.push(insert);
            }
            TopLevelLookupResult::Unresolved(diagnostic) => diagnostics.push(diagnostic),
        }
    }

    TypeReferenceBind {
        table,
        inserts,
        diagnostics,
    }
}

#[derive(Debug)]
pub struct PackageQualifiedTypeReferenceBind {
    table: ResolutionTable,
    inserts: Vec<ResolutionInsert>,
    diagnostics: Vec<ResolutionDiagnostic>,
}

impl PackageQualifiedTypeReferenceBind {
    pub fn table(&self) -> &ResolutionTable {
        &self.table
    }

    pub fn inserts(&self) -> &[ResolutionInsert] {
        &self.inserts
    }

    pub fn diagnostics(&self) -> &[ResolutionDiagnostic] {
        &self.diagnostics
    }
}

pub fn bind_package_qualified_type_references(
    metadata: &ModuleMetadata,
    references: &[ParsedTypeNameReference],
    declarations: &DeclarationIndex,
    interner: &mut SymbolInterner,
) -> PackageQualifiedTypeReferenceBind {
    let mut table = ResolutionTable::new();
    let mut inserts = Vec::new();
    let mut diagnostics = Vec::new();

    for reference in references {
        let Some((package_text, name_text)) = reference.name.rsplit_once('.') else {
            continue;
        };
        if package_text.is_empty() {
            continue;
        }
        let Ok(package) = PackageNamespace::parse(package_text) else {
            diagnostics.push(ResolutionDiagnostic::new(
                ResolutionDiagnosticKind::UnresolvedName,
                reference.name_span,
            ));
            continue;
        };

        let name = interner.intern(name_text);
        match declarations.lookup_top_level(TopLevelLookup::new(
            metadata.name().clone(),
            package,
            name,
            DeclarationKind::Type,
            reference.name_span,
        )) {
            TopLevelLookupResult::Found(declaration) => {
                let insert = table.insert(ResolvedName::new(
                    reference.reference,
                    declaration.key().name(),
                ));
                inserts.push(insert);
            }
            TopLevelLookupResult::Unresolved(diagnostic) => diagnostics.push(diagnostic),
        }
    }

    PackageQualifiedTypeReferenceBind {
        table,
        inserts,
        diagnostics,
    }
}

#[derive(Debug)]
pub struct AcceptedNameReferenceBind {
    table: ResolutionTable,
    inserts: Vec<ResolutionInsert>,
    diagnostics: Vec<ResolutionDiagnostic>,
}

impl AcceptedNameReferenceBind {
    pub fn table(&self) -> &ResolutionTable {
        &self.table
    }

    pub fn inserts(&self) -> &[ResolutionInsert] {
        &self.inserts
    }

    pub fn diagnostics(&self) -> &[ResolutionDiagnostic] {
        &self.diagnostics
    }
}

#[allow(clippy::too_many_arguments)]
pub fn bind_accepted_name_references(
    metadata: &ModuleMetadata,
    arena: &AstArena,
    name_references: &[ParsedNameReference],
    type_name_references: &[ParsedTypeNameReference],
    scopes: &LocalScopeTree,
    local_bindings: &LocalBindingIndex,
    declarations: &DeclarationIndex,
    interner: &mut SymbolInterner,
) -> AcceptedNameReferenceBind {
    let mut table = ResolutionTable::new();
    let mut inserts = Vec::new();
    let mut diagnostics = Vec::new();

    let function_bind = bind_unqualified_function_references(
        metadata,
        arena,
        name_references,
        scopes,
        local_bindings,
        declarations,
        interner,
    );
    merge_resolution_bind(
        &mut table,
        &mut inserts,
        &mut diagnostics,
        function_bind.table(),
        function_bind.diagnostics(),
    );

    let mut unqualified_type_references = Vec::new();
    let mut package_qualified_type_references = Vec::new();
    for reference in type_name_references {
        if reference.name.contains('.') {
            package_qualified_type_references.push(reference.clone());
        } else {
            unqualified_type_references.push(reference.clone());
        }
    }

    let type_bind = bind_unqualified_type_references(
        metadata,
        arena,
        &unqualified_type_references,
        scopes,
        local_bindings,
        declarations,
        interner,
    );
    merge_resolution_bind(
        &mut table,
        &mut inserts,
        &mut diagnostics,
        type_bind.table(),
        type_bind.diagnostics(),
    );

    let package_type_bind = bind_package_qualified_type_references(
        metadata,
        &package_qualified_type_references,
        declarations,
        interner,
    );
    merge_resolution_bind(
        &mut table,
        &mut inserts,
        &mut diagnostics,
        package_type_bind.table(),
        package_type_bind.diagnostics(),
    );

    AcceptedNameReferenceBind {
        table,
        inserts,
        diagnostics,
    }
}

fn merge_resolution_bind(
    table: &mut ResolutionTable,
    inserts: &mut Vec<ResolutionInsert>,
    diagnostics: &mut Vec<ResolutionDiagnostic>,
    source_table: &ResolutionTable,
    source_diagnostics: &[ResolutionDiagnostic],
) {
    for resolved_name in source_table.resolved_names() {
        inserts.push(table.insert(*resolved_name));
    }
    diagnostics.extend_from_slice(source_diagnostics);
}

fn local_binding_is_visible(
    arena: &AstArena,
    binding: &LocalBinding,
    reference_span: ByteSpan,
) -> bool {
    arena
        .node(binding.binding())
        .is_some_and(|node| node.span.end() <= reference_span.start())
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

pub fn build_scoped_binding_index(
    arena: &AstArena,
    parameters: &[ParsedFunctionParameter],
    bindings: &[ParsedLocalBindingName],
    scopes: &LocalScopeTree,
    interner: &mut SymbolInterner,
) -> LocalBindingIndexBuild {
    let mut result = build_function_parameter_binding_index(arena, parameters, scopes, interner);

    for binding in bindings {
        let Some(scope) = containing_block_scope(arena, scopes, binding.binding) else {
            continue;
        };
        let key = LocalBindingKey::new(scope, interner.intern(&binding.name));
        let insert = result
            .index
            .insert(LocalBinding::new(key, binding.binding, binding.kind));
        if matches!(insert, LocalBindingInsert::Duplicate { .. }) {
            result.diagnostics.push(ResolutionDiagnostic::new(
                ResolutionDiagnosticKind::DuplicateName,
                binding.name_span,
            ));
        }
        result.inserts.push(insert);
    }

    result
}

pub fn build_function_parameter_binding_index(
    arena: &AstArena,
    parameters: &[ParsedFunctionParameter],
    scopes: &LocalScopeTree,
    interner: &mut SymbolInterner,
) -> LocalBindingIndexBuild {
    let mut index = LocalBindingIndex::new();
    let mut inserts = Vec::new();
    let mut diagnostics = Vec::new();

    for parameter in parameters {
        let Some(scope) = function_body_scope(arena, scopes, parameter.function) else {
            continue;
        };
        let name = interner.intern(&parameter.name);
        let key = LocalBindingKey::new(scope, name);
        let insert = index.insert(LocalBinding::new(
            key,
            parameter.parameter,
            LocalBindingKind::Immutable,
        ));
        if matches!(insert, LocalBindingInsert::Duplicate { .. }) {
            diagnostics.push(ResolutionDiagnostic::new(
                ResolutionDiagnosticKind::DuplicateName,
                parameter.name_span,
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

fn function_body_scope(
    arena: &AstArena,
    scopes: &LocalScopeTree,
    function: AstNodeId,
) -> Option<LocalScopeId> {
    let function = arena.node(function)?;
    scopes
        .scopes()
        .iter()
        .find(|scope| {
            arena.node(scope.owner()).is_some_and(|owner| {
                owner.kind == AstNodeKind::Block && owner.span.start() == function.span.end()
            })
        })
        .map(|scope| scope.id())
}

fn containing_block_scope(
    arena: &AstArena,
    scopes: &LocalScopeTree,
    node: AstNodeId,
) -> Option<LocalScopeId> {
    let node_span = arena.node(node)?.span;
    scopes
        .scopes()
        .iter()
        .filter(|scope| {
            let Some(owner) = arena.node(scope.owner()) else {
                return false;
            };
            owner.kind == AstNodeKind::Block
                && owner.span.start() <= node_span.start()
                && node_span.end() <= owner.span.end()
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
