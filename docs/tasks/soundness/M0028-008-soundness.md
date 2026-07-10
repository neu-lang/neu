# Soundness Report: M0028-008

## Metadata

- Task ID: `M0028-008`
- Milestone: `M0028`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0028-008-return-statement-block-metadata.md`
- Milestone file: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md`
  - `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`
- Changed files:
  - `crates/compiler/src/parser.rs`
  - `crates/compiler/tests/parser.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets` (284 passed, 14 suites)

## Safety Invariants Checked

- [x] This parser-only task does not alter ownership, borrowing, threading,
  coroutines, unsafe, or FFI behavior.
- [x] A return in a deferred nested block retains that block, not the function
  body block.

## Attacks Attempted

Attack: a return nested in an `if` body is recorded as a direct function-body
return and later proves a straight-line return path.

Expected result: nested return records the nested block.

Actual result: the parser test distinguishes both direct returns from the
branch return.

Source of truth: ADR-0041 and ADR-0042.

Outcome: pass.

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/parser.rs`
- Tests run:
  - `cargo test -p compiler --test parser m0028_records_return_statement_enclosing_blocks_in_source_order`
- Result:
  - 1 passed.

## Findings

- None.

## Ambiguities

- None.

## Decision

Pass. The containment record is sufficient for later straight-line analysis and
does not itself decide reachability.
