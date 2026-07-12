# ADR-0101: Structured Concurrency Runtime Semantics

Status: Accepted

## Question

What source-level structured-concurrency contract does Neu provide before
parallel execution, I/O, and a standard library exist?

## Competing Designs

- Kotlin-like lexical scopes with compiler-owned `Task<T>` values.
- Detached task creation with explicit joins.
- OS-thread primitives exposed directly to Neu source.
- Metadata-only coroutine analysis with no executable concurrency surface.

## Recommended Choice

Neu uses cooperative, single-threaded structured concurrency. The runtime owns
the scheduler, task frames, task handles, and cancellation state. Neu source
does not name an executor, runtime object, Tokio type, thread, or reference.

The initial source vocabulary is:

- `suspend func` declares a function that may suspend.
- `scope { statements }` creates a lexical child-task scope and evaluates to
  `Unit`.
- `spawn { expression }` creates a child task in the innermost active scope and
  returns an opaque `Task<T>` for the closure result type `T`.
- `await(task)` suspends the current task until the task completes, consumes
  that task handle, and returns its owned `T` result.

`await` is valid only in a `suspend func` or a `scope`/spawn body whose enclosing
function is suspendable. A task must be spawned inside an active `scope`; no
detached spawn or task escape is accepted. Explicit cancellation is a later
surface defined by ADR-0104, and channels are a later surface defined by
ADR-0103.

## Scope And Failure Semantics

A scope registers every child before `spawn` returns. Normal scope completion
waits for all registered children, including children created by nested scopes.
Returning, breaking, or otherwise leaving a scope performs the same join and
cleanup before control leaves it. A child cannot outlive its owning scope.

If a child completes with a failure or cancellation, the scope requests
cancellation of unfinished siblings, waits for their cleanup, and then reports
the first failure in deterministic creation order. No exception syntax or
recoverable error syntax is added by this ADR. Runtime traps remain traps under
the existing compiler-owned runtime contract.

The scheduler is cooperative and deterministic: runnable tasks are selected in
FIFO order, and a task yields only at an accepted suspension operation such as
`await`. There is no fairness, timing, or progress guarantee beyond eventual
selection of runnable tasks. No OS threads or parallel execution are implied.

## Ownership And Suspension

`Task<T>` is opaque and move-only. `spawn` transfers or copies captures using
the existing inferred ownership effects and requires `Send` for transferred
captures. Shared captures require `Share`; mutable shared captures and borrowed
captures are rejected. Copyable captures remain usable, while move-only
captures become unusable after successful spawn.

`await(task)` consumes the handle exactly once. The completed result is a new
owned value; copyable results follow normal copy rules. Awaiting a failed,
cancelled, or already-consumed task is deterministic and diagnosed when the
state is statically known, otherwise it produces the defined runtime failure.

No borrow may cross `await` when the suspended frame can be concurrently
accessed or when the borrowed value may outlive its scope. A borrow proven to
remain in the same non-concurrent task scope may cross suspension according to
ADR-0009 and the existing coroutine analysis. Pinned frames, self-referential
frames, lifetime annotations, and advanced borrow annotations remain deferred.

## ABI, Diagnostics, And Deferrals

HIR preserves scope identity, parent/child relationships, callable and capture
facts, suspension points, task identity, result type, ownership transitions,
cleanup edges, and source spans. MIR preserves scope entry/exit, spawn,
suspend/resume, task completion, await, failure, and cleanup operations. The
backend uses compiler-private runtime declarations; `Task<T>` and frames have
no stable public layout or FFI ABI.

Diagnostics identify the task or scope operation, ownership transition,
suspension point, and source span. Required stable categories include
`task_scope_escape`, `await_outside_suspend`, `borrow_across_suspension`,
`invalid_task_state`, and `missing_task_capability`.

Channels, explicit cancellation syntax, timers, I/O, OS threads, parallel
execution, detached tasks, task groups, exceptions, reflection, public runtime
types, and standard-library concurrency APIs remain deferred.

## Dependencies

- Supersedes the source-level concurrency deferral portions of ADR-0008,
  ADR-0009, ADR-0037, and ADR-0038.
- Preserves ownership and capability rules from ADR-0014, ADR-0035, ADR-0062,
  and ADR-0089.
- Preserves compiler-private host runtime/linking boundaries from ADR-0100.
