# Soundness Report: M0028-013

## Metadata

- Task ID: `M0028-013`
- Milestone: `M0028`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task: `docs/tasks/M0028-013-direct-call-checker.md`
- Authority: ADR-0041 and ADR-0051.
- Changed implementation: `crates/compiler/src/type_check.rs`.
- Tests: `crates/compiler/tests/type_check.rs`.
- Ordinary validation: format, Clippy, 290 workspace tests, and the focused
  `docs/tests/m0028-direct-call-checker.sh` validator passed.

## Safety Invariants Checked

- [x] Calls resolve only to one same-package top-level function with a body.
- [x] Invalid, arity-mismatched, type-mismatched, and recursive calls receive no successful result type.
- [x] Recursive calls attach `recursive_call_unsupported` to the whole call expression.
- [x] Existing ownership, borrow, thread, coroutine, unsafe, and FFI analyses are unchanged.

## Attacks Attempted

```text
Attack: Form a three-function cycle: first -> second -> third -> first.
Expected result: Each edge reports recursive_call_unsupported and no call result type is recorded.
Actual result: Three recursive_call_unsupported diagnostics; zero result types.
Source of truth: ADR-0041, ADR-0051.
Outcome: pass
```

## Adversarial Tests

- Tests added: `m0028_direct_calls_reject_every_edge_in_a_recursive_cycle`.
- Tests run: `cargo test -p compiler --test type_check m0028_direct_calls`.
- Result: pass.

## Findings

None.

## Ambiguities

None.

## Decision

Pass. The direct-call checker does not admit recursive call results or weaken existing safety analyses.
