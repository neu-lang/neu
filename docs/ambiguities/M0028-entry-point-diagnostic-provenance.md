# Ambiguity Report: M0028 Entry-Point Diagnostic Provenance

## Metadata

- Report ID: `M0028-ENTRY-POINT-DIAGNOSTIC-PROVENANCE`
- Related Task: `M0028-007`
- Related Milestone: `M0028`
- Filed By: `main-task diagnostics review`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`, ADR-0015 and ADR-0040 summaries.
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
  - `docs/adr/ADR-0040-bootstrap-program-entry-point.md`
- Milestone:
  - `docs/milestones/M0028-executable-expression-frontend-completion.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0040 requires missing_entry_point, duplicate_entry_point, and
invalid_entry_point_signature, but does not define primary locations,
multi-source provenance, recovery, or safe suggestions. ADR-0015 and
ADR-0024 require every diagnostic to define a primary span or external input
location.
```

## Competing Interpretations

1. Emit all entry diagnostics on an arbitrary selected source-file AST node.
2. Emit one diagnostic per invalid or duplicate candidate and use an external
   invocation location when no candidate exists.
3. Treat each source file independently, avoiding cross-file entry checking.

## Why Guessing Is Unsafe

- Arbitrary AST-node choice can misattribute a module-level failure.
- AST node IDs are arena-local and collide across parsed source files, so they
  cannot alone identify multi-file diagnostics.
- Per-file checking contradicts ADR-0040's one-program entry contract.

## Affected Work

- Tasks blocked:
  - `M0028-007`
- Milestones affected:
  - `M0028`
- Tests blocked:
  - package-scoped missing, duplicate, and invalid-entry diagnostic tests.
- Implementation areas blocked:
  - `crates/compiler/src/type_check.rs`

## Recommended Resolution Path

- [ ] main-task language review determines whether existing text resolves it.
- [ ] main-task semantic design drafts ADR or spec revision defining locations,
  recovery, and suggestions.
- [ ] main-task adversarial check reviews multi-source attribution risk.
- [ ] main-task diagnostics check reviews the diagnostic contract.
- [ ] main-task simplicity check reviews the minimal provenance boundary.
- [ ] main task approves final resolution.

## Temporary Rule

Do not implement package-scoped entry-point diagnostics until accepted
authority defines their source or external-input provenance.

## Resolution

- Decision:
  - ADR-0049 defines source-or-external-input provenance, recovery, and safe
    suggestions for all ADR-0040 entry diagnostics.
- Source of truth updated:
  - `docs/adr/ADR-0049-bootstrap-entry-point-diagnostic-provenance.md`
  - `docs/SPEC.md`
- Date resolved:
  - `2026-07-11`
