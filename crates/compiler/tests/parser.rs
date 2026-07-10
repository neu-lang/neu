use compiler::ast::AstNodeKind;
use compiler::name_resolution::{DeclarationKind, LocalBindingKind};
use compiler::parser::{DiagnosticKind, ParsedLiteralKind, parse_source};
use compiler::parser::{ParsedBinaryOperator, ParsedUnaryOperator};
use compiler::source::{ByteSpan, SourceFileId};

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
fn m0020_generic_parameter_metadata_preserves_parameters_and_capability_bounds() {
    let source = "struct Box<T: capability.Send & Share, U> {} fun wrap<V: Send>() {}";
    let file = SourceFileId::from_raw(200);
    let output = parse_source(file, source);

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.generic_parameters.len(), 3);

    let box_parameter = &output.generic_parameters[0];
    assert_eq!(box_parameter.name, "T");
    assert_eq!(
        box_parameter.name_span,
        ByteSpan::new(file, 11, 12).unwrap()
    );
    assert_eq!(box_parameter.capability_bounds.len(), 2);
    assert_eq!(box_parameter.capability_bounds[0].name, "capability.Send");
    assert_eq!(
        box_parameter.capability_bounds[0].name_span,
        ByteSpan::new(file, 14, 29).unwrap()
    );
    assert_eq!(box_parameter.capability_bounds[1].name, "Share");
    assert_eq!(
        box_parameter.capability_bounds[1].name_span,
        ByteSpan::new(file, 32, 37).unwrap()
    );
    assert_ne!(
        box_parameter.parameter,
        box_parameter.capability_bounds[0].bound
    );

    assert_eq!(output.generic_parameters[1].name, "U");
    assert!(output.generic_parameters[1].capability_bounds.is_empty());
    assert_eq!(output.generic_parameters[2].name, "V");
    assert_eq!(
        output.generic_parameters[2].capability_bounds[0].name,
        "Send"
    );
}

#[test]
fn m0020_generic_parameter_metadata_excludes_malformed_lists_and_arguments() {
    let output = parse_source(
        SourceFileId::from_raw(201),
        "struct Bad<T: > {} fun use(): Box<Send> {};",
    );

    assert!(
        output
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.kind == DiagnosticKind::MissingGenericBound)
    );
    assert!(output.generic_parameters.is_empty());
}

#[test]
fn m0021_enum_variants_preserve_enclosing_enum_order_and_spans() {
    let file = SourceFileId::from_raw(202);
    let output = parse_source(file, "enum Signal { Red, Yellow; Green }");

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.enum_variants.len(), 3);
    assert!(output.node_kinds().contains(&AstNodeKind::EnumVariant));

    let declaration = output.declaration_names[0].declaration;
    assert_eq!(output.enum_variants[0].enum_declaration, declaration);
    assert_eq!(output.enum_variants[0].name, "Red");
    assert_eq!(
        output.enum_variants[0].name_span,
        ByteSpan::new(file, 14, 17).unwrap()
    );
    assert_eq!(output.enum_variants[1].name, "Yellow");
    assert_eq!(output.enum_variants[2].name, "Green");
    assert!(
        output
            .enum_variants
            .iter()
            .all(|variant| variant.enum_declaration == declaration)
    );
}

#[test]
fn m0021_enum_variants_exclude_empty_and_payload_shaped_entries() {
    let empty = parse_source(SourceFileId::from_raw(203), "enum Empty {}");
    assert!(empty.diagnostics.is_empty());
    assert!(empty.enum_variants.is_empty());

    let payload = parse_source(SourceFileId::from_raw(204), "enum Bad { Value(Int) }");
    assert!(!payload.diagnostics.is_empty());
    assert!(payload.enum_variants.is_empty());
}

