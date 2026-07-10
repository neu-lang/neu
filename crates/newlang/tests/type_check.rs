use newlang::{
    ast::AstNodeId,
    ast::AstNodeKind,
    name_resolution::{
        LocalBinding, LocalBindingKey, LocalBindingKind, LocalScopeId, ResolutionTable,
        ResolvedName,
    },
    parser::{
        parse_source, ParsedAssignmentStatement, ParsedGroupedExpression, ParsedLiteralExpression,
        ParsedLiteralKind,
    },
    source::ByteSpan,
    source::SourceFileId,
    symbol::SymbolId,
    type_check::{
        known_local_symbol_types, type_assignment_statements, type_grouped_expressions,
        type_literal_expressions, type_m0018_accepted_expressions, type_m0018_core,
        type_m0018_local_declaration_initializers, type_parser_literals,
        type_primitive_local_declarations, type_primitive_local_initializer_declarations,
        type_resolved_name_expressions, type_unsupported_m0018_expressions, AmbiguousTypeRule,
        AssignmentCheck, DeclarationSignature, ExpressionType, KnownSymbolType,
        LiteralExpressionInput, LiteralKind, RefinedExpressionType, RefinementRecord,
        TypeCheckDiagnostic, TypeCheckDiagnosticKind, TypeCheckReport, TypeRuleDiagnostic,
    },
    types::{NullableType, PrimitiveType, TypeArena, TypeId, TypeKind, TypeRecord},
};

#[test]
fn m0019_flow_diagnostic_constructors_preserve_rule_node_and_types() {
    let node = AstNodeId::from_raw(190);
    let expected = TypeId::from_raw(1);
    let actual = TypeId::from_raw(2);

    let invalid_nullable = TypeCheckDiagnostic::invalid_nullable_use(
        TypeRuleDiagnostic::NullableValueWithoutRefinement,
        node,
        expected,
        actual,
    );
    assert_eq!(
        invalid_nullable.kind(),
        TypeCheckDiagnosticKind::InvalidNullableUse
    );
    assert_eq!(
        invalid_nullable.rule(),
        TypeRuleDiagnostic::NullableValueWithoutRefinement
    );
    assert_eq!(invalid_nullable.node(), node);
    assert_eq!(invalid_nullable.expected_type(), Some(expected));
    assert_eq!(invalid_nullable.actual_type(), Some(actual));

    let invalidated = TypeCheckDiagnostic::invalidated_refinement(
        TypeRuleDiagnostic::RegionExitInvalidatedRefinement,
        node,
        expected,
        actual,
    );
    assert_eq!(
        invalidated.kind(),
        TypeCheckDiagnosticKind::InvalidatedRefinement
    );

    let unsupported = TypeCheckDiagnostic::unsupported_flow_rule(
        TypeRuleDiagnostic::BooleanCombinationRefinementDeferred,
        node,
    );
    assert_eq!(
        unsupported.kind(),
        TypeCheckDiagnosticKind::UnsupportedFlowRule
    );
    assert_eq!(unsupported.expected_type(), None);
    assert_eq!(unsupported.actual_type(), None);

    let ambiguous = TypeCheckDiagnostic::ambiguous_flow_rule(
        TypeRuleDiagnostic::AmbiguousLocalBindingFlow,
        node,
    );
    assert_eq!(ambiguous.kind(), TypeCheckDiagnosticKind::AmbiguousFlowRule);
}

#[test]
fn m0019_flow_rule_diagnostic_identifiers_cover_adr0028_examples() {
    let rules = [
        TypeRuleDiagnostic::NullableValueWithoutRefinement,
        TypeRuleDiagnostic::NullableAssignmentWithoutRefinement,
        TypeRuleDiagnostic::AssignmentInvalidatedRefinement,
        TypeRuleDiagnostic::RegionExitInvalidatedRefinement,
        TypeRuleDiagnostic::MutableLocalRefinementDeferred,
        TypeRuleDiagnostic::BooleanCombinationRefinementDeferred,
        TypeRuleDiagnostic::MemberRefinementDeferred,
        TypeRuleDiagnostic::CallResultRefinementDeferred,
        TypeRuleDiagnostic::ExclusiveBorrowRefinementDeferred,
        TypeRuleDiagnostic::AmbiguousLocalBindingFlow,
        TypeRuleDiagnostic::AmbiguousNullTestRegion,
    ];

    assert_eq!(rules.len(), 11);
}

#[test]
fn m0019_type_check_report_records_flow_refinements_in_insertion_order() {
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(1), SymbolId::from_raw(2)),
        AstNodeId::from_raw(191),
        LocalBindingKind::Val,
    );
    let first = RefinementRecord::new(
        AstNodeId::from_raw(192),
        AstNodeId::from_raw(193),
        AstNodeId::from_raw(194),
        binding.clone(),
        TypeId::from_raw(10),
        TypeId::from_raw(11),
    );
    let second = RefinementRecord::new(
        AstNodeId::from_raw(195),
        AstNodeId::from_raw(196),
        AstNodeId::from_raw(197),
        binding,
        TypeId::from_raw(12),
        TypeId::from_raw(13),
    );
    let mut report = TypeCheckReport::new();

    report.record_refinement(first.clone());
    report.record_refinement(second.clone());

    assert_eq!(report.refinements(), &[first.clone(), second]);
    assert_eq!(report.refinement(AstNodeId::from_raw(192)), Some(&first));
    assert_eq!(first.originating_null_test(), AstNodeId::from_raw(194));
    assert_eq!(first.original_nullable_type(), TypeId::from_raw(10));
    assert_eq!(first.refined_non_null_type(), TypeId::from_raw(11));
}

#[test]
fn m0019_type_check_report_records_refined_expression_types_as_per_use_views() {
    let first = RefinedExpressionType::new(
        AstNodeId::from_raw(200),
        AstNodeId::from_raw(201),
        TypeId::from_raw(20),
        TypeId::from_raw(21),
    );
    let second = RefinedExpressionType::new(
        AstNodeId::from_raw(202),
        AstNodeId::from_raw(203),
        TypeId::from_raw(22),
        TypeId::from_raw(23),
    );
    let mut report = TypeCheckReport::new();

    report.record_refined_expression_type(first);
    report.record_refined_expression_type(second);

    assert_eq!(report.refined_expression_types(), &[first, second]);
    assert_eq!(
        report.refined_expression_type(AstNodeId::from_raw(200)),
        Some(TypeId::from_raw(21))
    );
    assert_eq!(first.original_nullable_type(), TypeId::from_raw(20));
    assert_eq!(first.refined_non_null_type(), TypeId::from_raw(21));
}

