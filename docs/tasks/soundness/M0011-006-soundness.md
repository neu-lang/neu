# Soundness Report: M0011-006

## Metadata

- Task ID: `M0011-006`
- Milestone: `M0011`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0011-006-declaration-parser-fixtures.md`
- Milestone file: `docs/milestones/M0011-declaration-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0022-declaration-syntax.md`
- Changed files:
  - `docs/tests/m0011-declaration-parser-fixtures.sh`
  - `tests/fixtures/parser/declarations/positive.fixture.toml`
  - `tests/fixtures/parser/declarations/negative.fixture.toml`
  - `tests/fixtures/parser/declarations/diagnostics.fixture.toml`
- Ordinary test results:
  - `docs/tests/m0011-declaration-parser-fixtures.sh`
  - `docs/tests/m0011-declaration-syntax-accepted.sh`
  - `docs/tests/m0011-declaration-parser-blocked.sh`

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
Attack: Fixtures encode concrete type, expression, statement, or pattern grammar before authority exists.
Expected result: Fixture validator rejects expected_type, expected_expression, and expected_statement; fixture cases use only ADR-0022 placeholders.
Actual result: No deferred grammar expectations are present.
Source of truth: docs/adr/ADR-0022-declaration-syntax.md
Outcome: pass

Attack: Fixture task quietly introduces parser implementation.
Expected result: Parser source and executable parser tests remain absent.
Actual result: crates/newlang/src/parser.rs and crates/newlang/tests/parser.rs remain absent.
Source of truth: docs/tasks/M0011-006-declaration-parser-fixtures.md
Outcome: pass

Attack: Fixtures cite ecosystem precedent instead of accepted language authority.
Expected result: Fixtures cite docs/adr/ADR-0022-declaration-syntax.md and do not cite Kotlin, Rust, or Go.
Actual result: Fixture metadata cites accepted ADR-0022 only.
Source of truth: docs/AGENTS.md; docs/adr/ADR-0022-declaration-syntax.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0011-declaration-parser-fixtures.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0011-006-declaration-parser-fixtures.md`
  - `docs/tests/m0011-declaration-parser-fixtures.sh`
- Result:
  - `pass`

## Findings

None.

## Ambiguities

- Executable parser tests remain blocked until parser APIs and AST declaration nodes are implemented.
- Type, generic, expression, statement, and pattern grammar remain out of scope for these fixtures.

## Decision

Pass.
