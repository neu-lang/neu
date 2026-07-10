# Ambiguity Report: M0026 Unsafe FFI Boundary Semantics

## Metadata

- Report ID: `M0026-unsafe-ffi-boundary-semantics`
- Related Task: `M0026-001`
- Related Milestone: `M0026`
- Filed By: `main task`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0018-unsafe-ffi-and-trust-boundaries.md`
  - `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Milestone:
  - `docs/milestones/M0026-unsafe-and-ffi-boundary-analysis.md`

## Exact Ambiguous Text Or Missing Rule

```text
Explicit unsafe functions and blocks, with module-level audit boundaries and
safe wrappers required for ordinary use.

FFI nullability, ownership, lifetime, and thread guarantees must be declared.

Unsafe block syntax is deferred.
```

Missing rules:

- Which unsafe functions, unsafe blocks, module audit boundaries, and FFI
  declarations are approved as source syntax.
- Whether M0026 is source-syntax based or metadata-only like M0023 through
  M0025.
- Which records represent unsafe contexts, unsafe operations, FFI declarations,
  safety metadata, target awareness, and safe-wrapper status.
- What exact conditions reject unsafe operations outside approved contexts.
- What exact metadata is required for FFI declarations.
- How diagnostics distinguish proven compiler-checked safety from trusted
  assertions.
- Which diagnostic identifiers and primary/secondary spans are required.
- Whether target-pack validation is in scope or only metadata presence is
  checked.

## Competing Interpretations

1. Define concrete unsafe and FFI source syntax now.
2. Use metadata-only unsafe-context, unsafe-operation, and FFI-declaration
   records for M0026.
3. Treat unsafe and FFI implementation as entirely deferred because unsafe block
   syntax is deferred.
4. Wait for target packs before validating any FFI metadata.

## Why Guessing Is Unsafe

- Unsafe and FFI are explicit trust boundaries; accepting too much silently
  weakens safe-code guarantees.
- FFI metadata affects nullability, ownership, lifetime, and thread-safety
  claims that safe callers may rely on.
- Target-aware semantics must not be confused with host-specific behavior.
- Diagnostics must make clear whether safety is proven by the compiler or only
  trusted by an unsafe declaration.

## Affected Work

- Tasks blocked:
  - `M0026-002` and later implementation tasks.
- Milestones affected:
  - `M0026`
  - backend and target-pack milestones that consume FFI metadata.
- Tests blocked:
  - Positive unsafe-boundary fixtures.
  - Negative unsafe misuse fixtures.
  - FFI metadata tests.
  - Diagnostic snapshots.
- Implementation areas blocked:
  - Unsafe boundary analysis.
  - FFI metadata validation.
  - Safe-wrapper checks.

## Recommended Resolution Path

- [ ] main-task language review determines whether existing text resolves it.
- [ ] main-task semantic design drafts ADR or spec revision if new semantics are
  required.
- [ ] main-task adversarial check reviews safe-code guarantee risk.
- [ ] main-task diagnostics check reviews proven-versus-trusted diagnostic
  consequences.
- [ ] main-task simplicity check reviews metadata surface.
- [ ] main task approves final resolution.

## Temporary Rule

No implementation may proceed on M0026 unsafe or FFI boundary analysis until the
source of truth defines either an approved source-syntax subset or a
metadata-only bootstrap subset with required diagnostics.

## Resolution

- Decision:
  - Accepted `docs/adr/ADR-0039-bootstrap-unsafe-ffi-boundary-analysis.md`.
- Source of truth updated:
  - `docs/SPEC.md`, `ADR-0039: Bootstrap Unsafe FFI Boundary Analysis`.
- Date resolved:
  - `2026-07-11`
