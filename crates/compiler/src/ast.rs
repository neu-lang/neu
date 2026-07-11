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
    FunctionParameter,
    StructDeclaration,
    EnumDeclaration,
    EnumVariant,
    InterfaceDeclaration,
    DeclarationBody,
    NamedType,
    NullableType,
    GenericParameter,
    GenericArgument,
    CapabilityBound,
    FunctionType,
    GroupedType,
    Block,
    LiteralExpression,
    NameExpression,
    GroupedExpression,
    IfExpression,
    WhenExpression,
    BinaryExpression,
    UnaryExpression,
    CallExpression,
    MemberExpression,
    VariableDeclarationStatement,
    AssignmentStatement,
    ReturnStatement,
    ExpressionStatement,
    ForStatement,
    BreakStatement,
    ContinueStatement,
    WildcardPattern,
    LiteralPattern,
    BindingPattern,
    QualifiedCasePattern,
    GroupedPattern,
    MatchArm,
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

    pub fn add_function_parameter(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::FunctionParameter, span)
    }

    pub fn add_struct_declaration(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::StructDeclaration, span)
    }

    pub fn add_enum_declaration(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::EnumDeclaration, span)
    }

    pub fn add_enum_variant(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::EnumVariant, span)
    }

    pub fn add_interface_declaration(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::InterfaceDeclaration, span)
    }

    pub fn add_declaration_body(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::DeclarationBody, span)
    }

    pub fn add_named_type(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::NamedType, span)
    }

    pub fn add_nullable_type(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::NullableType, span)
    }

    pub fn add_generic_parameter(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::GenericParameter, span)
    }

    pub fn add_generic_argument(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::GenericArgument, span)
    }

    pub fn add_capability_bound(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::CapabilityBound, span)
    }

    pub fn add_function_type(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::FunctionType, span)
    }

    pub fn add_grouped_type(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::GroupedType, span)
    }

    pub fn add_block(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::Block, span)
    }

    pub fn add_literal_expression(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::LiteralExpression, span)
    }

    pub fn add_name_expression(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::NameExpression, span)
    }

    pub fn add_grouped_expression(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::GroupedExpression, span)
    }

    pub fn add_if_expression(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::IfExpression, span)
    }

    pub fn add_when_expression(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::WhenExpression, span)
    }

    pub fn add_binary_expression(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::BinaryExpression, span)
    }

    pub fn add_unary_expression(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::UnaryExpression, span)
    }

    pub fn add_call_expression(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::CallExpression, span)
    }

    pub fn add_member_expression(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::MemberExpression, span)
    }

    pub fn add_variable_declaration_statement(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::VariableDeclarationStatement, span)
    }

    pub fn add_assignment_statement(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::AssignmentStatement, span)
    }

    pub fn add_return_statement(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::ReturnStatement, span)
    }

    pub fn add_expression_statement(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::ExpressionStatement, span)
    }

    pub fn add_for_statement(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::ForStatement, span)
    }

    pub fn add_break_statement(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::BreakStatement, span)
    }

    pub fn add_continue_statement(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::ContinueStatement, span)
    }

    pub fn add_wildcard_pattern(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::WildcardPattern, span)
    }

    pub fn add_literal_pattern(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::LiteralPattern, span)
    }

    pub fn add_binding_pattern(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::BindingPattern, span)
    }

    pub fn add_qualified_case_pattern(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::QualifiedCasePattern, span)
    }

    pub fn add_grouped_pattern(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::GroupedPattern, span)
    }

    pub fn add_match_arm(&mut self, span: ByteSpan) -> AstNodeId {
        self.push(AstNodeKind::MatchArm, span)
    }

    pub fn node(&self, id: AstNodeId) -> Option<&AstNode> {
        self.nodes.get(id.index())
    }

    pub fn nodes(&self) -> &[AstNode] {
        &self.nodes
    }

    fn push(&mut self, kind: AstNodeKind, span: ByteSpan) -> AstNodeId {
        let id = AstNodeId(self.nodes.len());
        self.nodes.push(AstNode { id, kind, span });
        id
    }
}
