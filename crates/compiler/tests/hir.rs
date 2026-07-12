use compiler::{
    hir::{
        CheckedHirSource, HirAssignment, HirBinaryOperator, HirDirectCall, HirExpression,
        HirExpressionId, HirExpressionKind, HirFunction, HirFunctionId, HirLocal, HirLocalId,
        HirModule, HirParameter, HirParameterId, HirReturn, HirSafetyFacts, HirUnaryOperator,
        HirUnsupportedForm, lower_checked_hir_source,
    },
    module::{ModuleName, PackageNamespace},
    parser::parse_source,
    source::{ByteSpan, SourceFileId},
    type_check::{
        ExecutableSourceTypes, ExpressionType, FunctionSignature, apply_direct_call_results,
        check_direct_calls, type_executable_core_in, type_function_signatures_in,
        type_parser_literals,
    },
    types::{GenericSpecializationIdentity, TypeId},
};

#[test]
fn hir_model_preserves_typed_source_mapped_executable_facts() {
    let file = SourceFileId::from_raw(200);
    let function_span = ByteSpan::new(file, 0, 48).unwrap();
    let parameter_span = ByteSpan::new(file, 14, 24).unwrap();
    let local_span = ByteSpan::new(file, 27, 38).unwrap();
    let argument_span = ByteSpan::new(file, 42, 43).unwrap();
    let call_span = ByteSpan::new(file, 35, 44).unwrap();
    let return_span = ByteSpan::new(file, 45, 47).unwrap();
    let int = TypeId::from_raw(1);
    let function = HirFunction::new(
        HirFunctionId::from_raw(0),
        ModuleName::parse("app").unwrap(),
        PackageNamespace::parse("app").unwrap(),
        function_span,
        true,
        int,
        vec![HirParameter::new(
            HirParameterId::from_raw(0),
            parameter_span,
            int,
        )],
        vec![HirLocal::new(
            HirLocalId::from_raw(0),
            local_span,
            int,
            false,
        )],
        vec![
            HirExpression::int_literal(HirExpressionId::from_raw(0), argument_span, int, 1),
            HirExpression::direct_call(
                HirExpressionId::from_raw(1),
                call_span,
                int,
                HirDirectCall::new(
                    HirFunctionId::from_raw(1),
                    vec![HirExpressionId::from_raw(0)],
                ),
            ),
        ],
        vec![HirReturn::new(return_span, HirExpressionId::from_raw(1))],
        HirSafetyFacts::executable_subset_checked(),
        vec![HirUnsupportedForm::new(
            ByteSpan::new(file, 50, 60).unwrap(),
        )],
    );
    let module = HirModule::new(ModuleName::parse("app").unwrap(), vec![function]);

    let function = &module.functions()[0];
    assert!(function.is_entry());
    assert_eq!(function.parameters()[0].span(), parameter_span);
    assert!(!function.locals()[0].is_mutable());
    assert_eq!(function.expressions()[1].ty(), int);
    assert_eq!(
        function
            .direct_call(HirExpressionId::from_raw(1))
            .unwrap()
            .arguments(),
        &[HirExpressionId::from_raw(0)]
    );
    assert_eq!(
        function.returns()[0].expression(),
        HirExpressionId::from_raw(1)
    );
    assert!(function.safety_facts().is_executable_subset_checked());
    assert_eq!(
        function.unsupported_forms()[0].span(),
        ByteSpan::new(file, 50, 60).unwrap()
    );
}

#[test]
fn hir_preserves_specialization_identity() {
    let file = SourceFileId::from_raw(1085);
    let span = ByteSpan::new(file, 0, 1).unwrap();
    let identity = GenericSpecializationIdentity::new(
        compiler::ast::AstNodeId::from_raw(1085),
        vec![TypeId::from_raw(1)],
    );
    let function = HirFunction::new(
        HirFunctionId::from_raw(0),
        ModuleName::parse("specialized").unwrap(),
        PackageNamespace::root(),
        span,
        false,
        TypeId::from_raw(1),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        HirSafetyFacts::executable_subset_checked(),
        Vec::new(),
    )
    .with_specialization_identity(identity.clone());
    assert_eq!(function.specialization_identity(), Some(&identity));
}

#[test]
fn hir_preserves_function_reference_and_indirect_call_kinds() {
    let file = SourceFileId::from_raw(1087);
    let span = ByteSpan::new(file, 0, 1).unwrap();
    let int = TypeId::from_raw(1);
    let function_type = TypeId::from_raw(2);
    let reference = HirExpression::function_reference(
        HirExpressionId::from_raw(0),
        span,
        function_type,
        HirFunctionId::from_raw(3),
    );
    let call = HirExpression::indirect_call(
        HirExpressionId::from_raw(1),
        span,
        int,
        reference.id(),
        Vec::new(),
    );
    assert!(matches!(
        reference.kind(),
        HirExpressionKind::FunctionReference(_)
    ));
    assert!(matches!(
        call.kind(),
        HirExpressionKind::IndirectCall { .. }
    ));
}

