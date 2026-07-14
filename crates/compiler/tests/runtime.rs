use compiler::runtime::{
    RUNTIME_ABI_VERSION, RUNTIME_SYMBOLS, RuntimeBoundary, RuntimeBoundaryError, RuntimeTaskError,
};
use target_lexicon::Triple;

#[test]
fn current_thread_runtime_boundary_executes_without_public_tokio_types() {
    let runtime = RuntimeBoundary::new(Triple::host()).unwrap();
    assert_eq!(runtime.block_on(async { 7 }), 7);
    assert_eq!(RUNTIME_ABI_VERSION, "neu-runtime-v1");
    assert!(RUNTIME_SYMBOLS
        .iter()
        .all(|symbol| !symbol.name().contains("tokio") && !symbol.signature().contains("tokio")));
}

#[test]
fn private_runtime_symbol_contract_is_stable() {
    let symbols = RUNTIME_SYMBOLS
        .iter()
        .map(|symbol| (symbol.name(), symbol.signature()))
        .collect::<Vec<_>>();
    assert_eq!(
        symbols,
        vec![
            ("neu_runtime_init", "() -> Void"),
            ("neu_runtime_shutdown", "() -> Void"),
            ("neu_runtime_task_spawn", "(opaque) -> opaque"),
            ("neu_runtime_task_await", "(opaque) -> opaque"),
            ("neu_runtime_task_cancel", "(opaque) -> Void"),
        ]
    );
}

#[test]
fn runtime_boundary_rejects_non_host_targets_before_initialization() {
    let target = "x86_64-unknown-linux-gnu".parse::<Triple>().unwrap();
    if target == Triple::host() {
        return;
    }
    assert!(matches!(
        RuntimeBoundary::new(target.clone()),
        Err(RuntimeBoundaryError::UnsupportedTarget(rejected)) if rejected == target
    ));
}

#[test]
fn opaque_task_handles_complete_and_can_be_cancelled() {
    let runtime = RuntimeBoundary::new(Triple::host()).unwrap();
    let task = runtime.spawn(async { 7 });
    assert_eq!(runtime.await_task(task), Ok(7));

    let task = runtime.spawn(async {
        std::future::pending::<()>().await;
        9
    });
    runtime.cancel_task(&task);
    assert_eq!(runtime.await_task(task), Err(RuntimeTaskError::Cancelled));
}

#[test]
fn runtime_boundary_can_be_explicitly_shutdown() {
    let runtime = RuntimeBoundary::new(Triple::host()).unwrap();
    runtime.shutdown();
}

#[test]
fn runtime_scope_tracks_children_until_they_complete() {
    let runtime = RuntimeBoundary::new(Triple::host()).unwrap();
    let mut scope = runtime.scope();
    let task = scope.spawn(async { 7 });
    assert_eq!(scope.active_children(), 1);
    assert_eq!(scope.await_task(task), Ok(7));
    scope.finish().unwrap();
    assert_eq!(scope.active_children(), 0);
}

#[test]
fn runtime_scope_cancels_unfinished_children_before_finishing() {
    let runtime = RuntimeBoundary::new(Triple::host()).unwrap();
    let mut scope = runtime.scope();
    let _task = scope.spawn(async { std::future::pending::<()>().await });
    scope.cancel_children();
    assert_eq!(scope.finish(), Err(RuntimeTaskError::Cancelled));
    assert_eq!(scope.active_children(), 0);
}

#[test]
fn failed_tasks_are_translated_to_the_private_runtime_error() {
    let runtime = RuntimeBoundary::new(Triple::host()).unwrap();
    let task = runtime.spawn(async { panic!("task failure") });
    assert_eq!(runtime.await_task(task), Err(RuntimeTaskError::Failed));
    assert!(!format!("{:?}", RuntimeTaskError::Failed).contains("tokio"));
}

#[test]
fn scope_failure_cancels_and_waits_for_unfinished_siblings() {
    let runtime = RuntimeBoundary::new(Triple::host()).unwrap();
    let mut scope = runtime.scope();
    let _failing = scope.spawn(async { panic!("child failure") });
    let _pending = scope.spawn(async { std::future::pending::<()>().await });
    assert_eq!(scope.finish(), Err(RuntimeTaskError::Failed));
    assert_eq!(scope.active_children(), 0);
}

#[test]
fn dropping_a_task_handle_requests_cancellation() {
    let runtime = RuntimeBoundary::new(Triple::host()).unwrap();
    let mut scope = runtime.scope();
    let task = scope.spawn(async { std::future::pending::<()>().await });
    drop(task);
    assert_eq!(scope.finish(), Err(RuntimeTaskError::Cancelled));
    assert_eq!(scope.active_children(), 0);
}