#[test]
fn ambiguous_type_rule_diagnostic_preserves_rule_and_node() {
    let node = AstNodeId::from_raw(17);

    let diagnostic =
        TypeCheckDiagnostic::ambiguous_type_rule(AmbiguousTypeRule::LiteralTyping, node);

    assert_eq!(
        diagnostic.kind(),
        TypeCheckDiagnosticKind::AmbiguousTypeRule
    );
    assert_eq!(diagnostic.rule(), AmbiguousTypeRule::LiteralTyping);
    assert_eq!(diagnostic.node(), node);
}

#[test]
fn ambiguous_type_rules_cover_m0018_blockers() {
    let blockers = [
        AmbiguousTypeRule::LiteralTyping,
        AmbiguousTypeRule::PrimitiveScalarCatalog,
        AmbiguousTypeRule::AssignmentCompatibility,
        AmbiguousTypeRule::CallResolution,
        AmbiguousTypeRule::FunctionTypeApplication,
    ];

    assert_eq!(blockers.len(), 5);
}

#[test]
fn unresolved_type_rule_diagnostic_preserves_rule_and_node_without_types() {
    let node = AstNodeId::from_raw(18);

    let diagnostic =
        TypeCheckDiagnostic::unresolved_type_rule(TypeRuleDiagnostic::MissingAnnotationType, node);

    assert_eq!(
        diagnostic.kind(),
        TypeCheckDiagnosticKind::UnresolvedTypeRule
    );
    assert_eq!(diagnostic.rule(), TypeRuleDiagnostic::MissingAnnotationType);
    assert_eq!(diagnostic.node(), node);
    assert_eq!(diagnostic.expected_type(), None);
    assert_eq!(diagnostic.actual_type(), None);
}

#[test]
fn unsupported_type_rule_diagnostic_preserves_rule_and_node_without_types() {
    let node = AstNodeId::from_raw(19);

    let diagnostic =
        TypeCheckDiagnostic::unsupported_type_rule(TypeRuleDiagnostic::DirectCallDeferred, node);

    assert_eq!(
        diagnostic.kind(),
        TypeCheckDiagnosticKind::UnsupportedTypeRule
    );
    assert_eq!(diagnostic.rule(), TypeRuleDiagnostic::DirectCallDeferred);
    assert_eq!(diagnostic.node(), node);
    assert_eq!(diagnostic.expected_type(), None);
    assert_eq!(diagnostic.actual_type(), None);
}

#[test]
fn type_rule_diagnostic_identifiers_cover_adr0027_examples() {
    let rules = [
        TypeRuleDiagnostic::MissingAnnotationType,
        TypeRuleDiagnostic::MissingResolvedNameType,
        TypeRuleDiagnostic::DirectCallDeferred,
        TypeRuleDiagnostic::FunctionTypeApplicationDeferred,
        TypeRuleDiagnostic::MemberExpressionDeferred,
        TypeRuleDiagnostic::BinaryExpressionDeferred,
        TypeRuleDiagnostic::UnaryExpressionDeferred,
        TypeRuleDiagnostic::IfValueDeferred,
    ];

    assert_eq!(rules.len(), 8);
}

#[test]
fn unsupported_expression_diagnostics_report_adr0027_deferred_forms() {
    let parsed = parse_source(
        SourceFileId::from_raw(85),
        "fun run() { val answer: Int = compute(); next = next + 1; logger.info(next); if (ready) { val inner = next; } else { val other = answer; } }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let report = type_unsupported_m0018_expressions(&parsed.arena);

    assert_eq!(report.expression_types(), &[]);
    assert_eq!(report.declaration_signatures(), &[]);
    assert_eq!(report.assignment_checks(), &[]);
    assert_eq!(report.diagnostics().len(), 5);

    let diagnostics: Vec<_> = report
        .diagnostics()
        .iter()
        .map(|diagnostic| (diagnostic.kind(), diagnostic.rule()))
        .collect();
    assert_eq!(
        diagnostics,
        [
            (
                TypeCheckDiagnosticKind::UnsupportedTypeRule,
                TypeRuleDiagnostic::DirectCallDeferred,
            ),
            (
                TypeCheckDiagnosticKind::UnsupportedTypeRule,
                TypeRuleDiagnostic::BinaryExpressionDeferred,
            ),
            (
                TypeCheckDiagnosticKind::UnsupportedTypeRule,
                TypeRuleDiagnostic::MemberExpressionDeferred,
            ),
            (
                TypeCheckDiagnosticKind::UnsupportedTypeRule,
                TypeRuleDiagnostic::DirectCallDeferred,
            ),
            (
                TypeCheckDiagnosticKind::UnsupportedTypeRule,
                TypeRuleDiagnostic::IfValueDeferred,
            ),
        ]
    );

    for diagnostic in report.diagnostics() {
        assert_eq!(diagnostic.expected_type(), None);
        assert_eq!(diagnostic.actual_type(), None);
    }
}

#[test]
fn unsupported_expression_diagnostics_ignore_accepted_and_non_expression_nodes() {
    let parsed = parse_source(
        SourceFileId::from_raw(86),
        "fun run() { val source: Int = 1; val copy: Int = (source); copy = source; return; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());
    assert!(parsed
        .arena
        .nodes()
        .iter()
        .any(|node| node.kind == AstNodeKind::ReturnStatement));

    let report = type_unsupported_m0018_expressions(&parsed.arena);

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.expression_types(), &[]);
    assert_eq!(report.declaration_signatures(), &[]);
    assert_eq!(report.assignment_checks(), &[]);
}

#[test]
fn unsupported_expression_diagnostics_report_unary_expression_nodes() {
    let file = SourceFileId::from_raw(87);
    let mut arena = newlang::ast::AstArena::new();
    arena.add_source_file(ByteSpan::new(file, 0, 1).unwrap());
    let unary = arena.add_unary_expression(ByteSpan::new(file, 2, 6).unwrap());

    let report = type_unsupported_m0018_expressions(&arena);

    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::UnsupportedTypeRule
    );
    assert_eq!(
        report.diagnostics()[0].rule(),
        TypeRuleDiagnostic::UnaryExpressionDeferred
    );
    assert_eq!(report.diagnostics()[0].node(), unary);
}

