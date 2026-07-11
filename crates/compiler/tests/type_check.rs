use compiler::{
    ast::{AstArena, AstNodeId, AstNodeKind},
    module::PackageNamespace,
    name_resolution::{
        LocalBinding, LocalBindingKey, LocalBindingKind, LocalScopeId, ResolutionTable,
        ResolvedLocalBinding, ResolvedName, bind_local_name_references, build_local_scope_tree,
        build_scoped_local_binding_index,
    },
    parser::{
        ParsedAssignmentStatement, ParsedBinaryOperator, ParsedGroupedExpression,
        ParsedIfExpression, ParsedLiteralExpression, ParsedLiteralKind, ParsedLocalDeclaration,
        parse_source,
    },
    source::ByteSpan,
    source::SourceFileId,
    symbol::{SymbolId, SymbolInterner},
    type_check::{
        AmbiguousTypeRule, AssignmentCheck, DeclarationSignature, DirectCallDiagnosticKind,
        EligibleNullTestRefinement, EntryPointDiagnosticKind, EntryPointFile,
        ExecutableSourceTypes, ExpressionType, KnownSymbolType, LiteralExpressionInput,
        LiteralKind, NullTestRefinedBranch, RecognizedNullTest, RefinedExpressionType,
        RefinementRecord, ReturnPathDiagnosticKind, TypeCheckDiagnostic, TypeCheckDiagnosticKind,
        TypeCheckReport, TypeRuleDiagnostic, apply_m0028_direct_call_results,
        build_m0020_capability_bound_records, build_m0020_generic_parameter_types,
        check_m0028_direct_calls, check_m0028_entry_point, check_m0028_straight_line_returns,
        known_local_symbol_types, recognize_m0019_null_tests, record_m0019_branch_refinements,
        record_m0019_refined_expression_types, select_m0019_eligible_null_tests,
        type_assignment_statements, type_grouped_expressions, type_literal_expressions,
        type_m0018_accepted_expressions, type_m0018_core,
        type_m0018_local_declaration_initializers, type_m0019_assignment_statements,
        type_m0019_local_declaration_initializers, type_m0019_region_exit_refinement_invalidations,
        type_m0028_executable_core, type_m0028_executable_core_in,
        type_m0028_executable_int_operators, type_m0028_function_signatures,
        type_m0028_function_signatures_in, type_m0028_static_integer_diagnostics,
        type_parser_literals, type_primitive_local_declarations,
        type_primitive_local_initializer_declarations, type_resolved_name_expressions,
        type_unsupported_m0018_expressions,
    },
    types::{NullableType, PrimitiveType, TypeArena, TypeId, TypeKind, TypeRecord},
};

#[test]
fn m0019_mutation_invalidation_classifies_only_exact_post_region_bare_name_initializer() {
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nullable_int = types.insert(TypeRecord::nullable(NullableType::new(int)));
    let file = SourceFileId::from_raw(430);
    let mut ast = AstArena::new();
    ast.add_source_file(ByteSpan::new(file, 0, 380).unwrap());
    let _enclosing_block = ast.add_block(ByteSpan::new(file, 0, 380).unwrap());
    let then_block = ast.add_block(ByteSpan::new(file, 40, 100).unwrap());
    let else_block = ast.add_block(ByteSpan::new(file, 110, 150).unwrap());
    let _nested_block = ast.add_block(ByteSpan::new(file, 50, 90).unwrap());
    let second_then_block = ast.add_block(ByteSpan::new(file, 200, 240).unwrap());
    let before = ast.add_name_expression(ByteSpan::new(file, 20, 25).unwrap());
    let condition = ast.add_name_expression(ByteSpan::new(file, 30, 35).unwrap());
    let guarded_descendant = ast.add_name_expression(ByteSpan::new(file, 60, 65).unwrap());
    let shadowed = ast.add_name_expression(ByteSpan::new(file, 70, 75).unwrap());
    let sibling_else = ast.add_name_expression(ByteSpan::new(file, 120, 125).unwrap());
    let after = ast.add_name_expression(ByteSpan::new(file, 170, 175).unwrap());
    let second_condition = ast.add_name_expression(ByteSpan::new(file, 190, 195).unwrap());
    let second_after = ast.add_name_expression(ByteSpan::new(file, 270, 275).unwrap());
    let unrefined = ast.add_name_expression(ByteSpan::new(file, 290, 295).unwrap());
    let grouped_name = ast.add_name_expression(ByteSpan::new(file, 320, 325).unwrap());
    let grouped = ast.add_grouped_expression(ByteSpan::new(file, 318, 327).unwrap());
    let if_expression = ast.add_if_expression(ByteSpan::new(file, 30, 150).unwrap());
    let second_if_expression = ast.add_if_expression(ByteSpan::new(file, 190, 260).unwrap());
    let annotation = ast.add_named_type(ByteSpan::new(file, 330, 333).unwrap());
    let declarations = [
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(431),
            annotation: Some(annotation),
            initializer: Some(before),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(432),
            annotation: Some(annotation),
            initializer: Some(guarded_descendant),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(433),
            annotation: Some(annotation),
            initializer: Some(shadowed),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(434),
            annotation: Some(annotation),
            initializer: Some(sibling_else),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(435),
            annotation: Some(annotation),
            initializer: Some(after),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(436),
            annotation: Some(annotation),
            initializer: Some(second_after),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(441),
            annotation: Some(annotation),
            initializer: Some(unrefined),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(439),
            annotation: Some(annotation),
            initializer: Some(grouped),
        },
    ];
    let signatures = declarations
        .each_ref()
        .map(|declaration| DeclarationSignature::new(declaration.declaration, int));
    let expression_types = [
        ExpressionType::new(before, nullable_int),
        ExpressionType::new(guarded_descendant, nullable_int),
        ExpressionType::new(shadowed, nullable_int),
        ExpressionType::new(sibling_else, nullable_int),
        ExpressionType::new(after, nullable_int),
        ExpressionType::new(second_after, nullable_int),
        ExpressionType::new(unrefined, nullable_int),
        ExpressionType::new(grouped_name, nullable_int),
        ExpressionType::new(grouped, nullable_int),
    ];
    let outer_binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(430)),
        AstNodeId::from_raw(437),
        LocalBindingKind::Immutable,
    );
    let shadow_binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(1), SymbolId::from_raw(430)),
        AstNodeId::from_raw(438),
        LocalBindingKind::Immutable,
    );
    let unrefined_binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(431)),
        AstNodeId::from_raw(440),
        LocalBindingKind::Immutable,
    );
    let mut flow = TypeCheckReport::new();
    flow.record_refinement(RefinementRecord::new(
        then_block,
        condition,
        condition,
        outer_binding.clone(),
        nullable_int,
        int,
    ));
    flow.record_refinement(RefinementRecord::new(
        second_then_block,
        second_condition,
        second_condition,
        outer_binding.clone(),
        nullable_int,
        int,
    ));
    flow.record_refined_expression_type(RefinedExpressionType::new(
        guarded_descendant,
        then_block,
        nullable_int,
        int,
    ));
    let resolved = [
        ResolvedLocalBinding::new(before, outer_binding.clone()),
        ResolvedLocalBinding::new(guarded_descendant, outer_binding.clone()),
        ResolvedLocalBinding::new(shadowed, shadow_binding),
        ResolvedLocalBinding::new(sibling_else, outer_binding.clone()),
        ResolvedLocalBinding::new(after, outer_binding.clone()),
        ResolvedLocalBinding::new(second_after, outer_binding.clone()),
        ResolvedLocalBinding::new(unrefined, unrefined_binding),
        ResolvedLocalBinding::new(grouped_name, outer_binding),
    ];
    let if_expressions = [
        ParsedIfExpression {
            expression: if_expression,
            condition,
            then_block,
            else_block: Some(else_block),
            span: ByteSpan::new(file, 30, 150).unwrap(),
        },
        ParsedIfExpression {
            expression: second_if_expression,
            condition: second_condition,
            then_block: second_then_block,
            else_block: None,
            span: ByteSpan::new(file, 190, 260).unwrap(),
        },
    ];

    let report = type_m0019_region_exit_refinement_invalidations(
        &declarations,
        &signatures,
        &expression_types,
        &flow,
        &resolved,
        &if_expressions,
        &ast,
        &types,
    );

    assert_eq!(
        report.assignment_checks(),
        &[AssignmentCheck::new(declarations[1].declaration, int, int)]
    );
    assert_eq!(report.diagnostics().len(), 7);
    assert_eq!(
        report.diagnostics()[3],
        TypeCheckDiagnostic::invalidated_refinement(
            TypeRuleDiagnostic::RegionExitInvalidatedRefinement,
            after,
            int,
            nullable_int,
        )
    );
    // The diagnostic is represented only by the later bare-name node: ADR-0031 has no secondary span.
    assert_eq!(report.diagnostics()[3].node(), after);
    assert_eq!(report.diagnostics()[3].actual_type(), Some(nullable_int));
    assert_eq!(
        ast.node(report.diagnostics()[3].node()).unwrap().span,
        ByteSpan::new(file, 170, 175).unwrap()
    );
    assert!(report.refined_expression_types().is_empty());
    assert_eq!(report.refined_expression_type(after), None);
    assert_eq!(
        report.diagnostics()[4],
        TypeCheckDiagnostic::invalidated_refinement(
            TypeRuleDiagnostic::RegionExitInvalidatedRefinement,
            second_after,
            int,
            nullable_int,
        )
    );
    assert_eq!(report.diagnostics()[4].node(), second_after);
    assert_eq!(report.diagnostics()[4].actual_type(), Some(nullable_int));
    assert_eq!(
        ast.node(report.diagnostics()[4].node()).unwrap().span,
        ByteSpan::new(file, 270, 275).unwrap()
    );
    assert_eq!(report.refined_expression_type(second_after), None);
    assert!(report.diagnostics()[..3].iter().all(|diagnostic| {
        diagnostic.kind() == TypeCheckDiagnosticKind::InvalidNullableUse
            && diagnostic.rule() == TypeRuleDiagnostic::NullableAssignmentWithoutRefinement
    }));
    assert_eq!(
        report.diagnostics()[..3]
            .iter()
            .map(TypeCheckDiagnostic::node)
            .collect::<Vec<_>>(),
        vec![before, shadowed, sibling_else]
    );
    assert_eq!(
        report.diagnostics()[5].kind(),
        TypeCheckDiagnosticKind::InvalidNullableUse
    );
    assert_eq!(
        report.diagnostics()[5].rule(),
        TypeRuleDiagnostic::NullableAssignmentWithoutRefinement
    );
    assert_eq!(report.diagnostics()[5].node(), unrefined);
    assert_eq!(
        report.diagnostics()[6].kind(),
        TypeCheckDiagnosticKind::TypeMismatch
    );
    assert_eq!(report.diagnostics()[6].node(), grouped);
}

#[test]
fn m0020_generic_parameter_types_preserve_parameter_identity_and_source_order() {
    let parsed = parse_source(
        SourceFileId::from_raw(500),
        "struct Pair<T: Send & Share, U> {} struct Other<T> {}",
    );
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.generic_parameters.len(), 3);

    let mut symbols = SymbolInterner::new();
    let mut types = TypeArena::new();
    let records =
        build_m0020_generic_parameter_types(&parsed.generic_parameters, &mut symbols, &mut types);

    assert_eq!(records.len(), 3);
    assert_eq!(types.records().len(), 3);
    assert_eq!(
        records[0].parameter(),
        parsed.generic_parameters[0].parameter
    );
    assert_eq!(
        records[1].parameter(),
        parsed.generic_parameters[1].parameter
    );
    assert_eq!(
        records[2].parameter(),
        parsed.generic_parameters[2].parameter
    );
    assert_ne!(records[0].ty(), records[2].ty());

    let first = match types.get(records[0].ty()).unwrap().kind() {
        TypeKind::GenericParameter(parameter) => parameter,
        other => panic!("expected generic parameter type, got {other:?}"),
    };
    let third = match types.get(records[2].ty()).unwrap().kind() {
        TypeKind::GenericParameter(parameter) => parameter,
        other => panic!("expected generic parameter type, got {other:?}"),
    };
    assert_eq!(first.declaration(), parsed.generic_parameters[0].parameter);
    assert_eq!(third.declaration(), parsed.generic_parameters[2].parameter);
    assert_eq!(first.symbol(), third.symbol());
    assert_eq!(symbols.resolve(first.symbol()), Some("T"));
    assert_eq!(symbols.resolve(SymbolId::from_raw(1)), Some("U"));

    let empty = build_m0020_generic_parameter_types(&[], &mut symbols, &mut types);
    assert!(empty.is_empty());
    assert_eq!(types.records().len(), 3);
    assert_eq!(symbols.symbols(), &["T".to_owned(), "U".to_owned()]);
}

