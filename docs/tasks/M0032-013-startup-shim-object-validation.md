# Task: M0032-013 Startup-Shim Object Validation

## Task Metadata

- Task ID: `M0032-013`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Validate that the resolved startup-shim artifact is a parseable object in the
manifest-declared format and exports the manifest-declared platform entry
symbol.

## Authority Extract

- ADR-0046 requires the bootstrap link path to arrange a platform entry that
  calls language `main`.
- ADR-0047 requires a target-specific startup-shim object.
- ADR-0057 requires the manifest to identify the native object format and
  platform entry symbol.
- M0032-004 and M0032-009 resolve pack-relative artifacts and manifest fields.

## Scope

- Parse the startup-shim object through the object-format reader.
- Reject malformed startup-shim bytes.
- Reject an object format that differs from the manifest.
- Require the manifest platform entry symbol in the startup object.
- Add a real Cranelift-produced host object fixture in target-pack tests.

## Out Of Scope

- Proving the startup shim calls the language entry symbol.
- Generating or distributing linker and startup-shim artifacts.
- Executable linking, process execution, or runtime trap implementation.
- Cross-target object-format policy or new language semantics.

## Tests

- A real host-format startup object with the platform symbol resolves.
- Malformed bytes are rejected.
- A format mismatch is rejected.
- A valid object without the platform symbol is rejected.

## Acceptance Criteria

- Startup-shim validation runs after pack-relative path validation.
- Only the manifest-declared object format is accepted.
- The platform entry symbol must be present in the startup object.
- No host `PATH` lookup or linker process is introduced.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-009
  loads logical manifest fields but accepts placeholder startup bytes.
  handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the new
  malformed-startup and symbol-validation tests fail because startup object
  validation is absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  resolver parses the startup object, checks its declared format, and requires
  the platform entry symbol after safe path resolution. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused validator and all target-pack tests passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  malformed, mismatched-format, and missing-symbol cases are rejected without
  linker or PATH access. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0046,
  ADR-0047, ADR-0057, M0032-004, and M0032-009 compliance confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter, current
  example backend audit, startup-shim, executable-runner, link execution and
  output, and target-pack validators, Clippy, all workspace tests, and diff
  check. handoff=commit

## Required Outputs

- Authority read: ADR-0046, ADR-0047, ADR-0057, M0032-004, M0032-009, and M0032.
- Files changed: target-pack loader and tests, compiler dependency metadata,
  this task, review and soundness reports, and the validator.
- Tests written before implementation and expected failure: malformed,
  mismatch, and missing-symbol cases fail until object validation exists.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/current-example-backend-surface.sh`,
  `docs/tests/m0032-startup-shim-validation.sh`,
  `docs/tests/m0032-executable-runner.sh`,
  `docs/tests/m0032-link-execution.sh`,
  `docs/tests/m0032-link-output.sh`,
  `docs/tests/m0032-target-pack-manifest.sh`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets`, and `git diff --check` all passed.
- Open questions: startup-shim call relocation and actual pack distribution
  remain later tasks.
- Remaining risk and next main-task action: commit and push; startup-shim call
  relocation and actual pack distribution remain later tasks.
