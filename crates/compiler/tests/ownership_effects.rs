use compiler::{
    ast::AstNodeId,
    borrow::BorrowKind,
    name_resolution::{LocalBinding, LocalBindingKey, LocalBindingKind, LocalScopeId},
    ownership_effects::{
        BindingState, EffectEvent, EffectKind, EffectProjectionRegion, OwnershipEffectContract,
        OwnershipEffectDiagnosticKind, analyze_effect_events, infer_parameter_effects,
        infer_source_parameter_effects,
    },
    parser::parse_source,
    source::SourceFileId,
    symbol::SymbolId,
};

fn binding(raw: usize, mutable: bool) -> LocalBinding {
    LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(raw)),
        AstNodeId::from_raw(raw + 100),
        if mutable {
            LocalBindingKind::Var
        } else {
            LocalBindingKind::Immutable
        },
    )
}

#[test]
fn read_effect_keeps_a_move_only_binding_available() {
    let value = binding(1, false);
    let report = analyze_effect_events(
        &[(value.clone(), BindingState::Available)],
        &[EffectEvent::use_value(
            AstNodeId::from_raw(10),
            value,
            EffectKind::Read,
        )],
    );

    assert!(report.diagnostics().is_empty());
    assert_eq!(report.state(0), Some(BindingState::Available));
}

#[test]
fn consuming_effect_rejects_a_later_use() {
    let value = binding(2, false);
    let report = analyze_effect_events(
        &[(value.clone(), BindingState::Available)],
        &[
            EffectEvent::use_value(AstNodeId::from_raw(20), value.clone(), EffectKind::Consume),
            EffectEvent::use_value(AstNodeId::from_raw(21), value, EffectKind::Read),
        ],
    );

    assert_eq!(report.state(0), Some(BindingState::Consumed));
    assert_eq!(
        report.diagnostics()[0].kind(),
        OwnershipEffectDiagnosticKind::UseAfterConsumption
    );
}

#[test]
fn consuming_var_can_be_atomically_rebound() {
    let value = binding(3, true);
    let report = analyze_effect_events(
        &[(value.clone(), BindingState::Available)],
        &[EffectEvent::consume_and_rebind(
            AstNodeId::from_raw(30),
            value,
        )],
    );

    assert!(report.diagnostics().is_empty());
    assert_eq!(report.state(0), Some(BindingState::Available));
}

#[test]
fn branch_join_preserves_possible_consumption() {
    let value = binding(4, false);
    let report = analyze_effect_events(
        &[(value.clone(), BindingState::Available)],
        &[
            EffectEvent::branch(
                AstNodeId::from_raw(40),
                vec![EffectEvent::use_value(
                    AstNodeId::from_raw(41),
                    value.clone(),
                    EffectKind::Consume,
                )],
                vec![],
            ),
            EffectEvent::use_value(AstNodeId::from_raw(42), value, EffectKind::Read),
        ],
    );

    assert_eq!(report.state(0), Some(BindingState::MaybeConsumed));
    assert_eq!(
        report.diagnostics().last().unwrap().kind(),
        OwnershipEffectDiagnosticKind::PossibleUseAfterConsumption
    );
}

#[test]
fn parameter_effects_record_the_strongest_inferred_effect() {
    let effects = infer_parameter_effects(
        2,
        &[
            (0, EffectKind::Read),
            (0, EffectKind::Mutate),
            (1, EffectKind::ReturnOwned),
        ],
    );

    assert_eq!(effects[0].effect(), EffectKind::Mutate);
    assert_eq!(effects[1].effect(), EffectKind::ReturnOwned);
}

#[test]
fn source_parameter_effects_preserve_mutation_and_return_facts() {
    let parsed = parse_source(
        SourceFileId::from_raw(1005),
        "fun rebuild(value: Int): Int { value = 1; return value; }",
    );
    assert!(parsed.lex_diagnostics.is_empty());
    assert!(parsed.diagnostics.is_empty());
    let function = parsed.function_declarations[0].declaration;
    let contract = infer_source_parameter_effects(&parsed, function);

    assert!(contract.parameters()[0].contains(EffectKind::Mutate));
    assert!(contract.parameters()[0].contains(EffectKind::ReturnOwned));
}

#[test]
fn loop_join_preserves_possible_consumption_from_an_iteration() {
    let value = binding(5, false);
    let report = analyze_effect_events(
        &[(value.clone(), BindingState::Available)],
        &[EffectEvent::loop_body(
            AstNodeId::from_raw(50),
            vec![EffectEvent::use_value(
                AstNodeId::from_raw(51),
                value,
                EffectKind::Consume,
            )],
        )],
    );

    assert_eq!(report.state(0), Some(BindingState::MaybeConsumed));
}

#[test]
fn effect_contract_can_preserve_projection_regions() {
    let value = binding(6, false);
    let region = EffectProjectionRegion::new(
        AstNodeId::from_raw(61),
        value.clone(),
        vec![AstNodeId::from_raw(62)],
        BorrowKind::Exclusive,
    );
    let contract = OwnershipEffectContract::new(
        AstNodeId::from_raw(60),
        infer_parameter_effects(1, &[(0, EffectKind::Mutate)]),
        true,
    )
    .with_regions(vec![region]);
    assert_eq!(contract.regions().len(), 1);
    assert_eq!(
        contract.regions()[0].projection(),
        &[AstNodeId::from_raw(62)]
    );
    assert_eq!(contract.parameters()[0].parameter(), 0);
    assert_eq!(value.binding(), AstNodeId::from_raw(106));
}
