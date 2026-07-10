# Task: M0024-001 Thread Capability Semantics Blocker

## Task Metadata

- Task ID: `M0024-001`
- Milestone: `M0024`
- Milestone File: `docs/milestones/M0024-thread-safety-capability-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Language Designer`

## Objective

Record the missing accepted semantics that block M0024 thread-safety capability
analysis implementation.

## Authority Extract

- `docs/SPEC.md`, “ADR-0014: Thread Safety And Data-Race Freedom”.
- `docs/adr/ADR-0014-thread-safety-and-data-race-freedom.md`, “Recommended
  Choice” and “Downstream Consequences”.
- `docs/milestones/M0024-thread-safety-capability-analysis.md`.
- `docs/ambiguities/M0024-thread-capability-semantics.md`.

## Blocker

M0024 requires send/share capability representation, approved boundary checks,
and diagnostics for invalid cross-boundary captures. Accepted source of truth
selects compile-time send/share capabilities but does not define the bootstrap
capability catalog, derivation rules, concurrent boundary records, capture
rules, missing-capability diagnostics, or treatment of shared mutable state in
the absence of approved synchronization abstractions.

## Required Resolution

ADR-0037 is accepted and defines the M0024 metadata-only bootstrap
thread-capability subset, including `Send` and `Share`, type-category
derivation rules, boundary and capture records, `missing_thread_capability`, and
explicit deferral of source-level concurrency forms.

## Execution Log

- 2026-07-11 agent=Main phase=blocker-recorded result=blocked evidence=ADR-0014 selects send/share capability direction but does not define testable M0024 boundary or diagnostic semantics. handoff=Language-Designer
- 2026-07-11 agent=Main phase=resolution-accepted result=pass evidence=ADR-0037 accepted; SPEC and ambiguity report updated. handoff=Task-Decomposer
