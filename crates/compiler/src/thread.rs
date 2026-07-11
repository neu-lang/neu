use crate::{
    ast::AstNodeId,
    name_resolution::LocalBinding,
    types::{PrimitiveType, TypeArena, TypeId, TypeKind},
};

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
        TypeKind::Primitive(
            PrimitiveType::Bool
            | PrimitiveType::Int
            | PrimitiveType::Unit
            | PrimitiveType::Float
            | PrimitiveType::Byte,
        ) => true,
        TypeKind::Primitive(PrimitiveType::Null) => true,
        TypeKind::Primitive(PrimitiveType::String) => capability == ThreadCapability::Send,
        TypeKind::Nullable(nullable) => {
            satisfies_thread_capability(types, nullable.base(), capability)
        }
        TypeKind::Nominal(_) | TypeKind::GenericParameter(_) => false,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThreadCapture {
    capture: AstNodeId,
    binding: LocalBinding,
    ty: TypeId,
    required: ThreadCapability,
}

impl ThreadCapture {
    pub fn new(
        capture: AstNodeId,
        binding: LocalBinding,
        ty: TypeId,
        required: ThreadCapability,
    ) -> Self {
        Self {
            capture,
            binding,
            ty,
            required,
        }
    }

    pub fn capture(&self) -> AstNodeId {
        self.capture
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }

    pub fn ty(&self) -> TypeId {
        self.ty
    }

    pub fn required(&self) -> ThreadCapability {
        self.required
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThreadBoundary {
    boundary: AstNodeId,
    captures: Vec<ThreadCapture>,
}

impl ThreadBoundary {
    pub fn new(boundary: AstNodeId, captures: Vec<ThreadCapture>) -> Self {
        Self { boundary, captures }
    }

    pub fn boundary(&self) -> AstNodeId {
        self.boundary
    }

    pub fn captures(&self) -> &[ThreadCapture] {
        &self.captures
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadDiagnosticKind {
    MissingThreadCapability,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThreadDiagnostic {
    kind: ThreadDiagnosticKind,
    capture: AstNodeId,
    boundary: AstNodeId,
    binding: LocalBinding,
    ty: TypeId,
    required: ThreadCapability,
}

impl ThreadDiagnostic {
    pub fn missing_thread_capability(
        capture: AstNodeId,
        boundary: AstNodeId,
        binding: LocalBinding,
        ty: TypeId,
        required: ThreadCapability,
    ) -> Self {
        Self {
            kind: ThreadDiagnosticKind::MissingThreadCapability,
            capture,
            boundary,
            binding,
            ty,
            required,
        }
    }

    pub fn kind(&self) -> ThreadDiagnosticKind {
        self.kind
    }

    pub fn capture(&self) -> AstNodeId {
        self.capture
    }

    pub fn boundary(&self) -> AstNodeId {
        self.boundary
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }

    pub fn ty(&self) -> TypeId {
        self.ty
    }

    pub fn required(&self) -> ThreadCapability {
        self.required
    }
}

pub fn analyze_thread_boundaries(
    boundaries: &[ThreadBoundary],
    types: &TypeArena,
) -> Vec<ThreadDiagnostic> {
    let mut diagnostics = Vec::new();

    for boundary in boundaries {
        for capture in boundary.captures() {
            if !satisfies_thread_capability(types, capture.ty(), capture.required()) {
                diagnostics.push(ThreadDiagnostic::missing_thread_capability(
                    capture.capture(),
                    boundary.boundary(),
                    capture.binding().clone(),
                    capture.ty(),
                    capture.required(),
                ));
            }
        }
    }

    diagnostics
}
