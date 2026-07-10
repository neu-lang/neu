use compiler::ast::{AstArena, AstNodeKind};
use compiler::source::{ByteSpan, SourceFileId};

#[test]
fn source_file_root_node_preserves_span() {
    let file = SourceFileId::from_raw(7);
    let span = ByteSpan::new(file, 0, 12).unwrap();
    let mut arena = AstArena::new();

    let root = arena.add_source_file(span);
    let node = arena.node(root).unwrap();

    assert_eq!(root.index(), 0);
    assert_eq!(node.id, root);
    assert_eq!(node.kind, AstNodeKind::SourceFile);
    assert_eq!(node.span, span);
}

#[test]
fn ast_node_ids_are_stable_in_insertion_order() {
    let file = SourceFileId::from_raw(1);
    let mut arena = AstArena::new();

    let first = arena.add_source_file(ByteSpan::new(file, 0, 1).unwrap());
    let second = arena.add_source_file(ByteSpan::new(file, 2, 3).unwrap());

    assert_eq!(first.index(), 0);
    assert_eq!(second.index(), 1);
    assert_eq!(arena.node(first).unwrap().span.start(), 0);
    assert_eq!(arena.node(second).unwrap().span.start(), 2);
}

#[test]
fn invalid_ast_node_ids_are_rejected() {
    let arena = AstArena::new();

    assert!(arena.node(compiler::ast::AstNodeId::from_raw(99)).is_none());
}

#[test]
fn declaration_shell_nodes_preserve_kind_and_span() {
    let file = SourceFileId::from_raw(2);
    let mut arena = AstArena::new();

    let cases = [
        (
            arena.add_package_declaration(ByteSpan::new(file, 0, 12).unwrap()),
            AstNodeKind::PackageDeclaration,
            0,
        ),
        (
            arena.add_import_declaration(ByteSpan::new(file, 13, 27).unwrap()),
            AstNodeKind::ImportDeclaration,
            13,
        ),
        (
            arena.add_function_declaration(ByteSpan::new(file, 28, 40).unwrap()),
            AstNodeKind::FunctionDeclaration,
            28,
        ),
        (
            arena.add_struct_declaration(ByteSpan::new(file, 41, 54).unwrap()),
            AstNodeKind::StructDeclaration,
            41,
        ),
        (
            arena.add_enum_declaration(ByteSpan::new(file, 55, 67).unwrap()),
            AstNodeKind::EnumDeclaration,
            55,
        ),
        (
            arena.add_interface_declaration(ByteSpan::new(file, 68, 90).unwrap()),
            AstNodeKind::InterfaceDeclaration,
            68,
        ),
    ];

    for (id, kind, start) in cases {
        let node = arena.node(id).unwrap();
        assert_eq!(node.kind, kind);
        assert_eq!(node.span.start(), start);
    }
}

#[test]
fn declaration_body_node_is_syntax_only() {
    let file = SourceFileId::from_raw(3);
    let span = ByteSpan::new(file, 10, 20).unwrap();
    let mut arena = AstArena::new();

    let body = arena.add_declaration_body(span);
    let node = arena.node(body).unwrap();

    assert_eq!(node.kind, AstNodeKind::DeclarationBody);
    assert_eq!(node.span, span);
}

#[test]
fn type_and_generic_shell_nodes_preserve_kind_and_span() {
    let file = SourceFileId::from_raw(4);
    let mut arena = AstArena::new();

    let cases = [
        (
            arena.add_named_type(ByteSpan::new(file, 0, 3).unwrap()),
            AstNodeKind::NamedType,
            0,
        ),
        (
            arena.add_nullable_type(ByteSpan::new(file, 4, 6).unwrap()),
            AstNodeKind::NullableType,
            4,
        ),
        (
            arena.add_generic_parameter(ByteSpan::new(file, 7, 8).unwrap()),
            AstNodeKind::GenericParameter,
            7,
        ),
        (
            arena.add_generic_argument(ByteSpan::new(file, 9, 12).unwrap()),
            AstNodeKind::GenericArgument,
            9,
        ),
        (
            arena.add_capability_bound(ByteSpan::new(file, 13, 17).unwrap()),
            AstNodeKind::CapabilityBound,
            13,
        ),
        (
            arena.add_function_type(ByteSpan::new(file, 18, 26).unwrap()),
            AstNodeKind::FunctionType,
            18,
        ),
        (
            arena.add_grouped_type(ByteSpan::new(file, 27, 30).unwrap()),
            AstNodeKind::GroupedType,
            27,
        ),
    ];

    for (id, kind, start) in cases {
        let node = arena.node(id).unwrap();
        assert_eq!(node.kind, kind);
        assert_eq!(node.span.start(), start);
    }
}

