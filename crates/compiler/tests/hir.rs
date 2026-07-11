use compiler::{
    hir::{
        CheckedHirSource, HirAssignment, HirBinaryOperator, HirDirectCall, HirExpression,
        HirExpressionId, HirFunction, HirFunctionId, HirLocal, HirLocalId, HirModule, HirParameter,
        HirParameterId, HirReturn, HirSafetyFacts, HirUnaryOperator, HirUnsupportedForm,
        lower_checked_hir_source,
    },
    module::{ModuleName, PackageNamespace},
    parser::parse_source,
    source::{ByteSpan, SourceFileId},
    type_check::{
        ExecutableSourceTypes, apply_m0028_direct_call_results, check_m0028_direct_calls,
        type_m0028_executable_core_in, type_m0028_function_signatures_in,
    },
    types::TypeId,
};

#[test]
fn m0029_hir_model_preserves_typed_source_mapped_executable_facts() {
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
fn m0029_hir_executable_expressions_preserve_ordered_operands_and_assignments() {
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
fn m0029_checked_source_lowers_integer_helpers_and_direct_calls() {
    let parsed = parse_source(
        SourceFileId::from_raw(203),
        "fun helper(): Int { return 1 + 2; } fun main(): Int { return helper(); }",
    );
    let mut types = compiler::types::TypeArena::new();
    let signatures = type_m0028_function_signatures_in(
        &mut types,
        &parsed.function_declarations,
        &parsed.function_parameters,
        &parsed.type_name_references,
    );
    let mut expressions = type_m0028_executable_core_in(
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
    let calls = check_m0028_direct_calls(&[ExecutableSourceTypes::new(
        &package,
        &parsed,
        &signatures,
        expressions.expression_types(),
    )]);
    apply_m0028_direct_call_results(&mut expressions, &parsed, &calls);
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
fn m0032_checked_source_preserves_function_symbol_identity() {
    m0029_checked_source_lowers_integer_helpers_and_direct_calls();
}
