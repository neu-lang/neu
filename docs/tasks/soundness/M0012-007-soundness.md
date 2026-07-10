# Soundness Report: M0012-007

## Metadata

- Task ID: `M0012-007`
- Milestone: `M0012`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0012-007-type-ast-shell.md`
- Milestone file: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
- Changed files:
  - `crates/compiler/src/ast.rs`
  - `crates/compiler/tests/ast.rs`
  - `docs/tests/m0012-type-ast-shell.sh`
  - M0012 validator updates.
- Ordinary test results:
  - Focused AST and M0012 validators pass before this report.

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
Attack: Treat AST node kinds as semantic type representation.
Expected result: AST nodes remain syntax-only and do not contain type IDs, resolved symbols, constraints, or capability sets.
Actual result: `AstNode` still contains only id, kind, and source span.
Source of truth: docs/tasks/M0012-007-type-ast-shell.md
Outcome: pass
```

```text
Attack: Smuggle parser behavior into the AST task.
Expected result: parser source remains without type/generic parser APIs.
Actual result: `docs/tests/m0012-type-ast-shell.sh` checks parser source for absent type/generic parser entry points.
Source of truth: docs/tests/m0012-type-ast-shell.sh
Outcome: pass
```

```text
Attack: Encode capability, borrow, lifetime, coroutine, or unsafe semantics in AST names or fields.
Expected result: AST shell names mirror syntax categories only.
Actual result: validator rejects semantic markers in `crates/compiler/src/ast.rs`.
Source of truth: docs/tests/m0012-type-ast-shell.sh
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0012-type-ast-shell.sh`
  - `type_and_generic_shell_nodes_preserve_kind_and_span`
- Tests run:
  - focused AST test
  - `docs/tests/m0012-type-ast-shell.sh`
- Result:
  - Pass.

## Findings

No soundness findings.

## Ambiguities

- None blocking this AST shell task.
- Semantic type representation remains deferred to M0017 and later milestones.

## Decision

Pass.