#[test]
fn m0020_capability_bound_records_preserve_occurrences_without_interpretation() {
    let parsed = parse_source(
        SourceFileId::from_raw(501),
        "struct Pair<T: Send & Share, U: Send> {}",
    );
    assert!(parsed.diagnostics.is_empty());

    let mut symbols = SymbolInterner::new();
    let mut types = TypeArena::new();
    let parameter_types =
        build_m0020_generic_parameter_types(&parsed.generic_parameters, &mut symbols, &mut types);
    let bounds = build_m0020_capability_bound_records(
        &parsed.generic_parameters,
        &parameter_types,
        &mut symbols,
    );

    assert_eq!(bounds.len(), 3);
    assert_eq!(
        bounds[0].parameter(),
        parsed.generic_parameters[0].parameter
    );
    assert_eq!(bounds[0].ty(), parameter_types[0].ty());
    assert_eq!(
        bounds[0].bound(),
        parsed.generic_parameters[0].capability_bounds[0].bound
    );
    assert_eq!(symbols.resolve(bounds[0].symbol()), Some("Send"));
    assert_eq!(symbols.resolve(bounds[1].symbol()), Some("Share"));
    assert_eq!(
        bounds[2].parameter(),
        parsed.generic_parameters[1].parameter
    );
    assert_eq!(bounds[2].ty(), parameter_types[1].ty());
    assert_eq!(bounds[2].symbol(), bounds[0].symbol());

    let missing =
        build_m0020_capability_bound_records(&parsed.generic_parameters, &[], &mut symbols);
    assert!(missing.is_empty());
}

#[test]
fn m0019_null_test_recognition_accepts_direct_not_equal_forms() {
    let parsed = parse_source(
        SourceFileId::from_raw(190),
        "fun check() { if (maybe != null) { const definite = maybe; }; if (null != other) { const also = other; } }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let recognized = recognize_m0019_null_tests(
        &parsed.binary_expressions,
        &parsed.literal_expressions,
        &parsed.arena,
    );

    assert_eq!(recognized.len(), 2);
    assert_eq!(recognized[0].refined_branch(), NullTestRefinedBranch::Then);
    assert_eq!(recognized[1].refined_branch(), NullTestRefinedBranch::Then);
    assert_eq!(recognized[0].operator(), ParsedBinaryOperator::NotEqual);
    assert_eq!(
        parsed
            .arena
            .node(recognized[0].name_expression())
            .unwrap()
            .kind,
        AstNodeKind::NameExpression
    );
    assert_eq!(
        parsed
            .arena
            .node(recognized[0].null_literal())
            .unwrap()
            .kind,
        AstNodeKind::LiteralExpression
    );
}

#[test]
fn m0019_null_test_recognition_accepts_direct_equal_forms_as_else_refinements() {
    let parsed = parse_source(
        SourceFileId::from_raw(191),
        "fun check() { if (maybe == null) { const fallback = \"missing\"; } else { const definite = maybe; }; if (null == other) { const fallback2 = \"missing\"; } else { const also = other; } }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let recognized = recognize_m0019_null_tests(
        &parsed.binary_expressions,
        &parsed.literal_expressions,
        &parsed.arena,
    );

    assert_eq!(recognized.len(), 2);
    assert!(
        recognized
            .iter()
            .all(|test| test.refined_branch() == NullTestRefinedBranch::Else)
    );
    assert!(
        recognized
            .iter()
            .all(|test| test.operator() == ParsedBinaryOperator::Equal)
    );
}

#[test]
fn m0019_null_test_recognition_ignores_unsupported_condition_shapes() {
    let parsed = parse_source(
        SourceFileId::from_raw(192),
        "fun check() { if (left == right) { const a = left; }; if (null == null) { const b = null; }; if (maybe < null) { const c = maybe; }; if (maybe == 1) { const d = maybe; } }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let recognized = recognize_m0019_null_tests(
        &parsed.binary_expressions,
        &parsed.literal_expressions,
        &parsed.arena,
    );

    assert!(recognized.is_empty());
}

#[test]
fn m0019_null_test_eligibility_accepts_immutable_nullable_local() {
    let null_test = RecognizedNullTest::new(
        AstNodeId::from_raw(200),
        AstNodeId::from_raw(201),
        AstNodeId::from_raw(202),
        ParsedBinaryOperator::NotEqual,
        NullTestRefinedBranch::Then,
    );
    let symbol = SymbolId::from_raw(200);
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), symbol),
        AstNodeId::from_raw(203),
        LocalBindingKind::Immutable,
    );
    let mut resolutions = ResolutionTable::new();
    resolutions.insert(ResolvedName::new(null_test.name_expression(), symbol));
    let mut types = TypeArena::new();
    let string = types.insert(TypeRecord::primitive(PrimitiveType::String));
    let nullable_string = types.insert(TypeRecord::nullable(NullableType::new(string)));
    let signatures = [DeclarationSignature::new(
        binding.binding(),
        nullable_string,
    )];

    let (eligible, report) = select_m0019_eligible_null_tests(
        &[null_test],
        &resolutions,
        std::slice::from_ref(&binding),
        &signatures,
        &types,
    );

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(
        eligible,
        vec![EligibleNullTestRefinement::new(
            null_test,
            binding,
            nullable_string,
            string
        )]
    );
    assert_eq!(eligible[0].null_test(), null_test);
    assert_eq!(eligible[0].original_nullable_type(), nullable_string);
    assert_eq!(eligible[0].refined_non_null_type(), string);
}

#[test]
fn m0019_null_test_eligibility_rejects_mutable_local_with_flow_diagnostic() {
    let null_test = RecognizedNullTest::new(
        AstNodeId::from_raw(210),
        AstNodeId::from_raw(211),
        AstNodeId::from_raw(212),
        ParsedBinaryOperator::Equal,
        NullTestRefinedBranch::Else,
    );
    let symbol = SymbolId::from_raw(210);
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), symbol),
        AstNodeId::from_raw(213),
        LocalBindingKind::Var,
    );
    let mut resolutions = ResolutionTable::new();
    resolutions.insert(ResolvedName::new(null_test.name_expression(), symbol));
    let mut types = TypeArena::new();
    let string = types.insert(TypeRecord::primitive(PrimitiveType::String));
    let nullable_string = types.insert(TypeRecord::nullable(NullableType::new(string)));
    let signatures = [DeclarationSignature::new(
        binding.binding(),
        nullable_string,
    )];

    let (eligible, report) = select_m0019_eligible_null_tests(
        &[null_test],
        &resolutions,
        &[binding],
        &signatures,
        &types,
    );

    assert!(eligible.is_empty());
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::UnsupportedFlowRule
    );
    assert_eq!(
        report.diagnostics()[0].rule(),
        TypeRuleDiagnostic::MutableLocalRefinementDeferred
    );
    assert_eq!(report.diagnostics()[0].node(), null_test.name_expression());
}

#[test]
fn m0019_null_test_eligibility_ignores_non_nullable_and_incomplete_inputs() {
    let non_nullable_test = RecognizedNullTest::new(
        AstNodeId::from_raw(220),
        AstNodeId::from_raw(221),
        AstNodeId::from_raw(222),
        ParsedBinaryOperator::NotEqual,
        NullTestRefinedBranch::Then,
    );
    let unresolved_test = RecognizedNullTest::new(
        AstNodeId::from_raw(223),
        AstNodeId::from_raw(224),
        AstNodeId::from_raw(225),
        ParsedBinaryOperator::NotEqual,
        NullTestRefinedBranch::Then,
    );
    let missing_signature_test = RecognizedNullTest::new(
        AstNodeId::from_raw(226),
        AstNodeId::from_raw(227),
        AstNodeId::from_raw(228),
        ParsedBinaryOperator::NotEqual,
        NullTestRefinedBranch::Then,
    );
    let non_nullable_symbol = SymbolId::from_raw(220);
    let missing_signature_symbol = SymbolId::from_raw(221);
    let non_nullable_binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), non_nullable_symbol),
        AstNodeId::from_raw(229),
        LocalBindingKind::Immutable,
    );
    let missing_signature_binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), missing_signature_symbol),
        AstNodeId::from_raw(230),
        LocalBindingKind::Immutable,
    );
    let mut resolutions = ResolutionTable::new();
    resolutions.insert(ResolvedName::new(
        non_nullable_test.name_expression(),
        non_nullable_symbol,
    ));
    resolutions.insert(ResolvedName::new(
        missing_signature_test.name_expression(),
        missing_signature_symbol,
    ));
    let mut types = TypeArena::new();
    let string = types.insert(TypeRecord::primitive(PrimitiveType::String));
    let signatures = [DeclarationSignature::new(
        non_nullable_binding.binding(),
        string,
    )];

    let (eligible, report) = select_m0019_eligible_null_tests(
        &[non_nullable_test, unresolved_test, missing_signature_test],
        &resolutions,
        &[non_nullable_binding, missing_signature_binding],
        &signatures,
        &types,
    );

    assert!(eligible.is_empty());
    assert_eq!(report.diagnostics(), &[]);
}

#[test]
fn m0019_null_test_eligibility_reports_ambiguous_local_binding_match() {
    let null_test = RecognizedNullTest::new(
        AstNodeId::from_raw(240),
        AstNodeId::from_raw(241),
        AstNodeId::from_raw(242),
        ParsedBinaryOperator::NotEqual,
        NullTestRefinedBranch::Then,
    );
    let symbol = SymbolId::from_raw(240);
    let first = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), symbol),
        AstNodeId::from_raw(243),
        LocalBindingKind::Immutable,
    );
    let second = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(1), symbol),
        AstNodeId::from_raw(244),
        LocalBindingKind::Immutable,
    );
    let mut resolutions = ResolutionTable::new();
    resolutions.insert(ResolvedName::new(null_test.name_expression(), symbol));
    let types = TypeArena::new();

    let (eligible, report) =
        select_m0019_eligible_null_tests(&[null_test], &resolutions, &[first, second], &[], &types);

    assert!(eligible.is_empty());
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::AmbiguousFlowRule
    );
    assert_eq!(
        report.diagnostics()[0].rule(),
        TypeRuleDiagnostic::AmbiguousLocalBindingFlow
    );
    assert_eq!(report.diagnostics()[0].node(), null_test.name_expression());
}

#[test]
fn m0019_branch_refinement_records_then_branch_for_not_equal_tests() {
    let null_test = RecognizedNullTest::new(
        AstNodeId::from_raw(300),
        AstNodeId::from_raw(301),
        AstNodeId::from_raw(302),
        ParsedBinaryOperator::NotEqual,
        NullTestRefinedBranch::Then,
    );
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(300)),
        AstNodeId::from_raw(303),
        LocalBindingKind::Immutable,
    );
    let eligible = EligibleNullTestRefinement::new(
        null_test,
        binding.clone(),
        TypeId::from_raw(10),
        TypeId::from_raw(11),
    );
    let if_expression = ParsedIfExpression {
        expression: AstNodeId::from_raw(304),
        condition: null_test.expression(),
        then_block: AstNodeId::from_raw(305),
        else_block: Some(AstNodeId::from_raw(306)),
        span: ByteSpan::new(SourceFileId::from_raw(300), 0, 10).unwrap(),
    };
    let then_block = if_expression.then_block;

    let report = record_m0019_branch_refinements(&[eligible], &[if_expression]);

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.refinements().len(), 1);
    let refinement = &report.refinements()[0];
    assert_eq!(refinement.region(), then_block);
    assert_eq!(refinement.binding_use(), null_test.name_expression());
    assert_eq!(refinement.originating_null_test(), null_test.expression());
    assert_eq!(refinement.binding(), &binding);
    assert_eq!(refinement.original_nullable_type(), TypeId::from_raw(10));
    assert_eq!(refinement.refined_non_null_type(), TypeId::from_raw(11));
    assert!(report.refined_expression_types().is_empty());
}

#[test]
fn m0019_branch_refinement_records_else_branch_for_equal_tests() {
    let null_test = RecognizedNullTest::new(
        AstNodeId::from_raw(310),
        AstNodeId::from_raw(311),
        AstNodeId::from_raw(312),
        ParsedBinaryOperator::Equal,
        NullTestRefinedBranch::Else,
    );
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(310)),
        AstNodeId::from_raw(313),
        LocalBindingKind::Immutable,
    );
    let eligible = EligibleNullTestRefinement::new(
        null_test,
        binding.clone(),
        TypeId::from_raw(20),
        TypeId::from_raw(21),
    );
    let if_expression = ParsedIfExpression {
        expression: AstNodeId::from_raw(314),
        condition: null_test.expression(),
        then_block: AstNodeId::from_raw(315),
        else_block: Some(AstNodeId::from_raw(316)),
        span: ByteSpan::new(SourceFileId::from_raw(310), 0, 10).unwrap(),
    };
    let else_block = if_expression.else_block.unwrap();

    let report = record_m0019_branch_refinements(&[eligible], &[if_expression]);

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.refinements().len(), 1);
    let refinement = &report.refinements()[0];
    assert_eq!(refinement.region(), else_block);
    assert_eq!(refinement.binding_use(), null_test.name_expression());
    assert_eq!(refinement.originating_null_test(), null_test.expression());
    assert_eq!(refinement.binding(), &binding);
    assert_eq!(refinement.original_nullable_type(), TypeId::from_raw(20));
    assert_eq!(refinement.refined_non_null_type(), TypeId::from_raw(21));
}

