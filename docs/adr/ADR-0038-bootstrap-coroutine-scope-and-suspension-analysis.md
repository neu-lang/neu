# ADR-0038: Bootstrap Coroutine Scope And Suspension Analysis

Status: Accepted

## Question

What bootstrap coroutine scope and suspension semantics are sufficient for
M0025 without defining coroutine source syntax, runtime scheduling, detached
task APIs, pinned frames, or cancellation machinery?

## Competing Designs

1. Define concrete coroutine and task-spawn syntax now.
2. Implement runtime scheduler semantics before static coroutine checks.
3. Use metadata-only task-scope and suspension records for M0025.
4. Defer M0025 entirely until async syntax and runtime scheduling exist.

## Trade-offs

Concrete source syntax would exercise the eventual user-facing model, but
ADR-0024, ADR-0037, and prior milestone scoping deliberately keep unsupported
concurrency forms rejected until their semantics are accepted.

Runtime-first implementation would make examples more visible, but it reverses
the roadmap priority: sound static semantics must precede execution support.

Metadata-only records let M0025 validate the safety rules selected by ADR-0008
and ADR-0009 while avoiding speculative syntax, scheduling, allocation, or
runtime cancellation design.

Full deferral preserves optionality, but leaves structured concurrency and
suspension borrowing without incremental validation after M0024.

## Recommended Choice

Use a metadata-only bootstrap coroutine scope and suspension model.

M0025 introduces no source-level coroutine, `async`, `await`, task-spawn,
detached-task, cancellation, pinned-frame, closure, channel, synchronization, or
scheduler syntax. Existing unsupported concurrency-like source forms remain
rejected or unsupported.

M0025 input records are compiler side-table facts supplied by earlier or test
analysis scaffolding. A structured scope record has a scope node and ordered
child-task records. A child-task record has a task node, the containing scope
node, and the scope node in which the child is proven to complete or be
cancelled.

A child task is valid in M0025 only when its completion-or-cancellation scope is
the same scope as its containing structured scope. Nested, sibling,
path-sensitive, control-flow-sensitive, dynamically selected, detached, and
ownership-transferred task escape rules are deferred.

M0025 reports `task_scope_escape` when a child task is not proven to complete
or be cancelled in its containing structured scope. The primary diagnostic span
is the child task node. The secondary span is the containing scope node. The
diagnostic must identify that the child task would outlive its structured
scope.

A suspension point record has a suspension node and a suspended-frame scope
node. A suspended-borrow record has a suspension node, borrowed local binding,
borrow node, borrow kind (`shared` or `exclusive`), borrowed-value lifetime
scope, suspended-frame scope, and a flag stating whether the suspended frame may
be concurrently accessed.

A borrow crossing a suspension point is valid in M0025 only when both of these
conditions hold:

- the suspended frame is not concurrently accessible; and
- the suspended-frame scope is the same scope as the borrowed value's lifetime
  scope.

This bootstrap rule allows either shared or exclusive borrows across suspension
only under the exact proof above. Nested lifetime containment, non-lexical
scope reasoning, path-sensitive frame access, pinned frames, self-referential
frames, generator lowering, advanced annotations, and runtime frame allocation
are deferred.

M0025 reports `borrow_across_suspension` when a suspended-borrow record fails
either condition. The primary diagnostic span is the suspension node. The
secondary span is the borrow node. The diagnostic must identify the borrowed
local and whether rejection is due to possible concurrent frame access,
outliving the borrowed value, or both.

Cancellation resource-safety in M0025 is limited to the static structured-scope
completion-or-cancellation check above. Runtime cancellation propagation,
destructor execution during cancellation, cancellation handlers, cancellation
masking, and async drop are deferred.

M0024 thread capability facts may be consumed by later task-boundary records,
but M0025 does not infer task captures or add new `Send` or `Share` rules.
When a task boundary also has thread-capability capture records, ADR-0037
continues to govern those diagnostics independently.

## Downstream Consequences

- M0025 can implement structured-scope and suspension-borrow checkers without
  source-level coroutine syntax.
- Parser support for `async`, `await`, task spawning, and detached work remains
  deferred and must be introduced by a later ADR before user code may use it.
- Runtime scheduling, cancellation propagation, and async frame lowering remain
  later milestones.
- M0027 and later diagnostics can rely on stable diagnostic identifiers
  `task_scope_escape` and `borrow_across_suspension`.
- More expressive coroutine borrowing requires a later ADR that supersedes the
  exact-scope bootstrap proof.

## Dependencies

- ADR-0001
- ADR-0002
- ADR-0003
- ADR-0008
- ADR-0009
- ADR-0014
- ADR-0015
- ADR-0036
- ADR-0037
