# Task: M0032-008 Bootstrap Outcome Model

## Task Metadata

- Task ID: `M0032-008`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Model the startup shim's process outcome for a language `main` result or a
bootstrap runtime trap without launching a process.

## Authority Extract

- ADR-0040 limits the bootstrap smoke result to `Int` in `0..255`.
- ADR-0043 defines runtime integer traps.
- ADR-0047 requires non-success trap behavior and rejects unsupported exit
  values without inventing panic formatting.
- ADR-0057 supplies the target-pack non-success status.

## Scope

- Map `Int` results in `0..255` to successful process statuses.
- Map unsupported result values to a target-pack failure status.
- Preserve each bootstrap trap reason in an inspectable failure outcome.

## Out Of Scope

- Process spawning, platform syscalls, linker invocation, startup object
  generation, panic formatting, or standard-library behavior.
- New language return or trap semantics.

## Tests

- Boundary results `0` and `255` succeed.
- Negative and greater-than-255 results fail with the supplied status.
- Every ADR-0043 bootstrap trap maps to a failure outcome preserving its reason.

## Acceptance Criteria

- Mapping is deterministic and side-effect free.
- No failure outcome reports success.
- Trap reasons remain distinguishable for tests without user-facing formatting.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-007
  now emits the canonical language-entry symbol required by the startup shim.
  handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=bootstrap
  outcome module and mapping API are not yet implemented. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=outcome
  mapping accepts only `0..255`, maps unsupported results to the configured
  failure status, and preserves all bootstrap trap reasons. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  outcome tests and validator passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=no
  failure outcome can report success and no platform side effect exists.
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0040,
  ADR-0043, ADR-0047, and ADR-0057 compliance confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; example audit; bootstrap validator; diff check.
  handoff=commit

## Required Outputs

- Authority read: ADR-0040, ADR-0043, ADR-0047, ADR-0057, and M0032.
- Files changed: `crates/compiler/src/bootstrap.rs`, `crates/compiler/src/lib.rs`,
  bootstrap tests, this task, review and soundness reports, and the validator.
- Tests written before implementation and expected failure: outcome boundary and
  trap-preservation tests fail because the module is absent.
- Validation commands and results: all required gates passed.
- Open questions: platform-specific startup code remains a later task.
- Remaining risk and next main-task action: run full CI; platform startup code
  remains a later task.
