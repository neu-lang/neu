use compiler::{
    ast::AstNodeId,
    coroutine::{
        ChildTask, CoroutineDiagnostic, CoroutineDiagnosticKind, StructuredTaskScope,
        analyze_structured_task_scopes,
    },
};

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