#[test]
fn type_check_report_records_blockers_without_successful_output() {
    let diagnostic = TypeCheckDiagnostic::ambiguous_type_rule(
        AmbiguousTypeRule::CallResolution,
        AstNodeId::from_raw(4),
    );
    let report = TypeCheckReport::blocked(vec![diagnostic.clone()]);

    assert!(report.is_blocked());
    assert_eq!(report.diagnostics(), &[diagnostic]);
}

#[test]
fn type_check_report_records_expression_types_in_insertion_order() {
    let mut report = TypeCheckReport::new();
    let first = ExpressionType::new(AstNodeId::from_raw(10), TypeId::from_raw(1));
    let second = ExpressionType::new(AstNodeId::from_raw(11), TypeId::from_raw(2));

    report.record_expression_type(first);
    report.record_expression_type(second);

    assert!(!report.is_blocked());
    assert_eq!(report.expression_types(), &[first, second]);
    assert_eq!(
        report.expression_type(AstNodeId::from_raw(10)),
        Some(TypeId::from_raw(1))
    );
    assert_eq!(
        report.expression_type(AstNodeId::from_raw(99)),
        None,
        "reports must not synthesize missing expression type entries"
    );
}

#[test]
fn type_check_report_records_declaration_signatures_by_node() {
    let mut report = TypeCheckReport::new();
    let signature = DeclarationSignature::new(AstNodeId::from_raw(20), TypeId::from_raw(3));

    report.record_declaration_signature(signature);

    assert_eq!(report.declaration_signatures(), &[signature]);
    assert_eq!(
        report.declaration_signature(AstNodeId::from_raw(20)),
        Some(TypeId::from_raw(3))
    );
    assert_eq!(report.declaration_signature(AstNodeId::from_raw(21)), None);
}

#[test]
fn type_check_report_records_assignment_checks_by_statement_node() {
    let mut report = TypeCheckReport::new();
    let accepted = AssignmentCheck::new(
        AstNodeId::from_raw(30),
        TypeId::from_raw(4),
        TypeId::from_raw(4),
    );

    report.record_assignment_check(accepted);

    assert_eq!(report.assignment_checks(), &[accepted]);
    assert_eq!(
        report.assignment_check(AstNodeId::from_raw(30)),
        Some(accepted)
    );
    assert_eq!(report.assignment_check(AstNodeId::from_raw(31)), None);
}

#[test]
fn literal_expression_typing_records_adr0027_primitive_types() {
    let inputs = [
        LiteralExpressionInput::new(AstNodeId::from_raw(40), LiteralKind::BoolTrue),
        LiteralExpressionInput::new(AstNodeId::from_raw(41), LiteralKind::BoolFalse),
        LiteralExpressionInput::new(AstNodeId::from_raw(42), LiteralKind::AcceptedInteger),
        LiteralExpressionInput::new(AstNodeId::from_raw(43), LiteralKind::AcceptedString),
        LiteralExpressionInput::new(AstNodeId::from_raw(44), LiteralKind::Null),
    ];

    let (arena, report) = type_literal_expressions(&inputs);

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.declaration_signatures(), &[]);
    assert_eq!(report.assignment_checks(), &[]);
    assert_eq!(
        report.expression_types(),
        &[
            ExpressionType::new(AstNodeId::from_raw(40), TypeId::from_raw(0)),
            ExpressionType::new(AstNodeId::from_raw(41), TypeId::from_raw(0)),
            ExpressionType::new(AstNodeId::from_raw(42), TypeId::from_raw(1)),
            ExpressionType::new(AstNodeId::from_raw(43), TypeId::from_raw(2)),
            ExpressionType::new(AstNodeId::from_raw(44), TypeId::from_raw(4)),
        ]
    );

    assert_eq!(arena.records().len(), 5);
    assert_eq!(
        arena.get(TypeId::from_raw(0)).unwrap().kind(),
        &TypeKind::Primitive(PrimitiveType::Bool)
    );
    assert_eq!(
        arena.get(TypeId::from_raw(1)).unwrap().kind(),
        &TypeKind::Primitive(PrimitiveType::Int)
    );
    assert_eq!(
        arena.get(TypeId::from_raw(2)).unwrap().kind(),
        &TypeKind::Primitive(PrimitiveType::String)
    );
    assert_eq!(
        arena.get(TypeId::from_raw(3)).unwrap().kind(),
        &TypeKind::Primitive(PrimitiveType::Unit)
    );
    assert_eq!(
        arena.get(TypeId::from_raw(4)).unwrap().kind(),
        &TypeKind::Primitive(PrimitiveType::Null)
    );
}

#[test]
fn literal_expression_typing_does_not_synthesize_missing_expression_types() {
    let (_arena, report) = type_literal_expressions(&[LiteralExpressionInput::new(
        AstNodeId::from_raw(50),
        LiteralKind::AcceptedString,
    )]);

    assert_eq!(
        report.expression_type(AstNodeId::from_raw(50)),
        Some(TypeId::from_raw(2))
    );
    assert_eq!(report.expression_type(AstNodeId::from_raw(51)), None);
}

#[test]
fn parser_literal_metadata_types_to_adr0027_primitives() {
    let parsed = parse_source(
        SourceFileId::from_raw(60),
        "fun run() { val a = true; val b = 7; val c = \"text\"; val d = null; }",
    );

    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let (arena, report) = type_parser_literals(&parsed.literal_expressions);

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.expression_types().len(), 4);
    assert_eq!(arena.records().len(), 5);

    let literal_nodes: Vec<_> = parsed
        .literal_expressions
        .iter()
        .map(|literal| literal.expression)
        .collect();
    assert_eq!(
        report.expression_type(literal_nodes[0]),
        Some(TypeId::from_raw(0))
    );
    assert_eq!(
        report.expression_type(literal_nodes[1]),
        Some(TypeId::from_raw(1))
    );
    assert_eq!(
        report.expression_type(literal_nodes[2]),
        Some(TypeId::from_raw(2))
    );
    assert_eq!(
        report.expression_type(literal_nodes[3]),
        Some(TypeId::from_raw(4))
    );
    assert_eq!(report.expression_type(AstNodeId::from_raw(999)), None);
}

