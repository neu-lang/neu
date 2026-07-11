# Task: M0032-010 Link Invocation Execution

## Task Metadata

- Task ID: `M0032-010`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Execute the linker selected by the resolved target pack and report launch or
non-success failures without consulting the host environment.

## Authority Extract

- ADR-0057 requires link invocation to consume only the validated pack linker
  path and generated object inputs, with no `PATH` lookup or host fallback.
- ADR-0047 requires a bundled linker path for the initial host executable
  pipeline.
- M0032-005 defines the deterministic invocation plan consumed by this task.

## Scope

- Add a process execution operation to `LinkInvocation`.
- Map an unavailable linker to a structured error.
- Map a non-success linker status to a structured error.
- Add host-side tests using an explicit executable fixture path.

## Out Of Scope

- Discovering tools through `PATH` or selecting a host fallback.
- Generating a linker, startup shim, object file, or executable.
- Validating executable contents or running the linked output.
- Cross-target packs, platform ABI changes, or new language semantics.

## Tests

- An executable at the resolved pack linker path is invoked successfully.
- A linker process with a non-success status produces a structured failure.
- The invocation continues to use the resolved pack path.

## Acceptance Criteria

- Execution starts exactly `LinkInvocation::program()` with its planned
  arguments.
- A successful process returns success.
- Launch failure and non-success status are distinguishable errors.
- No `PATH` lookup, fallback linker, or unrelated environment input is used.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-005
  provides a process-free plan but no execution operation. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the new
  execution tests do not compile because `LinkInvocation::execute` is absent.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  execution adapter runs the resolver-provided linker path and preserves the
  planned arguments. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused validator and all linker tests passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  unavailable-linker and non-success status cases are distinguished, with no
  fallback path. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0047,
  ADR-0057, and M0032-005 compliance confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter, current
  example backend audit, link-execution validator, target-pack validator,
  Clippy, all workspace tests, and diff check. handoff=commit

## Required Outputs

- Authority read: ADR-0047, ADR-0057, M0032-005, and M0032.
- Files changed: linker execution module and tests, this task, review and
  soundness reports, and the validator.
- Tests written before implementation and expected failure: execution tests
  fail because the execution API is absent.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/current-example-backend-surface.sh`,
  `docs/tests/m0032-link-execution.sh`,
  `docs/tests/m0032-target-pack-manifest.sh`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets`, and `git diff --check` all passed.
- Open questions: none.
- Remaining risk and next main-task action: commit and push; executable startup
  and linked-program smoke remain later M0032 tasks.
