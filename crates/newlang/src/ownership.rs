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
