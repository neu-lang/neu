# Soundness Report: M0019-011

## Metadata

- Task ID: `M0019-011`
- Milestone: `M0019`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-011-local-binding-resolution-identity.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Changed files:
  - `crates/compiler/src/name_resolution.rs`
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0019-local-binding-resolution-identity.sh`
- Ordinary test results:
  - `cargo test -p compiler --test name_resolution m0019_local_binding_resolution_identity`: pass
  - `cargo test -p compiler --test name_resolution`: pass
  - `sh docs/tests/m0019-local-binding-resolution-identity.sh`: pass

## Safety Invariants Checked

- [x] Each identity record comes directly from a successful M0016 local lookup.
- [x] Same-name outer and nested bindings retain distinct `LocalBinding` identities.
- [x] Uses before a nested shadowing declaration resolve to the visible outer binding.
- [x] Uses after a nested shadowing declaration resolve to the nested binding.
- [x] Uses after leaving the nested scope resolve to the outer binding again.
- [x] Unresolved uses do not receive a local binding identity record.
- [x] Existing symbol resolution and diagnostics remain unchanged.

## Attacks Attempted

```text
Attack: Use the same source name before and after a nested shadowing declaration.
Expected result: The first use records the outer binding and the second records the nested binding.
Actual result: Exact outer and nested LocalBinding records are preserved in source order.
Source of truth: ADR-0028 Shadowing And Nested Scope Rules.
Outcome: pass
```

```text
Attack: Use the shadowed name again after leaving the nested scope.
Expected result: The use records the outer binding rather than retaining the nested binding.
Actual result: The third record identifies the outer binding.
Source of truth: SPEC.md lexical scope rules and ADR-0028 binding-identity rules.
Outcome: pass
```

```text
Attack: Attempt to obtain binding identity for an unresolved name.
Expected result: No identity record is produced and the existing unresolved-name diagnostic remains.
Actual result: The identity output is empty and one UnresolvedName diagnostic is retained.
Source of truth: M0016 local name resolution and ADR-0028 exact-resolution requirement.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/name_resolution.rs`
- Tests run:
  - `cargo test -p compiler --test name_resolution m0019_local_binding_resolution_identity`
  - `cargo test -p compiler --test name_resolution`
  - `sh docs/tests/m0019-local-binding-resolution-identity.sh`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-011-local-binding-resolution-identity.md`
- Result:
  - pass

## Findings

No blocking, high, medium, or low soundness findings.

## Ambiguities

None. This task exposes existing local lookup identity and does not define when flow refinements apply.

## Decision

Pass. Future M0019 per-use refinement logic can compare exact local bindings without relying on textual names.
