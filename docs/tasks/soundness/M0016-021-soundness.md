# Soundness Report: M0016-021

## Metadata

- Task ID: `M0016-021`
- Milestone: `M0016`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-021-bind-unqualified-function-references.md`
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

- [x] Local bindings are checked before top-level function fallback.
- [x] Top-level fallback is restricted to the current source file's package namespace.
- [x] Type declarations are not treated as function expression fallback.
- [x] Imports, cross-module lookup, member lookup, overloads, extensions, and type-directed lookup remain inactive.
- [x] Missing candidates produce `UnresolvedName` at the original parser reference span.

## Attacks Attempted

```text
Attack: Reference a function declared in another package in the same module.
Expected result: UnresolvedName diagnostic and no ResolutionTable entry.
Actual result: Covered by unqualified_function_reference_binding_rejects_other_package_top_level.
Source of truth: ADR-0026 current-package unqualified lookup rule.
Outcome: pass

Attack: Reference a type declaration from expression position.
Expected result: Function fallback does not bind the type declaration.
Actual result: Covered by unqualified_function_reference_binding_does_not_treat_types_as_function_fallback.
Source of truth: M0016-021 scope and out-of-scope type-name lookup.
Outcome: pass

Attack: Shadow a top-level function with a visible local.
Expected result: Local lookup succeeds before top-level fallback and no unresolved diagnostic is emitted.
Actual result: Covered by unqualified_function_reference_binding_keeps_local_lookup_before_top_level.
Source of truth: ADR-0026 lookup order and shadowing rules.
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

- None for function-only same-package fallback.

## Decision

Pass.
