# Task: M0032-003 Bundled Linker And Startup Contract

## Task Metadata

- Task ID: `M0032-003`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Resolve the missing target-pack contract required to link the M0032 host object
into a runnable executable without a hidden host-tool dependency.

## Authority Extract

- ADR-0020 requires Go-like bundled target packs and no hidden host dependency.
- ADR-0046 defines the language `main` ABI boundary but defers platform entry.
- ADR-0047 requires a bundled linker path and a tiny no-stdlib startup shim.
- M0032 acceptance requires an executable smoke test and documented toolchain
  inputs.

## Scope

- File the exact ambiguity around linker artifact, target-pack ownership,
  invocation contract, startup shim input, and bootstrap trap behavior.
- Compare linker/runtime designs against ADR-0020 and ADR-0047.
- Define the acceptance inputs required before implementation can resume.

## Out Of Scope

- Invoking `/usr/bin/ld`, `clang`, or another host linker as a substitute for a
  bundled target-pack tool.
- Implementing a linker, startup shim, executable entry path, or runtime trap
  handler while the authority is unresolved.
- Cross-target packs, standard library, printing, allocation, or CLI arguments.

## Tests

- `docs/tests/m0032-bundled-linker-contract.sh` verifies the ambiguity report,
  task status, and the still-unchecked linker/runtime milestone gates.
- No compiler implementation test is appropriate until the source-of-truth
  contract is accepted.

## Acceptance Criteria

- The ambiguity report quotes every missing linker/startup rule.
- Competing designs and their ADR-0020/ADR-0047 trade-offs are recorded.
- The affected M0032 implementation now has explicit target-pack inputs and
  remains prohibited from invoking hidden host dependencies.
- ADR-0057 and the SPEC summary name the target-pack inputs needed by
  M0032-004 and later link tasks.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-002
  emits a native object, while ADR-0047 still leaves the bundled link path
  unspecified. handoff=ambiguity-report
- 2026-07-11 main_task=main phase=ambiguity-audit result=blocked evidence=this
  host exposes `clang` but no bundled `lld`; using either would violate the
  accepted no-hidden-host-dependency direction. handoff=main-task architecture
- 2026-07-11 main_task=main phase=validation result=pass evidence=contract
  validator confirms the open ambiguity and unchanged linker/runtime checklist.
  handoff=main-task architecture
- 2026-07-11 main_task=main phase=resolution result=pass evidence=ADR-0057
  accepts a pack-owned pinned `lld`, startup shim, explicit pack root, and no
  host `PATH` fallback. handoff=M0032-004

## Required Outputs

- Authority read: ADR-0020, ADR-0046, ADR-0047, ADR-0057, and M0032.
- Files changed: this task, the accepted ADR and reviews, the SPEC summary,
  the resolved ambiguity report, the milestone input list, and M0032-004.
- Tests written before implementation and expected failure: no compiler test;
  the contract validator is the applicable pre-implementation check.
- Validation commands and results: the contract validator and `git diff --check`
  passed before and after ADR-0057 acceptance.
- Open questions: actual pack artifact distribution and linker invocation are
  later M0032 tasks.
- Remaining risk and next main-task action: implement M0032-004's resolver;
  actual linker invocation and startup validation remain later tasks.
