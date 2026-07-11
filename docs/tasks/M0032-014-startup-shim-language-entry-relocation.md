# Task: M0032-014 Startup-Shim Language-Entry Relocation

## Task Metadata

- Task ID: `M0032-014`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Require the startup-shim object to contain a relocation to the manifest’s
canonical language-entry symbol.

## Authority Extract

- ADR-0046 requires the platform entry path to call the compiled language
  `main` symbol.
- ADR-0057 requires the startup shim to call the canonical language-entry
  symbol identified by the target-pack manifest.
- M0032-013 validates startup object format and platform entry presence.

## Scope

- Inspect startup-object relocation targets through the object reader.
- Resolve relocation symbol names and compare them with the manifest language
  symbol.
- Reject startup objects with no matching language-entry relocation.
- Add a real Cranelift object fixture containing an imported language-main call.

## Out Of Scope

- Proving the call’s ABI signature or exit-status mapping.
- Generating the startup shim or linker artifacts.
- Linking, running, or interpreting the executable.
- Cross-target relocation policy or new language semantics.

## Tests

- A startup object with a relocation to the manifest language symbol resolves.
- An object with no language-entry relocation is rejected.
- A relocation to a different symbol is rejected.

## Acceptance Criteria

- Validation checks relocation targets, not merely symbol-table membership.
- The required relocation names the manifest’s canonical language symbol.
- No host `PATH` lookup or linker process is introduced.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-013
  validates the platform entry but deliberately leaves the language-entry call
  out of scope. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the new
  relocation tests fail because target-pack resolution does not inspect
  startup-object relocations. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  resolver follows relocation targets to the manifest language symbol and
  handles the host object symbol prefix. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused validator and all target-pack tests passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  absent and mismatched relocation cases are rejected without linker or PATH
  access. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0046,
  ADR-0057, M0032-013, and M0032 compliance confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter, current
  example backend audit, startup-shim, executable-runner, link execution and
  output, and target-pack validators, Clippy, all workspace tests, and diff
  check. handoff=commit

## Required Outputs

- Authority read: ADR-0046, ADR-0057, M0032-013, and M0032.
- Files changed: target-pack loader and tests, this task, review and soundness
  reports, and the validator.
- Tests written before implementation and expected failure: missing and
  mismatched relocation cases fail until relocation validation exists.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/current-example-backend-surface.sh`,
  `docs/tests/m0032-startup-shim-validation.sh`,
  `docs/tests/m0032-executable-runner.sh`,
  `docs/tests/m0032-link-execution.sh`,
  `docs/tests/m0032-link-output.sh`,
  `docs/tests/m0032-target-pack-manifest.sh`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets`, and `git diff --check` all passed.
- Open questions: startup-call ABI signature and actual pack distribution remain
  later tasks.
- Remaining risk and next main-task action: commit and push; startup-call ABI
  signature and actual pack distribution remain later tasks.
