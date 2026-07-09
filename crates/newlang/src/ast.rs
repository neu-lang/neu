use crate::source::ByteSpan;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AstNodeId(usize);

impl AstNodeId {
    pub fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AstNodeKind {
    SourceFile,
    PackageDeclaration,
    ImportDeclaration,
    FunctionDeclaration,
    StructDeclaration,
    EnumDeclaration,
    InterfaceDeclaration,
    DeclarationBody,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AstNode {
    pub id: AstNodeId,
    pub kind: AstNodeKind,
    pub span: ByteSpan,
}

#[derive(Debug, Default)]
pub struct AstArena {
    nodes: Vec<AstNode>,
}

impl AstArena {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_source_file(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::SourceFile, span)
    }

    pub fn add_package_declaration(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::PackageDeclaration, span)
    }

    pub fn add_import_declaration(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::ImportDeclaration, span)
    }

    pub fn add_function_declaration(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::FunctionDeclaration, span)
    }

    pub fn add_struct_declaration(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::StructDeclaration, span)
    }

    pub fn add_enum_declaration(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::EnumDeclaration, span)
    }

    pub fn add_interface_declaration(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::InterfaceDeclaration, span)
    }

    pub fn add_declaration_body(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::DeclarationBody, span)
    }

    pub fn node(&self, id: AstNodeId) -> Option<&AstNode> {
        self.nodes.get(id.index())
    }

    fn push(&mut self, kind: AstNodeKind, span: ByteSpan) -> AstNodeId {
        let id = AstNodeId(self.nodes.len());
        self.nodes.push(AstNode { id, kind, span });
        id
    }
}
