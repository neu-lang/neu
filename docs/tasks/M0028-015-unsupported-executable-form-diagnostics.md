# Task: M0028-015 Unsupported Executable Form Diagnostics

## Task Metadata

- Task ID: `M0028-015`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Reject parsed forms outside the ADR-0042 executable subset before HIR.

## Authority Extract

- ADR-0042 exact executable subset and unsupported-form requirement.
- ADR-0015 diagnostic semantics.

## Scope

- Diagnose deferred parsed executable forms before HIR.
- Preserve more-specific accepted diagnostics where they already apply.

## Out Of Scope

- Accepting a new language form, HIR, MIR, backend, runtime, or linker work.

## Resolved Dependency

ADR-0053 resolves provenance, recovery, and nested-form suppression. Its
implementation remains the next task action.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=blocked evidence=unsupported executable diagnostic contract is incomplete. handoff=semantic-design
- 2026-07-11 main_task=main phase=semantic-resolution result=pass evidence=ADR-0053 accepted after language, diagnostics, adversarial, simplicity, and spec-compliance reviews. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=unsupported executable-form checker API was absent. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=source-qualified outermost unsupported forms are recorded while specific-form descendants are suppressed. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0028-unsupported-executable-form-diagnostics.sh. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=nested unsupported descendants are suppressed and no executable fact is recorded; docs/tasks/soundness/M0028-015-soundness.md. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0042 and ADR-0053 compliance confirmed; docs/tasks/reviews/M0028-015-review.md. handoff=commit
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0028-unsupported-executable-form-diagnostics.sh. handoff=commit
