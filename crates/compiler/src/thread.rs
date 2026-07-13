use crate::{
    ast::AstNodeId,
    name_resolution::LocalBinding,
    types::{PrimitiveType, TypeArena, TypeId, TypeKind},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadCapability {
    Copy,
    Send,
    Share,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClosureCaptureKind {
    Moved,
    Shared,
    Borrowed,
    MutableShared,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClosureCapture {
    capture: AstNodeId,
    binding: LocalBinding,
    ty: TypeId,
    kind: ClosureCaptureKind,
}

impl ClosureCapture {
    pub fn new(
        capture: AstNodeId,
        binding: LocalBinding,
        ty: TypeId,
        kind: ClosureCaptureKind,
    ) -> Self {
        Self {
            capture,
            binding,
            ty,
            kind,
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

    pub fn kind(&self) -> ClosureCaptureKind {
        self.kind
    }

    fn required_capability(&self) -> Option<ThreadCapability> {
        match self.kind {
            ClosureCaptureKind::Moved | ClosureCaptureKind::MutableShared => {
                Some(ThreadCapability::Send)
            }
            ClosureCaptureKind::Shared => Some(ThreadCapability::Share),
            ClosureCaptureKind::Borrowed => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClosureBoundary {
    boundary: AstNodeId,
    captures: Vec<ClosureCapture>,
}

impl ClosureBoundary {
    pub fn new(boundary: AstNodeId, captures: Vec<ClosureCapture>) -> Self {
        Self { boundary, captures }
    }

    pub fn boundary(&self) -> AstNodeId {
        self.boundary
    }

    pub fn captures(&self) -> &[ClosureCapture] {
        &self.captures
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClosureConcurrencyDiagnosticKind {
    MissingCapability,
    BorrowedCapture,
    MutableSharedCapture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClosureConcurrencyDiagnostic {
    kind: ClosureConcurrencyDiagnosticKind,
    capture: AstNodeId,
    boundary: AstNodeId,
    binding: LocalBinding,
    ty: TypeId,
    required: ThreadCapability,
}

impl ClosureConcurrencyDiagnostic {
    fn new(
        kind: ClosureConcurrencyDiagnosticKind,
        capture: &ClosureCapture,
        boundary: AstNodeId,
        required: ThreadCapability,
    ) -> Self {
        Self {
            kind,
            capture: capture.capture(),
            boundary,
            binding: capture.binding().clone(),
            ty: capture.ty(),
            required,
        }
    }

    pub fn borrowed_capture(
        capture: AstNodeId,
        boundary: AstNodeId,
        binding: LocalBinding,
        ty: TypeId,
    ) -> Self {
        Self::new(
            ClosureConcurrencyDiagnosticKind::BorrowedCapture,
            &ClosureCapture::new(capture, binding, ty, ClosureCaptureKind::Borrowed),
            boundary,
            ThreadCapability::Send,
        )
    }

    pub fn kind(&self) -> ClosureConcurrencyDiagnosticKind {
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

pub fn analyze_closure_boundaries(
    boundaries: &[ClosureBoundary],
    types: &TypeArena,
) -> Vec<ClosureConcurrencyDiagnostic> {
    let mut diagnostics = Vec::new();

    for boundary in boundaries {
        for capture in boundary.captures() {
            match capture.kind() {
                ClosureCaptureKind::Borrowed => {
                    diagnostics.push(ClosureConcurrencyDiagnostic::new(
                        ClosureConcurrencyDiagnosticKind::BorrowedCapture,
                        capture,
                        boundary.boundary(),
                        ThreadCapability::Send,
                    ))
                }
                ClosureCaptureKind::MutableShared => {
                    diagnostics.push(ClosureConcurrencyDiagnostic::new(
                        ClosureConcurrencyDiagnosticKind::MutableSharedCapture,
                        capture,
                        boundary.boundary(),
                        ThreadCapability::Share,
                    ))
                }
                ClosureCaptureKind::Moved | ClosureCaptureKind::Shared => {
                    let required = capture
                        .required_capability()
                        .expect("owned closure capture has a capability");
                    if !satisfies_thread_capability(types, capture.ty(), required) {
                        diagnostics.push(ClosureConcurrencyDiagnostic::new(
                            ClosureConcurrencyDiagnosticKind::MissingCapability,
                            capture,
                            boundary.boundary(),
                            required,
                        ));
                    }
                }
            }
        }
    }

    diagnostics
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
        TypeKind::Function(_) => {
            capability == ThreadCapability::Copy
                || capability == ThreadCapability::Send
                || capability == ThreadCapability::Share
        }
        TypeKind::Primitive(PrimitiveType::String) => capability == ThreadCapability::Send,
        TypeKind::Nullable(nullable) => {
            satisfies_thread_capability(types, nullable.base(), capability)
        }
        TypeKind::Array(array) => satisfies_thread_capability(types, array.element(), capability),
        TypeKind::DynamicArray(array) => {
            capability == ThreadCapability::Send
                && satisfies_thread_capability(types, array.element(), capability)
        }
        TypeKind::Nominal(_) | TypeKind::GenericParameter(_) | TypeKind::GenericInstance(_) => {
            false
        }
        TypeKind::Task(_) => false,
        TypeKind::Channel(_) => capability != ThreadCapability::Copy,
        TypeKind::ChannelResult(result) => {
            satisfies_thread_capability(types, result.element(), capability)
        }
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