#[test]
fn m0021_when_expression_records_subject_and_ordered_arms() {
    let file = SourceFileId::from_raw(205);
    let output = parse_source(
        file,
        "fun code() { when (signal) { Signal.Red -> 0; _ -> 1 } }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.when_expressions.len(), 1);
    assert_eq!(output.match_arms.len(), 2);
    assert!(output.node_kinds().contains(&AstNodeKind::WhenExpression));
    assert!(output.node_kinds().contains(&AstNodeKind::MatchArm));

    let when = &output.when_expressions[0];
    assert_eq!(when.arms.len(), 2);
    assert_eq!(when.subject, output.name_references[0].reference);
    assert_eq!(output.match_arms[0].arm, when.arms[0]);
    assert_eq!(
        output.match_arms[0].pattern_kind,
        AstNodeKind::QualifiedCasePattern
    );
    assert_eq!(output.match_arms[1].arm, when.arms[1]);
    assert_eq!(
        output.match_arms[1].pattern_kind,
        AstNodeKind::WildcardPattern
    );
    assert_eq!(
        output.arena.node(output.match_arms[0].body).unwrap().span,
        ByteSpan::new(file, 43, 44).unwrap()
    );
}

#[test]
fn m0021_when_expression_rejects_incomplete_or_unsupported_arms() {
    let missing_body = parse_source(
        SourceFileId::from_raw(206),
        "fun bad() { when (x) { A.B -> } }",
    );
    assert!(!missing_body.diagnostics.is_empty());
    assert!(missing_body.match_arms.is_empty());

    let binding = parse_source(
        SourceFileId::from_raw(207),
        "fun bad() { when (x) { value -> 1 } }",
    );
    assert!(!binding.diagnostics.is_empty());
    assert!(binding.match_arms.is_empty());
}

#[test]
fn m0021_qualified_case_pattern_records_exact_identifier_metadata() {
    let file = SourceFileId::from_raw(208);
    let output = parse_source(file, "fun code() { when (signal) { Signal.Red -> 0 } }");

    assert_eq!(output.qualified_case_patterns.len(), 1);
    let pattern = &output.qualified_case_patterns[0];
    assert_eq!(pattern.enum_name, "Signal");
    assert_eq!(pattern.variant_name, "Red");
    assert_eq!(pattern.enum_name_span, ByteSpan::new(file, 29, 35).unwrap());
    assert_eq!(
        pattern.variant_name_span,
        ByteSpan::new(file, 36, 39).unwrap()
    );

    let unsupported = parse_source(
        SourceFileId::from_raw(209),
        "fun code() { when (signal) { Signal.Red.Blue -> 0; Signal.Red(_) -> 1 } }",
    );
    assert!(unsupported.qualified_case_patterns.is_empty());
}

#[test]
fn m0021_typed_function_parameter_records_function_and_named_type() {
    let file = SourceFileId::from_raw(210);
    let output = parse_source(
        file,
        "fun code(signal: Signal) { when (signal) { _ -> 0 } }",
    );

    assert!(output.diagnostics.is_empty());
    assert_eq!(output.function_parameters.len(), 1);
    let parameter = &output.function_parameters[0];
    assert_eq!(parameter.name, "signal");
    assert_eq!(parameter.function, output.declaration_names[0].declaration);
    assert_eq!(
        output.arena.node(parameter.parameter).unwrap().kind,
        AstNodeKind::FunctionParameter
    );
    assert_eq!(
        output.arena.node(parameter.annotation).unwrap().kind,
        AstNodeKind::NamedType
    );

    let malformed = parse_source(
        SourceFileId::from_raw(211),
        "fun bad(signal Signal) { when (signal) { _ -> 0 } }",
    );
    assert!(!malformed.diagnostics.is_empty());
    assert!(malformed.function_parameters.is_empty());
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
        "fun run(): Int { const answer: Int = compute(); var next = answer; next = next + 1; logger.info(next); return next; }",
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
fn m0028_records_executable_binary_operator_metadata() {
    let output = parse_source(
        SourceFileId::from_raw(66),
        "fun run() { a + b - c * d / e % f ** g << h >> i & j ^ k | l; }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());

    let operators: Vec<_> = output
        .binary_expressions
        .iter()
        .map(|expression| expression.operator)
        .collect();
    assert!(operators.contains(&ParsedBinaryOperator::Plus));
    assert!(operators.contains(&ParsedBinaryOperator::Minus));
    assert!(operators.contains(&ParsedBinaryOperator::Star));
    assert!(operators.contains(&ParsedBinaryOperator::Slash));
    assert!(operators.contains(&ParsedBinaryOperator::Percent));
    assert!(operators.contains(&ParsedBinaryOperator::Exponent));
    assert!(operators.contains(&ParsedBinaryOperator::ShiftLeft));
    assert!(operators.contains(&ParsedBinaryOperator::ShiftRight));
    assert!(operators.contains(&ParsedBinaryOperator::BitwiseAnd));
    assert!(operators.contains(&ParsedBinaryOperator::BitwiseXor));
    assert!(operators.contains(&ParsedBinaryOperator::BitwiseOr));
}

#[test]
fn m0028_parses_executable_unary_operators() {
    let output = parse_source(
        SourceFileId::from_raw(67),
        "fun run() { const a = +value; const b = -value; const c = ~value; }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());

    let unary_count = output
        .arena
        .nodes()
        .iter()
        .filter(|node| node.kind == AstNodeKind::UnaryExpression)
        .count();
    assert_eq!(unary_count, 3);
}

#[test]
fn m0028_records_executable_unary_operator_metadata() {
    let output = parse_source(
        SourceFileId::from_raw(69),
        "fun run() { const a = +value; const b = -value; const c = ~value; }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.unary_expressions.len(), 3);
    assert_eq!(
        output
            .unary_expressions
            .iter()
            .map(|expression| expression.operator)
            .collect::<Vec<_>>(),
        vec![
            ParsedUnaryOperator::Plus,
            ParsedUnaryOperator::Minus,
            ParsedUnaryOperator::BitwiseNot,
        ]
    );
    assert!(
        output
            .unary_expressions
            .iter()
            .all(|expression| output.arena.node(expression.operand).is_some())
    );
}

#[test]
fn m0028_parses_exponentiation_right_associatively() {
    let output = parse_source(SourceFileId::from_raw(68), "fun run() { a ** b ** c; }");

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.binary_expressions.len(), 2);

    let outer = output
        .binary_expressions
        .iter()
        .find(|expression| expression.span.start() == 12)
        .expect("outer exponent expression recorded");
    let inner = output
        .binary_expressions
        .iter()
        .find(|expression| expression.expression == outer.right)
        .expect("right operand exponent expression recorded");

    assert_eq!(outer.operator, ParsedBinaryOperator::Exponent);
    assert_eq!(inner.operator, ParsedBinaryOperator::Exponent);
}

#[test]
fn records_simple_identifier_expression_name_references() {
    let file = SourceFileId::from_raw(20);
    let source = "fun run() { const answer = compute(); answer; next + answer; }";
    let output = parse_source(file, source);

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());

    let names: Vec<_> = output
        .name_references
        .iter()
        .map(|reference| reference.name.as_str())
        .collect();
    assert_eq!(names, ["compute", "answer", "next", "answer"]);

    let first = &output.name_references[0];
    assert_eq!(
        &source[first.name_span.start()..first.name_span.end()],
        "compute"
    );
    assert_eq!(
        output.arena.node(first.reference).unwrap().kind,
        AstNodeKind::NameExpression
    );
}

#[test]
fn records_literal_expression_metadata_for_type_checking() {
    let file = SourceFileId::from_raw(27);
    let source = "fun run() { const a = true; const b = false; const c = 42; const d = \"ok\"; const e = null; }";
    let output = parse_source(file, source);

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());

    let literals: Vec<_> = output
        .literal_expressions
        .iter()
        .map(|literal| literal.kind)
        .collect();
    assert_eq!(
        literals,
        [
            ParsedLiteralKind::BoolTrue,
            ParsedLiteralKind::BoolFalse,
            ParsedLiteralKind::AcceptedInteger,
            ParsedLiteralKind::AcceptedString,
            ParsedLiteralKind::Null,
        ]
    );

    let texts: Vec<_> = output
        .literal_expressions
        .iter()
        .map(|literal| &source[literal.span.start()..literal.span.end()])
        .collect();
    assert_eq!(texts, ["true", "false", "42", "\"ok\"", "null"]);

    for literal in &output.literal_expressions {
        assert_eq!(
            output.arena.node(literal.expression).unwrap().kind,
            AstNodeKind::LiteralExpression
        );
    }
}

#[test]
fn literal_expression_metadata_excludes_non_literal_expressions() {
    let output = parse_source(
        SourceFileId::from_raw(28),
        "fun run() { const item = compute(); const other = item; }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert!(output.literal_expressions.is_empty());
}

#[test]
fn records_grouped_expression_metadata_for_type_checking() {
    let output = parse_source(
        SourceFileId::from_raw(31),
        "fun run() { const answer = (42); const nested = ((answer)); }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.grouped_expressions.len(), 3);

    let first = &output.grouped_expressions[0];
    assert_eq!(
        output.arena.node(first.expression).unwrap().kind,
        AstNodeKind::GroupedExpression
    );
    assert_eq!(
        output.arena.node(first.inner).unwrap().kind,
        AstNodeKind::LiteralExpression
    );

    let outer_nested = &output.grouped_expressions[1];
    let inner_nested = &output.grouped_expressions[2];
    assert_eq!(
        output.arena.node(outer_nested.expression).unwrap().kind,
        AstNodeKind::GroupedExpression
    );
    assert_eq!(outer_nested.inner, inner_nested.expression);
    assert_eq!(
        output.arena.node(inner_nested.inner).unwrap().kind,
        AstNodeKind::NameExpression
    );
}

#[test]
fn grouped_expression_metadata_excludes_malformed_groups() {
    let output = parse_source(
        SourceFileId::from_raw(32),
        "fun run() { const broken = (42; const ok = (true); }",
    );

    assert!(
        output
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.kind == DiagnosticKind::UnexpectedTokenInExpression)
    );
    assert_eq!(output.grouped_expressions.len(), 1);
    assert_eq!(
        output
            .arena
            .node(output.grouped_expressions[0].inner)
            .unwrap()
            .kind,
        AstNodeKind::LiteralExpression
    );
}

#[test]
fn name_reference_metadata_excludes_member_import_and_package_names() {
    let output = parse_source(
        SourceFileId::from_raw(21),
        "package demo.core import demo.io fun run() { logger.info(value); }",
    );

    assert!(output.diagnostics.is_empty());
    let names: Vec<_> = output
        .name_references
        .iter()
        .map(|reference| reference.name.as_str())
        .collect();

    assert_eq!(names, ["logger", "value"]);
}

#[test]
fn records_named_type_reference_metadata() {
    let file = SourceFileId::from_raw(22);
    let source = "fun run(): demo.Result<Box<Int?>> { const item: Box<Int> = make(); }";
    let output = parse_source(file, source);

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());

    let names: Vec<_> = output
        .type_name_references
        .iter()
        .map(|reference| reference.name.as_str())
        .collect();
    assert_eq!(names, ["demo.Result", "Box", "Int", "Box", "Int"]);

    let first = &output.type_name_references[0];
    assert_eq!(
        &source[first.name_span.start()..first.name_span.end()],
        "demo.Result"
    );
    assert_eq!(
        output.arena.node(first.reference).unwrap().kind,
        AstNodeKind::NamedType
    );
}

#[test]
fn type_name_reference_metadata_records_grouped_and_function_types_in_order() {
    let file = SourceFileId::from_raw(23);
    let output = parse_source(file, "fun run(): ((Input) -> Output)?;");

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());

    let names: Vec<_> = output
        .type_name_references
        .iter()
        .map(|reference| reference.name.as_str())
        .collect();
    assert_eq!(names, ["Input", "Output"]);
}

#[test]
fn type_name_reference_metadata_excludes_package_import_expression_and_missing_types() {
    let output = parse_source(
        SourceFileId::from_raw(24),
        "package demo.core import demo.io fun run() { value; }",
    );
    let malformed = parse_source(SourceFileId::from_raw(25), "fun broken(): ;");

    let type_names: Vec<_> = output
        .type_name_references
        .iter()
        .map(|reference| reference.name.as_str())
        .collect();
    let expression_names: Vec<_> = output
        .name_references
        .iter()
        .map(|reference| reference.name.as_str())
        .collect();

    assert!(type_names.is_empty());
    assert!(malformed.type_name_references.is_empty());
    assert_eq!(expression_names, ["value"]);
}

#[test]
fn records_local_immutable_and_var_binding_name_metadata() {
    let file = SourceFileId::from_raw(18);
    let source = "fun run() { const answer: Int = compute(); var next = answer; }";
    let output = parse_source(file, source);

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.local_binding_names.len(), 2);

    let first = &output.local_binding_names[0];
    let second = &output.local_binding_names[1];

    assert_eq!(first.kind, LocalBindingKind::Immutable);
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
fn parses_const_declarations_and_val_binding_names() {
    let output = parse_source(
        SourceFileId::from_raw(93),
        "fun run() { const val: Int = 1; var val = 2; }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.local_binding_names.len(), 2);
    assert_eq!(
        output.local_binding_names[0].kind,
        LocalBindingKind::Immutable
    );
    assert_eq!(output.local_binding_names[0].name, "val");
    assert_eq!(output.local_binding_names[1].kind, LocalBindingKind::Var);
    assert_eq!(output.local_binding_names[1].name, "val");
}

#[test]
fn removed_val_introducer_uses_ordinary_recovery_without_a_binding_alias() {
    let source = "fun run() { val removed = 1; var retained = 2; }";
    let output = parse_source(SourceFileId::from_raw(94), source);

    assert!(output.lex_diagnostics.is_empty());
    assert_eq!(
        output
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.kind)
            .collect::<Vec<_>>(),
        vec![DiagnosticKind::UnexpectedTokenInStatement]
    );
    assert_eq!(
        &source[output.diagnostics[0].span.start()..output.diagnostics[0].span.end()],
        "val"
    );
    assert_eq!(output.local_binding_names.len(), 1);
    assert_eq!(output.local_binding_names[0].kind, LocalBindingKind::Var);
    assert_eq!(output.local_binding_names[0].name, "retained");
    assert!(
        output.local_declarations.iter().all(|declaration| {
            declaration.declaration == output.local_binding_names[0].binding
        })
    );
}

#[test]
fn const_is_rejected_in_prior_identifier_positions_by_ordinary_parser_diagnostics() {
    let source = "fun const();";
    let output = parse_source(SourceFileId::from_raw(95), source);

    assert!(output.lex_diagnostics.is_empty());
    assert_eq!(
        output
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.kind)
            .collect::<Vec<_>>(),
        vec![DiagnosticKind::MissingDeclarationName]
    );
    assert_eq!(
        &source[output.diagnostics[0].span.start()..output.diagnostics[0].span.end()],
        "fun"
    );
}

