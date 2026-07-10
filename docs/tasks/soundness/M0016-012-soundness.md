# Soundness Report: M0016-012

## Metadata

- Task ID: `M0016-012`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-012-local-binding-storage.md`
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

- [x] Same-scope duplicate local bindings cannot silently replace the original binding.
- [x] Same local name in distinct scopes is representable for later shadowing semantics.
- [x] `val` and `var` binding kinds are represented without inventing additional local declaration kinds.
- [x] Local binding storage does not implement lexical lookup before declaration-order semantics exist.
- [x] Local binding storage does not introduce parser extraction or expression traversal.
- [x] Unsupported import, cross-module, member, overload, extension, and type-directed lookup remain unsupported.
- [x] Diagnostics are not invented for local duplicates before the diagnostic emission task exists.

## Attacks Attempted

```text
Attack: Insert two bindings with the same LocalScopeId and SymbolId but different AST nodes and mutability kinds.
Expected result: Duplicate result preserves the original binding and does not append the attempted binding.
Actual result: duplicate_local_binding_key_preserves_existing_binding covers this behavior.
Source of truth: ADR-0026 same-scope duplicate local binding rule.
Outcome: pass
```

```text
Attack: Insert the same SymbolId into two different LocalScopeId values.
Expected result: Both bindings are accepted so later nested-scope shadowing can be represented.
Actual result: local_binding_index_allows_same_name_in_distinct_scopes covers this behavior.
Source of truth: ADR-0026 local binding scope rule.
Outcome: pass
```

```text
Attack: Check whether this storage task added a scope stack, local lookup algorithm, parser extraction, or resolver orchestration.
Expected result: No such implementation appears in the changed name-resolution or parser surfaces.
Actual result: M0016 data-model validator rejects scope-stack and resolver orchestration names; parser file was not modified.
Source of truth: M0016-012 out-of-scope section.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0016-012-local-binding-storage.md`
- Result:
  - pass

## Findings

No blocker, high, medium, or low severity findings.

## Ambiguities

- None for this storage-only slice.

## Decision

Pass. The change models local binding storage needed by ADR-0026 without implementing unresolved lexical lookup or parser behavior.
