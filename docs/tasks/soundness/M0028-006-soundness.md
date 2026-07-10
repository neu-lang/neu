# Soundness Report: M0028-006

## Metadata

- Task ID: `M0028-006`
- Milestone: `M0028`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0028-006-executable-function-body-metadata.md`
- Milestone file: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0040-bootstrap-program-entry-point.md`
  - `docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md`
  - `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`
- Changed files:
  - `crates/compiler/src/parser.rs`
  - `crates/compiler/tests/parser.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets` (280 passed, 14 suites)

## Safety Invariants Checked

- [x] This parser-only task does not alter ownership, borrowing, threading,
  coroutine, unsafe, or FFI behavior.
- [x] Returns are associated only with the currently parsed function body.
- [x] Call arguments retain parser source order and malformed calls do not
  receive complete executable metadata.

## Attacks Attempted

Attack: returns in helper and `main` functions are attributed to the wrong
function.

Expected result: each record carries its lexical enclosing function.

Actual result: the parser test asserts distinct helper and `main` identities.

Source of truth: ADR-0041.

Outcome: pass.

Attack: a direct call reorders or drops arguments.

Expected result: source order is preserved.

Actual result: the parser test checks two ordered argument nodes and spans.

Source of truth: ADR-0041.

Outcome: pass.

Attack: malformed call syntax produces a complete call record.

Expected result: no complete record.

Actual result: malformed-call test observes none.

Source of truth: ADR-0024 and M0028.

Outcome: pass.

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/parser.rs`
- Tests run:
  - `cargo test -p compiler --test parser m0028_`
- Result:
  - 7 passed.

## Findings

- None.

## Ambiguities

- None.

## Decision

Pass. The task preserves only syntactic relationships and does not make
unapproved executable or safety decisions.
