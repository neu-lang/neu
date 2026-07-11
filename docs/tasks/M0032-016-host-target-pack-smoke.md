# Task: M0032-016 Host Target-Pack Smoke

## Task Metadata

- Task ID: `M0032-016`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Materialize the initial host target pack and prove an emitted Cranelift object
can link and run through its pack-owned linker and startup shim.

## Authority Extract

- ADR-0047 requires a no-stdlib runnable bootstrap executable.
- ADR-0057 requires a pack-owned pinned linker and target-specific startup
  object with explicit pack-relative paths.
- M0032-013 validates the startup object format and platform entry.
- M0032-014 validates its relocation to the language entry symbol.
- M0032-015 verifies bootstrap process outcomes.

## Scope

- Add the current-host target-pack manifest and provenance documentation.
- Add the pinned host linker artifact and startup-shim object.
- Add the startup-shim source used to produce the object.
- Add one end-to-end integration smoke from Cranelift object emission through
  link and process exit.

## Out Of Scope

- Cross-target packs or distribution automation.
- General linker flag configuration or public ABI policy.
- Standard library, printing, CLI arguments, or runtime services.
- New language syntax or semantic changes.

## Tests

- The pack manifest resolves for the current host target.
- The startup object and linker pass existing pack validation.
- A Cranelift-emitted `main` object links through the pack and runs.
- The process exit matches the emitted bootstrap `main` result.

## Acceptance Criteria

- No linker is discovered through `PATH`.
- The pack-owned linker is the process launched by `LinkInvocation`.
- The startup shim calls the emitted canonical language symbol.
- A minimal Cranelift-produced program exits with its `main` result.
- Artifact provenance and licensing are recorded beside the pack.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-015
  completed all in-process contracts but the repository contains no real host
  target-pack artifacts. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the new
  end-to-end smoke cannot resolve the absent host target-pack manifest.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  host pack contains a standalone LFS-backed lld, startup shim source/object,
  manifest, provenance, and a Cranelift-to-process smoke. Entry linkage is
  exported and the Darwin host link arguments are explicit. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  host-pack validator and end-to-end smoke passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  pack-owned linker is launched by explicit path, the startup shim resolves the
  exported language entry, and no PATH fallback is used. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0047,
  ADR-0057, M0032-013, M0032-014, M0032-015, and M0032 compliance confirmed.
  handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter, current
  example audit, host-pack, bootstrap-outcome, startup-shim, executable-runner,
  link, target-pack validators, Clippy, all workspace tests, diff check, and
  LFS attribute check. handoff=commit

## Required Outputs

- Authority read: ADR-0047, ADR-0057, M0032-013, M0032-014, M0032-015, and M0032.
- Files changed: host target-pack artifacts and provenance, linker entry export
  and host link arguments, integration tests, this task, review and soundness
  reports, and the validator.
- Tests written before implementation and expected failure: the integration
  smoke fails because the host pack does not exist yet.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/current-example-backend-surface.sh`,
  `docs/tests/m0032-host-target-pack.sh`,
  `docs/tests/m0032-bootstrap-outcome-smoke.sh`,
  `docs/tests/m0032-startup-shim-validation.sh`,
  `docs/tests/m0032-executable-runner.sh`,
  `docs/tests/m0032-link-execution.sh`,
  `docs/tests/m0032-link-output.sh`,
  `docs/tests/m0032-target-pack-manifest.sh`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets`, `git diff --check`, and
  `git check-attr filter -- target-packs/aarch64-apple-darwin/bin/ld64.lld` all
  passed.
- Open questions: cross-target pack distribution remains M0033 work.
- Remaining risk and next main-task action: commit and push; cross-target pack
  distribution remains M0033 work.
