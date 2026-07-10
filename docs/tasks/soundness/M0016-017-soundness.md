# Soundness Report: M0016-017

## Metadata

- Task ID: `M0016-017`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-017-build-scoped-local-bindings.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Changed files:
  - `crates/newlang/src/name_resolution.rs`
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Ordinary test results:
  - `cargo test -p newlang --test name_resolution`: pass
  - `docs/tests/m0016-name-resolution-data-model.sh`: pass
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`: pass

## Safety Invariants Checked

- [x] Local bindings are assigned to the nearest containing block scope.
- [x] Nested local bindings with the same name are accepted in distinct scopes.
- [x] Same-block duplicate local bindings keep the first binding and report `duplicate_name`.
- [x] Binding kinds from parser metadata are preserved.
- [x] Declaration-body scopes are not used as owners for block-local `val` or `var` bindings.
- [x] Builder does not implement local lookup or declaration-order visibility.

## Attacks Attempted

```text
Attack: Parse an outer local binding and an inner if-block binding.
Expected result: The outer binding uses the outer block scope; the inner binding uses the nested block scope.
Actual result: scoped_local_binding_builder_assigns_nearest_block_scope covers this behavior.
Source of truth: ADR-0026 nearest containing lexical block rule.
Outcome: pass
```

```text
Attack: Use the same local name in an outer and nested block.
Expected result: Both bindings are inserted because shadowing across nested scopes is allowed.
Actual result: scoped_local_binding_builder_allows_nested_shadowing covers this behavior.
Source of truth: ADR-0026 shadowing rule.
Outcome: pass
```

```text
Attack: Use the same local name twice in one block.
Expected result: The second insert is duplicate and emits `duplicate_name` at the second binding span.
Actual result: scoped_local_binding_builder_reports_same_block_duplicates covers this behavior.
Source of truth: ADR-0026 duplicate local binding diagnostic rule.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0016-017-build-scoped-local-bindings.md`
- Result:
  - pass

## Findings

No blocker, high, medium, or low severity findings.

## Ambiguities

- None for assigning parsed local bindings to nearest block scopes.

## Decision

Pass. The builder assigns local bindings to block scopes and reports same-scope duplicates without implementing lookup.