#[test]
fn primitive_local_declaration_annotations_record_declaration_signatures() {
    let parsed = parse_source(
        SourceFileId::from_raw(61),
        "fun run() { val ready: Bool = true; val count: Int = 1; val label: String = \"x\"; val done: Unit; val absent: Null = null; }",
    );

    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let (arena, report) =
        type_primitive_local_declarations(&parsed.local_declarations, &parsed.type_name_references);

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.expression_types(), &[]);
    assert_eq!(report.assignment_checks(), &[]);
    assert_eq!(arena.records().len(), 5);

    let declarations: Vec<_> = parsed
        .local_declarations
        .iter()
        .map(|declaration| declaration.declaration)
        .collect();
    assert_eq!(
        report.declaration_signatures(),
        &[
            DeclarationSignature::new(declarations[0], TypeId::from_raw(0)),
            DeclarationSignature::new(declarations[1], TypeId::from_raw(1)),
            DeclarationSignature::new(declarations[2], TypeId::from_raw(2)),
            DeclarationSignature::new(declarations[3], TypeId::from_raw(3)),
            DeclarationSignature::new(declarations[4], TypeId::from_raw(4)),
        ]
    );
    assert_eq!(
        arena.get(TypeId::from_raw(0)).unwrap().kind(),
        &TypeKind::Primitive(PrimitiveType::Bool)
    );
    assert_eq!(
        arena.get(TypeId::from_raw(4)).unwrap().kind(),
        &TypeKind::Primitive(PrimitiveType::Null)
    );
}

#[test]
fn primitive_local_declaration_annotations_do_not_synthesize_unknown_signatures() {
    let parsed = parse_source(
        SourceFileId::from_raw(62),
        "fun run() { val inferred = true; val custom: UserId = value; val count: Int = 1; }",
    );

    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let (_arena, report) =
        type_primitive_local_declarations(&parsed.local_declarations, &parsed.type_name_references);

    let inferred = parsed.local_declarations[0].declaration;
    let custom = parsed.local_declarations[1].declaration;
    let count = parsed.local_declarations[2].declaration;

    assert_eq!(report.declaration_signature(inferred), None);
    assert_eq!(report.declaration_signature(custom), None);
    assert_eq!(
        report.declaration_signature(count),
        Some(TypeId::from_raw(1))
    );
    assert_eq!(report.expression_types(), &[]);
    assert_eq!(report.assignment_checks(), &[]);
    assert_eq!(report.diagnostics().len(), 2);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::UnresolvedTypeRule
    );
    assert_eq!(
        report.diagnostics()[0].rule(),
        TypeRuleDiagnostic::MissingAnnotationType
    );
    assert_eq!(report.diagnostics()[0].node(), inferred);
    assert_eq!(
        report.diagnostics()[1].kind(),
        TypeCheckDiagnosticKind::UnresolvedTypeRule
    );
    assert_eq!(
        report.diagnostics()[1].rule(),
        TypeRuleDiagnostic::MissingAnnotationType
    );
    assert_eq!(report.diagnostics()[1].node(), custom);
}

#[test]
fn primitive_local_initializer_checks_record_matching_assignments() {
    let parsed = parse_source(
        SourceFileId::from_raw(63),
        "fun run() { val ready: Bool = true; val count: Int = 1; val label: String = \"x\"; }",
    );

    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let (arena, report) = type_primitive_local_initializer_declarations(
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
    );

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(arena.records().len(), 5);
    assert_eq!(report.declaration_signatures().len(), 3);
    assert_eq!(report.expression_types().len(), 3);

    let declarations: Vec<_> = parsed
        .local_declarations
        .iter()
        .map(|declaration| declaration.declaration)
        .collect();
    assert_eq!(
        report.assignment_checks(),
        &[
            AssignmentCheck::new(declarations[0], TypeId::from_raw(0), TypeId::from_raw(0)),
            AssignmentCheck::new(declarations[1], TypeId::from_raw(1), TypeId::from_raw(1)),
            AssignmentCheck::new(declarations[2], TypeId::from_raw(2), TypeId::from_raw(2)),
        ]
    );
}

#[test]
fn primitive_local_initializer_checks_diagnose_mismatched_literals() {
    let parsed = parse_source(
        SourceFileId::from_raw(64),
        "fun run() { val ready: Bool = 1; val count: Int = 2; val custom: UserId = 3; val later: String = compute(); }",
    );

    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let (_arena, report) = type_primitive_local_initializer_declarations(
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
    );

    let ready_initializer = parsed.local_declarations[0].initializer.unwrap();
    let count_declaration = parsed.local_declarations[1].declaration;
    let custom_declaration = parsed.local_declarations[2].declaration;
    let later_declaration = parsed.local_declarations[3].declaration;

    assert_eq!(report.diagnostics().len(), 2);
    let diagnostic = &report.diagnostics()[0];
    assert_eq!(diagnostic.kind(), TypeCheckDiagnosticKind::TypeMismatch);
    assert_eq!(diagnostic.node(), ready_initializer);
    assert_eq!(diagnostic.expected_type(), Some(TypeId::from_raw(0)));
    assert_eq!(diagnostic.actual_type(), Some(TypeId::from_raw(1)));
    assert_eq!(
        report.diagnostics()[1].kind(),
        TypeCheckDiagnosticKind::UnresolvedTypeRule
    );
    assert_eq!(
        report.diagnostics()[1].rule(),
        TypeRuleDiagnostic::MissingAnnotationType
    );
    assert_eq!(report.diagnostics()[1].node(), custom_declaration);

    assert_eq!(
        report.assignment_check(count_declaration),
        Some(AssignmentCheck::new(
            count_declaration,
            TypeId::from_raw(1),
            TypeId::from_raw(1)
        ))
    );
    assert_eq!(report.assignment_check(custom_declaration), None);
    assert_eq!(report.assignment_check(later_declaration), None);
}

