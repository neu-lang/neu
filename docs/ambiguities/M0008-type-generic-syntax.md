# Ambiguity Report: M0008 Type And Generic Syntax

## Metadata

- Report ID: `M0008-TYPE-GENERIC-SYNTAX`
- Related Task: `M0008-001`
- Related Milestone: `M0008`
- Filed By: `Language Lawyer`
- Date: `2026-07-09`
- Status: `open`
- Required Owner: `Language Designer`
- Blocking milestone: `M0012`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`

## Exact Ambiguous Text Or Missing Rule

```text
Accepted ADRs define nullable types, nominal user-defined types, interfaces or protocols, constrained generics, and capability bounds semantically, but they do not define concrete type grammar, nullable marker placement, generic parameter lists, generic argument lists, callable type syntax, or capability-bound syntax.
```

## Competing Interpretations

1. Use Kotlin-like nullable and generic syntax.
2. Use Rust-like generic and bound syntax with Kotlin-like surface declarations.
3. Define a smaller custom type grammar in a future accepted ADR.
4. Defer type parser fixtures until grammar authority exists.

## Why guessing Is Unsafe

- Type syntax affects public API shape and diagnostics.
- Generic bounds are tied to ownership, borrowing, thread-safety, and capability checking.
- Nullable syntax must align with flow typing and smart-cast semantics.
- Callable type syntax can constrain coroutine and suspension grammar.

## Affected Work

- Tasks blocked:
  - M0012 type and generic syntax parser tasks.
- Milestones affected:
  - M0012
  - M0017
  - M0018
  - M0020
- Tests blocked:
  - Concrete type parser fixtures.
  - Generic syntax fixtures.
- Implementation areas blocked:
  - Type syntax AST.
  - Generic parameter and bound parsing.

## Recommended Resolution Path

- [ ] Language Designer drafts type and generic syntax ADR or `docs/SPEC.md` revision.
- [ ] Language Lawyer audits consistency with nullability and generic ADRs.
- [ ] Adversarial Engineer reviews capability-bound and variance risks.
- [ ] Diagnostics Engineer reviews type syntax diagnostic obligations.
- [ ] Chief Architect approves final source-of-truth update.

## Temporary Rule

No parser implementation may accept concrete type or generic syntax until accepted source of truth defines it.

## Resolution

- Decision:
  - unresolved
- Source of truth updated:
  - none
- Date resolved:
  - unresolved
