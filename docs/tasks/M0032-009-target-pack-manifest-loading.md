# Task: M0032-009 Target-Pack Manifest Loading

## Task Metadata

- Task ID: `M0032-009`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Load the ADR-0057 logical target-pack fields from a pack-owned manifest on disk
and feed them into the existing safe resolver.

## Authority Extract

- ADR-0057 defines the logical manifest fields and pack-relative artifact rule.
- M0032-004 validates an in-memory manifest and artifacts.
- M0032-005 consumes the resolved pack without host lookup.

## Scope

- Define the pack manifest serialization used by the current compiler toolchain.
- Parse target, formats, linker/shim paths, entry symbols, and trap status.
- Reuse `TargetPack::resolve` for target and path validation.
- Add valid and malformed-manifest tests.

## Out Of Scope

- Linker process execution, startup-shim generation, artifact distribution,
  cross-target packs, or new language semantics.

## Tests

- A valid pack manifest loads and resolves its existing artifacts.
- Missing fields, malformed target triples, and invalid trap status fail before
  artifact resolution.

## Acceptance Criteria

- On-disk manifest fields map exactly to `TargetPackManifest`.
- The loader does not bypass target or pack-relative path validation.
- No host environment lookup or process launch occurs.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-004
  currently requires a Rust-built manifest while ADR-0057 describes a pack
  manifest as the target-pack input. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=manifest
  loading API is not yet implemented. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=strict
  TOML fields map to the logical manifest and resolver validation is reused.
  handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=valid
  and malformed manifest tests plus validator passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=unknown
  fields, malformed triples, missing fields, and invalid trap status fail before
  artifact resolution. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0057 and
  M0032-004 compliance confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter, current
  example backend audit, target-pack validator, Clippy, all workspace tests,
  and diff check. handoff=commit

## Required Outputs

- Authority read: ADR-0057, M0032-004, M0032-005, and M0032.
- Files changed: `Cargo.toml`, `Cargo.lock`, target-pack loader, target-pack
  tests, this task, review and soundness reports, and the validator.
- Tests written before implementation and expected failure: manifest loading
  test fails because the loader API is absent.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/current-example-backend-surface.sh`,
  `docs/tests/m0032-target-pack-manifest.sh`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets`, and `git diff --check` all passed.
- Open questions: actual linker and startup artifacts remain distribution work.
- Remaining risk and next main-task action: commit and push; actual linker and
  startup artifacts remain distribution work.
