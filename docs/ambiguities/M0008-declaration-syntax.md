# Ambiguity Report: M0008 Declaration Syntax

## Metadata

- Report ID: `M0008-DECLARATION-SYNTAX`
- Related Task: `M0008-001`
- Related Milestone: `M0008`
- Filed By: `Language Lawyer`
- Date: `2026-07-09`
- Status: `open`
- Required Owner: `Language Designer`
- Blocking milestone: `M0011`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`

## Exact Ambiguous Text Or Missing Rule

```text
Accepted ADRs define modules, nominal types, interfaces, sealed sums, and API visibility at a semantic level, but they do not define package declarations, imports, visibility modifiers, function declarations, struct declarations, enum or sealed sum declarations, interface declarations, or member declaration grammar.
```

## Competing Interpretations

1. Infer Kotlin-like declaration syntax.
2. Infer Rust-like declaration forms where ownership semantics resemble Rust.
3. Define a smaller custom declaration grammar in a future accepted ADR.
4. Defer concrete declaration parser fixtures until grammar authority exists.

## Why guessing Is Unsafe

- Parser fixtures would become de facto language syntax.
- Declaration forms affect visibility, module identity, generics, ownership, and diagnostics.
- Kotlin-like syntax is a project constraint, not a complete grammar.
- Later changes would break early parser tests or require compatibility with accidental syntax.

## Affected Work

- Tasks blocked:
  - M0011 declaration parser tasks that parse concrete declarations.
- Milestones affected:
  - M0011
  - M0014
  - M0016
- Tests blocked:
  - Concrete declaration parser fixtures.
- Implementation areas blocked:
  - Declaration AST shape tied to concrete syntax.
  - Declaration recovery diagnostics.

## Recommended Resolution Path

- [ ] Language Designer drafts declaration syntax ADR or `docs/SPEC.md` revision.
- [ ] Language Lawyer audits the draft against existing ADRs.
- [ ] Diagnostics Engineer reviews declaration diagnostic obligations.
- [ ] Simplicity Guardian reviews grammar complexity.
- [ ] Chief Architect approves final source-of-truth update.

## Temporary Rule

No parser implementation may accept concrete declaration syntax until accepted source of truth defines it.

## Resolution

- Decision:
  - unresolved
- Source of truth updated:
  - none
- Date resolved:
  - unresolved
