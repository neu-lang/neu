# Soundness Report: M0016-020

## Metadata

- Task ID: `M0016-020`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-020-bind-local-name-references.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Changed files:
  - `crates/compiler/src/name_resolution.rs`
  - `crates/compiler/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Ordinary test results:
  - `cargo test -p compiler --test name_resolution`: pass
  - `docs/tests/m0016-name-resolution-data-model.sh`: pass
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`: pass

## Safety Invariants Checked

- [x] Local references cannot bind to declarations that appear later in the same block.
- [x] Local references cannot bind through top-level fallback in this local-only task.
- [x] Local reference binding uses the existing scoped local lookup rule.
- [x] Unsupported import, member, cross-module, and full-module resolution remain absent.
- [x] Diagnostics preserve the source span of the unresolved reference.

## Attacks Attempted

```text
Attack: Reference a local before its declaration.
Expected result: UnresolvedName diagnostic and no ResolutionTable entry.
Actual result: Covered by local_reference_binding_reports_reference_before_declaration.
Source of truth: ADR-0026 local visibility policy and M0016 unresolved-name acceptance criteria.
Outcome: pass

Attack: Reference a top-level declaration from a function body.
Expected result: Local-only binder reports UnresolvedName because top-level fallback is out of scope.
Actual result: Covered by local_reference_binding_does_not_use_top_level_fallback.
Source of truth: M0016-020 Out Of Scope.
Outcome: pass

Attack: Smuggle broader resolver orchestration into the data-model step.
Expected result: No import/member/cross-module/full resolver APIs are added.
Actual result: M0016 data-model validator still rejects forbidden resolver orchestration names.
Source of truth: ADR-0026 exclusions and M0016-020 Forbidden Changes.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/name_resolution.rs`
- Tests run:
  - `cargo test -p compiler --test name_resolution`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Result:
  - pass

## Findings

No blocking, high, medium, or low findings.

## Ambiguities

- None for this local-only binding slice.

## Decision

Pass.
