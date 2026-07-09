# Soundness Report: M0004-001

## Metadata

- Task ID: `M0004-001`
- Milestone: `M0004`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0004-001-diagnostic-contract.md`
- Milestone file: `docs/milestones/M0004-diagnostic-infrastructure-contract.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Changed files:
  - `docs/diagnostics.md`
  - `docs/tests/m0004-diagnostic-contract.sh`
  - `tests/golden/diagnostics/M0004-inert.diagnostic.toml`
- Ordinary test results:
  - `docs/tests/m0004-diagnostic-contract.sh && docs/tests/m0003-fixture-layout.sh && docs/tests/m0002-workspace-ci.sh` passed.

## Safety Invariants Checked

- [x] Ownership cannot be bypassed by this task because no ownership diagnostics or semantics are implemented.
- [x] Moved values cannot be reused by this task because no move diagnostics or semantics are implemented.
- [x] Shared and exclusive borrows cannot conflict by this task because no borrow diagnostics or semantics are implemented.
- [x] Borrowed data cannot outlive its owner by this task because no lifetime diagnostics or semantics are implemented.
- [x] Nullability refinements cannot be used after invalidation by this task because no nullability diagnostics or semantics are implemented.
- [x] Thread send/share capabilities are not weakened because no concurrency diagnostics or semantics are implemented.
- [x] Coroutine scopes are not affected because no coroutine diagnostics or semantics are implemented.
- [x] Borrows across suspension are not affected because no async diagnostics or semantics are implemented.
- [x] Unsafe and FFI boundaries are not affected because no unsafe or FFI diagnostics are implemented.
- [x] Diagnostics do not hide or misstate safety failures because this task defines shape only and forbids unspecified diagnostic expectations.

## Attacks Attempted

```text
Attack: Check whether the inert diagnostic snapshot encodes a specific compiler error.
Expected result: It contains shape metadata only and compiler_behavior = "none".
Actual result: docs/tests/m0004-diagnostic-contract.sh passed the inert snapshot checks.
Source of truth: docs/milestones/M0004-diagnostic-infrastructure-contract.md
Outcome: pass

Attack: Check whether the diagnostic contract permits internal compiler jargon in user-facing diagnostics.
Expected result: The contract forbids internal compiler jargon.
Actual result: docs/diagnostics.md contains the required prohibition.
Source of truth: docs/adr/ADR-0015-diagnostics-as-semantics.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0004-diagnostic-contract.sh`
- Tests run:
  - `docs/tests/m0004-diagnostic-contract.sh`
- Result:
  - pass

## Findings

No blocking findings.

## Ambiguities

- None.

## Decision

Pass.

