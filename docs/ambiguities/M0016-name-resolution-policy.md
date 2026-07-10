# Ambiguity Report: M0016 Name Resolution Policy

## Metadata

- Report ID: `M0016-NAME-RESOLUTION-POLICY`
- Related Task: `M0016-001`
- Related Milestone: `M0016`
- Filed By: `Language Lawyer`
- Date: `2026-07-10`
- Status: `open`
- Required Owner: `Language Designer`
- Blocking milestone: `M0016`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0017-modules-visibility-and-api-evolution.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Milestone:
  - `docs/milestones/M0016-name-resolution-pass.md`

## Exact Ambiguous Text Or Missing Rule

```text
M0016 requires approved local, module, and declaration name resolution, duplicate and unresolved-name diagnostics where specified, and positive and negative resolution tests. The accepted ADRs define syntax, module/package/visibility metadata, and symbol infrastructure prerequisites, but they do not define lookup order, import semantics, lexical or declaration scope boundaries, duplicate-name behavior, unresolved-name diagnostics, dependency lookup, shadowing, ambiguity handling, or the exact subset of approved names for the first name-resolution pass.
```

## Competing Interpretations

1. Resolve only top-level declarations inside one module and reject imports, local names, and member names.
2. Resolve package-qualified top-level names across all source files in one module, with imports remaining syntax-only.
3. Resolve local bindings, declaration names, and package-qualified references in one pass with lexical shadowing.
4. Treat import declarations as active aliases before module dependency semantics are defined.

## Why Guessing Is Unsafe

- Name resolution determines which declaration later type checking, ownership, borrowing, and diagnostics refer to.
- Import semantics affect public API shape and module dependency behavior.
- Duplicate-name behavior changes whether programs are accepted or rejected.
- Scope and shadowing rules affect local bindings, pattern bindings, and declaration lookup.
- Unresolved-name diagnostics are user-facing language behavior under ADR-0015.

## Affected Work

- Tasks blocked:
  - M0016 name resolution implementation.
  - M0016 unresolved-name diagnostics.
  - M0016 duplicate-name diagnostics.
- Milestones affected:
  - M0016
  - M0017
  - M0018
  - M0022
  - M0023
- Tests blocked:
  - Resolution fixture tests.
  - Negative unresolved-name tests.
  - Diagnostic snapshots for resolution failures.
- Implementation areas blocked:
  - Name resolution pass.
  - Resolution diagnostics.
  - Import lookup.
  - Scope hierarchy.
  - Duplicate declaration policy.

## Recommended Resolution Path

- [ ] Language Lawyer determines whether existing text resolves it.
- [ ] Language Designer drafts a name-resolution policy ADR or `docs/SPEC.md` revision.
- [ ] Adversarial Engineer reviews soundness risk.
- [ ] Diagnostics Engineer reviews unresolved-name and duplicate-name diagnostics.
- [ ] Simplicity Guardian reviews the bootstrap subset for overreach.
- [ ] Chief Architect approves final source-of-truth update.

## Temporary Rule

No implementation may define name resolution, lookup order, import semantics, scope boundaries, shadowing, duplicate-name behavior, unresolved-name diagnostics, dependency lookup, or ambiguity handling until accepted source of truth resolves M0016.

Symbol interning and name table storage from M0015 may remain as infrastructure, but they must not be treated as resolution policy.

## Resolution

- Decision:
  - unresolved
- Source of truth updated:
  - none
- Date resolved:
  - none