#[test]
fn hir_preserves_non_integer_primitive_literal_payloads() {
    let file = SourceFileId::from_raw(904);
    let span = ByteSpan::new(file, 0, 4).unwrap();
    let bool_type = TypeId::from_raw(1);
    let unit_type = TypeId::from_raw(2);
    let float_type = TypeId::from_raw(3);
    let byte_type = TypeId::from_raw(4);

    let bool_literal =
        HirExpression::bool_literal(HirExpressionId::from_raw(0), span, bool_type, true);
    let unit_literal = HirExpression::unit_literal(HirExpressionId::from_raw(1), span, unit_type);
    let float_literal =
        HirExpression::float_literal(HirExpressionId::from_raw(2), span, float_type, -0.0);
    let byte_literal =
        HirExpression::byte_literal(HirExpressionId::from_raw(3), span, byte_type, 255);

    assert_eq!(bool_literal.kind(), &HirExpressionKind::BoolLiteral(true));
    assert_eq!(unit_literal.kind(), &HirExpressionKind::UnitLiteral);
    assert_eq!(
        float_literal.kind(),
        &HirExpressionKind::FloatLiteral((-0.0f64).to_bits())
    );
    assert_eq!(byte_literal.kind(), &HirExpressionKind::ByteLiteral(255));
    assert_eq!(float_literal.ty(), float_type);
    assert_eq!(byte_literal.span(), span);
}

#[test]
fn hir_preserves_channel_operation_kinds() {
    let file = SourceFileId::from_raw(1210);
    let span = ByteSpan::new(file, 0, 4).unwrap();
    let channel = TypeId::from_raw(10);
    let unit = TypeId::from_raw(11);
    let result = TypeId::from_raw(12);
    let capacity = HirExpressionId::from_raw(0);
    let value = HirExpressionId::from_raw(1);
    let channel_value = HirExpressionId::from_raw(2);
    assert!(matches!(
        HirExpression::channel_create(HirExpressionId::from_raw(3), span, channel, capacity).kind(),
        HirExpressionKind::ChannelCreate(id) if *id == capacity
    ));
    assert!(matches!(
        HirExpression::channel_send(HirExpressionId::from_raw(4), span, unit, channel_value, value).kind(),
        HirExpressionKind::ChannelSend { channel: actual_channel, value: actual_value }
            if *actual_channel == channel_value && *actual_value == value
    ));
    assert!(matches!(
        HirExpression::channel_receive(HirExpressionId::from_raw(5), span, result, channel_value).kind(),
        HirExpressionKind::ChannelReceive(id) if *id == channel_value
    ));
    assert!(matches!(
        HirExpression::channel_close(HirExpressionId::from_raw(6), span, unit, channel_value).kind(),
        HirExpressionKind::ChannelClose(id) if *id == channel_value
    ));
}

