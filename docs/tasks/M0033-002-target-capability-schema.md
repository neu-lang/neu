# Task: M0033-002 Target Capability Schema

## Task Metadata

- Task ID: `M0033-002`
- Milestone: `M0033`
- Milestone File: `docs/milestones/M0033-target-packs-and-cross-compilation-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `blocked`

## Goal

Define and implement the accepted target-pack capability schema required for
cross-target compilation.

## Authority Extract

- ADR-0020 requires target-specific capability declarations.
- ADR-0057 defines artifact and entry metadata but does not define capability
  metadata.
- `docs/ambiguities/M0033-target-capability-schema.md` records the unresolved
  authority gap.

## Scope

- Resolve the capability-schema ambiguity through an accepted ADR or spec
  revision.
- Add the smallest manifest and compiler validation surface justified by that
  accepted decision.

## Out Of Scope

- Non-host target artifacts before the schema is accepted.
- ABI or language semantic changes not covered by the resolution.
- Host target behavior already validated by M0032.

## Tests

- Pending the accepted capability schema.

## Acceptance Criteria

- The ambiguity report is resolved by accepted source-of-truth text.
- Capability ownership, fields, validation, and diagnostics are explicit.
- Tests cover accepted positive and negative target capability cases.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=blocked evidence=the
  accepted ADRs do not define the target capability schema required by M0033.
  handoff=ambiguity-resolution

## Required Outputs

- Authority read: ADR-0020, ADR-0057, M0033, and the ambiguity report.
- Files changed: this task and the accepted source-of-truth and implementation
  files named by the resolution.
- Tests written before implementation and expected pre-implementation failure:
  pending resolution; no implementation may begin while ambiguous.
- Validation commands and results recorded after resolution.
- Open question: resolve `M0033-TARGET-CAPABILITY-SCHEMA`.
- Remaining risk: cross-target work cannot proceed soundly until resolution.
