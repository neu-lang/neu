# Soundness Report: M0012-001

## Metadata

- Task ID: `M0012-001`
- Milestone: `M0012`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `block pending ambiguity`

## Inputs Read

- Task file: `docs/tasks/M0012-001-type-generic-syntax-blocker.md`
- Milestone file: `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Changed files:
  - `docs/tasks/M0012-001-type-generic-syntax-blocker.md`
  - `docs/tests/m0012-type-generic-parser-blocked.sh`
  - `docs/milestones/M0012-type-and-generic-syntax-parser.md`
- Ordinary test results:
  - `docs/tests/m0012-type-generic-parser-blocked.sh`

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
Attack: Infer Kotlin-like nullable or generic syntax from project ergonomics.
Expected result: Blocked because accepted ADRs do not define concrete grammar.
Actual result: Blocker validator requires M0008 type/generic ambiguity to remain open and rejects parser type/generic APIs.
Source of truth: docs/ambiguities/M0008-type-generic-syntax.md
Outcome: pass

Attack: Add type AST nodes or type/generic fixtures before accepted syntax.
Expected result: Rejected by blocker validator.
Actual result: No type AST nodes, type fixtures, or generic fixtures exist.
Source of truth: docs/syntax/grammar-authority-ledger.md
Outcome: pass

Attack: Smuggle capability-bound parsing into the declaration parser.
Expected result: Rejected by validator.
Actual result: Parser source does not expose parse_type, parse_generic, parse_capability, or capability-bound structures.
Source of truth: docs/adr/ADR-0016-generics-and-parametric-polymorphism.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0012-type-generic-parser-blocked.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0012-001-type-generic-syntax-blocker.md`
  - `docs/tests/m0012-type-generic-parser-blocked.sh`
- Result:
  - `pass`

## Findings

```text
Severity: blocker
Invariant: Accepted source of truth controls parser semantics.
Finding: M0012 cannot safely implement parser behavior until type and generic syntax is accepted.
Evidence: docs/ambiguities/M0008-type-generic-syntax.md remains open.
Required fix: Accept type and generic syntax ADR or SPEC revision.
```

## Ambiguities

- Type syntax.
- Nullable marker placement.
- Generic parameter syntax.
- Generic argument syntax.
- Capability-bound syntax.
- Function type syntax.

## Decision

Block pending ambiguity.
