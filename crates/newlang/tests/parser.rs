use newlang::ast::AstNodeKind;
use newlang::name_resolution::{DeclarationKind, LocalBindingKind};
use newlang::parser::{parse_source, DiagnosticKind};
use newlang::source::SourceFileId;

#[test]
fn parses_package_import_and_function_declaration() {
    let output = parse_source(
        SourceFileId::from_raw(1),
        "package demo.core import demo.io as io public fun main();",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(
        output.node_kinds(),
        vec![
            AstNodeKind::SourceFile,
            AstNodeKind::PackageDeclaration,
            AstNodeKind::ImportDeclaration,
            AstNodeKind::FunctionDeclaration,
        ]
    );
}

#[test]
fn parses_nested_declaration_body_shells() {
    let output = parse_source(
        SourceFileId::from_raw(2),
        "struct Module { fun build(); interface Service { fun run(); } enum State {} }",
    );

    assert!(output.diagnostics.is_empty());
    assert_eq!(
        output.node_kinds(),
        vec![
            AstNodeKind::SourceFile,
            AstNodeKind::StructDeclaration,
            AstNodeKind::DeclarationBody,
            AstNodeKind::FunctionDeclaration,
            AstNodeKind::InterfaceDeclaration,
            AstNodeKind::DeclarationBody,
            AstNodeKind::FunctionDeclaration,
            AstNodeKind::EnumDeclaration,
            AstNodeKind::DeclarationBody,
        ]
    );
}

#[test]
fn reports_misplaced_package_and_import() {
    let output = parse_source(
        SourceFileId::from_raw(3),
        "fun main(); package misplaced import too.late",
    );

    let kinds: Vec<_> = output
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.kind)
        .collect();
    assert_eq!(
        kinds,
        vec![
            DiagnosticKind::MisplacedPackageDeclaration,
            DiagnosticKind::MisplacedImportDeclaration,
        ]
    );
    assert!(output.diagnostics[0].span.start() < output.diagnostics[1].span.start());
}

#[test]
fn reports_duplicate_visibility_and_missing_name() {
    let output = parse_source(SourceFileId::from_raw(4), "public private fun ();");

    let kinds: Vec<_> = output
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.kind)
        .collect();
    assert_eq!(
        kinds,
        vec![
            DiagnosticKind::DuplicateVisibilityModifier,
            DiagnosticKind::MissingDeclarationName,
        ]
    );
}

#[test]
fn rejects_deferred_expression_and_field_syntax() {
    let output = parse_source(
        SourceFileId::from_raw(5),
        "struct Box { val size: Int } fun answer() = 42",
    );

    let kinds: Vec<_> = output
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.kind)
        .collect();
    assert!(kinds.contains(&DiagnosticKind::UnexpectedTokenInDeclarationBody));
    assert!(kinds.contains(&DiagnosticKind::MalformedDeclarationHeader));
}

#[test]
fn parses_type_and_generic_syntax() {
    let output = parse_source(
        SourceFileId::from_raw(6),
        "struct Box<T: Send & Share, U> {} fun wrap<T>(value): ((Box<T?>) -> U)?;",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());

    let kinds = output.node_kinds();
    assert!(kinds.contains(&AstNodeKind::StructDeclaration));
    assert!(kinds.contains(&AstNodeKind::FunctionDeclaration));
    assert!(kinds.contains(&AstNodeKind::GenericParameter));
    assert!(kinds.contains(&AstNodeKind::CapabilityBound));
    assert!(kinds.contains(&AstNodeKind::NamedType));
    assert!(kinds.contains(&AstNodeKind::GenericArgument));
    assert!(kinds.contains(&AstNodeKind::NullableType));
    assert!(kinds.contains(&AstNodeKind::FunctionType));
    assert!(kinds.contains(&AstNodeKind::GroupedType));
}

#[test]
fn reports_malformed_type_and_generic_syntax() {
    let output = parse_source(
        SourceFileId::from_raw(7),
        "struct Bad<> {} fun wrong<T: Send, Share>(): T??; fun broken(): (T) ->;",
    );

    let kinds: Vec<_> = output
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.kind)
        .collect();

    assert!(kinds.contains(&DiagnosticKind::MalformedGenericParameterList));
    assert!(kinds.contains(&DiagnosticKind::MalformedCapabilityBound));
    assert!(kinds.contains(&DiagnosticKind::MalformedNullableType));
    assert!(kinds.contains(&DiagnosticKind::MalformedFunctionType));
    assert!(kinds.contains(&DiagnosticKind::MissingTypeName));
}

