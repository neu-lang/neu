# Ambiguity Report: M0018 Type Checking Core

## Metadata

- Report ID: `M0018-type-checking-core`
- Related Task: `M0018-001`
- Related Milestone: `M0018`
- Filed By: `Language Lawyer`
- Date: `2026-07-10`
- Status: `resolved`
- Required Owner: `Language Designer`
- Resolution Source: `docs/adr/ADR-0027-type-checking-core.md`
- Resolved Date: `2026-07-10`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone:
  - `docs/milestones/M0018-type-checking-core.md`

## Exact Ambiguous Text Or Missing Rule

```text
M0018 scope includes "Approved expression and declaration type checking" and "Assignment and call checking only where specified."

M0018 risks state "Literal typing and overload rules may be unspecified" and "Call syntax and function type rules may require ambiguity reports."

M0017 risks state "Primitive type set may be unspecified."
```

## Competing Interpretations

1. Begin with default literal types and nominal primitive declarations chosen by the compiler implementation.
2. Treat literals, primitive scalar categories, calls, and assignment compatibility as blocked until accepted semantic rules exist.
3. Type check only explicit nominal identity equality and defer all expression forms that require literal, call, or assignment rules.

## Why Guessing Is Unsafe

- Literal defaults affect overload resolution, numeric conversions, ABI expectations, and diagnostics.
- Assignment compatibility decides when values can be moved or coerced, which affects later ownership and borrow checking.
- Call resolution may require overload, function type, and generic rules that are not accepted.
- Primitive scalar categories are explicitly listed as a M0017 risk and cannot be invented by M0018.

## Affected Work

- Tasks blocked:
  - `M0018-001`
- Milestones affected:
  - `M0018`
  - `M0019`
  - `M0020`
  - `M0022`
  - `M0023`
- Tests blocked:
  - Positive type-check fixtures for literals, calls, assignments, and primitive operations.
  - Negative type mismatch fixtures that depend on unspecified compatibility rules.
- Implementation areas blocked:
  - Type checker expression typing.
  - Type mismatch diagnostics for literals, calls, and assignments.
  - Primitive scalar representation.

## Recommended Resolution Path

- [x] Language Lawyer determines whether existing text resolves it.
- [x] Language Designer drafts ADR or spec revision if new semantics are required.
- [x] Adversarial Engineer reviews soundness risk.
- [x] Diagnostics Engineer reviews diagnostic consequences.
- [x] Simplicity Guardian reviews complexity.
- [x] Chief Architect approves final resolution.

## Temporary Rule

Implementation may define type checking only as specified by accepted ADR-0027.

## Resolution

- Decision:
  - ADR-0027 accepted a small bootstrap type-checking core with primitive type-checking identities, literal typing, exact assignment compatibility, diagnostic obligations, and explicit deferrals.
- Source of truth updated:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/SPEC.md`
- Date resolved:
  - `2026-07-10`
