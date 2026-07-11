# Task: M0032-004 Target-Pack Resolver

## Task Metadata

- Task ID: `M0032-004`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Implement the explicit target-pack manifest and artifact-resolution boundary
required by ADR-0057, without invoking a linker yet.

## Authority Extract

- ADR-0020 requires explicit target triples and no hidden host dependency.
- ADR-0047 requires a bundled linker path and startup shim for M0032.
- ADR-0057 defines pack-owned linker/shim artifacts, logical manifest fields,
  relative paths, and no `PATH` fallback.

## Scope

- Model the validated logical target-pack inputs needed by later link work.
- Resolve pack-relative linker and startup-shim paths for one exact target.
- Reject missing manifests, target mismatches, absolute paths, traversal, and
  missing artifacts.
- Add focused positive and negative tests.

## Out Of Scope

- Running `lld`, `ld`, `clang`, or any linker.
- Generating the startup shim, invoking the linker, executable output, exit
  status, runtime traps, cross-target packs, or distribution packaging.
- New language semantics or backend lowering changes.

## Tests

- Valid fixture resolves its exact target and both pack-owned artifacts.
- Missing, mismatched, absolute, traversal, and absent-artifact inputs fail
  explicitly.
- No test may pass by consulting the host `PATH`.

## Acceptance Criteria

- Resolver accepts only a complete manifest for the requested target triple.
- All resolved artifact paths remain inside the pack root.
- Linker and startup-shim paths are explicit outputs for the later link task.
- No linker process is started and no host fallback exists.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0057
  resolves M0032-003's linker/startup ambiguity. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=resolver
  module and API are not yet implemented; target-pack tests fail to compile.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=resolver
  validates the exact target, required manifest fields, pack-relative paths,
  artifact existence, and no traversal/absolute-path fallback. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  resolver tests and validator passed, including invalid-manifest and missing
  artifact cases.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  resolver canonicalizes artifacts inside the pack root and never consults
  `PATH` or starts a process.
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0020,
  ADR-0047, and ADR-0057 compliance confirmed. handoff=validation
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; target-pack validator; diff check. handoff=commit

## Required Outputs

- Authority read: ADR-0020, ADR-0047, ADR-0057, and M0032.
- Files changed: `crates/compiler/src/target_pack.rs`, `crates/compiler/src/lib.rs`,
  resolver tests, this task, review and soundness reports, and the validator.
- Tests written before implementation and expected failure: valid, mismatch,
  path-safety, and missing-artifact tests fail because the resolver API is
  absent.
- Validation commands and results: all required gates passed.
- Open questions: actual pack artifact distribution and linker invocation are
  later M0032 tasks.
- Remaining risk and next main-task action: link invocation and startup-shim
  validation remain for the next M0032 task.
