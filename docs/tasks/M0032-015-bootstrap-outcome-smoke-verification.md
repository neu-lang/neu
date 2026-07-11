# Task: M0032-015 Bootstrap Outcome Smoke Verification

## Task Metadata

- Task ID: `M0032-015`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Verify that a launched executable reports the process status required by the
bootstrap `main` outcome mapping.

## Authority Extract

- ADR-0040 maps a valid non-negative `main` result to the process exit code.
- ADR-0043 limits the bootstrap result mapping to `0..255`.
- ADR-0047 and ADR-0057 require unsupported results to fail with the pack’s
  non-success trap status.
- M0032-008 models `BootstrapOutcome` and M0032-012 runs explicit output paths.

## Scope

- Add a smoke-verification operation to `LinkInvocation`.
- Map valid `main` results through the existing `BootstrapOutcome` model.
- Use the target-pack trap status for unsupported result values.
- Report unavailable, signaled, and unexpected process outcomes distinctly.
- Add process-fixture tests for valid, unsupported, and mismatched outcomes.

## Out Of Scope

- Generating or linking the startup shim.
- Inferring the language `main` result from machine state.
- Panic formatting, stdlib, CLI arguments, or runtime services.
- Cross-target execution or new language semantics.

## Tests

- A process exit matching a valid `main` result is accepted.
- An unsupported `main` result expects the target-pack trap status.
- A different process exit is rejected with expected and actual statuses.

## Acceptance Criteria

- Valid results use the exact `BootstrapOutcome::Exit` status.
- Unsupported results use the resolved pack trap status.
- Signaled or unavailable processes never count as a successful smoke.
- No linker or host-tool discovery is introduced.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-008
  models bootstrap outcomes and M0032-012 runs the output, but no operation
  verifies their process-status contract. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the new
  outcome-verification tests do not compile because the verification API is
  absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  runner maps valid and unsupported main results through BootstrapOutcome and
  compares the expected status with the process outcome. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused validator and all linker tests passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  mismatch, unavailable, and signaled process paths cannot count as a passing
  smoke. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0040,
  ADR-0043, ADR-0047, ADR-0057, M0032-008, and M0032-012 compliance
  confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter, current
  example backend audit, bootstrap-outcome, startup-shim, executable-runner,
  link execution and output, and target-pack validators, Clippy, all workspace
  tests, and diff check. handoff=commit

## Required Outputs

- Authority read: ADR-0040, ADR-0043, ADR-0047, ADR-0057, M0032-008, M0032-012,
  and M0032.
- Files changed: linker runner and tests, this task, review and soundness
  reports, and the validator.
- Tests written before implementation and expected failure: verification tests
  fail because the API is absent.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/current-example-backend-surface.sh`,
  `docs/tests/m0032-bootstrap-outcome-smoke.sh`,
  `docs/tests/m0032-startup-shim-validation.sh`,
  `docs/tests/m0032-executable-runner.sh`,
  `docs/tests/m0032-link-execution.sh`,
  `docs/tests/m0032-link-output.sh`,
  `docs/tests/m0032-target-pack-manifest.sh`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets`, and `git diff --check` all passed.
- Open questions: actual pack-owned linker and startup artifacts remain later
  distribution work.
- Remaining risk and next main-task action: commit and push; actual pack-owned
  linker and startup artifacts remain later distribution work.
