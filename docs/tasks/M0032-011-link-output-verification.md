# Task: M0032-011 Link Output Verification

## Task Metadata

- Task ID: `M0032-011`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Require a successful linker invocation to leave the requested executable
output as a regular file.

## Authority Extract

- ADR-0047 requires the first pipeline to produce a runnable executable.
- ADR-0057 requires the link adapter to consume the caller-selected output
  path through the validated target-pack invocation.
- M0032-010 executes the selected linker and reports process status.

## Scope

- Preserve the requested output path in `LinkInvocation`.
- Verify the output exists as a regular file after a successful process.
- Return a structured missing-output error.
- Add tests for linkers that do and do not create the requested output.

## Out Of Scope

- Inspecting executable format, entry symbols, or startup behavior.
- Running the produced executable.
- Generating target-pack artifacts or changing linker arguments.
- Cross-target support or new language semantics.

## Tests

- A successful explicit linker fixture that creates the output is accepted.
- A successful explicit linker fixture that creates no output is rejected.

## Acceptance Criteria

- The output path checked is the same path passed in the invocation plan.
- Non-success linker processes retain their existing structured failure.
- A zero exit status without a regular output file returns a distinct error.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-010
  executes the linker but does not verify the requested output artifact.
  handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the new
  missing-output test fails because a successful process currently returns
  success without checking its output. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  invocation retains its requested output and verifies a regular file after a
  successful linker status. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused validator and all linker tests passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  no-output success path is rejected while process and launch failures retain
  distinct errors. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0047,
  ADR-0057, and M0032-010 compliance confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter, current
  example backend audit, link execution and output validators, target-pack
  validator, Clippy, all workspace tests, and diff check. handoff=commit

## Required Outputs

- Authority read: ADR-0047, ADR-0057, M0032-010, and M0032.
- Files changed: linker execution module and tests, this task, review and
  soundness reports, and the validator.
- Tests written before implementation and expected failure: the missing-output
  test fails until output verification exists.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/current-example-backend-surface.sh`,
  `docs/tests/m0032-link-execution.sh`,
  `docs/tests/m0032-link-output.sh`,
  `docs/tests/m0032-target-pack-manifest.sh`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets`, and `git diff --check` all passed.
- Open questions: none.
- Remaining risk and next main-task action: commit and push; executable startup
  and linked-program smoke remain later M0032 tasks.