#[test]
fn records_local_declaration_type_and_initializer_metadata() {
    let file = SourceFileId::from_raw(29);
    let source = "fun run() { const answer: Int = 42; var next = answer; const pending: String; }";
    let output = parse_source(file, source);

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.local_declarations.len(), 3);

    let annotated_initialized = &output.local_declarations[0];
    let inferred_initialized = &output.local_declarations[1];
    let annotated_uninitialized = &output.local_declarations[2];

    assert_eq!(
        output
            .arena
            .node(annotated_initialized.declaration)
            .unwrap()
            .kind,
        AstNodeKind::VariableDeclarationStatement
    );
    assert_eq!(
        output
            .arena
            .node(annotated_initialized.annotation.unwrap())
            .unwrap()
            .kind,
        AstNodeKind::NamedType
    );
    assert_eq!(
        output
            .arena
            .node(annotated_initialized.initializer.unwrap())
            .unwrap()
            .kind,
        AstNodeKind::LiteralExpression
    );
    assert_eq!(inferred_initialized.annotation, None);
    assert_eq!(
        output
            .arena
            .node(inferred_initialized.initializer.unwrap())
            .unwrap()
            .kind,
        AstNodeKind::NameExpression
    );
    assert_eq!(annotated_uninitialized.initializer, None);
    assert_eq!(
        output
            .arena
            .node(annotated_uninitialized.annotation.unwrap())
            .unwrap()
            .kind,
        AstNodeKind::NamedType
    );
    assert!(
        annotated_initialized.declaration < inferred_initialized.declaration
            && inferred_initialized.declaration < annotated_uninitialized.declaration
    );
}

