use crate::types::{PrimitiveType, TypeArena, TypeId, TypeKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadCapability {
    Send,
    Share,
}

pub fn satisfies_thread_capability(
    types: &TypeArena,
    ty: TypeId,
    capability: ThreadCapability,
) -> bool {
    let Some(record) = types.get(ty) else {
        return false;
    };

    match record.kind() {
        TypeKind::Primitive(PrimitiveType::Bool | PrimitiveType::Int | PrimitiveType::Unit) => true,
        TypeKind::Primitive(PrimitiveType::Null) => true,
        TypeKind::Primitive(PrimitiveType::String) => capability == ThreadCapability::Send,
        TypeKind::Nullable(nullable) => {
            satisfies_thread_capability(types, nullable.base(), capability)
        }
        TypeKind::Nominal(_) | TypeKind::GenericParameter(_) => false,
    }
}