#[test]
fn parses_adr0024_body_statements_and_expressions() {
    let output = parse_source(
        SourceFileId::from_raw(8),
        "fun run(): Int { val answer: Int = compute(); var next = answer; next = next + 1; logger.info(next); return next; }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());

    let kinds = output.node_kinds();
    assert!(kinds.contains(&AstNodeKind::Block));
    assert!(kinds.contains(&AstNodeKind::VariableDeclarationStatement));
    assert!(kinds.contains(&AstNodeKind::AssignmentStatement));
    assert!(kinds.contains(&AstNodeKind::ExpressionStatement));
    assert!(kinds.contains(&AstNodeKind::ReturnStatement));
    assert!(kinds.contains(&AstNodeKind::CallExpression));
    assert!(kinds.contains(&AstNodeKind::MemberExpression));
    assert!(kinds.contains(&AstNodeKind::BinaryExpression));
    assert!(kinds.contains(&AstNodeKind::NameExpression));
}

#[test]
fn records_local_val_and_var_binding_name_metadata() {
    let file = SourceFileId::from_raw(18);
    let source = "fun run() { val answer: Int = compute(); var next = answer; }";
    let output = parse_source(file, source);

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.local_binding_names.len(), 2);

    let first = &output.local_binding_names[0];
    let second = &output.local_binding_names[1];

    assert_eq!(first.kind, LocalBindingKind::Val);
    assert_eq!(first.name, "answer");
    assert_eq!(
        &source[first.name_span.start()..first.name_span.end()],
        "answer"
    );
    assert_eq!(second.kind, LocalBindingKind::Var);
    assert_eq!(second.name, "next");
    assert_eq!(
        &source[second.name_span.start()..second.name_span.end()],
        "next"
    );
    assert_ne!(first.binding, second.binding);
}

#[test]
fn local_binding_name_metadata_excludes_malformed_declarations() {
    let output = parse_source(
        SourceFileId::from_raw(19),
        "fun broken() { val : Int = compute(); var ok = value; }",
    );

    assert_eq!(output.local_binding_names.len(), 1);
    assert_eq!(output.local_binding_names[0].kind, LocalBindingKind::Var);
    assert_eq!(output.local_binding_names[0].name, "ok");
}

#[test]
fn parses_trailing_expression_and_if_expression_body() {
    let output = parse_source(
        SourceFileId::from_raw(9),
        "fun choose(): Int { val ready = check(); if (ready) { service.run(arg, 2); } else { fallback(); } }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());

    let kinds = output.node_kinds();
    assert!(kinds.contains(&AstNodeKind::Block));
    assert!(kinds.contains(&AstNodeKind::IfExpression));
    assert!(kinds.contains(&AstNodeKind::CallExpression));
    assert!(kinds.contains(&AstNodeKind::MemberExpression));
    assert!(kinds.contains(&AstNodeKind::ExpressionStatement));
}

#[test]
fn reports_adr0024_body_diagnostics() {
    let output = parse_source(
        SourceFileId::from_raw(10),
        "fun broken() { val : Int = compute(); target = ; return + ; service.(arg,); if ready { nope(); } }",
    );

    let kinds: Vec<_> = output
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.kind)
        .collect();

    assert!(kinds.contains(&DiagnosticKind::MalformedVariableDeclaration));
    assert!(kinds.contains(&DiagnosticKind::MalformedAssignment));
    assert!(kinds.contains(&DiagnosticKind::MalformedReturnStatement));
    assert!(kinds.contains(&DiagnosticKind::MalformedCallExpression));
    assert!(kinds.contains(&DiagnosticKind::MalformedMemberAccess));
    assert!(kinds.contains(&DiagnosticKind::MalformedConditional));
    assert!(output
        .diagnostics
        .iter()
        .all(|diagnostic| diagnostic.span.start() <= diagnostic.span.end()));
}

#[test]
fn rejects_deferred_body_forms() {
    let output = parse_source(
        SourceFileId::from_raw(11),
        "fun deferred() { while (ready) { run(); } unsafe { run(); } when (value) { } items[0]; }",
    );

    let kinds: Vec<_> = output
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.kind)
        .collect();

    assert!(kinds.contains(&DiagnosticKind::UnsupportedStatementForm));
    assert!(kinds.contains(&DiagnosticKind::MalformedUnsafeBlock));
    assert!(kinds.contains(&DiagnosticKind::UnsupportedExpressionForm));
}

#[test]
fn records_top_level_function_declaration_name_metadata() {
    let output = parse_source(SourceFileId::from_raw(12), "public fun main(): Int;");

    assert!(output.diagnostics.is_empty());
    assert_eq!(output.declaration_names.len(), 1);

    let declaration = &output.declaration_names[0];
    assert_eq!(declaration.kind, DeclarationKind::Function);
    assert_eq!(declaration.name, "main");
    assert_eq!(declaration.name_span.start(), 11);
    assert_eq!(declaration.name_span.end(), 15);
    assert_eq!(
        output.arena.node(declaration.declaration).unwrap().kind,
        AstNodeKind::FunctionDeclaration
    );
}

#[test]
fn records_top_level_type_declaration_name_metadata() {
    let output = parse_source(
        SourceFileId::from_raw(13),
        "struct Box {} enum State {} interface Service {}",
    );

    assert!(output.diagnostics.is_empty());
    let names: Vec<_> = output
        .declaration_names
        .iter()
        .map(|declaration| (declaration.kind, declaration.name.as_str()))
        .collect();

    assert_eq!(
        names,
        vec![
            (DeclarationKind::Type, "Box"),
            (DeclarationKind::Type, "State"),
            (DeclarationKind::Type, "Service"),
        ]
    );
}

#[test]
fn declaration_name_metadata_excludes_nested_declarations_and_missing_names() {
    let nested = parse_source(
        SourceFileId::from_raw(14),
        "struct Module { fun build(); enum State {} } fun ();",
    );

    let names: Vec<_> = nested
        .declaration_names
        .iter()
        .map(|declaration| declaration.name.as_str())
        .collect();

    assert_eq!(names, vec!["Module"]);
    assert!(nested
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.kind == DiagnosticKind::MissingDeclarationName));
}
