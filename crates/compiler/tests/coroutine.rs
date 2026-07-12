use compiler::{
    ast::AstNodeId,
    borrow::BorrowKind,
    coroutine::{
        ChildTask, ClosureCleanupEvent, CoroutineDiagnostic, CoroutineDiagnosticKind,
        StructuredTaskScope, SuspendedBorrow, SuspensionDiagnostic, SuspensionDiagnosticKind,
        SuspensionRejection, analyze_structured_task_scopes, analyze_suspended_borrows,
        closure_cleanup_facts,
    },
    name_resolution::{LocalBinding, LocalBindingKey, LocalBindingKind, LocalScopeId},
    symbol::SymbolId,
};

#[test]
fn m0089_closure_cleanup_covers_completion_and_cancellation() {
    let facts = closure_cleanup_facts(
        AstNodeId::from_raw(8800),
        &[AstNodeId::from_raw(8801), AstNodeId::from_raw(8802)],
    );

    assert_eq!(facts.len(), 4);
    assert_eq!(facts[0].closure(), AstNodeId::from_raw(8800));
    assert_eq!(facts[0].capture(), AstNodeId::from_raw(8801));
    assert_eq!(facts[0].event(), ClosureCleanupEvent::Completion);
    assert_eq!(facts[1].event(), ClosureCleanupEvent::Cancellation);
    assert_eq!(facts[2].capture(), AstNodeId::from_raw(8802));
}

#[test]
fn m0025_structured_scope_accepts_completed_children() {
    let scope = AstNodeId::from_raw(100);
    let child = ChildTask::new(AstNodeId::from_raw(101), scope, scope);

    let diagnostics =
        analyze_structured_task_scopes(&[StructuredTaskScope::new(scope, vec![child])]);

    assert!(diagnostics.is_empty());
}

#[test]
fn m0025_structured_scope_reports_child_task_escape() {
    let containing_scope = AstNodeId::from_raw(200);
    let completion_scope = AstNodeId::from_raw(201);
    let task = AstNodeId::from_raw(202);
    let child = ChildTask::new(task, containing_scope, completion_scope);

    let diagnostics =
        analyze_structured_task_scopes(&[StructuredTaskScope::new(containing_scope, vec![child])]);

    assert_eq!(
        diagnostics,
        [CoroutineDiagnostic::task_scope_escape(
            task,
            containing_scope,
            completion_scope,
        )]
    );
    assert_eq!(
        diagnostics[0].kind(),
        CoroutineDiagnosticKind::TaskScopeEscape
    );
}

#[test]
fn m0025_structured_scope_diagnostics_preserve_order_and_spans() {
    let first_scope = AstNodeId::from_raw(300);
    let second_scope = AstNodeId::from_raw(400);
    let first_task = AstNodeId::from_raw(301);
    let second_task = AstNodeId::from_raw(401);
    let first_completion = AstNodeId::from_raw(302);
    let second_completion = AstNodeId::from_raw(402);

    let diagnostics = analyze_structured_task_scopes(&[
        StructuredTaskScope::new(
            first_scope,
            vec![
                ChildTask::new(AstNodeId::from_raw(303), first_scope, first_scope),
                ChildTask::new(first_task, first_scope, first_completion),
            ],
        ),
        StructuredTaskScope::new(
            second_scope,
            vec![ChildTask::new(second_task, second_scope, second_completion)],
        ),
    ]);

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].task(), first_task);
    assert_eq!(diagnostics[0].containing_scope(), first_scope);
    assert_eq!(diagnostics[0].completion_scope(), first_completion);
    assert_eq!(diagnostics[1].task(), second_task);
    assert_eq!(diagnostics[1].containing_scope(), second_scope);
    assert_eq!(diagnostics[1].completion_scope(), second_completion);
}

#[test]
fn m0025_suspension_accepts_non_concurrent_same_scope_borrows() {
    let scope = AstNodeId::from_raw(500);
    let binding = local_binding(50, 500);

    let diagnostics = analyze_suspended_borrows(&[
        SuspendedBorrow::new(
            AstNodeId::from_raw(501),
            binding.clone(),
            AstNodeId::from_raw(502),
            BorrowKind::Shared,
            scope,
            scope,
            false,
        ),
        SuspendedBorrow::new(
            AstNodeId::from_raw(503),
            binding,
            AstNodeId::from_raw(504),
            BorrowKind::Exclusive,
            scope,
            scope,
            false,
        ),
    ]);

    assert!(diagnostics.is_empty());
}

