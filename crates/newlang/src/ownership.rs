use crate::{
    ast::AstNodeId,
    name_resolution::{LocalBinding, ResolvedLocalBinding},
    parser::{ParsedAssignmentStatement, ParsedLocalDeclaration},
    type_check::DeclarationSignature,
    types::{PrimitiveType, TypeArena, TypeId, TypeKind},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnershipCategory {
    Copyable,
    MoveOnly,
}

pub fn classify_ownership_category(types: &TypeArena, ty: TypeId) -> Option<OwnershipCategory> {
    match types.get(ty)?.kind() {
        TypeKind::Primitive(PrimitiveType::Bool | PrimitiveType::Int | PrimitiveType::Unit) => {
            Some(OwnershipCategory::Copyable)
        }
        TypeKind::Primitive(PrimitiveType::Null) => Some(OwnershipCategory::Copyable),
        TypeKind::Primitive(PrimitiveType::String) | TypeKind::Nominal(_) => {
            Some(OwnershipCategory::MoveOnly)
        }
        TypeKind::GenericParameter(_) | TypeKind::Nullable(_) => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnershipTransferKind {
    LocalInitializer,
    Assignment,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnershipTransfer {
    kind: OwnershipTransferKind,
    site: AstNodeId,
    source_use: AstNodeId,
    source_binding: LocalBinding,
}

impl OwnershipTransfer {
    pub fn new(
        kind: OwnershipTransferKind,
        site: AstNodeId,
        source_use: AstNodeId,
        source_binding: LocalBinding,
    ) -> Self {
        Self {
            kind,
            site,
            source_use,
            source_binding,
        }
    }

    pub fn kind(&self) -> OwnershipTransferKind {
        self.kind
    }

    pub fn site(&self) -> AstNodeId {
        self.site
    }

    pub fn source_use(&self) -> AstNodeId {
        self.source_use
    }

    pub fn source_binding(&self) -> &LocalBinding {
        &self.source_binding
    }
}

pub fn collect_ownership_transfers(
    declarations: &[ParsedLocalDeclaration],
    assignments: &[ParsedAssignmentStatement],
    resolved_local_bindings: &[ResolvedLocalBinding],
    declaration_signatures: &[DeclarationSignature],
    types: &TypeArena,
) -> Vec<OwnershipTransfer> {
    let mut transfers = Vec::new();

    for declaration in declarations {
        let Some(initializer) = declaration.initializer else {
            continue;
        };
        if let Some(source_binding) = move_only_source_binding(
            initializer,
            resolved_local_bindings,
            declaration_signatures,
            types,
        ) {
            transfers.push(OwnershipTransfer::new(
                OwnershipTransferKind::LocalInitializer,
                declaration.declaration,
                initializer,
                source_binding,
            ));
        }
    }

    for assignment in assignments {
        if let Some(source_binding) = move_only_source_binding(
            assignment.value,
            resolved_local_bindings,
            declaration_signatures,
            types,
        ) {
            transfers.push(OwnershipTransfer::new(
                OwnershipTransferKind::Assignment,
                assignment.statement,
                assignment.value,
                source_binding,
            ));
        }
    }

    transfers
}

fn move_only_source_binding(
    source_use: AstNodeId,
    resolved_local_bindings: &[ResolvedLocalBinding],
    declaration_signatures: &[DeclarationSignature],
    types: &TypeArena,
) -> Option<LocalBinding> {
    let resolved = resolved_local_bindings
        .iter()
        .find(|resolved| resolved.reference() == source_use)?;
    let signature = declaration_signatures
        .iter()
        .find(|signature| signature.declaration() == resolved.binding().binding())?;

    match classify_ownership_category(types, signature.ty()) {
        Some(OwnershipCategory::MoveOnly) => Some(resolved.binding().clone()),
        Some(OwnershipCategory::Copyable) | None => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnershipDiagnosticKind {
    UseAfterMove,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OwnershipDiagnostic {
    kind: OwnershipDiagnosticKind,
    node: AstNodeId,
    move_origin: AstNodeId,
}

impl OwnershipDiagnostic {
    pub fn use_after_move(node: AstNodeId, move_origin: AstNodeId) -> Self {
        Self {
            kind: OwnershipDiagnosticKind::UseAfterMove,
            node,
            move_origin,
        }
    }

    pub fn kind(&self) -> OwnershipDiagnosticKind {
        self.kind
    }

    pub fn node(&self) -> AstNodeId {
        self.node
    }

    pub fn move_origin(&self) -> AstNodeId {
        self.move_origin
    }
}

pub fn analyze_use_after_move(
    resolved_local_bindings: &[ResolvedLocalBinding],
    transfers: &[OwnershipTransfer],
) -> Vec<OwnershipDiagnostic> {
    let mut diagnostics = Vec::new();

    for resolved in resolved_local_bindings {
        for transfer in transfers {
            if resolved.binding() != transfer.source_binding()
                || resolved.reference() <= transfer.source_use()
            {
                continue;
            }
            diagnostics.push(OwnershipDiagnostic::use_after_move(
                resolved.reference(),
                transfer.source_use(),
            ));
        }
    }

    diagnostics
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnershipReport {
    transfers: Vec<OwnershipTransfer>,
    diagnostics: Vec<OwnershipDiagnostic>,
}

impl OwnershipReport {
    pub fn new(transfers: Vec<OwnershipTransfer>, diagnostics: Vec<OwnershipDiagnostic>) -> Self {
        Self {
            transfers,
            diagnostics,
        }
    }

    pub fn transfers(&self) -> &[OwnershipTransfer] {
        &self.transfers
    }

    pub fn diagnostics(&self) -> &[OwnershipDiagnostic] {
        &self.diagnostics
    }
}

pub fn analyze_ownership(
    declarations: &[ParsedLocalDeclaration],
    assignments: &[ParsedAssignmentStatement],
    resolved_local_bindings: &[ResolvedLocalBinding],
    declaration_signatures: &[DeclarationSignature],
    types: &TypeArena,
) -> OwnershipReport {
    let transfers = collect_ownership_transfers(
        declarations,
        assignments,
        resolved_local_bindings,
        declaration_signatures,
        types,
    );
    let diagnostics = analyze_use_after_move(resolved_local_bindings, &transfers);

    OwnershipReport::new(transfers, diagnostics)
}
