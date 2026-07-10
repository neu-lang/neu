# Soundness Report: M0016-008

## Metadata

- Task ID: `M0016-008`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-008-parser-declaration-name-metadata.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Changed files:
  - `crates/newlang/src/parser.rs`
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Ordinary test results:
  - Parser tests, M0016 data-model validator, and M0016 accepted-state validator passed.

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
Attack: Accidentally record nested/member declaration names as top-level names.
Expected result: Nested declarations inside declaration bodies are excluded from parser declaration-name metadata.
Actual result: Parser test records only the top-level struct name and excludes nested function and enum names.
Source of truth: crates/newlang/tests/parser.rs
Outcome: pass

Attack: Create declaration metadata for missing names.
Expected result: Missing declaration names produce diagnostics but no declaration-name metadata.
Actual result: Parser test includes a missing function name and metadata excludes it.
Source of truth: crates/newlang/tests/parser.rs
Outcome: pass

Attack: Populate declaration index or implement lookup in the parser.
Expected result: Parser only records metadata.
Actual result: Validator rejects DeclarationIndex, resolver, lookup, and collection behavior in parser.rs.
Source of truth: docs/tests/m0016-name-resolution-data-model.sh
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Tests run:
  - `cargo test -p newlang --test parser && docs/tests/m0016-name-resolution-data-model.sh && docs/tests/m0016-name-resolution-policy-accepted.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- Parser-backed population of `DeclarationIndex` remains deferred.

## Decision

Pass for parser declaration-name metadata.