#[test]
fn local_declaration_metadata_excludes_malformed_and_other_statements() {
    let output = parse_source(
        SourceFileId::from_raw(30),
        "fun run() { const : Int = value; value; return value; var ok: Int = 1; }",
    );

    assert_eq!(output.local_declarations.len(), 1);
    assert_eq!(
        output
            .arena
            .node(output.local_declarations[0].declaration)
            .unwrap()
            .kind,
        AstNodeKind::VariableDeclarationStatement
    );
    assert!(output.local_declarations[0].annotation.is_some());
    assert!(output.local_declarations[0].initializer.is_some());
}

#[test]
fn local_binding_name_metadata_excludes_malformed_declarations() {
    let output = parse_source(
        SourceFileId::from_raw(19),
        "fun broken() { const : Int = compute(); var ok = value; }",
    );

    assert_eq!(output.local_binding_names.len(), 1);
    assert_eq!(output.local_binding_names[0].kind, LocalBindingKind::Var);
    assert_eq!(output.local_binding_names[0].name, "ok");
}

#[test]
fn records_assignment_statement_metadata_for_type_checking() {
    let output = parse_source(
        SourceFileId::from_raw(33),
        "fun run() { target = value; object.field = 1; }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.assignment_statements.len(), 2);

    let first = &output.assignment_statements[0];
    assert_eq!(
        output.arena.node(first.statement).unwrap().kind,
        AstNodeKind::AssignmentStatement
    );
    assert_eq!(
        output.arena.node(first.target).unwrap().kind,
        AstNodeKind::NameExpression
    );
    assert_eq!(
        output.arena.node(first.value).unwrap().kind,
        AstNodeKind::NameExpression
    );

    let second = &output.assignment_statements[1];
    assert_eq!(
        output.arena.node(second.statement).unwrap().kind,
        AstNodeKind::AssignmentStatement
    );
    assert_eq!(
        output.arena.node(second.target).unwrap().kind,
        AstNodeKind::MemberExpression
    );
    assert_eq!(
        output.arena.node(second.value).unwrap().kind,
        AstNodeKind::LiteralExpression
    );
    assert!(first.statement < second.statement);
}

#[test]
fn assignment_statement_metadata_excludes_malformed_and_non_assignment_statements() {
    let output = parse_source(
        SourceFileId::from_raw(34),
        "fun run() { broken = ; value; const local = 1; ok = true; }",
    );

    assert!(
        output
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.kind == DiagnosticKind::MalformedAssignment)
    );
    assert_eq!(output.assignment_statements.len(), 1);
    let assignment = &output.assignment_statements[0];
    assert_eq!(
        output.arena.node(assignment.statement).unwrap().kind,
        AstNodeKind::AssignmentStatement
    );
    assert_eq!(
        output.arena.node(assignment.target).unwrap().kind,
        AstNodeKind::NameExpression
    );
    assert_eq!(
        output.arena.node(assignment.value).unwrap().kind,
        AstNodeKind::LiteralExpression
    );
}