#[test]
fn expression_statement_pattern_shell_nodes_preserve_kind_and_span() {
    let file = SourceFileId::from_raw(5);
    let mut arena = AstArena::new();

    let cases = [
        (
            arena.add_block(ByteSpan::new(file, 0, 3).unwrap()),
            AstNodeKind::Block,
            0,
        ),
        (
            arena.add_literal_expression(ByteSpan::new(file, 4, 5).unwrap()),
            AstNodeKind::LiteralExpression,
            4,
        ),
        (
            arena.add_name_expression(ByteSpan::new(file, 6, 11).unwrap()),
            AstNodeKind::NameExpression,
            6,
        ),
        (
            arena.add_grouped_expression(ByteSpan::new(file, 12, 17).unwrap()),
            AstNodeKind::GroupedExpression,
            12,
        ),
        (
            arena.add_if_expression(ByteSpan::new(file, 18, 35).unwrap()),
            AstNodeKind::IfExpression,
            18,
        ),
        (
            arena.add_binary_expression(ByteSpan::new(file, 36, 41).unwrap()),
            AstNodeKind::BinaryExpression,
            36,
        ),
        (
            arena.add_unary_expression(ByteSpan::new(file, 42, 48).unwrap()),
            AstNodeKind::UnaryExpression,
            42,
        ),
        (
            arena.add_call_expression(ByteSpan::new(file, 49, 56).unwrap()),
            AstNodeKind::CallExpression,
            49,
        ),
        (
            arena.add_member_expression(ByteSpan::new(file, 57, 66).unwrap()),
            AstNodeKind::MemberExpression,
            57,
        ),
        (
            arena.add_variable_declaration_statement(ByteSpan::new(file, 67, 80).unwrap()),
            AstNodeKind::VariableDeclarationStatement,
            67,
        ),
        (
            arena.add_assignment_statement(ByteSpan::new(file, 81, 91).unwrap()),
            AstNodeKind::AssignmentStatement,
            81,
        ),
        (
            arena.add_return_statement(ByteSpan::new(file, 92, 103).unwrap()),
            AstNodeKind::ReturnStatement,
            92,
        ),
        (
            arena.add_expression_statement(ByteSpan::new(file, 104, 116).unwrap()),
            AstNodeKind::ExpressionStatement,
            104,
        ),
        (
            arena.add_wildcard_pattern(ByteSpan::new(file, 117, 118).unwrap()),
            AstNodeKind::WildcardPattern,
            117,
        ),
        (
            arena.add_literal_pattern(ByteSpan::new(file, 119, 123).unwrap()),
            AstNodeKind::LiteralPattern,
            119,
        ),
        (
            arena.add_binding_pattern(ByteSpan::new(file, 124, 129).unwrap()),
            AstNodeKind::BindingPattern,
            124,
        ),
        (
            arena.add_qualified_case_pattern(ByteSpan::new(file, 130, 143).unwrap()),
            AstNodeKind::QualifiedCasePattern,
            130,
        ),
        (
            arena.add_grouped_pattern(ByteSpan::new(file, 144, 151).unwrap()),
            AstNodeKind::GroupedPattern,
            144,
        ),
    ];

    for (id, kind, start) in cases {
        let node = arena.node(id).unwrap();
        assert_eq!(node.kind, kind);
        assert_eq!(node.span.start(), start);
    }
}
