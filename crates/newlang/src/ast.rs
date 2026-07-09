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

    pub fn node(&self, id: AstNodeId) -> Option<&AstNode> {
        self.nodes.get(id.index())
    }

    fn push(&mut self, kind: AstNodeKind, span: ByteSpan) -> AstNodeId {
        let id = AstNodeId(self.nodes.len());
        self.nodes.push(AstNode { id, kind, span });
        id
    }
}