#[test]
fn m0019_branch_refinement_skips_missing_else_and_non_condition_tests() {
    let equal_without_else = RecognizedNullTest::new(
        AstNodeId::from_raw(320),
        AstNodeId::from_raw(321),
        AstNodeId::from_raw(322),
        ParsedBinaryOperator::Equal,
        NullTestRefinedBranch::Else,
    );
    let non_condition = RecognizedNullTest::new(
        AstNodeId::from_raw(323),
        AstNodeId::from_raw(324),
        AstNodeId::from_raw(325),
        ParsedBinaryOperator::NotEqual,
        NullTestRefinedBranch::Then,
    );
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(320)),
        AstNodeId::from_raw(326),
        LocalBindingKind::Immutable,
    );
    let eligible = [
        EligibleNullTestRefinement::new(
            equal_without_else,
            binding.clone(),
            TypeId::from_raw(30),
            TypeId::from_raw(31),
        ),
        EligibleNullTestRefinement::new(
            non_condition,
            binding,
            TypeId::from_raw(32),
            TypeId::from_raw(33),
        ),
    ];
    let if_expression = ParsedIfExpression {
        expression: AstNodeId::from_raw(327),
        condition: equal_without_else.expression(),
        then_block: AstNodeId::from_raw(328),
        else_block: None,
        span: ByteSpan::new(SourceFileId::from_raw(320), 0, 10).unwrap(),
    };

    let report = record_m0019_branch_refinements(&eligible, &[if_expression]);

    assert_eq!(report.diagnostics(), &[]);
    assert!(report.refinements().is_empty());
    assert!(report.refined_expression_types().is_empty());
}

#[test]
fn m0019_refined_expression_type_records_active_exact_binding_uses() {
    let source =
        "fun check() { const maybe: String? = null; if (maybe != null) { maybe; }; maybe; }";
    let file = SourceFileId::from_raw(330);
    let parsed = parse_source(file, source);
    assert!(parsed.diagnostics.is_empty());
    let maybe_references = parsed
        .name_references
        .iter()
        .filter(|reference| reference.name == "maybe")
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(maybe_references.len(), 3);
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = compiler::symbol::SymbolInterner::new();
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );
    let resolved = bind_local_name_references(
        &parsed.arena,
        &maybe_references,
        &scopes,
        locals.index(),
        &mut interner,
    );
    assert!(resolved.diagnostics().is_empty());
    let region = parsed.if_expressions[0].then_block;
    let nullable = TypeId::from_raw(40);
    let non_null = TypeId::from_raw(41);
    let mut report = TypeCheckReport::new();
    report.record_refinement(RefinementRecord::new(
        region,
        maybe_references[0].reference,
        parsed.if_expressions[0].condition,
        locals.index().bindings()[0].clone(),
        nullable,
        non_null,
    ));

    record_m0019_refined_expression_types(
        &mut report,
        &parsed.arena,
        resolved.resolved_local_bindings(),
    );

    assert_eq!(report.refined_expression_types().len(), 1);
    let refined = report.refined_expression_types()[0];
    assert_eq!(refined.expression(), maybe_references[1].reference);
    assert_eq!(refined.refinement(), region);
    assert_eq!(refined.original_nullable_type(), nullable);
    assert_eq!(refined.refined_non_null_type(), non_null);
    assert_eq!(
        report.refined_expression_type(maybe_references[0].reference),
        None
    );
    assert_eq!(
        report.refined_expression_type(maybe_references[2].reference),
        None
    );
    assert!(report.diagnostics().is_empty());
}

#[test]
fn m0019_refined_expression_type_records_honor_nested_shadowing_and_region_bounds() {
    let source = "fun check() { const maybe: String? = null; if (maybe != null) { maybe; if (ready) { maybe; const maybe: String? = null; maybe; }; }; maybe; }";
    let file = SourceFileId::from_raw(331);
    let parsed = parse_source(file, source);
    assert!(
        parsed.diagnostics.is_empty(),
        "unexpected parser diagnostics: {:?}",
        parsed.diagnostics
    );
    let maybe_references = parsed
        .name_references
        .iter()
        .filter(|reference| reference.name == "maybe")
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(maybe_references.len(), 5);
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = compiler::symbol::SymbolInterner::new();
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );
    assert_eq!(locals.index().bindings().len(), 2);
    let resolved = bind_local_name_references(
        &parsed.arena,
        &maybe_references,
        &scopes,
        locals.index(),
        &mut interner,
    );
    assert!(resolved.diagnostics().is_empty());
    let region = parsed.if_expressions[0].then_block;
    let mut report = TypeCheckReport::new();
    report.record_refinement(RefinementRecord::new(
        region,
        maybe_references[0].reference,
        parsed.if_expressions[0].condition,
        locals.index().bindings()[0].clone(),
        TypeId::from_raw(42),
        TypeId::from_raw(43),
    ));

    record_m0019_refined_expression_types(
        &mut report,
        &parsed.arena,
        resolved.resolved_local_bindings(),
    );

    assert_eq!(
        report
            .refined_expression_types()
            .iter()
            .map(|entry| entry.expression())
            .collect::<Vec<_>>(),
        [maybe_references[1].reference, maybe_references[2].reference]
    );
    assert_eq!(
        report.refined_expression_type(maybe_references[3].reference),
        None
    );
    assert_eq!(
        report.refined_expression_type(maybe_references[4].reference),
        None
    );
    assert!(report.diagnostics().is_empty());
}

#[test]
fn m0019_refined_expression_type_records_report_overlapping_regions() {
    let file = SourceFileId::from_raw(332);
    let mut arena = AstArena::new();
    arena.add_source_file(ByteSpan::new(file, 0, 100).unwrap());
    let outer_region = arena.add_block(ByteSpan::new(file, 10, 90).unwrap());
    let inner_region = arena.add_block(ByteSpan::new(file, 20, 80).unwrap());
    let expression = arena.add_name_expression(ByteSpan::new(file, 30, 35).unwrap());
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(332)),
        AstNodeId::from_raw(333),
        LocalBindingKind::Immutable,
    );
    let resolved = ResolvedLocalBinding::new(expression, binding.clone());
    let mut report = TypeCheckReport::new();
    report.record_refinement(RefinementRecord::new(
        outer_region,
        AstNodeId::from_raw(334),
        AstNodeId::from_raw(335),
        binding.clone(),
        TypeId::from_raw(44),
        TypeId::from_raw(45),
    ));
    report.record_refinement(RefinementRecord::new(
        inner_region,
        AstNodeId::from_raw(336),
        AstNodeId::from_raw(337),
        binding,
        TypeId::from_raw(44),
        TypeId::from_raw(45),
    ));

    record_m0019_refined_expression_types(&mut report, &arena, &[resolved]);

    assert!(report.refined_expression_types().is_empty());
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::AmbiguousFlowRule
    );
    assert_eq!(
        report.diagnostics()[0].rule(),
        TypeRuleDiagnostic::AmbiguousNullTestRegion
    );
    assert_eq!(report.diagnostics()[0].node(), expression);
}

#[test]
fn m0019_refined_expression_type_records_reject_non_name_and_cross_file_uses() {
    let branch_file = SourceFileId::from_raw(338);
    let other_file = SourceFileId::from_raw(339);
    let mut arena = AstArena::new();
    arena.add_source_file(ByteSpan::new(branch_file, 0, 100).unwrap());
    arena.add_source_file(ByteSpan::new(other_file, 0, 100).unwrap());
    let region = arena.add_block(ByteSpan::new(branch_file, 10, 90).unwrap());
    let non_name = arena.add_literal_expression(ByteSpan::new(branch_file, 30, 35).unwrap());
    let cross_file_name = arena.add_name_expression(ByteSpan::new(other_file, 30, 35).unwrap());
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(338)),
        AstNodeId::from_raw(340),
        LocalBindingKind::Immutable,
    );
    let resolved = [
        ResolvedLocalBinding::new(non_name, binding.clone()),
        ResolvedLocalBinding::new(cross_file_name, binding.clone()),
    ];
    let mut report = TypeCheckReport::new();
    report.record_refinement(RefinementRecord::new(
        region,
        AstNodeId::from_raw(341),
        AstNodeId::from_raw(342),
        binding,
        TypeId::from_raw(46),
        TypeId::from_raw(47),
    ));

    record_m0019_refined_expression_types(&mut report, &arena, &resolved);

    assert!(report.refined_expression_types().is_empty());
    assert!(report.diagnostics().is_empty());
}

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
        LocalBindingKind::Immutable,
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
        "fun run() { const answer: Int = compute(); next = next + 1; logger.info(next); if (ready) { const inner = next; } else { const other = answer; } }",
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
        "fun run() { const source: Int = 1; const copy: Int = (source); copy = source; return; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());
    assert!(
        parsed
            .arena
            .nodes()
            .iter()
            .any(|node| node.kind == AstNodeKind::ReturnStatement)
    );

    let report = type_unsupported_m0018_expressions(&parsed.arena);

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.expression_types(), &[]);
    assert_eq!(report.declaration_signatures(), &[]);
    assert_eq!(report.assignment_checks(), &[]);
}

#[test]
fn unsupported_expression_diagnostics_report_unary_expression_nodes() {
    let file = SourceFileId::from_raw(87);
    let mut arena = compiler::ast::AstArena::new();
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
        "fun run() { const a = true; const b = 7; const c = \"text\"; const d = null; }",
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
        "fun run() { const ready: Bool = true; const count: Int = 1; const label: String = \"x\"; const done: Unit; const absent: Null = null; }",
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
        "fun run() { const inferred = true; const custom: UserId = value; const count: Int = 1; }",
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
        "fun run() { const ready: Bool = true; const count: Int = 1; const label: String = \"x\"; }",
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
        "fun run() { const ready: Bool = 1; const count: Int = 2; const custom: UserId = 3; const later: String = compute(); }",
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
            LocalBindingKind::Immutable,
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
            LocalBindingKind::Immutable,
        ),
        LocalBinding::new(
            LocalBindingKey::new(scope, untyped_symbol),
            AstNodeId::from_raw(101),
            LocalBindingKind::Immutable,
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
            span: compiler::source::ByteSpan::new(SourceFileId::from_raw(70), 0, 4).unwrap(),
        },
        ParsedGroupedExpression {
            expression: AstNodeId::from_raw(112),
            inner: AstNodeId::from_raw(113),
            span: compiler::source::ByteSpan::new(SourceFileId::from_raw(70), 5, 9).unwrap(),
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
            span: compiler::source::ByteSpan::new(SourceFileId::from_raw(71), 0, 4).unwrap(),
        },
        ParsedGroupedExpression {
            expression: AstNodeId::from_raw(122),
            inner: AstNodeId::from_raw(123),
            span: compiler::source::ByteSpan::new(SourceFileId::from_raw(71), 5, 9).unwrap(),
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
            span: compiler::source::ByteSpan::new(SourceFileId::from_raw(72), 1, 5).unwrap(),
        },
        ParsedGroupedExpression {
            expression: outer_group,
            inner: inner_group,
            span: compiler::source::ByteSpan::new(SourceFileId::from_raw(72), 0, 6).unwrap(),
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
fn assignment_statement_type_checking_keeps_nullable_to_base_as_m0018_type_mismatch() {
    let mut arena = TypeArena::new();
    let int_id = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nullable_int_id = arena.insert(TypeRecord::nullable(NullableType::new(int_id)));
    let value = AstNodeId::from_raw(343);
    let assignments = [ParsedAssignmentStatement {
        statement: AstNodeId::from_raw(344),
        target: AstNodeId::from_raw(345),
        value,
    }];
    let expression_types = [
        ExpressionType::new(AstNodeId::from_raw(345), int_id),
        ExpressionType::new(value, nullable_int_id),
    ];

    let report = type_assignment_statements(&assignments, &expression_types, &arena);

    assert!(report.assignment_checks().is_empty());
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::TypeMismatch
    );
    assert_eq!(report.diagnostics()[0].actual_type(), Some(nullable_int_id));
}