#[test]
fn m0025_suspension_reports_concurrent_frame_access() {
    let scope = AstNodeId::from_raw(600);
    let suspension = AstNodeId::from_raw(601);
    let borrow = AstNodeId::from_raw(602);
    let binding = local_binding(60, 600);

    let diagnostics = analyze_suspended_borrows(&[SuspendedBorrow::new(
        suspension,
        binding.clone(),
        borrow,
        BorrowKind::Shared,
        scope,
        scope,
        true,
    )]);

    assert_eq!(
        diagnostics,
        [SuspensionDiagnostic::borrow_across_suspension(
            suspension,
            borrow,
            binding,
            BorrowKind::Shared,
            scope,
            scope,
            SuspensionRejection::concurrent_frame_access(),
        )]
    );
    assert_eq!(
        diagnostics[0].kind(),
        SuspensionDiagnosticKind::BorrowAcrossSuspension
    );
}

#[test]
fn m0025_suspension_reports_outliving_borrowed_value() {
    let borrowed_value_scope = AstNodeId::from_raw(700);
    let frame_scope = AstNodeId::from_raw(701);
    let suspension = AstNodeId::from_raw(702);
    let borrow = AstNodeId::from_raw(703);
    let binding = local_binding(70, 700);

    let diagnostics = analyze_suspended_borrows(&[SuspendedBorrow::new(
        suspension,
        binding.clone(),
        borrow,
        BorrowKind::Exclusive,
        borrowed_value_scope,
        frame_scope,
        false,
    )]);

    assert_eq!(
        diagnostics,
        [SuspensionDiagnostic::borrow_across_suspension(
            suspension,
            borrow,
            binding,
            BorrowKind::Exclusive,
            borrowed_value_scope,
            frame_scope,
            SuspensionRejection::outlives_borrowed_value(),
        )]
    );
    assert_eq!(diagnostics[0].suspension(), suspension);
    assert_eq!(diagnostics[0].borrow(), borrow);
    assert_eq!(diagnostics[0].borrowed_value_scope(), borrowed_value_scope);
    assert_eq!(diagnostics[0].frame_scope(), frame_scope);
    assert_eq!(
        diagnostics[0].rejection(),
        SuspensionRejection::outlives_borrowed_value()
    );
}

#[test]
fn m0025_suspension_reports_both_rejection_reasons_and_preserves_order() {
    let first_binding = local_binding(80, 800);
    let second_binding = local_binding(81, 801);
    let first_suspension = AstNodeId::from_raw(802);
    let second_suspension = AstNodeId::from_raw(803);

    let diagnostics = analyze_suspended_borrows(&[
        SuspendedBorrow::new(
            first_suspension,
            first_binding.clone(),
            AstNodeId::from_raw(804),
            BorrowKind::Shared,
            AstNodeId::from_raw(805),
            AstNodeId::from_raw(806),
            true,
        ),
        SuspendedBorrow::new(
            AstNodeId::from_raw(807),
            second_binding.clone(),
            AstNodeId::from_raw(808),
            BorrowKind::Exclusive,
            AstNodeId::from_raw(809),
            AstNodeId::from_raw(809),
            false,
        ),
        SuspendedBorrow::new(
            second_suspension,
            second_binding.clone(),
            AstNodeId::from_raw(810),
            BorrowKind::Exclusive,
            AstNodeId::from_raw(811),
            AstNodeId::from_raw(812),
            false,
        ),
    ]);

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].suspension(), first_suspension);
    assert_eq!(diagnostics[0].rejection(), SuspensionRejection::both());
    assert_eq!(diagnostics[1].suspension(), second_suspension);
    assert_eq!(diagnostics[1].binding(), &second_binding);
    assert_eq!(
        diagnostics[1].rejection(),
        SuspensionRejection::outlives_borrowed_value()
    );
}

fn local_binding(binding_raw: usize, symbol_raw: usize) -> LocalBinding {
    LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(1), SymbolId::from_raw(symbol_raw)),
        AstNodeId::from_raw(binding_raw),
        LocalBindingKind::Immutable,
    )
}
