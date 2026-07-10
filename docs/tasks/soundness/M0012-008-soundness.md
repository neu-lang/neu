# Soundness Report: M0012-008

## Metadata

- Task ID: `M0012-008`
- Milestone: `M0012`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0012-008-type-generic-parser-implementation.md`
- Milestone file: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
- Changed files:
  - `crates/newlang/src/parser.rs`
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0012-type-generic-parser-implementation.sh`
  - validator updates for M0012 implementation state.
- Ordinary test results:
  - Parser Rust tests and M0012 validators pass before this report.

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
Attack: Treat parsed capability bounds as capability semantics.
Expected result: parser records syntax nodes only.
Actual result: implementation creates syntax AST nodes and diagnostics only; no capability set or enforcement logic is added.
Source of truth: docs/adr/ADR-0023-type-and-generic-syntax.md
Outcome: pass
```

```text
Attack: Use type parser work to accept expression, statement, pattern, coroutine, or unsafe syntax.
Expected result: parser implementation remains limited to ADR-0023 type/generic positions.
Actual result: implementation validator rejects expression/statement/pattern/coroutine/unsafe parser markers.
Source of truth: docs/tests/m0012-type-generic-parser-implementation.sh
Outcome: pass
```

```text
Attack: Parse function parameter contents despite ADR-0022 placeholder scope.
Expected result: parameter contents remain balanced-parentheses placeholders.
Actual result: parser still uses balanced parenthesis consumption for function parameters and only parses return types after colon.
Source of truth: docs/tasks/M0012-008-type-generic-parser-implementation.md
Outcome: pass
```

```text
Attack: Hide malformed nullable, generic, capability-bound, or function type syntax without diagnostics.
Expected result: parser exposes ADR-0023 diagnostic kinds and tests assert malformed cases.
Actual result: parser tests assert malformed generic parameter list, malformed capability bound, malformed nullable type, malformed function type, and missing type name diagnostics.
Source of truth: crates/newlang/tests/parser.rs
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `parses_type_and_generic_syntax`
  - `reports_malformed_type_and_generic_syntax`
  - `docs/tests/m0012-type-generic-parser-implementation.sh`
- Tests run:
  - parser Rust tests
  - M0012 parser implementation validator
- Result:
  - Pass.

## Findings

No soundness findings.

## Ambiguities

- None blocking this task.
- M0013 expression, statement, and pattern syntax remains blocked on future accepted syntax authority.

## Decision

Pass.