#[test]
fn m0019_refinement_aware_assignment_accepts_valid_refined_value() {
    let mut type_arena = TypeArena::new();
    let int_id = type_arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nullable_int_id = type_arena.insert(TypeRecord::nullable(NullableType::new(int_id)));
    let file = SourceFileId::from_raw(343);
    let mut ast_arena = AstArena::new();
    ast_arena.add_source_file(ByteSpan::new(file, 0, 100).unwrap());
    let region = ast_arena.add_block(ByteSpan::new(file, 10, 60).unwrap());
    let inside_value = ast_arena.add_name_expression(ByteSpan::new(file, 20, 25).unwrap());
    let outside_value = ast_arena.add_name_expression(ByteSpan::new(file, 70, 75).unwrap());
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(343)),
        AstNodeId::from_raw(344),
        LocalBindingKind::Immutable,
    );
    let resolved = [
        ResolvedLocalBinding::new(inside_value, binding.clone()),
        ResolvedLocalBinding::new(outside_value, binding.clone()),
    ];
    let mut flow_report = TypeCheckReport::new();
    flow_report.record_refinement(RefinementRecord::new(
        region,
        AstNodeId::from_raw(345),
        AstNodeId::from_raw(346),
        binding,
        nullable_int_id,
        int_id,
    ));
    record_m0019_refined_expression_types(&mut flow_report, &ast_arena, &resolved);
    let inside_statement = AstNodeId::from_raw(347);
    let outside_statement = AstNodeId::from_raw(348);
    let inside_target = AstNodeId::from_raw(349);
    let outside_target = AstNodeId::from_raw(350);
    let assignments = [
        ParsedAssignmentStatement {
            statement: inside_statement,
            target: inside_target,
            value: inside_value,
        },
        ParsedAssignmentStatement {
            statement: outside_statement,
            target: outside_target,
            value: outside_value,
        },
    ];
    let expression_types = [
        ExpressionType::new(inside_target, int_id),
        ExpressionType::new(inside_value, nullable_int_id),
        ExpressionType::new(outside_target, int_id),
        ExpressionType::new(outside_value, nullable_int_id),
    ];

    let report = type_m0019_assignment_statements(
        &assignments,
        &expression_types,
        &flow_report,
        &resolved,
        &ast_arena,
        &type_arena,
    );

    assert_eq!(
        report.assignment_checks(),
        &[AssignmentCheck::new(inside_statement, int_id, int_id)]
    );
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(report.diagnostics()[0].node(), outside_value);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::InvalidNullableUse
    );
}

#[test]
fn m0019_refinement_aware_assignment_reports_unrefined_nullable_to_base() {
    let mut arena = TypeArena::new();
    let int_id = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nullable_int_id = arena.insert(TypeRecord::nullable(NullableType::new(int_id)));
    let statement = AstNodeId::from_raw(347);
    let target = AstNodeId::from_raw(348);
    let value = AstNodeId::from_raw(349);
    let assignments = [ParsedAssignmentStatement {
        statement,
        target,
        value,
    }];
    let expression_types = [
        ExpressionType::new(target, int_id),
        ExpressionType::new(value, nullable_int_id),
    ];

    let flow_report = TypeCheckReport::new();
    let ast_arena = AstArena::new();
    let report = type_m0019_assignment_statements(
        &assignments,
        &expression_types,
        &flow_report,
        &[],
        &ast_arena,
        &arena,
    );

    assert!(report.assignment_checks().is_empty());
    assert_eq!(report.diagnostics().len(), 1);
    let diagnostic = &report.diagnostics()[0];
    assert_eq!(
        diagnostic.kind(),
        TypeCheckDiagnosticKind::InvalidNullableUse
    );
    assert_eq!(
        diagnostic.rule(),
        TypeRuleDiagnostic::NullableAssignmentWithoutRefinement
    );
    assert_eq!(diagnostic.node(), value);
    assert_eq!(diagnostic.expected_type(), Some(int_id));
    assert_eq!(diagnostic.actual_type(), Some(nullable_int_id));
}

#[test]
fn m0019_refinement_aware_assignment_preserves_m0018_compatibility() {
    let mut arena = TypeArena::new();
    let int_id = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let null_id = arena.insert(TypeRecord::primitive(PrimitiveType::Null));
    let nullable_int_id = arena.insert(TypeRecord::nullable(NullableType::new(int_id)));
    let assignments = [
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(350),
            target: AstNodeId::from_raw(351),
            value: AstNodeId::from_raw(352),
        },
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(353),
            target: AstNodeId::from_raw(354),
            value: AstNodeId::from_raw(355),
        },
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(356),
            target: AstNodeId::from_raw(357),
            value: AstNodeId::from_raw(358),
        },
    ];
    let expression_types = [
        ExpressionType::new(AstNodeId::from_raw(351), int_id),
        ExpressionType::new(AstNodeId::from_raw(352), int_id),
        ExpressionType::new(AstNodeId::from_raw(354), nullable_int_id),
        ExpressionType::new(AstNodeId::from_raw(355), null_id),
        ExpressionType::new(AstNodeId::from_raw(357), nullable_int_id),
        ExpressionType::new(AstNodeId::from_raw(358), int_id),
    ];

    let flow_report = TypeCheckReport::new();
    let ast_arena = AstArena::new();
    let report = type_m0019_assignment_statements(
        &assignments,
        &expression_types,
        &flow_report,
        &[],
        &ast_arena,
        &arena,
    );

    assert!(report.diagnostics().is_empty());
    assert_eq!(report.assignment_checks().len(), 3);
    assert_eq!(report.assignment_checks()[0].value(), int_id);
    assert_eq!(report.assignment_checks()[1].value(), null_id);
    assert_eq!(report.assignment_checks()[2].value(), int_id);
}

#[test]
fn m0019_refinement_aware_assignment_keeps_other_mismatches() {
    let mut arena = TypeArena::new();
    let int_id = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let string_id = arena.insert(TypeRecord::primitive(PrimitiveType::String));
    let null_id = arena.insert(TypeRecord::primitive(PrimitiveType::Null));
    let nullable_string_id = arena.insert(TypeRecord::nullable(NullableType::new(string_id)));
    let assignments = [
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(359),
            target: AstNodeId::from_raw(360),
            value: AstNodeId::from_raw(361),
        },
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(362),
            target: AstNodeId::from_raw(363),
            value: AstNodeId::from_raw(364),
        },
    ];
    let expression_types = [
        ExpressionType::new(AstNodeId::from_raw(360), int_id),
        ExpressionType::new(AstNodeId::from_raw(361), null_id),
        ExpressionType::new(AstNodeId::from_raw(363), int_id),
        ExpressionType::new(AstNodeId::from_raw(364), nullable_string_id),
    ];

    let flow_report = TypeCheckReport::new();
    let ast_arena = AstArena::new();
    let report = type_m0019_assignment_statements(
        &assignments,
        &expression_types,
        &flow_report,
        &[],
        &ast_arena,
        &arena,
    );

    assert!(report.assignment_checks().is_empty());
    assert_eq!(report.diagnostics().len(), 2);
    assert!(
        report
            .diagnostics()
            .iter()
            .all(|diagnostic| diagnostic.kind() == TypeCheckDiagnosticKind::TypeMismatch)
    );
    assert_eq!(report.diagnostics()[0].actual_type(), Some(null_id));
    assert_eq!(
        report.diagnostics()[1].actual_type(),
        Some(nullable_string_id)
    );
}

#[test]
fn m0019_refinement_aware_assignment_ignores_inconsistent_refined_views() {
    let mut type_arena = TypeArena::new();
    let int_id = type_arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let string_id = type_arena.insert(TypeRecord::primitive(PrimitiveType::String));
    let nullable_int_id = type_arena.insert(TypeRecord::nullable(NullableType::new(int_id)));
    let nullable_string_id = type_arena.insert(TypeRecord::nullable(NullableType::new(string_id)));
    let file = SourceFileId::from_raw(365);
    let mut ast_arena = AstArena::new();
    ast_arena.add_source_file(ByteSpan::new(file, 0, 100).unwrap());
    let region = ast_arena.add_block(ByteSpan::new(file, 10, 90).unwrap());
    let first_value = ast_arena.add_name_expression(ByteSpan::new(file, 20, 25).unwrap());
    let second_value = ast_arena.add_name_expression(ByteSpan::new(file, 30, 35).unwrap());
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(365)),
        AstNodeId::from_raw(366),
        LocalBindingKind::Immutable,
    );
    let resolved = [
        ResolvedLocalBinding::new(first_value, binding.clone()),
        ResolvedLocalBinding::new(second_value, binding.clone()),
    ];
    let assignments = [
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(367),
            target: AstNodeId::from_raw(368),
            value: first_value,
        },
        ParsedAssignmentStatement {
            statement: AstNodeId::from_raw(369),
            target: AstNodeId::from_raw(370),
            value: second_value,
        },
    ];
    let expression_types = [
        ExpressionType::new(AstNodeId::from_raw(368), int_id),
        ExpressionType::new(first_value, nullable_int_id),
        ExpressionType::new(AstNodeId::from_raw(370), int_id),
        ExpressionType::new(second_value, nullable_int_id),
    ];
    let mut flow_report = TypeCheckReport::new();
    flow_report.record_refinement(RefinementRecord::new(
        region,
        AstNodeId::from_raw(371),
        AstNodeId::from_raw(372),
        binding,
        nullable_int_id,
        int_id,
    ));
    flow_report.record_refined_expression_type(RefinedExpressionType::new(
        first_value,
        region,
        nullable_string_id,
        string_id,
    ));
    flow_report.record_refined_expression_type(RefinedExpressionType::new(
        second_value,
        region,
        nullable_int_id,
        string_id,
    ));

    let report = type_m0019_assignment_statements(
        &assignments,
        &expression_types,
        &flow_report,
        &resolved,
        &ast_arena,
        &type_arena,
    );

    assert!(report.assignment_checks().is_empty());
    assert_eq!(report.diagnostics().len(), 2);
    assert!(report.diagnostics().iter().all(|diagnostic| {
        diagnostic.kind() == TypeCheckDiagnosticKind::InvalidNullableUse
            && diagnostic.rule() == TypeRuleDiagnostic::NullableAssignmentWithoutRefinement
    }));
}

#[test]
fn m0019_refinement_aware_assignment_rejects_duplicate_refined_views() {
    let mut type_arena = TypeArena::new();
    let int_id = type_arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nullable_int_id = type_arena.insert(TypeRecord::nullable(NullableType::new(int_id)));
    let file = SourceFileId::from_raw(373);
    let mut ast_arena = AstArena::new();
    ast_arena.add_source_file(ByteSpan::new(file, 0, 100).unwrap());
    let region = ast_arena.add_block(ByteSpan::new(file, 10, 90).unwrap());
    let value = ast_arena.add_name_expression(ByteSpan::new(file, 20, 25).unwrap());
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(373)),
        AstNodeId::from_raw(374),
        LocalBindingKind::Immutable,
    );
    let resolved = [ResolvedLocalBinding::new(value, binding.clone())];
    let statement = AstNodeId::from_raw(375);
    let target = AstNodeId::from_raw(376);
    let assignments = [ParsedAssignmentStatement {
        statement,
        target,
        value,
    }];
    let expression_types = [
        ExpressionType::new(target, int_id),
        ExpressionType::new(value, nullable_int_id),
    ];
    let mut flow_report = TypeCheckReport::new();
    flow_report.record_refinement(RefinementRecord::new(
        region,
        AstNodeId::from_raw(377),
        AstNodeId::from_raw(378),
        binding,
        nullable_int_id,
        int_id,
    ));
    flow_report.record_refined_expression_type(RefinedExpressionType::new(
        value,
        region,
        nullable_int_id,
        int_id,
    ));
    flow_report.record_refined_expression_type(RefinedExpressionType::new(
        value,
        region,
        nullable_int_id,
        int_id,
    ));

    let report = type_m0019_assignment_statements(
        &assignments,
        &expression_types,
        &flow_report,
        &resolved,
        &ast_arena,
        &type_arena,
    );

    assert!(report.assignment_checks().is_empty());
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::InvalidNullableUse
    );
    assert_eq!(
        report.diagnostics()[0].rule(),
        TypeRuleDiagnostic::NullableAssignmentWithoutRefinement
    );
}

#[test]
fn m0019_refinement_aware_assignment_rejects_forged_out_of_region_view() {
    let mut type_arena = TypeArena::new();
    let int_id = type_arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nullable_int_id = type_arena.insert(TypeRecord::nullable(NullableType::new(int_id)));
    let file = SourceFileId::from_raw(381);
    let mut ast_arena = AstArena::new();
    ast_arena.add_source_file(ByteSpan::new(file, 0, 100).unwrap());
    let region = ast_arena.add_block(ByteSpan::new(file, 10, 50).unwrap());
    let value = ast_arena.add_name_expression(ByteSpan::new(file, 70, 75).unwrap());
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(381)),
        AstNodeId::from_raw(382),
        LocalBindingKind::Immutable,
    );
    let resolved = [ResolvedLocalBinding::new(value, binding.clone())];
    let target = AstNodeId::from_raw(383);
    let assignments = [ParsedAssignmentStatement {
        statement: AstNodeId::from_raw(384),
        target,
        value,
    }];
    let expression_types = [
        ExpressionType::new(target, int_id),
        ExpressionType::new(value, nullable_int_id),
    ];
    let mut flow_report = TypeCheckReport::new();
    flow_report.record_refinement(RefinementRecord::new(
        region,
        AstNodeId::from_raw(385),
        AstNodeId::from_raw(386),
        binding,
        nullable_int_id,
        int_id,
    ));
    flow_report.record_refined_expression_type(RefinedExpressionType::new(
        value,
        region,
        nullable_int_id,
        int_id,
    ));

    let report = type_m0019_assignment_statements(
        &assignments,
        &expression_types,
        &flow_report,
        &resolved,
        &ast_arena,
        &type_arena,
    );

    assert!(report.assignment_checks().is_empty());
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::InvalidNullableUse
    );
}