#[test]
fn parses_trailing_expression_and_if_expression_body() {
    let output = parse_source(
        SourceFileId::from_raw(9),
        "fun choose(): Int { const ready = check(); if (ready) { service.run(arg, 2); } else { fallback(); } }",
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
fn m0019_records_binary_expression_metadata_for_flow_inputs() {
    let output = parse_source(
        SourceFileId::from_raw(90),
        "fun check() { const maybe: String? = null; if (maybe != null) { const definite = maybe; } }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.binary_expressions.len(), 1);

    let binary = &output.binary_expressions[0];
    assert_eq!(binary.operator, ParsedBinaryOperator::NotEqual);
    assert_eq!(
        output.arena.node(binary.expression).unwrap().kind,
        AstNodeKind::BinaryExpression
    );
    assert_eq!(
        output.arena.node(binary.left).unwrap().kind,
        AstNodeKind::NameExpression
    );
    assert_eq!(
        output.arena.node(binary.right).unwrap().kind,
        AstNodeKind::LiteralExpression
    );
    assert!(binary.span.start() < binary.span.end());
}

#[test]
fn m0019_records_if_expression_condition_and_branch_metadata() {
    let output = parse_source(
        SourceFileId::from_raw(91),
        "fun check() { if (null == maybe) { const fallback = \"missing\"; } else { const definite = maybe; } }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.if_expressions.len(), 1);

    let if_expression = &output.if_expressions[0];
    assert_eq!(
        output.arena.node(if_expression.expression).unwrap().kind,
        AstNodeKind::IfExpression
    );
    assert_eq!(
        output.arena.node(if_expression.condition).unwrap().kind,
        AstNodeKind::BinaryExpression
    );
    assert_eq!(
        output.arena.node(if_expression.then_block).unwrap().kind,
        AstNodeKind::Block
    );
    assert_eq!(
        output
            .arena
            .node(if_expression.else_block.unwrap())
            .unwrap()
            .kind,
        AstNodeKind::Block
    );
}

#[test]
fn m0019_records_if_expression_without_else_as_none() {
    let output = parse_source(
        SourceFileId::from_raw(92),
        "fun check() { if (maybe != null) { const definite = maybe; } }",
    );

    assert!(output.lex_diagnostics.is_empty());
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.if_expressions.len(), 1);
    assert_eq!(output.if_expressions[0].else_block, None);
}

#[test]
fn reports_adr0024_body_diagnostics() {
    let output = parse_source(
        SourceFileId::from_raw(10),
        "fun broken() { const : Int = compute(); target = ; return + ; service.(arg,); if ready { nope(); } }",
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
    assert!(
        output
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.span.start() <= diagnostic.span.end())
    );
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
}

#[test]
fn m0024_unspecified_concurrency_forms_remain_blocked() {
    let output = parse_source(
        SourceFileId::from_raw(83),
        "fun concurrent() { async { run(); } while (ready) { run(); } }",
    );

    let kinds: Vec<_> = output
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.kind)
        .collect();

    assert!(kinds.contains(&DiagnosticKind::MalformedCoroutineConstruct));
    assert!(kinds.contains(&DiagnosticKind::UnsupportedStatementForm));
    assert!(output.when_expressions.is_empty());
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
    assert!(
        nested
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.kind == DiagnosticKind::MissingDeclarationName)
    );
}
