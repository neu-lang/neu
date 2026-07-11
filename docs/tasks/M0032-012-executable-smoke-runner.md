# Task: M0032-012 Executable Smoke Runner

## Task Metadata

- Task ID: `M0032-012`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Run the explicit executable output from a completed link invocation and expose
its process outcome for the bootstrap smoke test.

## Authority Extract

- ADR-0040 maps the language `main` result to the bootstrap process exit code.
- ADR-0047 requires a runnable no-stdlib executable smoke.
- ADR-0057 requires explicit target-pack and output paths, with no host-tool
  fallback.
- M0032-011 verifies that linking produced the requested output file.

## Scope

- Add an executable process runner to `LinkInvocation`.
- Run only the retained output path and no CLI arguments.
- Preserve normal exit codes and distinguish signal termination.
- Report an unavailable output as a structured error.
- Add host-side process smoke tests.

## Out Of Scope

- Generating the startup shim or linker artifacts.
- Mapping language `Int` values inside the compiler.
- Adding CLI argument semantics, stdlib, printing, or runtime services.
- Cross-target process execution or new language semantics.

## Tests

- An explicit executable fixture runs with no arguments and preserves its exit
  code.
- An unavailable output produces a structured launch error.

## Acceptance Criteria

- The runner executes exactly the invocation's output path.
- The runner passes no CLI arguments to the executable.
- Normal exit status remains observable to the caller.
- Missing output or launch failure is distinguishable from a process exit.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-011
  verifies a linked output file but does not run it. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the new
  executable-runner tests do not compile because `LinkInvocation::run` is
  absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  runner launches only the retained output path with no arguments and exposes
  exited or signaled outcomes. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused validator and all linker tests passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  no-argument exit-code case and unavailable-output case are covered without a
  fallback executable path. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0040,
  ADR-0047, ADR-0057, and M0032-011 compliance confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter, current
  example backend audit, executable-runner, link execution and output, and
  target-pack validators, Clippy, all workspace tests, and diff check.
  handoff=commit

## Required Outputs

- Authority read: ADR-0040, ADR-0047, ADR-0057, M0032-011, and M0032.
- Files changed: linker execution module and tests, this task, review and
  soundness reports, and the validator.
- Tests written before implementation and expected failure: runner tests fail
  because the execution API is absent.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/current-example-backend-surface.sh`,
  `docs/tests/m0032-executable-runner.sh`,
  `docs/tests/m0032-link-execution.sh`,
  `docs/tests/m0032-link-output.sh`,
  `docs/tests/m0032-target-pack-manifest.sh`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets`, and `git diff --check` all passed.
- Open questions: actual pack-owned startup and linker artifacts remain later
  M0032 distribution work.
- Remaining risk and next main-task action: commit and push; actual pack-owned
  startup and linker artifacts remain later M0032 distribution work.
