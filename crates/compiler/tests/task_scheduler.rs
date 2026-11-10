use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    module::{ModuleName, PackageNamespace},
    source::SourceFileId,
};

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
    assert_eq!(status.code(), None, "{status:?}");
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
    assert_eq!(output.status.code(), None, "{:?}", output.status);
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
