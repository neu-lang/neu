# Soundness Report: M0016-025

## Metadata

- Task ID: `M0016-025`
- Milestone: `M0016`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-025-bind-accepted-name-references.md`
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

- [x] Combined binding uses only accepted M0016 reference classes already implemented.
- [x] Package-qualified type references are not also processed as unqualified type references.
- [x] Unresolved expression and type diagnostics are preserved.
- [x] Package-qualified expression/member lookup remains inactive.
- [x] Imports, cross-module lookup, member lookup, overloads, extensions, and type-directed lookup remain inactive.

## Attacks Attempted

```text
Attack: Combine simple expression, unqualified type, and package-qualified type references in one input.
Expected result: All supported references bind into one table.
Actual result: Covered by accepted_name_reference_binding_combines_expression_and_type_bindings.
Source of truth: M0016 accepted reference subset.
Outcome: pass

Attack: Use missing package-qualified type reference.
Expected result: Exactly one unresolved diagnostic from the package-qualified type binder.
Actual result: Covered by accepted_name_reference_binding_does_not_duplicate_package_qualified_type_diagnostics.
Source of truth: M0016-025 partitioning scope.
Outcome: pass

Attack: Use unsupported member expression syntax.
Expected result: Parser metadata still exposes only the base simple identifier, not member lookup.
Actual result: Existing parser and binding tests keep member names out of accepted binding.
Source of truth: ADR-0026 excluded member lookup.
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

- Package-qualified expression lookup remains deferred because parser metadata does not yet distinguish it from member access.

## Decision

Pass.
