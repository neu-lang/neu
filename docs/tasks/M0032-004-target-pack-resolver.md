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
- 2026-07-11 main_task=main phase=test-first result=pending evidence=tests
  must be written before resolver implementation. handoff=test

## Required Outputs

- Authority read: ADR-0020, ADR-0047, ADR-0057, and M0032.
- Files changed: pending.
- Tests written before implementation and expected failure: pending.
- Validation commands and results: pending.
- Open questions: actual pack artifact distribution and linker invocation are
  later M0032 tasks.
- Remaining risk and next main-task action: write failing resolver tests, then
  implement the smallest path-safe boundary.