#[test]
fn resolved_name_expression_typing_records_known_symbol_types_in_resolution_order() {
    let mut resolutions = ResolutionTable::new();
    resolutions.insert(ResolvedName::new(
        AstNodeId::from_raw(70),
        SymbolId::from_raw(1),
    ));
    resolutions.insert(ResolvedName::new(
        AstNodeId::from_raw(71),
        SymbolId::from_raw(2),
    ));
    resolutions.insert(ResolvedName::new(
        AstNodeId::from_raw(72),
        SymbolId::from_raw(1),
    ));

    let known = [
        KnownSymbolType::new(SymbolId::from_raw(1), TypeId::from_raw(1)),
        KnownSymbolType::new(SymbolId::from_raw(2), TypeId::from_raw(0)),
    ];
    let report = type_resolved_name_expressions(&resolutions, &known);

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.declaration_signatures(), &[]);
    assert_eq!(report.assignment_checks(), &[]);
    assert_eq!(
        report.expression_types(),
        &[
            ExpressionType::new(AstNodeId::from_raw(70), TypeId::from_raw(1)),
            ExpressionType::new(AstNodeId::from_raw(71), TypeId::from_raw(0)),
            ExpressionType::new(AstNodeId::from_raw(72), TypeId::from_raw(1)),
        ]
    );
}

#[test]
fn resolved_name_expression_typing_reports_unknown_symbol_types() {
    let mut resolutions = ResolutionTable::new();
    resolutions.insert(ResolvedName::new(
        AstNodeId::from_raw(80),
        SymbolId::from_raw(10),
    ));
    resolutions.insert(ResolvedName::new(
        AstNodeId::from_raw(81),
        SymbolId::from_raw(11),
    ));

    let known = [KnownSymbolType::new(
        SymbolId::from_raw(11),
        TypeId::from_raw(2),
    )];
    let report = type_resolved_name_expressions(&resolutions, &known);

    assert_eq!(report.expression_type(AstNodeId::from_raw(80)), None);
    assert_eq!(
        report.expression_type(AstNodeId::from_raw(81)),
        Some(TypeId::from_raw(2))
    );
    assert_eq!(
        report.expression_type(AstNodeId::from_raw(82)),
        None,
        "missing resolution entries must not synthesize expression types"
    );
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::UnresolvedTypeRule
    );
    assert_eq!(
        report.diagnostics()[0].rule(),
        TypeRuleDiagnostic::MissingResolvedNameType
    );
    assert_eq!(report.diagnostics()[0].node(), AstNodeId::from_raw(80));
}

#[test]
fn known_local_symbol_types_are_derived_from_declaration_signatures() {
    let scope = LocalScopeId::from_raw(1);
    let first_symbol = SymbolId::from_raw(20);
    let second_symbol = SymbolId::from_raw(21);
    let bindings = [
        LocalBinding::new(
            LocalBindingKey::new(scope, first_symbol),
            AstNodeId::from_raw(90),
            LocalBindingKind::Val,
        ),
        LocalBinding::new(
            LocalBindingKey::new(scope, second_symbol),
            AstNodeId::from_raw(91),
            LocalBindingKind::Var,
        ),
    ];
    let signatures = [
        DeclarationSignature::new(AstNodeId::from_raw(91), TypeId::from_raw(2)),
        DeclarationSignature::new(AstNodeId::from_raw(90), TypeId::from_raw(1)),
    ];

    let known = known_local_symbol_types(&bindings, &signatures);

    assert_eq!(
        known,
        [
            KnownSymbolType::new(first_symbol, TypeId::from_raw(1)),
            KnownSymbolType::new(second_symbol, TypeId::from_raw(2)),
        ]
    );
}

#[test]
fn known_local_symbol_types_skip_unsignatured_bindings_and_orphan_signatures() {
    let scope = LocalScopeId::from_raw(2);
    let typed_symbol = SymbolId::from_raw(30);
    let untyped_symbol = SymbolId::from_raw(31);
    let bindings = [
        LocalBinding::new(
            LocalBindingKey::new(scope, typed_symbol),
            AstNodeId::from_raw(100),
            LocalBindingKind::Val,
        ),
        LocalBinding::new(
            LocalBindingKey::new(scope, untyped_symbol),
            AstNodeId::from_raw(101),
            LocalBindingKind::Val,
        ),
    ];
    let signatures = [
        DeclarationSignature::new(AstNodeId::from_raw(100), TypeId::from_raw(0)),
        DeclarationSignature::new(AstNodeId::from_raw(999), TypeId::from_raw(4)),
    ];

    let known = known_local_symbol_types(&bindings, &signatures);

    assert_eq!(
        known,
        [KnownSymbolType::new(typed_symbol, TypeId::from_raw(0))]
    );
}

#[test]
fn grouped_expression_typing_propagates_inner_expression_types() {
    let grouped = [
        ParsedGroupedExpression {
            expression: AstNodeId::from_raw(110),
            inner: AstNodeId::from_raw(111),
            span: newlang::source::ByteSpan::new(SourceFileId::from_raw(70), 0, 4).unwrap(),
        },
        ParsedGroupedExpression {
            expression: AstNodeId::from_raw(112),
            inner: AstNodeId::from_raw(113),
            span: newlang::source::ByteSpan::new(SourceFileId::from_raw(70), 5, 9).unwrap(),
        },
    ];
    let known = [
        ExpressionType::new(AstNodeId::from_raw(113), TypeId::from_raw(2)),
        ExpressionType::new(AstNodeId::from_raw(111), TypeId::from_raw(1)),
    ];

    let report = type_grouped_expressions(&grouped, &known);

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.declaration_signatures(), &[]);
    assert_eq!(report.assignment_checks(), &[]);
    assert_eq!(
        report.expression_types(),
        &[
            ExpressionType::new(AstNodeId::from_raw(110), TypeId::from_raw(1)),
            ExpressionType::new(AstNodeId::from_raw(112), TypeId::from_raw(2)),
        ]
    );
}

#[test]
fn grouped_expression_typing_skips_unknown_inner_expressions() {
    let grouped = [
        ParsedGroupedExpression {
            expression: AstNodeId::from_raw(120),
            inner: AstNodeId::from_raw(121),
            span: newlang::source::ByteSpan::new(SourceFileId::from_raw(71), 0, 4).unwrap(),
        },
        ParsedGroupedExpression {
            expression: AstNodeId::from_raw(122),
            inner: AstNodeId::from_raw(123),
            span: newlang::source::ByteSpan::new(SourceFileId::from_raw(71), 5, 9).unwrap(),
        },
    ];
    let known = [ExpressionType::new(
        AstNodeId::from_raw(123),
        TypeId::from_raw(0),
    )];

    let report = type_grouped_expressions(&grouped, &known);

    assert_eq!(
        report.expression_types(),
        &[ExpressionType::new(
            AstNodeId::from_raw(122),
            TypeId::from_raw(0)
        )]
    );
    assert_eq!(report.expression_type(AstNodeId::from_raw(120)), None);
    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.assignment_checks(), &[]);
}

