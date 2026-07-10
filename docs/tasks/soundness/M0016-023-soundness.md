# Soundness Report: M0016-023

## Metadata

- Task ID: `M0016-023`
- Milestone: `M0016`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-023-bind-unqualified-type-references.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Changed files:
  - `crates/newlang/src/name_resolution.rs`
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Ordinary test results:
  - `cargo test -p newlang --test name_resolution`: pass
  - `docs/tests/m0016-name-resolution-data-model.sh`: pass
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`: pass

## Safety Invariants Checked

- [x] Type-name lookup follows ADR-0026 local-first lookup order.
- [x] Top-level type fallback is restricted to the current source file's package namespace.
- [x] Function declarations are not treated as type-name fallback.
- [x] Package-qualified, import, cross-module, member, overload, extension, and type-directed lookup remain inactive.
- [x] Missing candidates produce `UnresolvedName` at the parser type-name span.

## Attacks Attempted

```text
Attack: Reference a type declared in another package in the same module.
Expected result: UnresolvedName diagnostic and no ResolutionTable entry.
Actual result: Covered by unqualified_type_reference_binding_rejects_other_package_top_level.
Source of truth: ADR-0026 current-package unqualified lookup rule.
Outcome: pass

Attack: Reference a function declaration from type position.
Expected result: Type fallback does not bind the function declaration.
Actual result: Covered by unqualified_type_reference_binding_does_not_treat_functions_as_type_fallback.
Source of truth: M0016-023 Scope and ADR-0026 declaration kind rules.
Outcome: pass

Attack: Shadow a top-level type with a visible local binding under ADR-0026 shared lookup order.
Expected result: Local lookup succeeds before top-level type fallback.
Actual result: Covered by unqualified_type_reference_binding_keeps_local_lookup_before_top_level.
Source of truth: ADR-0026 type-name lookup rule.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/name_resolution.rs`
- Tests run:
  - `cargo test -p newlang --test name_resolution`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Result:
  - pass

## Findings

No blocking, high, medium, or low findings.

## Ambiguities

- None for unqualified type references under ADR-0026's accepted shared lookup order.

## Decision

Pass.