#[test]
fn m0019_refinement_aware_local_initializer_accepts_exact_active_view_and_preserves_original_records()
 {
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nullable_int = types.insert(TypeRecord::nullable(NullableType::new(int)));
    let file = SourceFileId::from_raw(390);
    let mut ast = AstArena::new();
    ast.add_source_file(ByteSpan::new(file, 0, 100).unwrap());
    let annotation = ast.add_named_type(ByteSpan::new(file, 5, 8).unwrap());
    let region = ast.add_block(ByteSpan::new(file, 10, 90).unwrap());
    let initializer = ast.add_name_expression(ByteSpan::new(file, 20, 25).unwrap());
    let declaration = AstNodeId::from_raw(391);
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(390)),
        AstNodeId::from_raw(392),
        LocalBindingKind::Immutable,
    );
    let declarations = [ParsedLocalDeclaration {
        declaration,
        annotation: Some(annotation),
        initializer: Some(initializer),
    }];
    let signatures = [DeclarationSignature::new(declaration, int)];
    let expression_types = [ExpressionType::new(initializer, nullable_int)];
    let resolved = [ResolvedLocalBinding::new(initializer, binding.clone())];
    let mut flow = TypeCheckReport::new();
    flow.record_refinement(RefinementRecord::new(
        region,
        AstNodeId::from_raw(393),
        AstNodeId::from_raw(394),
        binding,
        nullable_int,
        int,
    ));
    flow.record_refined_expression_type(RefinedExpressionType::new(
        initializer,
        region,
        nullable_int,
        int,
    ));

    let report = type_m0019_local_declaration_initializers(
        &declarations,
        &signatures,
        &expression_types,
        &flow,
        &resolved,
        &ast,
        &types,
    );

    assert!(report.diagnostics().is_empty());
    assert_eq!(report.declaration_signatures(), &signatures);
    assert_eq!(report.expression_types(), &expression_types);
    assert_eq!(
        report.assignment_checks(),
        &[AssignmentCheck::new(declaration, int, int)]
    );
    assert_eq!(
        flow.refined_expression_types()[0].original_nullable_type(),
        nullable_int
    );
    assert_eq!(flow.refinements()[0].original_nullable_type(), nullable_int);
}

#[test]
fn m0019_refinement_aware_local_initializer_diagnoses_exact_unrefined_name_only() {
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let string = types.insert(TypeRecord::primitive(PrimitiveType::String));
    let null = types.insert(TypeRecord::primitive(PrimitiveType::Null));
    let nullable_int = types.insert(TypeRecord::nullable(NullableType::new(int)));
    let nullable_string = types.insert(TypeRecord::nullable(NullableType::new(string)));
    let file = SourceFileId::from_raw(395);
    let mut ast = AstArena::new();
    ast.add_source_file(ByteSpan::new(file, 0, 100).unwrap());
    let annotation = ast.add_named_type(ByteSpan::new(file, 5, 8).unwrap());
    let nullable_name = ast.add_name_expression(ByteSpan::new(file, 10, 15).unwrap());
    let null_literal = ast.add_literal_expression(ByteSpan::new(file, 20, 24).unwrap());
    let unrelated_name = ast.add_name_expression(ByteSpan::new(file, 30, 35).unwrap());
    let declarations = [
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(396),
            annotation: Some(annotation),
            initializer: Some(nullable_name),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(397),
            annotation: Some(annotation),
            initializer: Some(null_literal),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(398),
            annotation: Some(annotation),
            initializer: Some(unrelated_name),
        },
    ];
    let signatures = [
        DeclarationSignature::new(declarations[0].declaration, int),
        DeclarationSignature::new(declarations[1].declaration, int),
        DeclarationSignature::new(declarations[2].declaration, int),
    ];
    let expression_types = [
        ExpressionType::new(nullable_name, nullable_int),
        ExpressionType::new(null_literal, null),
        ExpressionType::new(unrelated_name, nullable_string),
    ];
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(395)),
        AstNodeId::from_raw(399),
        LocalBindingKind::Immutable,
    );
    let resolved = [
        ResolvedLocalBinding::new(nullable_name, binding.clone()),
        ResolvedLocalBinding::new(unrelated_name, binding),
    ];

    let report = type_m0019_local_declaration_initializers(
        &declarations,
        &signatures,
        &expression_types,
        &TypeCheckReport::new(),
        &resolved,
        &ast,
        &types,
    );

    assert_eq!(report.diagnostics().len(), 3);
    let nullable = &report.diagnostics()[0];
    assert_eq!(nullable.kind(), TypeCheckDiagnosticKind::InvalidNullableUse);
    assert_eq!(
        nullable.rule(),
        TypeRuleDiagnostic::NullableAssignmentWithoutRefinement
    );
    assert_eq!(nullable.node(), nullable_name);
    assert_eq!(nullable.expected_type(), Some(int));
    assert_eq!(nullable.actual_type(), Some(nullable_int));
    assert!(report.diagnostics()[1..].iter().all(|diagnostic| {
        diagnostic.kind() == TypeCheckDiagnosticKind::TypeMismatch
            && diagnostic.rule()
                == TypeRuleDiagnostic::Ambiguous(AmbiguousTypeRule::AssignmentCompatibility)
    }));
    assert_eq!(report.diagnostics()[1].node(), null_literal);
    assert_eq!(report.diagnostics()[2].node(), unrelated_name);
}

#[test]
fn m0019_refinement_aware_local_initializer_preserves_nullable_compatibility() {
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let null = types.insert(TypeRecord::primitive(PrimitiveType::Null));
    let nullable_int = types.insert(TypeRecord::nullable(NullableType::new(int)));
    let file = SourceFileId::from_raw(409);
    let mut ast = AstArena::new();
    ast.add_source_file(ByteSpan::new(file, 0, 100).unwrap());
    let annotation = ast.add_named_type(ByteSpan::new(file, 5, 8).unwrap());
    let null_initializer = ast.add_literal_expression(ByteSpan::new(file, 10, 14).unwrap());
    let base_initializer = ast.add_name_expression(ByteSpan::new(file, 20, 25).unwrap());
    let declarations = [
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(409),
            annotation: Some(annotation),
            initializer: Some(null_initializer),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(410),
            annotation: Some(annotation),
            initializer: Some(base_initializer),
        },
    ];
    let signatures = [
        DeclarationSignature::new(declarations[0].declaration, nullable_int),
        DeclarationSignature::new(declarations[1].declaration, nullable_int),
    ];
    let expression_types = [
        ExpressionType::new(null_initializer, null),
        ExpressionType::new(base_initializer, int),
    ];

    let report = type_m0019_local_declaration_initializers(
        &declarations,
        &signatures,
        &expression_types,
        &TypeCheckReport::new(),
        &[],
        &ast,
        &types,
    );

    assert!(report.diagnostics().is_empty());
    assert_eq!(
        report.assignment_checks(),
        &[
            AssignmentCheck::new(declarations[0].declaration, nullable_int, null),
            AssignmentCheck::new(declarations[1].declaration, nullable_int, int),
        ]
    );
}

#[test]
fn m0019_refinement_aware_local_initializer_rejects_invalid_or_cross_use_views() {
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nullable_int = types.insert(TypeRecord::nullable(NullableType::new(int)));
    let file = SourceFileId::from_raw(400);
    let mut ast = AstArena::new();
    ast.add_source_file(ByteSpan::new(file, 0, 200).unwrap());
    let annotation = ast.add_named_type(ByteSpan::new(file, 5, 8).unwrap());
    let region = ast.add_block(ByteSpan::new(file, 10, 100).unwrap());
    let first = ast.add_name_expression(ByteSpan::new(file, 20, 25).unwrap());
    let second = ast.add_name_expression(ByteSpan::new(file, 30, 35).unwrap());
    let outside = ast.add_name_expression(ByteSpan::new(file, 120, 125).unwrap());
    let declarations = [
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(401),
            annotation: Some(annotation),
            initializer: Some(first),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(402),
            annotation: Some(annotation),
            initializer: Some(second),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(403),
            annotation: Some(annotation),
            initializer: Some(outside),
        },
    ];
    let signatures = declarations
        .each_ref()
        .map(|declaration| DeclarationSignature::new(declaration.declaration, int));
    let expression_types = [
        ExpressionType::new(first, nullable_int),
        ExpressionType::new(second, nullable_int),
        ExpressionType::new(outside, nullable_int),
    ];
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(400)),
        AstNodeId::from_raw(404),
        LocalBindingKind::Immutable,
    );
    let wrong_binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(401)),
        AstNodeId::from_raw(405),
        LocalBindingKind::Immutable,
    );
    let mut flow = TypeCheckReport::new();
    flow.record_refinement(RefinementRecord::new(
        region,
        AstNodeId::from_raw(406),
        AstNodeId::from_raw(407),
        binding.clone(),
        nullable_int,
        int,
    ));
    // Duplicate first, forged second, and out-of-region third must all be ignored.
    flow.record_refined_expression_type(RefinedExpressionType::new(
        first,
        region,
        nullable_int,
        int,
    ));
    flow.record_refined_expression_type(RefinedExpressionType::new(
        first,
        region,
        nullable_int,
        int,
    ));
    flow.record_refined_expression_type(RefinedExpressionType::new(
        second,
        region,
        nullable_int,
        int,
    ));
    flow.record_refined_expression_type(RefinedExpressionType::new(
        outside,
        region,
        nullable_int,
        int,
    ));
    let resolved = [
        ResolvedLocalBinding::new(first, binding),
        ResolvedLocalBinding::new(second, wrong_binding),
        ResolvedLocalBinding::new(
            outside,
            LocalBinding::new(
                LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(402)),
                AstNodeId::from_raw(408),
                LocalBindingKind::Immutable,
            ),
        ),
    ];

    let report = type_m0019_local_declaration_initializers(
        &declarations,
        &signatures,
        &expression_types,
        &flow,
        &resolved,
        &ast,
        &types,
    );

    assert!(report.assignment_checks().is_empty());
    assert_eq!(report.diagnostics().len(), 3);
    assert!(report.diagnostics().iter().all(|diagnostic| {
        diagnostic.kind() == TypeCheckDiagnosticKind::InvalidNullableUse
            && diagnostic.rule() == TypeRuleDiagnostic::NullableAssignmentWithoutRefinement
    }));
    assert_eq!(
        report
            .diagnostics()
            .iter()
            .map(TypeCheckDiagnostic::node)
            .collect::<Vec<_>>(),
        vec![first, second, outside]
    );
}

