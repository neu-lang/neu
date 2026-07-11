# Task: M0033-002 Target Capability Schema

## Task Metadata

- Task ID: `M0033-002`
- Milestone: `M0033`
- Milestone File: `docs/milestones/M0033-target-packs-and-cross-compilation-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Define and implement the accepted target-pack capability schema required for
cross-target compilation.

## Authority Extract

- ADR-0020 requires target-specific capability declarations.
- ADR-0057 defines artifact and entry metadata but does not define capability
  metadata.
- `docs/ambiguities/M0033-target-capability-schema.md` records the resolved
  authority gap.
- ADR-0058 defines the explicit typed bootstrap capability profile.

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
- 2026-07-11 main_task=main phase=ambiguity-resolution result=pass evidence=
  ADR-0058 and its required reviews were accepted under delegated Chief
  Architect authority; SPEC.md and the ambiguity report were updated.
  handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=capability
  registry tests failed because TargetCapabilities, manifest capability
  parsing, and InvalidCapabilities did not exist. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the host
  target pack declares the ADR-0058 profile and resolution validates it before
  artifact use. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused registry suite passed three tests. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=
  invalid bootstrap width is rejected and no host-derived capability fallback
  was added. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0058,
  SPEC.md, M0033, and task scope are aligned. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=`cargo fmt --all
  --check`, target-pack and ambiguity validators, current-example audit,
  Clippy with warnings denied, workspace tests, and `git diff --check` passed.
  handoff=commit

## Required Outputs

- Authority read: ADR-0020, ADR-0057, M0033, and the ambiguity report.
- Files changed: this task and the accepted source-of-truth and implementation
  files named by the resolution.
- Tests written before implementation and expected pre-implementation failure:
  capability-profile tests must fail because the manifest has no capability
  table or validation path yet.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/m0033-target-pack-registry.sh`,
  `docs/tests/m0033-target-capability-ambiguity.sh`,
  `docs/tests/current-example-backend-surface.sh`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, and
  `git diff --check` all passed.
- Open questions: `none`.
- Remaining risk: non-host packs still require their own accepted profiles and
  artifacts.
