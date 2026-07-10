# Ambiguity Report: M0014 Module, Package, And Visibility Model

## Metadata

- Report ID: `M0014-MODULE-PACKAGE-VISIBILITY-MODEL`
- Related Task: `M0014-001`
- Related Milestone: `M0014`
- Filed By: `Language Lawyer`
- Date: `2026-07-10`
- Status: `open`
- Required Owner: `Language Designer`
- Blocking milestone: `M0014`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0017 states that modules are explicit compilation and visibility units and that packages or namespaces organize declarations within modules. It does not define module identity, module naming, file-to-module assignment, package-to-module mapping, default visibility, exact public/private/internal meaning, visibility metadata shape, duplicate package handling, package declaration absence behavior, module dependency metadata, or diagnostics.
```

## Competing Interpretations

1. A module is an explicit manifest-defined compilation unit.
2. A module is inferred from a source root or command-line compilation set.
3. A package is a visibility boundary and a module is only an artifact boundary.
4. A module is the only visibility boundary and packages are namespace labels only.

## Why Guessing Is Unsafe

- Name resolution depends on deterministic module and package identity.
- Visibility affects API compatibility and user-observable access errors.
- `internal` visibility cannot be implemented without knowing the module boundary.
- Package declaration absence affects generated or single-file test inputs.
- Later target-pack artifacts depend on stable module identity and dependency metadata.

## Affected Work

- Tasks blocked:
  - M0014 module identity implementation.
  - M0014 visibility metadata implementation.
- Milestones affected:
  - M0014
  - M0015
  - M0016
  - M0021
  - M0031
- Tests blocked:
  - Deterministic module identity tests beyond placeholder blocked-state checks.
  - Visibility metadata extraction tests beyond syntax-only modifier detection.
- Implementation areas blocked:
  - Module metadata data structures.
  - Package namespace representation.
  - Visibility model representation.
  - Module-aware name resolution.

## Recommended Resolution Path

- [ ] Language Designer drafts a module, package, namespace, and visibility semantics ADR or `docs/SPEC.md` revision.
- [ ] Language Lawyer audits consistency with ADR-0017, ADR-0022, and future name resolution.
- [ ] Build Engineer reviews module identity compatibility with target packs and cross compilation.
- [ ] Spec Compliance Auditor confirms tests compare against accepted source of truth.
- [ ] Chief Architect approves final source-of-truth update.

## Temporary Rule

No implementation may define module identity, package-to-module mapping, default visibility, `internal` meaning, module dependency format, or visibility diagnostics beyond recording this ambiguity until accepted source of truth resolves M0014.