#[test]
fn grouped_expression_typing_supports_already_typed_nested_groups() {
    let inner_group = AstNodeId::from_raw(130);
    let outer_group = AstNodeId::from_raw(131);
    let literal = AstNodeId::from_raw(132);
    let grouped = [
        ParsedGroupedExpression {
            expression: inner_group,
            inner: literal,
            span: newlang::source::ByteSpan::new(SourceFileId::from_raw(72), 1, 5).unwrap(),
        },
        ParsedGroupedExpression {
            expression: outer_group,
            inner: inner_group,
            span: newlang::source::ByteSpan::new(SourceFileId::from_raw(72), 0, 6).unwrap(),
        },
    ];
    let known = [
        ExpressionType::new(literal, TypeId::from_raw(4)),
        ExpressionType::new(inner_group, TypeId::from_raw(4)),
    ];

    let report = type_grouped_expressions(&grouped, &known);

    assert_eq!(
        report.expression_types(),
        &[
            ExpressionType::new(inner_group, TypeId::from_raw(4)),
            ExpressionType::new(outer_group, TypeId::from_raw(4)),
        ]
    );
}

#[test]
fn assignment_statement_type_checking_records_exact_matches() {
    let mut arena = TypeArena::new();
    let bool_id = arena.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let int_id = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let assignments = [
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(140),
            target: AstNodeId::from_raw(141),
            value: AstNodeId::from_raw(142),
        },
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(143),
            target: AstNodeId::from_raw(144),
            value: AstNodeId::from_raw(145),
        },
    ];
    let expression_types = [
        ExpressionType::new(AstNodeId::from_raw(141), bool_id),
        ExpressionType::new(AstNodeId::from_raw(142), bool_id),
        ExpressionType::new(AstNodeId::from_raw(144), int_id),
        ExpressionType::new(AstNodeId::from_raw(145), int_id),
    ];

    let report = type_assignment_statements(&assignments, &expression_types, &arena);

    assert_eq!(report.expression_types(), &[]);
    assert_eq!(report.declaration_signatures(), &[]);
    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(
        report.assignment_checks(),
        &[
            AssignmentCheck::new(AstNodeId::from_raw(140), bool_id, bool_id),
            AssignmentCheck::new(AstNodeId::from_raw(143), int_id, int_id),
        ]
    );
}

#[test]
fn assignment_statement_type_checking_reports_mismatches_on_values() {
    let mut arena = TypeArena::new();
    let bool_id = arena.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let int_id = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let assignments = [ParsedAssignmentStatement {
        statement: AstNodeId::from_raw(150),
        target: AstNodeId::from_raw(151),
        value: AstNodeId::from_raw(152),
    }];
    let expression_types = [
        ExpressionType::new(AstNodeId::from_raw(151), bool_id),
        ExpressionType::new(AstNodeId::from_raw(152), int_id),
    ];

    let report = type_assignment_statements(&assignments, &expression_types, &arena);

    assert_eq!(report.assignment_checks(), &[]);
    assert_eq!(report.diagnostics().len(), 1);
    let diagnostic = &report.diagnostics()[0];
    assert_eq!(diagnostic.kind(), TypeCheckDiagnosticKind::TypeMismatch);
    assert_eq!(diagnostic.node(), AstNodeId::from_raw(152));
    assert_eq!(diagnostic.expected_type(), Some(bool_id));
    assert_eq!(diagnostic.actual_type(), Some(int_id));
}

#[test]
fn assignment_statement_type_checking_skips_unknown_sides() {
    let mut arena = TypeArena::new();
    let bool_id = arena.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let assignments = [
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(160),
            target: AstNodeId::from_raw(161),
            value: AstNodeId::from_raw(162),
        },
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(163),
            target: AstNodeId::from_raw(164),
            value: AstNodeId::from_raw(165),
        },
    ];
    let expression_types = [
        ExpressionType::new(AstNodeId::from_raw(161), bool_id),
        ExpressionType::new(AstNodeId::from_raw(165), bool_id),
    ];

    let report = type_assignment_statements(&assignments, &expression_types, &arena);

    assert_eq!(report.assignment_checks(), &[]);
    assert_eq!(report.diagnostics(), &[]);
}

#[test]
fn assignment_statement_type_checking_accepts_nullable_exceptions() {
    let mut arena = TypeArena::new();
    let int_id = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let null_id = arena.insert(TypeRecord::primitive(PrimitiveType::Null));
    let nullable_int_id = arena.insert(TypeRecord::nullable(NullableType::new(int_id)));
    let assignments = [
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(170),
            target: AstNodeId::from_raw(171),
            value: AstNodeId::from_raw(172),
        },
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(173),
            target: AstNodeId::from_raw(174),
            value: AstNodeId::from_raw(175),
        },
    ];
    let expression_types = [
        ExpressionType::new(AstNodeId::from_raw(171), nullable_int_id),
        ExpressionType::new(AstNodeId::from_raw(172), null_id),
        ExpressionType::new(AstNodeId::from_raw(174), nullable_int_id),
        ExpressionType::new(AstNodeId::from_raw(175), int_id),
    ];

    let report = type_assignment_statements(&assignments, &expression_types, &arena);

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(
        report.assignment_checks(),
        &[
            AssignmentCheck::new(AstNodeId::from_raw(170), nullable_int_id, null_id),
            AssignmentCheck::new(AstNodeId::from_raw(173), nullable_int_id, int_id),
        ]
    );
}

#[test]
fn assignment_statement_type_checking_rejects_null_for_non_nullable_targets() {
    let mut arena = TypeArena::new();
    let int_id = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let null_id = arena.insert(TypeRecord::primitive(PrimitiveType::Null));
    let assignments = [ParsedAssignmentStatement {
        statement: AstNodeId::from_raw(180),
        target: AstNodeId::from_raw(181),
        value: AstNodeId::from_raw(182),
    }];
    let expression_types = [
        ExpressionType::new(AstNodeId::from_raw(181), int_id),
        ExpressionType::new(AstNodeId::from_raw(182), null_id),
    ];

    let report = type_assignment_statements(&assignments, &expression_types, &arena);

    assert_eq!(report.assignment_checks(), &[]);
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(report.diagnostics()[0].node(), AstNodeId::from_raw(182));
    assert_eq!(report.diagnostics()[0].expected_type(), Some(int_id));
    assert_eq!(report.diagnostics()[0].actual_type(), Some(null_id));
}

