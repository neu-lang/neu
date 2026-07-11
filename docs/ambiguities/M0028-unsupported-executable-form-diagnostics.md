# Ambiguity Report: M0028 Unsupported Executable Form Diagnostics

## Metadata

- Report ID: `M0028-UNSUPPORTED-EXECUTABLE-FORM-DIAGNOSTICS`
- Related Task: `M0028-015`
- Related Milestone: `M0028`
- Filed By: `main-task diagnostics review`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`, ADR-0015 and ADR-0042 summaries.
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`
- Milestone:
  - `docs/milestones/M0028-executable-expression-frontend-completion.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0042: "Unsupported parsed forms must fail before backend lowering with an
unsupported_executable_form diagnostic or a more specific existing diagnostic
when one already applies."

The accepted authority does not say which source form is primary for a nested
unsupported construct, what recovery facts are retained, or whether enclosing
and nested unsupported forms produce one diagnostic or many.
```

## Competing Interpretations

1. Emit one diagnostic for every unsupported AST node.
2. Emit one diagnostic at the outermost unsupported source form and suppress
   its unsupported descendants.
3. Reuse only existing deferred diagnostics and introduce no general
   `unsupported_executable_form` diagnostic.

## Why Guessing Is Unsafe

- Per-node reporting can bury the actionable source form in duplicate errors.
- Outermost-only reporting without a specified recovery boundary can retain
  invalid expression types for later lowering.
- Existing deferred diagnostics do not cover every deferred declaration,
  type, pattern, or runtime form listed by ADR-0042.

## Affected Work

- Tasks blocked:
  - `M0028-015`
- Milestones affected:
  - `M0028`
- Tests blocked:
  - unsupported executable form provenance and recovery fixtures.
- Implementation areas blocked:
  - `crates/compiler/src/type_check.rs`

## Recommended Resolution Path

- [x] main-task language review determines whether existing text resolves it.
- [x] main-task semantic design drafts an ADR defining provenance and recovery.
- [x] main-task adversarial check reviews nested-form recovery.
- [x] main-task diagnostics check reviews error volume and source spans.
- [x] main-task simplicity check reviews the smallest traversal boundary.
- [x] main task approves the final resolution.

## Temporary Rule

No implementation may proceed on unsupported executable-form diagnostics until
accepted authority defines the diagnostic contract.

## Resolution

- Decision:
  - ADR-0053 defines source-qualified outermost-form provenance, recovery, and
    suppression of nested unsupported descendants.
- Source of truth updated:
  - `docs/adr/ADR-0053-bootstrap-unsupported-executable-form-diagnostics.md`
  - `docs/SPEC.md`
- Date resolved:
  - `2026-07-11`
