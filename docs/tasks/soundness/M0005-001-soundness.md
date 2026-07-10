# Soundness Report: M0005-001

## Metadata

- Task ID: `M0005-001`
- Milestone: `M0005`
- Filed By: `main-task adversarial check`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0005-001-source-spans.md`
- Milestone file: `docs/milestones/M0005-source-database-spans-and-file-identity.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Changed files:
  - `crates/newlang/src/lib.rs`
  - `crates/newlang/src/source.rs`
  - `docs/tests/m0005-source-spans.sh`
- Ordinary test results:
  - Focused source tests passed.
  - Full ordinary M0005 gate chain passed.

## Safety Invariants Checked

- [x] Ownership cannot be bypassed because this task stores owned source strings and exposes shared references only.
- [x] Moved values cannot be reused because no language move semantics are implemented.
- [x] Shared and exclusive borrows cannot conflict because no language borrow semantics are implemented.
- [x] Borrowed data cannot outlive its owner because returned source references are borrowed from `SourceDatabase`.
- [x] Nullability refinements are not affected.
- [x] Thread send/share capabilities are not affected.
- [x] Coroutine scopes are not affected.
- [x] Borrows across suspension are not affected.
- [x] Unsafe and FFI boundaries are not affected; crate-level unsafe code remains forbidden.
- [x] Diagnostics do not hide or misstate safety failures because this task maps spans only and emits no diagnostics.

## Attacks Attempted

```text
Attack: Construct spans outside file bounds.
Expected result: Invalid spans are rejected.
Actual result: invalid_offsets_and_spans_are_rejected passed.
Source of truth: docs/milestones/M0005-source-database-spans-and-file-identity.md
Outcome: pass

Attack: Use an unknown SourceFileId.
Expected result: Lookup and span construction fail.
Actual result: invalid SourceFileId span test passed.
Source of truth: docs/milestones/M0005-source-database-spans-and-file-identity.md
Outcome: pass

Attack: Check whether M0005 introduced lexer/parser/AST/HIR/MIR/backend files.
Expected result: No such files exist.
Actual result: docs/tests/m0005-source-spans.sh passed absence checks.
Source of truth: docs/tasks/M0005-001-source-spans.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0005-source-spans.sh`
- Tests run:
  - `docs/tests/m0005-source-spans.sh`
- Result:
  - pass

## Findings

No blocking findings.

## Ambiguities

- Unicode display column semantics remain deferred and are explicitly recorded in the task. Current tests cover ASCII byte-column mapping only.

## Decision

Pass.