#[test]
fn m0019_refinement_aware_local_initializer_rejects_after_sibling_shadowed_and_inconsistent_views()
{
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let string = types.insert(TypeRecord::primitive(PrimitiveType::String));
    let nullable_int = types.insert(TypeRecord::nullable(NullableType::new(int)));
    let nullable_string = types.insert(TypeRecord::nullable(NullableType::new(string)));
    let file = SourceFileId::from_raw(420);
    let mut ast = AstArena::new();
    ast.add_source_file(ByteSpan::new(file, 0, 200).unwrap());
    let annotation = ast.add_named_type(ByteSpan::new(file, 5, 8).unwrap());
    let then_region = ast.add_block(ByteSpan::new(file, 10, 50).unwrap());
    let _sibling_else_region = ast.add_block(ByteSpan::new(file, 60, 100).unwrap());
    let after_branch = ast.add_name_expression(ByteSpan::new(file, 110, 115).unwrap());
    let sibling_branch = ast.add_name_expression(ByteSpan::new(file, 70, 75).unwrap());
    let shadowed = ast.add_name_expression(ByteSpan::new(file, 30, 35).unwrap());
    let inconsistent = ast.add_name_expression(ByteSpan::new(file, 40, 45).unwrap());
    let declarations = [
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(421),
            annotation: Some(annotation),
            initializer: Some(after_branch),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(422),
            annotation: Some(annotation),
            initializer: Some(sibling_branch),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(423),
            annotation: Some(annotation),
            initializer: Some(shadowed),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(424),
            annotation: Some(annotation),
            initializer: Some(inconsistent),
        },
    ];
    let signatures = declarations
        .each_ref()
        .map(|declaration| DeclarationSignature::new(declaration.declaration, int));
    let expression_types = [
        ExpressionType::new(after_branch, nullable_int),
        ExpressionType::new(sibling_branch, nullable_int),
        ExpressionType::new(shadowed, nullable_int),
        ExpressionType::new(inconsistent, nullable_int),
    ];
    let outer_binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(420)),
        AstNodeId::from_raw(425),
        LocalBindingKind::Immutable,
    );
    let shadow_binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(1), SymbolId::from_raw(420)),
        AstNodeId::from_raw(426),
        LocalBindingKind::Immutable,
    );
    let mut flow = TypeCheckReport::new();
    flow.record_refinement(RefinementRecord::new(
        then_region,
        AstNodeId::from_raw(427),
        AstNodeId::from_raw(428),
        outer_binding.clone(),
        nullable_int,
        int,
    ));
    flow.record_refined_expression_type(RefinedExpressionType::new(
        after_branch,
        then_region,
        nullable_int,
        int,
    ));
    flow.record_refined_expression_type(RefinedExpressionType::new(
        sibling_branch,
        then_region,
        nullable_int,
        int,
    ));
    flow.record_refined_expression_type(RefinedExpressionType::new(
        shadowed,
        then_region,
        nullable_int,
        int,
    ));
    flow.record_refined_expression_type(RefinedExpressionType::new(
        inconsistent,
        then_region,
        nullable_string,
        string,
    ));
    let resolved = [
        ResolvedLocalBinding::new(after_branch, outer_binding.clone()),
        ResolvedLocalBinding::new(sibling_branch, outer_binding.clone()),
        ResolvedLocalBinding::new(shadowed, shadow_binding),
        ResolvedLocalBinding::new(inconsistent, outer_binding),
    ];

    let report = type_m0019_local_declaration_initializers(
        &declarations,
        &signatures,
        &expression_types,
        &flow,
        &resolved,
        &ast,
        &types,
    );

    assert!(report.assignment_checks().is_empty());
    assert_eq!(report.diagnostics().len(), 4);
    assert!(report.diagnostics().iter().all(|diagnostic| {
        diagnostic.kind() == TypeCheckDiagnosticKind::InvalidNullableUse
            && diagnostic.rule() == TypeRuleDiagnostic::NullableAssignmentWithoutRefinement
    }));
    assert_eq!(
        report
            .diagnostics()
            .iter()
            .map(TypeCheckDiagnostic::node)
            .collect::<Vec<_>>(),
        vec![after_branch, sibling_branch, shadowed, inconsistent]
    );
}

#[test]
fn m0019_refinement_aware_local_initializer_rejects_forged_mutable_binding_view() {
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nullable_int = types.insert(TypeRecord::nullable(NullableType::new(int)));
    let file = SourceFileId::from_raw(411);
    let mut ast = AstArena::new();
    ast.add_source_file(ByteSpan::new(file, 0, 100).unwrap());
    let annotation = ast.add_named_type(ByteSpan::new(file, 5, 8).unwrap());
    let region = ast.add_block(ByteSpan::new(file, 10, 90).unwrap());
    let initializer = ast.add_name_expression(ByteSpan::new(file, 20, 25).unwrap());
    let declaration = AstNodeId::from_raw(414);
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(411)),
        AstNodeId::from_raw(412),
        LocalBindingKind::Var,
    );
    let declarations = [ParsedLocalDeclaration {
        declaration,
        annotation: Some(annotation),
        initializer: Some(initializer),
    }];
    let signatures = [DeclarationSignature::new(declaration, int)];
    let expression_types = [ExpressionType::new(initializer, nullable_int)];
    let mut flow = TypeCheckReport::new();
    flow.record_refinement(RefinementRecord::new(
        region,
        AstNodeId::from_raw(413),
        AstNodeId::from_raw(415),
        binding.clone(),
        nullable_int,
        int,
    ));
    flow.record_refined_expression_type(RefinedExpressionType::new(
        initializer,
        region,
        nullable_int,
        int,
    ));
    let resolved = [ResolvedLocalBinding::new(initializer, binding)];

    let report = type_m0019_local_declaration_initializers(
        &declarations,
        &signatures,
        &expression_types,
        &flow,
        &resolved,
        &ast,
        &types,
    );

    assert!(report.assignment_checks().is_empty());
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::InvalidNullableUse
    );
    assert_eq!(
        report.diagnostics()[0].rule(),
        TypeRuleDiagnostic::NullableAssignmentWithoutRefinement
    );
}

#[test]
fn m0019_refinement_aware_local_initializer_does_not_consume_another_use_view() {
    let mut types = TypeArena::new();
    let int = types.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nullable_int = types.insert(TypeRecord::nullable(NullableType::new(int)));
    let file = SourceFileId::from_raw(410);
    let mut ast = AstArena::new();
    ast.add_source_file(ByteSpan::new(file, 0, 100).unwrap());
    let annotation = ast.add_named_type(ByteSpan::new(file, 5, 8).unwrap());
    let region = ast.add_block(ByteSpan::new(file, 10, 90).unwrap());
    let refined_use = ast.add_name_expression(ByteSpan::new(file, 20, 25).unwrap());
    let other_use = ast.add_name_expression(ByteSpan::new(file, 30, 35).unwrap());
    let binding = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(410)),
        AstNodeId::from_raw(411),
        LocalBindingKind::Immutable,
    );
    let declarations = [
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(412),
            annotation: Some(annotation),
            initializer: Some(refined_use),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(413),
            annotation: Some(annotation),
            initializer: Some(other_use),
        },
    ];
    let signatures = [
        DeclarationSignature::new(declarations[0].declaration, int),
        DeclarationSignature::new(declarations[1].declaration, int),
    ];
    let expression_types = [
        ExpressionType::new(refined_use, nullable_int),
        ExpressionType::new(other_use, nullable_int),
    ];
    let mut flow = TypeCheckReport::new();
    flow.record_refinement(RefinementRecord::new(
        region,
        AstNodeId::from_raw(414),
        AstNodeId::from_raw(415),
        binding.clone(),
        nullable_int,
        int,
    ));
    flow.record_refined_expression_type(RefinedExpressionType::new(
        refined_use,
        region,
        nullable_int,
        int,
    ));
    let resolved = [
        ResolvedLocalBinding::new(refined_use, binding.clone()),
        ResolvedLocalBinding::new(other_use, binding),
    ];

    let report = type_m0019_local_declaration_initializers(
        &declarations,
        &signatures,
        &expression_types,
        &flow,
        &resolved,
        &ast,
        &types,
    );

    assert_eq!(
        report.assignment_checks(),
        &[AssignmentCheck::new(declarations[0].declaration, int, int)]
    );
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(report.diagnostics()[0].node(), other_use);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::InvalidNullableUse
    );
    assert_eq!(
        report.diagnostics()[0].rule(),
        TypeRuleDiagnostic::NullableAssignmentWithoutRefinement
    );
}

#[test]
fn accepted_expression_composition_records_literals_names_and_groups() {
    let literal = AstNodeId::from_raw(190);
    let name = AstNodeId::from_raw(191);
    let group = AstNodeId::from_raw(192);
    let literals = [ParsedLiteralExpression {
        expression: literal,
        kind: ParsedLiteralKind::AcceptedInteger,
        span: compiler::source::ByteSpan::new(SourceFileId::from_raw(80), 0, 2).unwrap(),
    }];
    let grouped = [ParsedGroupedExpression {
        expression: group,
        inner: name,
        span: compiler::source::ByteSpan::new(SourceFileId::from_raw(80), 3, 9).unwrap(),
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
        span: compiler::source::ByteSpan::new(SourceFileId::from_raw(81), 2, 8).unwrap(),
    }];
    let grouped = [
        ParsedGroupedExpression {
            expression: outer_group,
            inner: inner_group,
            span: compiler::source::ByteSpan::new(SourceFileId::from_raw(81), 0, 10).unwrap(),
        },
        ParsedGroupedExpression {
            expression: inner_group,
            inner: literal,
            span: compiler::source::ByteSpan::new(SourceFileId::from_raw(81), 1, 9).unwrap(),
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
        span: compiler::source::ByteSpan::new(SourceFileId::from_raw(82), 0, 8).unwrap(),
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
        "fun run() { const source: Int = 1; const copy: Int = source; const grouped: Int = (source); }",
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
        "fun run() { const source: Int = 1; const bad: String = source; const unknown: UserId = source; const skipped: Int = missing; }",
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
        "fun run() { const source: Int = 1; const copy: Int = source; const grouped: Int = (copy); var next: Int = grouped; next = source; }",
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
            LocalBindingKind::Immutable,
        ),
        LocalBinding::new(
            LocalBindingKey::new(LocalScopeId::from_raw(0), copy_symbol),
            parsed.local_declarations[1].declaration,
            LocalBindingKind::Immutable,
        ),
        LocalBinding::new(
            LocalBindingKey::new(LocalScopeId::from_raw(0), grouped_symbol),
            parsed.local_declarations[2].declaration,
            LocalBindingKind::Immutable,
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
    assert!(
        report
            .assignment_check(parsed.assignment_statements[0].statement)
            .is_some()
    );
}

#[test]
fn m0018_core_reports_mismatch_unresolved_and_unsupported_diagnostics() {
    let parsed = parse_source(
        SourceFileId::from_raw(89),
        "fun run() { const source: Int = 1; const bad: String = source; const unknown: UserId = source; const missingKnown: Int = external; logger.info(source); }",
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
        LocalBindingKind::Immutable,
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

    assert!(
        report
            .diagnostics()
            .iter()
            .any(
                |diagnostic| diagnostic.kind() == TypeCheckDiagnosticKind::TypeMismatch
                    && diagnostic.node() == parsed.local_declarations[1].initializer.unwrap()
                    && diagnostic.expected_type() == Some(TypeId::from_raw(2))
                    && diagnostic.actual_type() == Some(TypeId::from_raw(1))
            )
    );
    assert!(
        report
            .diagnostics()
            .iter()
            .any(
                |diagnostic| diagnostic.kind() == TypeCheckDiagnosticKind::UnresolvedTypeRule
                    && diagnostic.rule() == TypeRuleDiagnostic::MissingAnnotationType
                    && diagnostic.node() == parsed.local_declarations[2].declaration
            )
    );
    assert!(
        report
            .diagnostics()
            .iter()
            .any(
                |diagnostic| diagnostic.kind() == TypeCheckDiagnosticKind::UnresolvedTypeRule
                    && diagnostic.rule() == TypeRuleDiagnostic::MissingResolvedNameType
            )
    );
    assert!(
        report
            .diagnostics()
            .iter()
            .any(
                |diagnostic| diagnostic.kind() == TypeCheckDiagnosticKind::UnsupportedTypeRule
                    && diagnostic.rule() == TypeRuleDiagnostic::MemberExpressionDeferred
            )
    );
    assert!(
        report
            .diagnostics()
            .iter()
            .any(
                |diagnostic| diagnostic.kind() == TypeCheckDiagnosticKind::UnsupportedTypeRule
                    && diagnostic.rule() == TypeRuleDiagnostic::DirectCallDeferred
            )
    );
    assert_eq!(
        report.assignment_check(parsed.local_declarations[2].declaration),
        None
    );
    assert_eq!(
        report.assignment_check(parsed.local_declarations[3].declaration),
        None
    );
}

#[test]
fn m0028_executable_int_operators_type_every_supported_operator() {
    let parsed = parse_source(
        SourceFileId::from_raw(90),
        "fun run() { const plus = +1; const minus = -1; const not = ~1; const add = 1 + 2; const subtract = 1 - 2; const multiply = 1 * 2; const divide = 1 / 2; const modulo = 1 % 2; const exponent = 1 ** 2; const and = 1 & 2; const or = 1 | 2; const xor = 1 ^ 2; const left = 1 << 2; const right = 1 >> 2; const nested = (1 + 2) * 3; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let (_arena, base_report) = type_m0018_accepted_expressions(
        &parsed.literal_expressions,
        &parsed.grouped_expressions,
        &ResolutionTable::new(),
        &[],
    );
    let report = type_m0028_executable_int_operators(
        &parsed.unary_expressions,
        &parsed.binary_expressions,
        &parsed.grouped_expressions,
        base_report.expression_types(),
        TypeId::from_raw(1),
    );

    assert_eq!(report.diagnostics(), &[]);
    for unary in &parsed.unary_expressions {
        assert_eq!(
            report.expression_type(unary.expression),
            Some(TypeId::from_raw(1))
        );
    }
    for binary in &parsed.binary_expressions {
        assert_eq!(
            report.expression_type(binary.expression),
            Some(TypeId::from_raw(1))
        );
    }
}

#[test]
fn m0028_executable_int_operators_reject_known_non_int_operands() {
    let parsed = parse_source(SourceFileId::from_raw(91), "fun run() { true + 1; }");
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let (_arena, base_report) = type_m0018_accepted_expressions(
        &parsed.literal_expressions,
        &parsed.grouped_expressions,
        &ResolutionTable::new(),
        &[],
    );
    let report = type_m0028_executable_int_operators(
        &parsed.unary_expressions,
        &parsed.binary_expressions,
        &parsed.grouped_expressions,
        base_report.expression_types(),
        TypeId::from_raw(1),
    );

    let binary = &parsed.binary_expressions[0];
    assert_eq!(report.expression_type(binary.expression), None);
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].kind(),
        TypeCheckDiagnosticKind::TypeMismatch
    );
    assert_eq!(report.diagnostics()[0].node(), binary.left);
    assert_eq!(
        report.diagnostics()[0].expected_type(),
        Some(TypeId::from_raw(1))
    );
    assert_eq!(
        report.diagnostics()[0].actual_type(),
        Some(TypeId::from_raw(0))
    );
}

#[test]
fn m0028_executable_int_operators_do_not_type_unknown_operands() {
    let parsed = parse_source(SourceFileId::from_raw(92), "fun run() { unknown + 1; }");
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let (_arena, base_report) = type_m0018_accepted_expressions(
        &parsed.literal_expressions,
        &parsed.grouped_expressions,
        &ResolutionTable::new(),
        &[],
    );
    let report = type_m0028_executable_int_operators(
        &parsed.unary_expressions,
        &parsed.binary_expressions,
        &parsed.grouped_expressions,
        base_report.expression_types(),
        TypeId::from_raw(1),
    );

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(
        report.expression_type(parsed.binary_expressions[0].expression),
        None
    );
}

#[test]
fn m0028_core_types_executable_operators_before_initializers_and_assignments() {
    let parsed = parse_source(
        SourceFileId::from_raw(93),
        "fun run() { const value: Int = (1 + 2) * 3; var next: Int = value; next = next << 1; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let value_symbol = SymbolId::from_raw(93);
    let next_symbol = SymbolId::from_raw(94);
    let mut resolutions = ResolutionTable::new();
    for reference in &parsed.name_references {
        let symbol = match reference.name.as_str() {
            "value" => value_symbol,
            "next" => next_symbol,
            other => panic!("unexpected reference {other}"),
        };
        resolutions.insert(ResolvedName::new(reference.reference, symbol));
    }
    let bindings = [
        LocalBinding::new(
            LocalBindingKey::new(LocalScopeId::from_raw(0), value_symbol),
            parsed.local_declarations[0].declaration,
            LocalBindingKind::Immutable,
        ),
        LocalBinding::new(
            LocalBindingKey::new(LocalScopeId::from_raw(0), next_symbol),
            parsed.local_declarations[1].declaration,
            LocalBindingKind::Var,
        ),
    ];

    let (_arena, report) = type_m0028_executable_core(
        &parsed.arena,
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.integer_literals,
        &parsed.grouped_expressions,
        &parsed.unary_expressions,
        &parsed.binary_expressions,
        &parsed.assignment_statements,
        &resolutions,
        &bindings,
    );

    assert_eq!(report.diagnostics(), &[]);
    assert_eq!(report.assignment_checks().len(), 3);
    assert!(
        report
            .diagnostics()
            .iter()
            .all(|diagnostic| diagnostic.rule() != TypeRuleDiagnostic::BinaryExpressionDeferred)
    );
}

#[test]
fn m0028_core_rejects_non_int_operator_operands_without_generic_deferral() {
    let parsed = parse_source(
        SourceFileId::from_raw(94),
        "fun run() { const bad: Int = true + 1; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let (_arena, report) = type_m0028_executable_core(
        &parsed.arena,
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.integer_literals,
        &parsed.grouped_expressions,
        &parsed.unary_expressions,
        &parsed.binary_expressions,
        &parsed.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );

    assert!(report.diagnostics().iter().any(|diagnostic| {
        diagnostic.kind() == TypeCheckDiagnosticKind::TypeMismatch
            && diagnostic.expected_type() == Some(TypeId::from_raw(1))
            && diagnostic.actual_type() == Some(TypeId::from_raw(0))
    }));
    assert!(
        report.diagnostics().iter().all(|diagnostic| {
            diagnostic.rule() != TypeRuleDiagnostic::BinaryExpressionDeferred
        })
    );
}

#[test]
fn m0028_core_keeps_non_executable_operators_deferred() {
    let parsed = parse_source(
        SourceFileId::from_raw(95),
        "fun run() { const logical = !true; const comparison = 1 == 2; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let (_arena, report) = type_m0028_executable_core(
        &parsed.arena,
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.integer_literals,
        &parsed.grouped_expressions,
        &parsed.unary_expressions,
        &parsed.binary_expressions,
        &parsed.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );

    let rules: Vec<_> = report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.rule())
        .collect();
    assert!(rules.contains(&TypeRuleDiagnostic::UnaryExpressionDeferred));
    assert!(rules.contains(&TypeRuleDiagnostic::BinaryExpressionDeferred));
}

#[test]
fn m0028_static_integer_diagnostics_cover_adr0043_failures() {
    let parsed = parse_source(
        SourceFileId::from_raw(97),
        "fun run() { const range = 9223372036854775808; const overflow = 9223372036854775807 + 1; const zero = 1 / 0; const exponent = 2 ** -1; const shift = 1 << 64; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let diagnostics = type_m0028_static_integer_diagnostics(
        &parsed.integer_literals,
        &parsed.grouped_expressions,
        &parsed.unary_expressions,
        &parsed.binary_expressions,
    );
    let rules: Vec<_> = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.rule())
        .collect();
    assert!(rules.contains(&TypeRuleDiagnostic::IntegerLiteralOutOfRange));
    assert!(rules.contains(&TypeRuleDiagnostic::IntegerOverflow));
    assert!(rules.contains(&TypeRuleDiagnostic::DivisionByZero));
    assert!(rules.contains(&TypeRuleDiagnostic::NegativeExponent));
    assert!(rules.contains(&TypeRuleDiagnostic::InvalidShiftCount));
}

