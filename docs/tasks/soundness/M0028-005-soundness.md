# Soundness Report: M0028-005

## Metadata

- Task ID: `M0028-005`
- Milestone: `M0028`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0028-005-static-integer-diagnostics.md`
- Milestone file: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0042-bootstrap-executable-operators.md`
  - `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`
  - `docs/adr/ADR-0048-bootstrap-integer-constant-expressions.md`
- Changed files:
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/type_check.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets` (278 passed, 14 suites)

## Safety Invariants Checked

- [x] This task does not introduce an ownership, borrowing, thread-safety,
  coroutine, unsafe, or FFI boundary.
- [x] Static integer diagnostics do not evaluate local bindings or calls.
- [x] Static arithmetic diagnostics identify a maximal constant tree without
  hiding an independent invalid literal.

## Attacks Attempted

Attack: `-9223372036854775808` is parsed as a literal magnitude followed by
unary minus.

Expected result: accepted as bootstrap `Int` minimum.

Actual result: no static diagnostic.

Source of truth: ADR-0043 and ADR-0048.

Outcome: pass.

Attack: overflow, division by zero, negative exponent, and invalid shift count
occur in literal trees.

Expected result: the applicable ADR-0043 diagnostic is emitted.

Actual result: all five diagnostic classes are covered by a focused test.

Source of truth: ADR-0043.

Outcome: pass.

Attack: an out-of-range literal appears alongside a nonconstant local-name
expression.

Expected result: no unapproved local-value evaluation and no cascading range
diagnostics.

Actual result: only the independent literal receives `integer_literal_out_of_range`.

Source of truth: ADR-0048.

Outcome: pass.

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/type_check.rs`
- Tests run:
  - `cargo test -p compiler --test type_check m0028_static_integer_diagnostics`
- Result:
  - 3 passed.

## Findings

- None.

## Ambiguities

- None. The prior constant-expression ambiguity is resolved by ADR-0048.

## Decision

Pass. The evaluator stays inside the accepted literal-tree boundary and its
diagnostic behavior is covered by focused adversarial cases.
