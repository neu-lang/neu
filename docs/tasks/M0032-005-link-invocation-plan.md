# Task: M0032-005 Link Invocation Plan

## Task Metadata

- Task ID: `M0032-005`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Build a deterministic, process-free link invocation plan from a resolved target
pack, startup shim, object input, and executable output.

## Authority Extract

- ADR-0057 requires linker and shim inputs to come from the resolved pack and
  forbids host fallback.
- ADR-0047 requires no standard library or hidden runtime input for the first
  executable path.
- M0032-004 supplies validated pack-owned linker and startup-shim paths.

## Scope

- Model the link program, ordered arguments, selected entry symbol, and
  canonical language-entry symbol.
- Use only the resolved pack linker/shim and caller-provided object/output
  paths.
- Reject missing object input before a future process-launch boundary.

## Out Of Scope

- Spawning a linker process.
- Providing a target-pack binary, startup shim implementation, runtime trap
  handler, or executable smoke test.
- Host `PATH` lookup, standard library, libraries, or cross-target behavior.

## Tests

- A valid pack produces a deterministic `lld` invocation plan with no standard
  library arguments.
- A missing object input is rejected explicitly.
- The plan never spawns a process or discovers a host tool.

## Acceptance Criteria

- Program path equals the resolved pack linker path.
- Startup shim precedes the generated object in the ordered inputs.
- Output and entry arguments are deterministic and inspectable.
- No process is launched by this task.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-004
  now resolves pack-owned linker and shim paths. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=invocation
  plan module and API are not yet implemented; linker tests fail to compile.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  typed plan contains only the resolved pack linker, startup shim, entry symbol,
  object, and output; no process is launched. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  deterministic-plan and missing-object tests plus validator passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  plan has no host lookup or process-launch API and carries no standard-library
  inputs. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0047 and
  ADR-0057 compliance confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; example audit; target-pack validator; link-plan
  validator; diff check. handoff=commit

## Required Outputs

- Authority read: ADR-0047, ADR-0057, M0032-004, and M0032.
- Files changed: `crates/compiler/src/linker.rs`, `crates/compiler/src/lib.rs`,
  linker tests, this task, review and soundness reports, and the validator.
- Tests written before implementation and expected failure: deterministic-plan
  and missing-object tests fail because the invocation API is absent.
- Validation commands and results: all required gates passed.
- Open questions: actual pack binary and process execution remain later tasks.
- Remaining risk and next main-task action: run full CI; actual linker process
  execution and startup validation remain later tasks.