#[test]
fn m0028_static_integer_diagnostics_accept_min_int_and_ignore_nonconstant_trees() {
    let parsed = parse_source(
        SourceFileId::from_raw(98),
        "fun run() { const min = -9223372036854775808; const value: Int = 1; const deferred = value + 9223372036854775808; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let diagnostics = type_m0028_static_integer_diagnostics(
        &parsed.integer_literals,
        &parsed.grouped_expressions,
        &parsed.unary_expressions,
        &parsed.binary_expressions,
    );
    assert_eq!(
        diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.rule() == TypeRuleDiagnostic::IntegerLiteralOutOfRange)
            .count(),
        1
    );
    assert!(diagnostics.iter().all(|diagnostic| {
        diagnostic.rule() != TypeRuleDiagnostic::IntegerOverflow
            && diagnostic.rule() != TypeRuleDiagnostic::DivisionByZero
            && diagnostic.rule() != TypeRuleDiagnostic::NegativeExponent
            && diagnostic.rule() != TypeRuleDiagnostic::InvalidShiftCount
    }));
}

#[test]
fn m0028_static_integer_diagnostics_accept_every_bootstrap_integer_operator() {
    let parsed = parse_source(
        SourceFileId::from_raw(99),
        "fun run() { const plus = 1 + 2; const minus = 3 - 2; const product = 2 * 3; const quotient = 8 / 2; const remainder = 7 % 3; const power = 2 ** 8; const left = 1 << 4; const right = -8 >> 2; const and = 6 & 3; const or = 6 | 3; const xor = 6 ^ 3; const unaryPlus = +1; const unaryMinus = -1; const inverted = ~1; const min = -(9223372036854775808); }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let diagnostics = type_m0028_static_integer_diagnostics(
        &parsed.integer_literals,
        &parsed.grouped_expressions,
        &parsed.unary_expressions,
        &parsed.binary_expressions,
    );
    assert_eq!(diagnostics, []);
}

#[test]
fn m0028_entry_point_selects_one_valid_main_in_the_explicit_package() {
    let entry_file = parse_source(SourceFileId::from_raw(102), "fun main(): Int { return 0; }");
    let other_file = parse_source(SourceFileId::from_raw(103), "fun main(): Int { return 1; }");
    let entry_package = PackageNamespace::parse("app").unwrap();
    let other_package = PackageNamespace::parse("other").unwrap();
    let files = [
        EntryPointFile::new(&entry_package, &entry_file),
        EntryPointFile::new(&other_package, &other_file),
    ];

    let report = check_m0028_entry_point(&entry_package, &files);

    assert_eq!(report.diagnostics(), []);
    assert_eq!(
        report.entry_point().map(|entry| entry.declaration()),
        Some(entry_file.function_declarations[0].declaration)
    );
}

#[test]
fn m0028_entry_point_diagnoses_missing_duplicate_and_invalid_candidates() {
    let package = PackageNamespace::parse("app").unwrap();
    let missing = parse_source(
        SourceFileId::from_raw(104),
        "fun helper(): Int { return 0; }",
    );
    let duplicate_first =
        parse_source(SourceFileId::from_raw(105), "fun main(): Int { return 0; }");
    let duplicate_second =
        parse_source(SourceFileId::from_raw(106), "fun main(): Int { return 1; }");
    let invalid = parse_source(
        SourceFileId::from_raw(107),
        "fun main(value: Int): String { return \"bad\"; }",
    );

    let missing_report =
        check_m0028_entry_point(&package, &[EntryPointFile::new(&package, &missing)]);
    assert!(
        missing_report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.kind() == EntryPointDiagnosticKind::MissingEntryPoint)
    );
    assert_eq!(missing_report.entry_point(), None);

    let duplicate_report = check_m0028_entry_point(
        &package,
        &[
            EntryPointFile::new(&package, &duplicate_first),
            EntryPointFile::new(&package, &duplicate_second),
        ],
    );
    assert_eq!(
        duplicate_report
            .diagnostics()
            .iter()
            .filter(|diagnostic| diagnostic.kind() == EntryPointDiagnosticKind::DuplicateEntryPoint)
            .count(),
        2
    );
    assert_eq!(duplicate_report.entry_point(), None);

    let invalid_report =
        check_m0028_entry_point(&package, &[EntryPointFile::new(&package, &invalid)]);
    assert!(invalid_report.diagnostics().iter().any(|diagnostic| {
        diagnostic.kind() == EntryPointDiagnosticKind::InvalidEntryPointSignature
            && diagnostic.source_span()
                == Some(
                    invalid
                        .arena
                        .node(invalid.function_declarations[0].declaration)
                        .unwrap()
                        .span,
                )
    }));
    assert_eq!(invalid_report.entry_point(), None);
}

#[test]
fn m0028_entry_point_rejects_every_non_entry_main_shape() {
    let package = PackageNamespace::parse("app").unwrap();
    for (file, source) in [
        (108, "fun main() { return 0; }"),
        (109, "fun main(): Int;"),
        (110, "struct Container { fun main(): Int { return 0; } }"),
    ] {
        let parsed = parse_source(SourceFileId::from_raw(file), source);
        assert!(parsed.diagnostics.is_empty());
        let report = check_m0028_entry_point(&package, &[EntryPointFile::new(&package, &parsed)]);

        if file == 110 {
            assert!(report.diagnostics().iter().any(|diagnostic| {
                diagnostic.kind() == EntryPointDiagnosticKind::MissingEntryPoint
            }));
        } else {
            assert!(report.diagnostics().iter().any(|diagnostic| {
                diagnostic.kind() == EntryPointDiagnosticKind::InvalidEntryPointSignature
            }));
        }
        assert_eq!(report.entry_point(), None);
    }
}

#[test]
fn m0028_straight_line_return_validation_reports_missing_and_unreachable_returns() {
    let parsed = parse_source(
        SourceFileId::from_raw(112),
        "fun valid(): Int { return 1; } fun missing(): Int { const value: Int = 1; } fun duplicate(): Int { return 1; return 2; } fun nested(): Int { if (true) { return 1; }; }",
    );
    assert!(parsed.diagnostics.is_empty());

    let report = check_m0028_straight_line_returns(&parsed);
    let kinds: Vec<_> = report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.kind())
        .collect();
    assert_eq!(
        kinds
            .iter()
            .filter(|kind| **kind == ReturnPathDiagnosticKind::MissingReturn)
            .count(),
        2
    );
    assert_eq!(
        kinds
            .iter()
            .filter(|kind| **kind == ReturnPathDiagnosticKind::UnreachableReturn)
            .count(),
        1
    );
    assert_eq!(
        report
            .diagnostics()
            .iter()
            .find(|diagnostic| diagnostic.kind() == ReturnPathDiagnosticKind::UnreachableReturn)
            .unwrap()
            .node(),
        parsed.return_statements[2].statement
    );
}

#[test]
fn m0028_function_signatures_type_explicit_int_parameters_and_returns() {
    let parsed = parse_source(
        SourceFileId::from_raw(113),
        "fun add(left: Int, right: Int): Int { return left + right; }",
    );
    assert!(parsed.diagnostics.is_empty());

    let (types, signatures) = type_m0028_function_signatures(
        &parsed.function_declarations,
        &parsed.function_parameters,
        &parsed.type_name_references,
    );
    assert_eq!(signatures.len(), 1);
    assert_eq!(signatures[0].parameter_types().len(), 2);
    assert_eq!(signatures[0].return_type(), TypeId::from_raw(1));
    assert!(types.records().get(TypeId::from_raw(1).index()).is_some());
}

