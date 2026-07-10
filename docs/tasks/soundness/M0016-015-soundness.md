# Soundness Report: M0016-015

## Metadata

- Task ID: `M0016-015`
- Milestone: `M0016`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-015-local-scope-tree-model.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/name_resolution.rs`
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Ordinary test results:
  - `cargo test -p newlang --test name_resolution`: pass
  - `docs/tests/m0016-name-resolution-data-model.sh`: pass
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`: pass

## Safety Invariants Checked

- [x] Scope ids are stable and allocated by insertion order.
- [x] Scope records preserve owner AST node ids.
- [x] Scope records preserve optional parent scope ids.
- [x] Unknown scope ids do not resolve to arbitrary scopes.
- [x] The model does not construct scopes from parser structure.
- [x] The model does not implement local lookup or declaration-order visibility.

## Attacks Attempted

```text
Attack: Add root and child scopes and inspect assigned ids.
Expected result: Ids are 0 and 1 in insertion order.
Actual result: local_scope_tree_allocates_stable_ids_in_insertion_order covers this behavior.
Source of truth: M0016 deterministic data-model requirement.
Outcome: pass
```

```text
Attack: Retrieve root and child scopes and inspect owner and parent data.
Expected result: Owner AST nodes and parent relationships are preserved exactly.
Actual result: local_scope_tree_preserves_owner_and_parent covers this behavior.
Source of truth: ADR-0026 lexical scope boundaries.
Outcome: pass
```

```text
Attack: Request an unknown scope id.
Expected result: No scope is returned.
Actual result: local_scope_tree_unknown_scope_id_returns_none covers this behavior.
Source of truth: Safe data-model behavior.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0016-015-local-scope-tree-model.md`
- Result:
  - pass

## Findings

No blocker, high, medium, or low severity findings.

## Ambiguities

- None for this storage-only scope tree model.

## Decision

Pass. The change adds a deterministic local scope tree model without implementing parser traversal, binding assignment, or lookup.
