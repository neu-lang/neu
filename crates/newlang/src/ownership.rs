use crate::types::{PrimitiveType, TypeArena, TypeId, TypeKind};

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
