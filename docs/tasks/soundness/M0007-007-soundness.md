# Soundness Report: M0007-007

## Metadata

- Task ID: `M0007-007`
- Milestone: `M0007`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0007-007-concrete-lexer-fixtures.md`
- Milestone file: `docs/milestones/M0007-lexer-implementation.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0021-lexical-grammar.md`
- Changed files:
  - `docs/lexer/token-model.md`
  - `docs/tests/m0007-lexer-fixtures.sh`
  - `tests/fixtures/lexer/*.fixture.toml`
- Ordinary test results:
  - `docs/tests/m0007-lexer-fixtures.sh` passed.

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
Attack: Encode parser precedence in lexer fixtures.
Expected result: Rejected.
Actual result: Fixtures contain expected token streams only and no parser precedence, AST, HIR, MIR, or backend expectations.
Source of truth: docs/adr/ADR-0021-lexical-grammar.md
Outcome: pass

Attack: Cite Kotlin precedent instead of accepted ADR-0021.
Expected result: Rejected.
Actual result: Fixtures cite docs/adr/ADR-0021-lexical-grammar.md and contain no Kotlin authority text.
Source of truth: docs/adr/ADR-0021-lexical-grammar.md
Outcome: pass

Attack: Add lexer code during fixture task.
Expected result: Rejected.
Actual result: No lexer module, token module, parser, or AST code was added.
Source of truth: docs/tasks/M0007-007-concrete-lexer-fixtures.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0007-lexer-fixtures.sh`
- Tests run:
  - `docs/tests/m0007-lexer-fixtures.sh`
- Result:
  - pass

## Findings

No soundness findings.

The fixture task does not alter memory-safety, ownership, borrowing, thread-safety, coroutine, unsafe, or FFI semantics.

## Ambiguities

- Executable lexer API remains for the implementation task.
- Parser grammar remains out of scope.

## Decision

Pass.