#[test]
fn accepted_expression_composition_records_literals_names_and_groups() {
    let literal = AstNodeId::from_raw(190);
    let name = AstNodeId::from_raw(191);
    let group = AstNodeId::from_raw(192);
    let literals = [ParsedLiteralExpression {
        expression: literal,
        kind: ParsedLiteralKind::AcceptedInteger,
        span: newlang::source::ByteSpan::new(SourceFileId::from_raw(80), 0, 2).unwrap(),
    }];
    let grouped = [ParsedGroupedExpression {
        expression: group,
        inner: name,
        span: newlang::source::ByteSpan::new(SourceFileId::from_raw(80), 3, 9).unwrap(),
    }];
    let mut resolutions = ResolutionTable::new();
    resolutions.insert(ResolvedName::new(name, SymbolId::from_raw(40)));
    let known = [KnownSymbolType::new(
        SymbolId::from_raw(40),
        TypeId::from_raw(0),
    )];

    let (arena, report) =
        type_m0018_accepted_expressions(&literals, &grouped, &resolutions, &known);

    assert_eq!(arena.records().len(), 5);
    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.declaration_signatures(), &[]);
    assert_eq!(report.assignment_checks(), &[]);
    assert_eq!(report.expression_type(literal), Some(TypeId::from_raw(1)));
    assert_eq!(report.expression_type(name), Some(TypeId::from_raw(0)));
    assert_eq!(report.expression_type(group), Some(TypeId::from_raw(0)));
}

#[test]
fn accepted_expression_composition_types_nested_groups_when_inner_becomes_known() {
    let literal = AstNodeId::from_raw(200);
    let inner_group = AstNodeId::from_raw(201);
    let outer_group = AstNodeId::from_raw(202);
    let literals = [ParsedLiteralExpression {
        expression: literal,
        kind: ParsedLiteralKind::AcceptedString,
        span: newlang::source::ByteSpan::new(SourceFileId::from_raw(81), 2, 8).unwrap(),
    }];
    let grouped = [
        ParsedGroupedExpression {
            expression: outer_group,
            inner: inner_group,
            span: newlang::source::ByteSpan::new(SourceFileId::from_raw(81), 0, 10).unwrap(),
        },
        ParsedGroupedExpression {
            expression: inner_group,
            inner: literal,
            span: newlang::source::ByteSpan::new(SourceFileId::from_raw(81), 1, 9).unwrap(),
        },
    ];
    let resolutions = ResolutionTable::new();

    let (_arena, report) = type_m0018_accepted_expressions(&literals, &grouped, &resolutions, &[]);

    assert_eq!(report.expression_type(literal), Some(TypeId::from_raw(2)));
    assert_eq!(
        report.expression_type(inner_group),
        Some(TypeId::from_raw(2))
    );
    assert_eq!(
        report.expression_type(outer_group),
        Some(TypeId::from_raw(2))
    );
}

#[test]
fn accepted_expression_composition_reports_unknown_resolved_name_types() {
    let unknown_name = AstNodeId::from_raw(210);
    let unknown_group = AstNodeId::from_raw(211);
    let grouped = [ParsedGroupedExpression {
        expression: unknown_group,
        inner: unknown_name,
        span: newlang::source::ByteSpan::new(SourceFileId::from_raw(82), 0, 8).unwrap(),
    }];
    let mut resolutions = ResolutionTable::new();
    resolutions.insert(ResolvedName::new(unknown_name, SymbolId::from_raw(41)));

    let (_arena, report) = type_m0018_accepted_expressions(&[], &grouped, &resolutions, &[]);

    assert_eq!(report.expression_type(unknown_name), None);
    assert_eq!(report.expression_type(unknown_group), None);
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::UnresolvedTypeRule
    );
    assert_eq!(
        report.diagnostics()[0].rule(),
        TypeRuleDiagnostic::MissingResolvedNameType
    );
    assert_eq!(report.diagnostics()[0].node(), unknown_name);
    assert_eq!(report.declaration_signatures(), &[]);
    assert_eq!(report.assignment_checks(), &[]);
}

#[test]
fn accepted_local_initializer_checks_names_and_grouped_names() {
    let parsed = parse_source(
        SourceFileId::from_raw(83),
        "fun run() { val source: Int = 1; val copy: Int = source; val grouped: Int = (source); }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let source_symbol = SymbolId::from_raw(50);
    let mut resolutions = ResolutionTable::new();
    for reference in &parsed.name_references {
        resolutions.insert(ResolvedName::new(reference.reference, source_symbol));
    }
    let known = [KnownSymbolType::new(source_symbol, TypeId::from_raw(1))];

    let (_arena, report) = type_m0018_local_declaration_initializers(
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.grouped_expressions,
        &resolutions,
        &known,
    );

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.declaration_signatures().len(), 3);
    let declarations: Vec<_> = parsed
        .local_declarations
        .iter()
        .map(|declaration| declaration.declaration)
        .collect();
    assert_eq!(
        report.assignment_checks(),
        &[
            AssignmentCheck::new(declarations[0], TypeId::from_raw(1), TypeId::from_raw(1)),
            AssignmentCheck::new(declarations[1], TypeId::from_raw(1), TypeId::from_raw(1)),
            AssignmentCheck::new(declarations[2], TypeId::from_raw(1), TypeId::from_raw(1)),
        ]
    );
}

