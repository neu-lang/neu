# Soundness Report: M0016-018

## Metadata

- Task ID: `M0016-018`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-018-local-binding-lookup.md`
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

- [x] Local lookup finds bindings only after their declaration statement.
- [x] Local lookup rejects references before a same-scope declaration.
- [x] Local lookup searches from the starting scope outward through parent scopes.
- [x] Visible inner bindings shadow visible outer bindings.
- [x] Not-yet-visible inner bindings do not hide visible outer bindings.
- [x] Missing local names return `unresolved_name` at the reference span.
- [x] Lookup does not inspect top-level declarations, imports, members, overloads, extensions, or type-directed candidates.

## Attacks Attempted

```text
Attack: Reference a local after its declaration.
Expected result: Lookup returns the declared local binding.
Actual result: local_binding_lookup_finds_visible_binding_after_declaration covers this behavior.
Source of truth: ADR-0026 local declaration-order visibility rule.
Outcome: pass
```

```text
Attack: Reference a local before its declaration.
Expected result: Lookup does not return the later local binding.
Actual result: local_binding_lookup_rejects_reference_before_declaration covers this behavior.
Source of truth: ADR-0026 local bindings are not visible before declaration.
Outcome: pass
```

```text
Attack: Put the same name in outer and inner scopes.
Expected result: Visible inner binding wins after its declaration; visible outer binding remains usable before the inner declaration.
Actual result: local_binding_lookup_uses_nearest_visible_scope and local_binding_lookup_continues_past_not_yet_visible_inner_binding cover this behavior.
Source of truth: ADR-0026 shadowing and declaration-order rules.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0016-018-local-binding-lookup.md`
- Result:
  - pass

## Findings

No blocker, high, medium, or low severity findings.

## Ambiguities

- None for direct local-only lookup over existing scope and binding indexes.

## Decision

Pass. The API implements local-only lookup and declaration-order visibility without adding full name-resolution orchestration.
