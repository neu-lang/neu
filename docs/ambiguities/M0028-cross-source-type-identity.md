# Ambiguity Report: M0028 Cross-Source Type Identity

## Metadata

- Report ID: `M0028-CROSS-SOURCE-TYPE-IDENTITY`
- Related Task: `M0028-011`
- Related Milestone: `M0028`
- Filed By: `main-task architecture review`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`, ADR-0027, ADR-0041, and ADR-0042 summaries.
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md`
  - `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0041 requires same-module/package calls across source files to compare
argument and parameter types, but the accepted authority does not define
whether bootstrap type identities are shared across source-file type passes.
```

## Competing Interpretations

1. Every source file owns an independent TypeArena and raw TypeId values may be
   compared across them.
2. One module-wide TypeArena owns all executable-source type identities.
3. Primitive types use a separate globally stable identity while other types
   remain source-local.

## Why Guessing Is Unsafe

- Independent arena insertion order can make unrelated raw IDs appear equal or
  equivalent types appear different.
- Direct-call compatibility and result typing would become order-dependent.
- A partial primitive-only workaround would hardcode an unaccepted type-identity
  architecture before later nominal types and HIR.

## Affected Work

- Tasks blocked: `M0028-011`.
- Milestones affected: `M0028`.
- Tests blocked: cross-source direct-call argument/result type checks.
- Implementation areas blocked: `crates/compiler/src/type_check.rs`.

## Recommended Resolution Path

- [ ] main-task semantic design selects the bootstrap cross-source type identity
  model.
- [ ] diagnostics, adversarial, and simplicity reviews assess consequences.
- [ ] main task accepts an ADR bundle.

## Temporary Rule

Do not compare TypeId values produced by separate source-file TypeArenas for
direct-call checking until accepted authority defines their identity domain.

## Resolution

- Decision: ADR-0052 requires one module-wide TypeArena.
- Source of truth updated: `docs/adr/ADR-0052-bootstrap-module-type-identity.md` and `docs/SPEC.md`.
- Date resolved: `2026-07-11`.
