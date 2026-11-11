use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    module::{ModuleName, PackageNamespace},
    source::SourceFileId,
};

/// A trap ends the process by signal on Unix. On Windows it is an illegal
/// instruction, which the process reports as this exit code.
const WINDOWS_ILLEGAL_INSTRUCTION: i32 = -1073741795;

fn is_trap(code: Option<i32>) -> bool {
    code.is_none() || code == Some(WINDOWS_ILLEGAL_INSTRUCTION)
}

fn run_status(source: &str, name: &str) -> std::process::ExitStatus {
    let workspace =
        std::env::temp_dir().join(format!("neu-scheduler-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(13000),
            ModuleName::parse(name).unwrap(),
            PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    let status = Command::new(&executable).status().unwrap();
    let _ = fs::remove_dir_all(workspace);
    status
}

fn compile_and_run(source: &str, name: &str) -> i32 {
    let workspace =
        std::env::temp_dir().join(format!("neu-scheduler-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(13000),
            ModuleName::parse(name).unwrap(),
            PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    let status = Command::new(&executable).status().unwrap();
    let code = status.code().unwrap_or(-1);
    let _ = fs::remove_dir_all(workspace);
    code
}

#[test]
fn fifo_tasks_interleave_when_a_bounded_channel_suspends_them() {
    // Each producer is queued at spawn. The consumer suspends on the empty
    // channel, so producer 0 runs first, then producer 1 and producer 2 suspend
    // on the full channel in FIFO order. Expected receive order: 0, 1, 2.
    let source = r#"
        suspend func main(): Int {
            scope {
                val queue = channel<Int>(1);
                val first = spawn { -> send(queue, 0) };
                val second = spawn { -> send(queue, 1) };
                val third = spawn { -> send(queue, 2) };
                val r1 = receive(queue);
                val a: Int = when (r1) {
                    ChannelResult.Message(value) -> value;
                    ChannelResult.Closed -> 9;
                };
                val r2 = receive(queue);
                val b: Int = when (r2) {
                    ChannelResult.Message(value) -> value;
                    ChannelResult.Closed -> 9;
                };
                val r3 = receive(queue);
                val c: Int = when (r3) {
                    ChannelResult.Message(value) -> value;
                    ChannelResult.Closed -> 9;
                };
                return a * 16 + b * 4 + c;
            }
            return 0;
        }
    "#;
    assert_eq!(compile_and_run(source, "fifo_interleave"), 6);
}

#[test]
fn spawned_task_does_not_run_before_the_parent_suspends() {
    // The child is not started by spawn. The parent's first receive finds the
    // value it sent itself, so the child only runs when the second receive
    // suspends on the empty channel. Expected result: 1 * 10 + 2.
    let source = r#"
        suspend func main(): Int {
            scope {
                val queue = channel<Int>(1);
                send(queue, 1);
                val child = spawn { -> send(queue, 2) };
                val r1 = receive(queue);
                val a: Int = when (r1) {
                    ChannelResult.Message(value) -> value;
                    ChannelResult.Closed -> 9;
                };
                val r2 = receive(queue);
                val b: Int = when (r2) {
                    ChannelResult.Message(value) -> value;
                    ChannelResult.Closed -> 9;
                };
                return a * 10 + b;
            }
            return 0;
        }
    "#;
    assert_eq!(compile_and_run(source, "deferred_start"), 12);
}

#[test]
fn normal_scope_exit_joins_children_before_the_function_continues() {
    // The child is queued by spawn and does not run until the scope exits.
    // The parent sends 3 inside the scope, so the join must deliver 4 before
    // the parent's send of 5. Expected receive order: 3, 4 (a skipped join
    // would give 3, 5; eager execution would give 4, 3).
    let source = r#"
        suspend func produce(queue: Channel<Int>): Int {
            scope {
                val child = spawn { -> send(queue, 4) };
                send(queue, 3);
            }
            send(queue, 5);
            val r1 = receive(queue);
            val a: Int = when (r1) {
                ChannelResult.Message(value) -> value;
                ChannelResult.Closed -> 9;
            };
            val r2 = receive(queue);
            val b: Int = when (r2) {
                ChannelResult.Message(value) -> value;
                ChannelResult.Closed -> 9;
            };
            return a * 10 + b;
        }
        suspend func main(): Int {
            val queue = channel<Int>(3);
            val done: Int = produce(queue);
            return done;
        }
    "#;
    assert_eq!(compile_and_run(source, "normal_join"), 34);
}

#[test]
fn awaiting_a_task_lets_earlier_siblings_run_first() {
    // main suspends on the second child. The earlier sibling is queued first, so
    // it runs (sending 1) before the awaited child returns 5 and resumes main.
    // Expected result: receive gives 1, so 1 * 10 + 5.
    let source = r#"
        suspend func main(): Int {
            scope {
                val queue = channel<Int>(2);
                val first = spawn { -> send(queue, 1) };
                val second = spawn { -> 5 };
                val got: Int = await(second);
                val r1 = receive(queue);
                val a: Int = when (r1) {
                    ChannelResult.Message(value) -> value;
                    ChannelResult.Closed -> 9;
                };
                return a * 10 + got;
            }
            return 0;
        }
    "#;
    assert_eq!(compile_and_run(source, "await_fifo"), 15);
}

#[test]
fn break_out_of_a_scope_joins_the_iteration_child() {
    // Each iteration spawns a child that sends 1. The iteration that breaks must
    // join its child before the loop exits, so three 1s are queued before main
    // sends 9. Expected receive order: 1, 1, 1 (a skipped join would give 1, 1, 9).
    let source = r#"
        suspend func main(): Int {
            val queue = channel<Int>(4);
            for (index in 0..5) {
                scope {
                    val child = spawn { -> send(queue, 1) };
                    if (index == 2) {
                        break;
                    }
                }
            }
            send(queue, 9);
            val r1 = receive(queue);
            val a: Int = when (r1) {
                ChannelResult.Message(value) -> value;
                ChannelResult.Closed -> 9;
            };
            val r2 = receive(queue);
            val b: Int = when (r2) {
                ChannelResult.Message(value) -> value;
                ChannelResult.Closed -> 9;
            };
            val r3 = receive(queue);
            val c: Int = when (r3) {
                ChannelResult.Message(value) -> value;
                ChannelResult.Closed -> 9;
            };
            return a * 16 + b * 4 + c;
        }
    "#;
    assert_eq!(compile_and_run(source, "break_join"), 21);
}

#[test]
fn cancelling_a_child_cancels_its_unfinished_siblings() {
    // A is cancelled before it starts. When main awaits B, A runs first, sees its
    // cancellation, and ends cancelled. That cancels B, which has not started, so
    // B's await traps instead of returning 7.
    let source = r#"
        suspend func main(): Int {
            scope {
                val first = spawn { -> 5 };
                val second = spawn { -> 7 };
                first.cancel();
                val got: Int = await(second);
                return got;
            }
            return 0;
        }
    "#;
    // A trap ends the process by signal, so there is no exit code.
    let status = run_status(source, "sibling_cancel");
    assert!(is_trap(status.code()), "{status:?}");
}

#[test]
fn unrelated_siblings_are_not_cancelled() {
    // Control for the test above: with no cancelled child, B's await returns 7.
    let source = r#"
        suspend func main(): Int {
            scope {
                val first = spawn { -> 5 };
                val second = spawn { -> 7 };
                val got: Int = await(second);
                return got;
            }
            return 0;
        }
    "#;
    assert_eq!(compile_and_run(source, "unrelated_siblings"), 7);
}

#[test]
fn deadlock_prints_a_diagnostic_and_traps() {
    // No task can run: main waits on a channel nothing will send to. The runtime
    // writes a diagnostic to stderr, then traps.
    let source = r#"
        suspend func main(): Int {
            scope {
                val queue = channel<Int>(1);
                val result = receive(queue);
            }
            return 0;
        }
    "#;
    let workspace = std::env::temp_dir().join(format!("neu-deadlock-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(13100),
            ModuleName::parse("deadlock").unwrap(),
            PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    let output = Command::new(&executable).output().unwrap();
    assert!(is_trap(output.status.code()), "{:?}", output.status);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("deadlock"), "{stderr}");
    let _ = fs::remove_dir_all(workspace);
}

#[test]
fn early_return_before_a_channel_local_is_declared_does_not_crash() {
    // The cleanup on the early return must skip the channel local, which was
    // never initialized.
    let source = r#"
        func early(): Int {
            if (true) {
                return 3;
            }
            val queue = channel<Int>(1);
            return 4;
        }
        public func main(): Int {
            return early();
        }
    "#;
    assert_eq!(compile_and_run(source, "early_return_cleanup"), 3);
}

#[test]
fn spawn_body_can_be_a_when_over_a_channel_receive() {
    let source = r#"
        suspend func main(): Int {
            val queue = channel<Int>(1);
            send(queue, 4);
            scope {
                val worker = spawn { -> when (receive(queue)) { ChannelResult.Message(value) -> value; ChannelResult.Closed -> 9; } };
                return await(worker);
            }
            return 0;
        }
    "#;
    assert_eq!(compile_and_run(source, "spawn_when_body"), 4);
}

#[test]
fn spawn_body_can_call_a_suspend_function() {
    let source = r#"
        suspend func child(): Int {
            return 6;
        }
        suspend func main(): Int {
            scope {
                val worker = spawn { -> child() };
                return await(worker);
            }
            return 0;
        }
    "#;
    assert_eq!(compile_and_run(source, "spawn_suspend_body"), 6);
}

fn run_capturing(source: &str, name: &str) -> (Option<i32>, String) {
    let workspace =
        std::env::temp_dir().join(format!("neu-scheduler-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(13200),
            ModuleName::parse(name).unwrap(),
            PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    let output = Command::new(&executable).output().unwrap();
    let _ = fs::remove_dir_all(workspace);
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn cancelled_receiver_unwinds_instead_of_deadlocking() {
    // The worker is cancelled while blocked on an empty channel. Cancellation
    // must wake it so it unwinds. The scope then fails (ADR-0101: a cancelled
    // child fails its scope), so the program traps without a deadlock.
    let source = r#"
        suspend func main(): Int {
            scope {
                val queue = channel<Int>(1);
                val ready = channel<Int>(1);
                val worker = spawn { -> when (receive(queue)) { ChannelResult.Message(value) -> value; ChannelResult.Closed -> 9; } };
                val helper = spawn { -> send(ready, 1) };
                val go = receive(ready);
                worker.cancel();
                return 5;
            }
            return 0;
        }
    "#;
    let (code, stderr) = run_capturing(source, "cancel_receiver");
    assert!(is_trap(code), "{stderr}");
    assert!(!stderr.contains("deadlock"), "{stderr}");
}

#[test]
fn cancelled_task_unwinds_the_children_it_is_joining() {
    // The outer task blocks joining its child, which is blocked on a channel.
    // Cancelling the outer task must reach the child, so both unwind and the
    // join completes. Without propagation the program would deadlock.
    let source = r#"
        suspend func outer_body(): Int {
            scope {
                val never = channel<Int>(1);
                val inner = spawn { -> when (receive(never)) { ChannelResult.Message(value) -> value; ChannelResult.Closed -> 9; } };
            }
            return 0;
        }
        suspend func main(): Int {
            scope {
                val outer = spawn { -> outer_body() };
                val helper = spawn { -> 1 };
                val got: Int = await(helper);
                outer.cancel();
                return 5;
            }
            return 0;
        }
    "#;
    let (code, stderr) = run_capturing(source, "cancel_propagates");
    assert!(is_trap(code), "{stderr}");
    assert!(!stderr.contains("deadlock"), "{stderr}");
}

#[test]
fn cancelled_awaiter_unwinds_instead_of_deadlocking() {
    // The waiter is cancelled while it awaits its child. It unwinds at the await
    // and cancels its child, so both finish and the scope fails.
    let source = r#"
        suspend func waiter_body(): Int {
            scope {
                val never = channel<Int>(1);
                val worker = spawn { -> when (receive(never)) { ChannelResult.Message(value) -> value; ChannelResult.Closed -> 9; } };
                return await(worker);
            }
            return 0;
        }
        suspend func main(): Int {
            scope {
                val waiter = spawn { -> waiter_body() };
                val helper = spawn { -> 1 };
                val got: Int = await(helper);
                waiter.cancel();
                return 5;
            }
            return 0;
        }
    "#;
    let (code, stderr) = run_capturing(source, "cancel_awaiter");
    assert!(is_trap(code), "{stderr}");
    assert!(!stderr.contains("deadlock"), "{stderr}");
}

#[test]
fn operators_on_a_local_inferred_from_a_suspend_call_result_are_typed() {
    // A local inferred from a suspend call's result must have that type when a
    // later initializer uses it in an operator, as it does for an ordinary call.
    let source = r#"
        suspend func produce(): Int { return 2; }
        suspend func main(): Int {
            val v = produce();
            val w = v + 1;
            return w;
        }
    "#;
    assert_eq!(compile_and_run(source, "infer_suspend_operator"), 3);
}

#[test]
fn operators_on_a_local_inferred_from_a_call_result_are_typed() {
    let source = r#"
        func produce(): Int { return 2; }
        func main(): Int {
            val v = produce();
            val w = v + 1;
            return w;
        }
    "#;
    assert_eq!(compile_and_run(source, "infer_call_operator"), 3);
}

#[test]
fn bool_operator_on_a_local_inferred_from_a_call_result_is_typed() {
    let source = r#"
        func is_big(): Bool { return true; }
        func main(): Int {
            val b = is_big();
            val c = !b;
            if c { return 1; }
            return 2;
        }
    "#;
    assert_eq!(compile_and_run(source, "infer_bool_operator"), 2);
}

#[test]
fn float_operator_on_a_local_inferred_from_a_call_result_is_typed() {
    let source = r#"
        func half(): Float { return 1.5; }
        func main(): Int {
            val f = half();
            val g = f + 1.0;
            if g == 2.5 { return 3; }
            return 4;
        }
    "#;
    assert_eq!(compile_and_run(source, "infer_float_operator"), 3);
}
