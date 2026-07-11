# Task: M0028-015 Unsupported Executable Form Diagnostics

## Task Metadata

- Task ID: `M0028-015`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

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
