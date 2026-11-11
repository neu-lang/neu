# ADR-0122: Cancellation Observation and Unwinding

Status: Accepted

## Question

How does a task that has already started observe cancellation, and what
happens to its owned values and children when it does? ADR-0104 defines
cancellation requests and says cancellation "is observed at accepted suspension
and cleanup points" and "runs ownership cleanup before completion", but it does
not define the mechanism. ADR-0101 defines the child failure rule. Today,
cancelling a task only skips its body if the task has not started; a started
task runs to completion.

## Relationship to Accepted ADRs

This proposal revises the following accepted decisions. It does not edit them.
On acceptance, ADR-0101 and ADR-0104 should be marked as revised by this ADR.

- ADR-0104: cancellation observation points, unwinding, and propagation to
  children are specified here.
- ADR-0101: the child failure rule is restated here, since its scope reporting
  was decided separately by the project owner.

## Decision

### Observation points

A task observes cancellation only at its accepted suspension points:

1. `await(task)` while the awaited task is unfinished;
2. scope exit while children are unfinished (the scope join);
3. a channel `send` that blocks on a full channel, and a channel `receive` that
   blocks on an empty channel.

A task that does not suspend is not interrupted. Cancellation takes effect at the
next observation point after the request.

### Where channel operations may appear

Channel `send` and `receive` are valid only inside `suspend` functions. Using
either in a function that is not `suspend` is a compile-time error. This keeps
every point that may block visible in the function signature, so unwinding only
crosses suspend frames. Every channel `send` can block when a bounded channel is
full, and every `receive` can block when a channel is empty. The compiler cannot
tell statically which calls will block, so the rule applies to all `send` and
`receive` operations. This ADR adds no non-blocking channel operations.

The diagnostic uses the stable rule name `channel_op_outside_suspend`.

### Unwinding

When an observation point finds cancellation requested, the task unwinds:

1. The operation returns a cancelled outcome. It does not block and does not
   produce a value.
2. Each suspend frame that the cancelled outcome passes through runs its owned
   value cleanup in reverse declaration order, then returns the cancelled outcome
   to its caller. Statements after the observation point do not run.
3. The task's entry completes with the cancelled result. An `await` of that task
   traps, as defined by ADR-0104.

Cleanup is non-suspending. It runs only compiler-generated ownership cleanup. It
cannot `await`, block on a channel, or start a child task.

### Propagation to children

When a task is cancelled, cancellation propagates to its unfinished children
(ADR-0104: "propagates from scopes to unfinished descendants"). The task does not
complete until each child has completed. Children complete through the same
unwinding rules.

### Scope failure

A child that completes as cancelled makes its scope fail. The scope cancels its
other unfinished children, joins all of them, and then traps. This is the rule
the project owner selected for ADR-0101. The trap happens after the join, so no
child is left running.

### Cancelling a finished task

Cancelling a task that has already completed has no effect.

## Consequences

- A non-`suspend` function has no suspension point, so unwinding never has to
  pass through one. Every frame that can block is a `suspend` frame, and each of
  those propagates the cancelled outcome.
- Existing tests and examples that use channel operations inside non-`suspend`
  functions must be updated to `suspend` functions. Affected tests include
  `crates/compiler/tests/channels.rs`, which uses `public func main()` for
  `send` and `receive`.
- The stack-based implementation is an internal choice, not part of this
  decision. Unwinding works by returning a cancelled status through suspend
  frames, not by an unwinding runtime.

## Open for Review

- Whether cleanup order for values with dependencies needs a separate rule.
  This proposal uses reverse declaration order, the same order as ordinary scope
  cleanup.

## Required Follow-Up

- Update `docs/SPEC.md` with the observation points, the channel rule, and the
  unwinding and propagation rules.
- Add stable diagnostic `channel_op_outside_suspend` to `docs/diagnostics.md`.
- Implement and test the behavior, including tests for each observation point,
  unwinding through nested suspend frames, child propagation, and cleanup order.
