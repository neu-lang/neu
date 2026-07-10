# Soundness Report: M0016-024

## Metadata

- Task ID: `M0016-024`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-024-bind-package-qualified-type-references.md`
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

- [x] Package-qualified type lookup uses the explicit package namespace in the current module.
- [x] Nested package paths split at the final dot.
- [x] Unqualified type names remain outside this binder.
- [x] Function declarations are not treated as package-qualified type candidates.
- [x] Imports, cross-module lookup, member lookup, overloads, extensions, and type-directed lookup remain inactive.

## Attacks Attempted

```text
Attack: Reference a same-module type through an explicit package from another source package.
Expected result: Type declaration binds through the explicit package namespace.
Actual result: Covered by package_qualified_type_reference_binding_uses_explicit_package_namespace.
Source of truth: ADR-0026 package-qualified lookup rule.
Outcome: pass

Attack: Use a nested package path.
Expected result: Split at the final dot and bind package `lib.core`, name `Result`.
Actual result: Covered by package_qualified_type_reference_binding_splits_nested_package_at_final_dot.
Source of truth: ADR-0026 and ADR-0025 qualified package namespaces.
Outcome: pass

Attack: Use a package-qualified function declaration in type position.
Expected result: UnresolvedName and no ResolutionTable entry.
Actual result: Covered by package_qualified_type_reference_binding_rejects_missing_and_function_candidates.
Source of truth: M0016-024 declaration-kind scope.
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
