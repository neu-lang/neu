# Soundness Report: M0011-008

## Metadata

- Task ID: `M0011-008`
- Milestone: `M0011`
- Filed By: `main-task adversarial check`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0011-008-declaration-parser-implementation.md`
- Milestone file: `docs/milestones/M0011-declaration-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0022-declaration-syntax.md`
- Changed files:
  - `crates/newlang/src/parser.rs`
  - `crates/newlang/src/lib.rs`
  - `crates/newlang/src/ast.rs`
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0011-declaration-parser-implementation.sh`
- Ordinary test results:
  - `cargo test --workspace --all-targets`
  - `docs/tests/m0011-declaration-parser-implementation.sh`

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
Attack: Parser accepts deferred type, expression, statement, or pattern grammar as real syntax.
Expected result: Implementation exposes no parse_type, parse_expression, parse_statement, or parse_pattern entry points.
Actual result: Validator rejects those parser symbols; return types remain placeholders and invalid expression/field syntax produces parser diagnostics.
Source of truth: docs/adr/ADR-0022-declaration-syntax.md
Outcome: pass

Attack: Parser introduces semantic analysis or lowering.
Expected result: No symbol, name-resolution, HIR, MIR, ownership, or borrow concepts are present.
Actual result: Parser output is flat AST shell nodes plus diagnostics; HIR and MIR modules remain absent.
Source of truth: docs/tasks/M0011-008-declaration-parser-implementation.md
Outcome: pass

Attack: Diagnostics lack primary spans.
Expected result: Parser diagnostics carry ByteSpan.
Actual result: Diagnostic struct stores kind and span; tests assert ordered spans for misplaced package/import diagnostics.
Source of truth: ADR-0015; ADR-0022
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0011-declaration-parser-implementation.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0011-008-declaration-parser-implementation.md`
  - `docs/tests/m0011-declaration-parser-implementation.sh`
- Result:
  - `pass`

## Findings

None.

## Ambiguities

- Parser output remains a flat AST shell because child relationships and semantic payloads are not yet specified.
- Type, generic, expression, statement, and pattern parsing remain outside M0011.

## Decision

Pass.
