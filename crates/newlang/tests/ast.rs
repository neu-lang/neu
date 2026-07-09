use newlang::ast::{AstArena, AstNodeKind};
use newlang::source::{ByteSpan, SourceFileId};

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

    assert!(arena.node(newlang::ast::AstNodeId::from_raw(99)).is_none());
}
