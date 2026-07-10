# Ambiguity Report: M0018 Type Checking Core

## Metadata

- Report ID: `M0018-type-checking-core`
- Related Task: `M0018-001`
- Related Milestone: `M0018`
- Filed By: `Language Lawyer`
- Date: `2026-07-10`
- Status: `open`
- Required Owner: `Language Designer`

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

- [ ] Language Lawyer determines whether existing text resolves it.
- [ ] Language Designer drafts ADR or spec revision if new semantics are required.
- [ ] Adversarial Engineer reviews soundness risk.
- [ ] Diagnostics Engineer reviews diagnostic consequences.
- [ ] Simplicity Guardian reviews complexity.
- [ ] Chief Architect approves final resolution.

## Temporary Rule

No implementation may proceed on literal typing, primitive scalar categories, assignment compatibility, call resolution, or function type application until the source of truth is updated or the ambiguity is ruled non-blocking by Chief Architect.

## Resolution

- Decision:
  - pending
- Source of truth updated:
  - pending
- Date resolved:
  - pending