#[test]
fn checked_source_lowers_bool_unit_and_float_literals_to_hir() {
    let file = SourceFileId::from_raw(907);
    let parsed = parse_source(
        file,
        "func flag(): Bool { return true; } func done(): Unit { return (); } func ratio(): Float { return 0.0; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());

    let (types, report) = type_parser_literals(&parsed.literal_expressions);
    let signatures = vec![
        FunctionSignature::new(
            parsed.function_declarations[0].declaration,
            vec![],
            TypeId::from_raw(0),
        ),
        FunctionSignature::new(
            parsed.function_declarations[1].declaration,
            vec![],
            TypeId::from_raw(3),
        ),
        FunctionSignature::new(
            parsed.function_declarations[2].declaration,
            vec![],
            TypeId::from_raw(5),
        ),
    ];
    let module = lower_checked_hir_source(CheckedHirSource::new(
        ModuleName::parse("examples").unwrap(),
        PackageNamespace::parse("current").unwrap(),
        &parsed,
        &signatures,
        report.expression_types(),
        true,
    ))
    .unwrap();

    assert!(matches!(
        module.functions()[0].expressions()[0].kind(),
        HirExpressionKind::BoolLiteral(true)
    ));
    assert!(matches!(
        module.functions()[1].expressions()[0].kind(),
        HirExpressionKind::UnitLiteral
    ));
    assert!(matches!(
        module.functions()[2].expressions()[0].kind(),
        HirExpressionKind::FloatLiteral(bits) if *bits == 0.0f64.to_bits()
    ));
    let _ = types;
}

#[test]
fn hir_preserves_primitive_operator_kinds_and_operand_order() {
    let file = SourceFileId::from_raw(909);
    let span = ByteSpan::new(file, 0, 5).unwrap();
    let bool_type = TypeId::from_raw(0);
    let left = HirExpressionId::from_raw(0);
    let right = HirExpressionId::from_raw(1);
    let logical = HirExpression::binary(
        HirExpressionId::from_raw(2),
        span,
        bool_type,
        HirBinaryOperator::LogicalAnd,
        left,
        right,
    );
    let not = HirExpression::unary(
        HirExpressionId::from_raw(3),
        span,
        bool_type,
        HirUnaryOperator::Not,
        left,
    );

    assert!(matches!(logical.kind(), HirExpressionKind::Binary(binary)
        if binary.operator() == HirBinaryOperator::LogicalAnd
            && binary.left() == left
            && binary.right() == right));
    assert!(matches!(not.kind(), HirExpressionKind::Unary(unary)
        if unary.operator() == HirUnaryOperator::Not && unary.operand() == left));
}

#[test]
fn checked_source_transports_contextual_byte_literal_to_hir() {
    let parsed = parse_source(
        SourceFileId::from_raw(918),
        "func main(): Byte { return 255; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());
    let mut types = compiler::types::TypeArena::new();
    let mut report = type_executable_core_in(
        &mut types,
        &parsed.arena,
        &[],
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.integer_literals,
        &parsed.grouped_expressions,
        &parsed.unary_expressions,
        &parsed.binary_expressions,
        &parsed.assignment_statements,
        &compiler::name_resolution::ResolutionTable::new(),
        &[],
    );
    assert!(
        report.diagnostics().is_empty(),
        "{:?}",
        report.diagnostics()
    );
    let return_expression = parsed.return_statements[0].value.unwrap();
    report.replace_expression_type(ExpressionType::new(return_expression, TypeId::from_raw(6)));
    let signatures = vec![FunctionSignature::new(
        parsed.function_declarations[0].declaration,
        vec![],
        TypeId::from_raw(6),
    )];
    let module = lower_checked_hir_source(
        CheckedHirSource::new(
            ModuleName::parse("app").unwrap(),
            PackageNamespace::parse("app").unwrap(),
            &parsed,
            &signatures,
            report.expression_types(),
            true,
        )
        .with_byte_type(TypeId::from_raw(6)),
    )
    .unwrap();

    assert!(matches!(
        module.functions()[0].expressions()[0].kind(),
        HirExpressionKind::ByteLiteral(255)
    ));
}

#[test]
fn checked_source_lowers_float_local_and_read_to_hir() {
    let parsed = parse_source(
        SourceFileId::from_raw(919),
        "func ratio(): Float { const value: Float = 1.5; return value; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());
    let mut types = compiler::types::TypeArena::new();
    let mut report = type_executable_core_in(
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
        &compiler::name_resolution::ResolutionTable::new(),
        &[],
    );
    let name = parsed
        .name_references
        .iter()
        .find(|name| name.name == "value")
        .unwrap();
    report.record_expression_type(ExpressionType::new(name.reference, TypeId::from_raw(5)));
    let signatures = type_function_signatures_in(
        &mut types,
        &parsed.function_declarations,
        &parsed.function_parameters,
        &parsed.type_name_references,
    );

    let module = lower_checked_hir_source(CheckedHirSource::new(
        ModuleName::parse("app").unwrap(),
        PackageNamespace::parse("app").unwrap(),
        &parsed,
        &signatures,
        report.expression_types(),
        true,
    ))
    .unwrap();

    assert_eq!(module.functions()[0].locals().len(), 1);
    assert!(matches!(
        module.functions()[0].expressions()[0].kind(),
        HirExpressionKind::FloatLiteral(bits) if *bits == 1.5f64.to_bits()
    ));
    assert!(matches!(
        module.functions()[0].expressions()[1].kind(),
        HirExpressionKind::LocalRead(_)
    ));
}

#[test]
fn hir_executable_expressions_preserve_ordered_operands_and_assignments() {
    let file = SourceFileId::from_raw(201);
    let span = ByteSpan::new(file, 0, 1).unwrap();
    let int = TypeId::from_raw(1);
    let function = HirFunction::new(
        HirFunctionId::from_raw(0),
        ModuleName::parse("app").unwrap(),
        PackageNamespace::parse("app").unwrap(),
        span,
        false,
        int,
        vec![],
        vec![HirLocal::new(HirLocalId::from_raw(0), span, int, true)],
        vec![
            HirExpression::local_read(
                HirExpressionId::from_raw(0),
                span,
                int,
                HirLocalId::from_raw(0),
            ),
            HirExpression::unary(
                HirExpressionId::from_raw(1),
                span,
                int,
                HirUnaryOperator::Minus,
                HirExpressionId::from_raw(0),
            ),
            HirExpression::binary(
                HirExpressionId::from_raw(2),
                span,
                int,
                HirBinaryOperator::Exponent,
                HirExpressionId::from_raw(0),
                HirExpressionId::from_raw(1),
            ),
        ],
        vec![],
        HirSafetyFacts::executable_subset_checked(),
        vec![],
    )
    .with_assignments(vec![HirAssignment::new(
        span,
        HirLocalId::from_raw(0),
        HirExpressionId::from_raw(2),
    )]);

    assert_eq!(
        function.local_read(HirExpressionId::from_raw(0)),
        Some(HirLocalId::from_raw(0))
    );
    assert_eq!(
        function
            .unary(HirExpressionId::from_raw(1))
            .unwrap()
            .operator(),
        HirUnaryOperator::Minus
    );
    let binary = function.binary(HirExpressionId::from_raw(2)).unwrap();
    assert_eq!(binary.operator(), HirBinaryOperator::Exponent);
    assert_eq!(binary.left(), HirExpressionId::from_raw(0));
    assert_eq!(binary.right(), HirExpressionId::from_raw(1));
    assert_eq!(
        function.assignments()[0].value(),
        HirExpressionId::from_raw(2)
    );
}

#[test]
fn hir_models_primitive_parameter_reads() {
    let file = SourceFileId::from_raw(924);
    let span = ByteSpan::new(file, 0, 3).unwrap();
    let float_type = TypeId::from_raw(5);
    let expression = HirExpression::parameter_read(
        HirExpressionId::from_raw(0),
        span,
        float_type,
        HirParameterId::from_raw(0),
    );

    assert!(matches!(
        expression.kind(),
        HirExpressionKind::ParameterRead(_)
    ));
}

#[test]
fn checked_source_lowers_primitive_parameter_reads() {
    let file = SourceFileId::from_raw(926);
    let parsed = parse_source(file, "func echo(value: Float): Float { return value; }");
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());
    let return_expression = parsed.return_statements[0].value.unwrap();
    let float_type = TypeId::from_raw(5);
    let expression_types = vec![ExpressionType::new(return_expression, float_type)];
    let signatures = vec![FunctionSignature::new(
        parsed.function_declarations[0].declaration,
        vec![float_type],
        float_type,
    )];

    let module = lower_checked_hir_source(CheckedHirSource::new(
        ModuleName::parse("app").unwrap(),
        PackageNamespace::parse("app").unwrap(),
        &parsed,
        &signatures,
        &expression_types,
        true,
    ))
    .unwrap();

    assert_eq!(module.functions()[0].parameters().len(), 1);
    assert!(matches!(
        module.functions()[0].expressions()[0].kind(),
        HirExpressionKind::ParameterRead(_)
    ));
}

#[test]
fn checked_source_lowers_integer_helpers_and_direct_calls() {
    let parsed = parse_source(
        SourceFileId::from_raw(203),
        "func helper(): Int { return 1 + 2; } func main(): Int { return helper(); }",
    );
    let mut types = compiler::types::TypeArena::new();
    let signatures = type_function_signatures_in(
        &mut types,
        &parsed.function_declarations,
        &parsed.function_parameters,
        &parsed.type_name_references,
    );
    let mut expressions = type_executable_core_in(
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
        &compiler::name_resolution::ResolutionTable::new(),
        &[],
    );
    let package = PackageNamespace::parse("app").unwrap();
    let calls = check_direct_calls(&[ExecutableSourceTypes::new(
        &package,
        &parsed,
        &signatures,
        expressions.expression_types(),
    )]);
    apply_direct_call_results(&mut expressions, &parsed, &calls);
    let module = lower_checked_hir_source(CheckedHirSource::new(
        ModuleName::parse("app").unwrap(),
        package,
        &parsed,
        &signatures,
        expressions.expression_types(),
        true,
    ))
    .unwrap();

    assert_eq!(module.functions().len(), 2);
    assert_eq!(module.functions()[0].expressions().len(), 3);
    assert_eq!(
        module.functions()[0].symbol_identity().unwrap().name(),
        "helper"
    );
    assert_eq!(
        module.functions()[1].symbol_identity().unwrap().name(),
        "main"
    );
    assert!(
        module.functions()[1]
            .direct_call(HirExpressionId::from_raw(0))
            .is_some()
    );
}

#[test]
fn checked_source_preserves_function_symbol_identity() {
    checked_source_lowers_integer_helpers_and_direct_calls();
}
