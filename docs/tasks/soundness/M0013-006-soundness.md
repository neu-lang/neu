# Soundness Report: M0013-006

## Metadata

- Task ID: `M0013-006`
- Milestone: `M0013`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0013-006-expression-statement-pattern-parser-fixtures.md`
- Milestone file: `docs/milestones/M0013-expression-statement-and-pattern-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh`
  - `tests/fixtures/parser/expressions/*.fixture.toml`
  - `tests/fixtures/parser/statements/*.fixture.toml`
  - `tests/fixtures/parser/patterns/*.fixture.toml`
  - M0013 validator updates.
- Ordinary test results:
  - Focused M0013 validators pass before this report.

## Safety Invariants Checked

- [x] Ownership cannot be bypassed.
- [x] Moved values cannot be reused.
- [x] Shared and exclusive borrows cannot conflict.
- [x] Borrowed data cannot outlive its owner.
- [x] Nullability refinements cannot be used after invalidation.
- [x] Thread send/share capabilities are enforced.
- [x] Coroutine scopes cannot outlive allowed ownership or borrow lifetimes.
- [x] Borrows across suspension are rejected unless proven safe by accepted semantics.
- [x] Unsafe and FFI boundaries do not weaken safe-code guarantees.
- [x] Diagnostics do not hide or misstate safety failures.

## Attacks Attempted

```text
Attack: Encode flow typing or value semantics in expression and statement fixtures.
Expected result: fixtures may record syntax forms only.
Actual result: fixtures record parser forms and diagnostics only; value typing and smart casts are not asserted.
Source of truth: docs/adr/ADR-0024-expression-statement-pattern-syntax.md
Outcome: pass
```

```text
Attack: Use pattern fixtures to accept match or when syntax early.
Expected result: pattern syntax is covered without accepting match or when constructs.
Actual result: pattern fixtures cover pattern forms only; unsupported match or when context is a negative fixture.
Source of truth: docs/adr/ADR-0024-expression-statement-pattern-syntax.md
Outcome: pass
```

```text
Attack: Accept unsafe, coroutine, loop, lambda, or indexing syntax through fixtures.
Expected result: deferred forms remain negative or absent.
Actual result: unsafe, coroutine, loop, lambda, and indexing-like examples are negative or deferred; no parser code changed.
Source of truth: docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh
Outcome: pass
```

```text
Attack: Rely on external language behavior as authority.
Expected result: fixture corpus cites ADR-0024 and avoids external language names.
Actual result: every fixture file cites `docs/adr/ADR-0024-expression-statement-pattern-syntax.md` and the validator rejects external language names.
Source of truth: docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh`
- Tests run:
  - `docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh`
  - M0013 source-of-truth validators.
- Result:
  - Pass.

## Findings

No soundness findings.

## Ambiguities

- None blocking this fixture task.
- Unsafe block syntax, coroutine syntax, loops, match or when syntax, lambda syntax, indexing, flow typing, and safety analysis remain deferred.

## Decision

Pass.
