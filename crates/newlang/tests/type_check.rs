use newlang::{
    ast::AstNodeId,
    name_resolution::{
        LocalBinding, LocalBindingKey, LocalBindingKind, LocalScopeId, ResolutionTable,
        ResolvedName,
    },
    parser::parse_source,
    source::SourceFileId,
    symbol::SymbolId,
    type_check::{
        known_local_symbol_types, type_literal_expressions, type_parser_literals,
        type_primitive_local_declarations, type_primitive_local_initializer_declarations,
        type_resolved_name_expressions, AmbiguousTypeRule, AssignmentCheck, DeclarationSignature,
        ExpressionType, KnownSymbolType, LiteralExpressionInput, LiteralKind, TypeCheckDiagnostic,
        TypeCheckDiagnosticKind, TypeCheckReport,
    },
    types::{PrimitiveType, TypeId, TypeKind},
};

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

    assert_eq!(report.diagnostics().len(), 1);
    let diagnostic = &report.diagnostics()[0];
    assert_eq!(diagnostic.kind(), TypeCheckDiagnosticKind::TypeMismatch);
    assert_eq!(diagnostic.node(), ready_initializer);
    assert_eq!(diagnostic.expected_type(), Some(TypeId::from_raw(0)));
    assert_eq!(diagnostic.actual_type(), Some(TypeId::from_raw(1)));

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
fn resolved_name_expression_typing_skips_unknown_symbols() {
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