#[test]
fn m0028_function_signatures_share_the_caller_owned_module_arena() {
    let first = parse_source(
        SourceFileId::from_raw(114),
        "fun first(value: Int): Int { return value; }",
    );
    let second = parse_source(
        SourceFileId::from_raw(115),
        "fun second(): Int { return 1; }",
    );
    let mut types = TypeArena::new();

    let first_signatures = type_m0028_function_signatures_in(
        &mut types,
        &first.function_declarations,
        &first.function_parameters,
        &first.type_name_references,
    );
    let second_signatures = type_m0028_function_signatures_in(
        &mut types,
        &second.function_declarations,
        &second.function_parameters,
        &second.type_name_references,
    );

    assert_eq!(types.records().len(), 5);
    assert_eq!(
        first_signatures[0].return_type(),
        second_signatures[0].return_type()
    );
}

#[test]
fn m0028_executable_expression_types_share_the_caller_owned_module_arena() {
    let first = parse_source(
        SourceFileId::from_raw(116),
        "fun first() { const value: Int = 1; }",
    );
    let second = parse_source(
        SourceFileId::from_raw(117),
        "fun second() { const value: Int = 2; }",
    );
    let mut types = TypeArena::new();
    let first_report = type_m0028_executable_core_in(
        &mut types,
        &first.arena,
        &first.local_declarations,
        &first.type_name_references,
        &first.literal_expressions,
        &first.integer_literals,
        &first.grouped_expressions,
        &first.unary_expressions,
        &first.binary_expressions,
        &first.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );
    let second_report = type_m0028_executable_core_in(
        &mut types,
        &second.arena,
        &second.local_declarations,
        &second.type_name_references,
        &second.literal_expressions,
        &second.integer_literals,
        &second.grouped_expressions,
        &second.unary_expressions,
        &second.binary_expressions,
        &second.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );

    assert_eq!(types.records().len(), 5);
    assert_eq!(
        first_report.expression_types()[0].ty(),
        second_report.expression_types()[0].ty()
    );
}

#[test]
fn m0028_direct_calls_type_same_package_helper_arguments() {
    let helper = parse_source(
        SourceFileId::from_raw(118),
        "fun helper(value: Int): Int { return value; }",
    );
    let caller = parse_source(
        SourceFileId::from_raw(119),
        "fun main(): Int { return helper(1); }",
    );
    let package = PackageNamespace::parse("app").unwrap();
    let mut types = TypeArena::new();
    let helper_signatures = type_m0028_function_signatures_in(
        &mut types,
        &helper.function_declarations,
        &helper.function_parameters,
        &helper.type_name_references,
    );
    let caller_signatures = type_m0028_function_signatures_in(
        &mut types,
        &caller.function_declarations,
        &caller.function_parameters,
        &caller.type_name_references,
    );
    let helper_types = type_m0028_executable_core_in(
        &mut types,
        &helper.arena,
        &helper.local_declarations,
        &helper.type_name_references,
        &helper.literal_expressions,
        &helper.integer_literals,
        &helper.grouped_expressions,
        &helper.unary_expressions,
        &helper.binary_expressions,
        &helper.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );
    let caller_types = type_m0028_executable_core_in(
        &mut types,
        &caller.arena,
        &caller.local_declarations,
        &caller.type_name_references,
        &caller.literal_expressions,
        &caller.integer_literals,
        &caller.grouped_expressions,
        &caller.unary_expressions,
        &caller.binary_expressions,
        &caller.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );
    let report = check_m0028_direct_calls(&[
        ExecutableSourceTypes::new(
            &package,
            &helper,
            &helper_signatures,
            helper_types.expression_types(),
        ),
        ExecutableSourceTypes::new(
            &package,
            &caller,
            &caller_signatures,
            caller_types.expression_types(),
        ),
    ]);

    assert_eq!(report.diagnostics(), []);
    assert_eq!(report.expression_types().len(), 1);
}

#[test]
fn m0028_direct_calls_report_invalid_target_and_arity() {
    let target = parse_source(
        SourceFileId::from_raw(120),
        "fun helper(value: Int): Int { return value; }",
    );
    let caller = parse_source(
        SourceFileId::from_raw(121),
        "fun main(): Int { return missing(); } fun other(): Int { return helper(); }",
    );
    let package = PackageNamespace::parse("app").unwrap();
    let mut types = TypeArena::new();
    let target_signatures = type_m0028_function_signatures_in(
        &mut types,
        &target.function_declarations,
        &target.function_parameters,
        &target.type_name_references,
    );
    let caller_signatures = type_m0028_function_signatures_in(
        &mut types,
        &caller.function_declarations,
        &caller.function_parameters,
        &caller.type_name_references,
    );
    let target_types = type_m0028_executable_core_in(
        &mut types,
        &target.arena,
        &target.local_declarations,
        &target.type_name_references,
        &target.literal_expressions,
        &target.integer_literals,
        &target.grouped_expressions,
        &target.unary_expressions,
        &target.binary_expressions,
        &target.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );
    let caller_types = type_m0028_executable_core_in(
        &mut types,
        &caller.arena,
        &caller.local_declarations,
        &caller.type_name_references,
        &caller.literal_expressions,
        &caller.integer_literals,
        &caller.grouped_expressions,
        &caller.unary_expressions,
        &caller.binary_expressions,
        &caller.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );
    let report = check_m0028_direct_calls(&[
        ExecutableSourceTypes::new(
            &package,
            &target,
            &target_signatures,
            target_types.expression_types(),
        ),
        ExecutableSourceTypes::new(
            &package,
            &caller,
            &caller_signatures,
            caller_types.expression_types(),
        ),
    ]);
    let kinds: Vec<_> = report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.kind())
        .collect();
    assert!(kinds.contains(&DirectCallDiagnosticKind::InvalidCallTarget));
    assert!(kinds.contains(&DirectCallDiagnosticKind::ArgumentCountMismatch));
}

#[test]
fn m0028_direct_calls_reject_every_edge_in_a_recursive_cycle() {
    let parsed = parse_source(
        SourceFileId::from_raw(122),
        "fun first(): Int { return second(); } fun second(): Int { return third(); } fun third(): Int { return first(); }",
    );
    let package = PackageNamespace::parse("app").unwrap();
    let mut types = TypeArena::new();
    let signatures = type_m0028_function_signatures_in(
        &mut types,
        &parsed.function_declarations,
        &parsed.function_parameters,
        &parsed.type_name_references,
    );
    let executable_types = type_m0028_executable_core_in(
        &mut types,
        &parsed.arena,
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.integer_literals,
        &parsed.grouped_expressions,
        &parsed.unary_expressions,
        &parsed.binary_expressions,
        &parsed.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );

    let report = check_m0028_direct_calls(&[ExecutableSourceTypes::new(
        &package,
        &parsed,
        &signatures,
        executable_types.expression_types(),
    )]);

    assert_eq!(report.expression_types(), []);
    assert_eq!(report.diagnostics().len(), 3);
    assert!(
        report
            .diagnostics()
            .iter()
            .all(|diagnostic| diagnostic.kind()
                == DirectCallDiagnosticKind::RecursiveCallUnsupported)
    );
}

#[test]
fn m0028_direct_calls_reject_mismatched_arguments_and_declarations_without_bodies() {
    let target = parse_source(
        SourceFileId::from_raw(123),
        "fun helper(value: Int): Int { return value; } fun declared(): Int;",
    );
    let caller = parse_source(
        SourceFileId::from_raw(124),
        "fun main(): Int { return helper(true); } fun other(): Int { return declared(); }",
    );
    let package = PackageNamespace::parse("app").unwrap();
    let mut types = TypeArena::new();
    let target_signatures = type_m0028_function_signatures_in(
        &mut types,
        &target.function_declarations,
        &target.function_parameters,
        &target.type_name_references,
    );
    let caller_signatures = type_m0028_function_signatures_in(
        &mut types,
        &caller.function_declarations,
        &caller.function_parameters,
        &caller.type_name_references,
    );
    let target_types = type_m0028_executable_core_in(
        &mut types,
        &target.arena,
        &target.local_declarations,
        &target.type_name_references,
        &target.literal_expressions,
        &target.integer_literals,
        &target.grouped_expressions,
        &target.unary_expressions,
        &target.binary_expressions,
        &target.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );
    let caller_types = type_m0028_executable_core_in(
        &mut types,
        &caller.arena,
        &caller.local_declarations,
        &caller.type_name_references,
        &caller.literal_expressions,
        &caller.integer_literals,
        &caller.grouped_expressions,
        &caller.unary_expressions,
        &caller.binary_expressions,
        &caller.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );

    let report = check_m0028_direct_calls(&[
        ExecutableSourceTypes::new(
            &package,
            &target,
            &target_signatures,
            target_types.expression_types(),
        ),
        ExecutableSourceTypes::new(
            &package,
            &caller,
            &caller_signatures,
            caller_types.expression_types(),
        ),
    ]);

    assert_eq!(report.expression_types(), []);
    assert_eq!(report.diagnostics().len(), 2);
    assert!(
        report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.kind() == DirectCallDiagnosticKind::ArgumentTypeMismatch)
    );
    assert!(
        report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.kind() == DirectCallDiagnosticKind::InvalidCallTarget)
    );
}

#[test]
fn m0028_executable_core_accepts_checked_direct_calls() {
    let helper = parse_source(
        SourceFileId::from_raw(125),
        "fun increment(value: Int): Int { return value + 1; }",
    );
    let caller = parse_source(
        SourceFileId::from_raw(126),
        "fun main(): Int { return increment(41); }",
    );
    let package = PackageNamespace::parse("app").unwrap();
    let mut types = TypeArena::new();
    let helper_signatures = type_m0028_function_signatures_in(
        &mut types,
        &helper.function_declarations,
        &helper.function_parameters,
        &helper.type_name_references,
    );
    let caller_signatures = type_m0028_function_signatures_in(
        &mut types,
        &caller.function_declarations,
        &caller.function_parameters,
        &caller.type_name_references,
    );
    let helper_types = type_m0028_executable_core_in(
        &mut types,
        &helper.arena,
        &helper.local_declarations,
        &helper.type_name_references,
        &helper.literal_expressions,
        &helper.integer_literals,
        &helper.grouped_expressions,
        &helper.unary_expressions,
        &helper.binary_expressions,
        &helper.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );
    let mut caller_types = type_m0028_executable_core_in(
        &mut types,
        &caller.arena,
        &caller.local_declarations,
        &caller.type_name_references,
        &caller.literal_expressions,
        &caller.integer_literals,
        &caller.grouped_expressions,
        &caller.unary_expressions,
        &caller.binary_expressions,
        &caller.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );
    let calls = check_m0028_direct_calls(&[
        ExecutableSourceTypes::new(
            &package,
            &helper,
            &helper_signatures,
            helper_types.expression_types(),
        ),
        ExecutableSourceTypes::new(
            &package,
            &caller,
            &caller_signatures,
            caller_types.expression_types(),
        ),
    ]);

    apply_m0028_direct_call_results(&mut caller_types, &caller, &calls);

    let call = &caller.call_expressions[0];
    assert_eq!(
        caller_types.expression_type(call.expression),
        Some(calls.expression_types()[0].ty())
    );
    assert!(!caller_types.diagnostics().iter().any(|diagnostic| {
        diagnostic.kind() == TypeCheckDiagnosticKind::UnsupportedTypeRule
            && diagnostic.rule() == TypeRuleDiagnostic::DirectCallDeferred
            && diagnostic.node() == call.expression
    }));
}

#[test]
fn m0028_executable_core_keeps_invalid_direct_calls_deferred() {
    let parsed = parse_source(
        SourceFileId::from_raw(127),
        "fun main(): Int { return missing(); }",
    );
    let package = PackageNamespace::parse("app").unwrap();
    let mut types = TypeArena::new();
    let signatures = type_m0028_function_signatures_in(
        &mut types,
        &parsed.function_declarations,
        &parsed.function_parameters,
        &parsed.type_name_references,
    );
    let mut executable_types = type_m0028_executable_core_in(
        &mut types,
        &parsed.arena,
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.integer_literals,
        &parsed.grouped_expressions,
        &parsed.unary_expressions,
        &parsed.binary_expressions,
        &parsed.assignment_statements,
        &ResolutionTable::new(),
        &[],
    );
    let calls = check_m0028_direct_calls(&[ExecutableSourceTypes::new(
        &package,
        &parsed,
        &signatures,
        executable_types.expression_types(),
    )]);

    apply_m0028_direct_call_results(&mut executable_types, &parsed, &calls);

    let call = &parsed.call_expressions[0];
    assert_eq!(executable_types.expression_type(call.expression), None);
    assert!(executable_types.diagnostics().iter().any(|diagnostic| {
        diagnostic.kind() == TypeCheckDiagnosticKind::UnsupportedTypeRule
            && diagnostic.rule() == TypeRuleDiagnostic::DirectCallDeferred
            && diagnostic.node() == call.expression
    }));
}
