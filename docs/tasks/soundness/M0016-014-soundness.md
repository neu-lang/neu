# Soundness Report: M0016-014

## Metadata

- Task ID: `M0016-014`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-014-build-local-binding-index.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- Changed files:
  - `crates/compiler/src/name_resolution.rs`
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Ordinary test results:
  - `cargo test -p compiler --test name_resolution`: pass
  - `docs/tests/m0016-name-resolution-data-model.sh`: pass
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`: pass

## Safety Invariants Checked

- [x] Parsed local binding names are interned before insertion into `LocalBindingIndex`.
- [x] Duplicate bindings in the supplied scope preserve the first binding and report `duplicate_name`.
- [x] Builder preserves insertion results for later diagnostics and review.
- [x] Builder preserves parsed `val` and `var` binding kinds.
- [x] Builder does not discover lexical scopes or assign scope ids from parser structure.
- [x] Builder does not implement local lookup or declaration-order visibility.

## Attacks Attempted

```text
Attack: Build an index from val and var parser metadata and check symbol order and binding kinds.
Expected result: Both bindings are inserted in parsed order with the supplied scope and original kind.
Actual result: builds_local_binding_index_from_parser_metadata covers this behavior.
Source of truth: ADR-0026 local val/var binding positions.
Outcome: pass
```

```text
Attack: Build an index from duplicate local names in the same supplied scope.
Expected result: The first binding remains canonical, the duplicate insert is preserved, and duplicate_name is emitted at the later binding span.
Actual result: local_binding_index_builder_reports_same_scope_duplicates covers this behavior.
Source of truth: ADR-0026 duplicate local binding diagnostic rule.
Outcome: pass
```

```text
Attack: Check whether the builder introduced lexical scope discovery, local lookup, or resolver orchestration.
Expected result: No lookup or scope construction APIs are introduced by this task.
Actual result: M0016 data-model validator still rejects resolver orchestration and scope stack names.
Source of truth: M0016-014 out-of-scope section.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0016-014-build-local-binding-index.md`
- Result:
  - pass

## Findings

No blocker, high, medium, or low severity findings.

## Ambiguities

- None for this explicit-scope builder slice.

## Decision

Pass. The builder converts parser metadata into local binding storage without implementing unresolved lexical scope or lookup semantics.