#[test]
fn accepted_local_initializer_checks_diagnose_mismatched_accepted_initializers() {
    let parsed = parse_source(
        SourceFileId::from_raw(84),
        "fun run() { val source: Int = 1; val bad: String = source; val unknown: UserId = source; val skipped: Int = missing; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let source_symbol = SymbolId::from_raw(51);
    let mut resolutions = ResolutionTable::new();
    for reference in &parsed.name_references {
        if reference.name == "source" {
            resolutions.insert(ResolvedName::new(reference.reference, source_symbol));
        }
    }
    let known = [KnownSymbolType::new(source_symbol, TypeId::from_raw(1))];

    let (_arena, report) = type_m0018_local_declaration_initializers(
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.grouped_expressions,
        &resolutions,
        &known,
    );

    let bad_initializer = parsed.local_declarations[1].initializer.unwrap();
    let unknown_declaration = parsed.local_declarations[2].declaration;
    let skipped_declaration = parsed.local_declarations[3].declaration;

    assert_eq!(report.diagnostics().len(), 2);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::TypeMismatch
    );
    assert_eq!(report.diagnostics()[0].node(), bad_initializer);
    assert_eq!(
        report.diagnostics()[1].kind(),
        TypeCheckDiagnosticKind::UnresolvedTypeRule
    );
    assert_eq!(
        report.diagnostics()[1].rule(),
        TypeRuleDiagnostic::MissingAnnotationType
    );
    assert_eq!(report.diagnostics()[1].node(), unknown_declaration);
    assert_eq!(
        report.diagnostics()[0].expected_type(),
        Some(TypeId::from_raw(2))
    );
    assert_eq!(
        report.diagnostics()[0].actual_type(),
        Some(TypeId::from_raw(1))
    );
    assert_eq!(report.assignment_check(unknown_declaration), None);
    assert_eq!(report.assignment_check(skipped_declaration), None);
}

#[test]
fn m0018_core_types_well_typed_accepted_fixture() {
    let parsed = parse_source(
        SourceFileId::from_raw(88),
        "fun run() { val source: Int = 1; val copy: Int = source; val grouped: Int = (copy); var next: Int = grouped; next = source; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let source_symbol = SymbolId::from_raw(60);
    let copy_symbol = SymbolId::from_raw(61);
    let grouped_symbol = SymbolId::from_raw(62);
    let next_symbol = SymbolId::from_raw(63);
    let mut resolutions = ResolutionTable::new();
    for reference in &parsed.name_references {
        let symbol = match reference.name.as_str() {
            "source" => source_symbol,
            "copy" => copy_symbol,
            "grouped" => grouped_symbol,
            "next" => next_symbol,
            other => panic!("unexpected reference {other}"),
        };
        resolutions.insert(ResolvedName::new(reference.reference, symbol));
    }
    let bindings = [
        LocalBinding::new(
            LocalBindingKey::new(LocalScopeId::from_raw(0), source_symbol),
            parsed.local_declarations[0].declaration,
            LocalBindingKind::Val,
        ),
        LocalBinding::new(
            LocalBindingKey::new(LocalScopeId::from_raw(0), copy_symbol),
            parsed.local_declarations[1].declaration,
            LocalBindingKind::Val,
        ),
        LocalBinding::new(
            LocalBindingKey::new(LocalScopeId::from_raw(0), grouped_symbol),
            parsed.local_declarations[2].declaration,
            LocalBindingKind::Val,
        ),
        LocalBinding::new(
            LocalBindingKey::new(LocalScopeId::from_raw(0), next_symbol),
            parsed.local_declarations[3].declaration,
            LocalBindingKind::Var,
        ),
    ];

    let (_arena, report) = type_m0018_core(
        &parsed.arena,
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.grouped_expressions,
        &parsed.assignment_statements,
        &resolutions,
        &bindings,
    );

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.declaration_signatures().len(), 4);
    assert_eq!(report.assignment_checks().len(), 5);
    for declaration in &parsed.local_declarations {
        assert_eq!(
            report.declaration_signature(declaration.declaration),
            Some(TypeId::from_raw(1))
        );
        assert!(report.assignment_check(declaration.declaration).is_some());
    }
    assert!(report
        .assignment_check(parsed.assignment_statements[0].statement)
        .is_some());
}

#[test]
fn m0018_core_reports_mismatch_unresolved_and_unsupported_diagnostics() {
    let parsed = parse_source(
        SourceFileId::from_raw(89),
        "fun run() { val source: Int = 1; val bad: String = source; val unknown: UserId = source; val missingKnown: Int = external; logger.info(source); }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let source_symbol = SymbolId::from_raw(70);
    let external_symbol = SymbolId::from_raw(71);
    let mut resolutions = ResolutionTable::new();
    for reference in &parsed.name_references {
        let symbol = match reference.name.as_str() {
            "source" => source_symbol,
            "external" => external_symbol,
            "logger" => external_symbol,
            "info" => external_symbol,
            other => panic!("unexpected reference {other}"),
        };
        resolutions.insert(ResolvedName::new(reference.reference, symbol));
    }
    let bindings = [LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), source_symbol),
        parsed.local_declarations[0].declaration,
        LocalBindingKind::Val,
    )];

    let (_arena, report) = type_m0018_core(
        &parsed.arena,
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.grouped_expressions,
        &parsed.assignment_statements,
        &resolutions,
        &bindings,
    );

    assert!(report
        .diagnostics()
        .iter()
        .any(
            |diagnostic| diagnostic.kind() == TypeCheckDiagnosticKind::TypeMismatch
                && diagnostic.node() == parsed.local_declarations[1].initializer.unwrap()
                && diagnostic.expected_type() == Some(TypeId::from_raw(2))
                && diagnostic.actual_type() == Some(TypeId::from_raw(1))
        ));
    assert!(report
        .diagnostics()
        .iter()
        .any(
            |diagnostic| diagnostic.kind() == TypeCheckDiagnosticKind::UnresolvedTypeRule
                && diagnostic.rule() == TypeRuleDiagnostic::MissingAnnotationType
                && diagnostic.node() == parsed.local_declarations[2].declaration
        ));
    assert!(report
        .diagnostics()
        .iter()
        .any(
            |diagnostic| diagnostic.kind() == TypeCheckDiagnosticKind::UnresolvedTypeRule
                && diagnostic.rule() == TypeRuleDiagnostic::MissingResolvedNameType
        ));
    assert!(report
        .diagnostics()
        .iter()
        .any(
            |diagnostic| diagnostic.kind() == TypeCheckDiagnosticKind::UnsupportedTypeRule
                && diagnostic.rule() == TypeRuleDiagnostic::MemberExpressionDeferred
        ));
    assert!(report
        .diagnostics()
        .iter()
        .any(
            |diagnostic| diagnostic.kind() == TypeCheckDiagnosticKind::UnsupportedTypeRule
                && diagnostic.rule() == TypeRuleDiagnostic::DirectCallDeferred
        ));
    assert_eq!(
        report.assignment_check(parsed.local_declarations[2].declaration),
        None
    );
    assert_eq!(
        report.assignment_check(parsed.local_declarations[3].declaration),
        None
    );
}
