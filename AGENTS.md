# Main Task Operating Rules

This compiler project is executed in the main task. Do not delegate repository
work or rely on role-specific configuration.

## Authority

1. Project owner instructions.
2. `docs/SPEC.md`.
3. Accepted ADRs under `docs/adr/`.
4. This file.
5. Roadmap, milestone, and task files.
6. Existing implementation behavior.

Existing behavior never overrides the specification or accepted ADRs. Do not
invent language semantics. When accepted authority is ambiguous, missing, or
contradictory, file an ambiguity report and stop the affected implementation.

## Main Task Workflow

1. Read the assigned task and its Authority Extract.
2. Read only the cited specification sections, ADRs, source paths, and tests.
3. Create or update the task before implementation when no accepted task
   exists.
4. Write tests before implementation and verify their expected failure.
5. Implement the smallest change justified by accepted authority.
6. Run ordinary tests, then adversarial checks, then review checks, then CI.
7. Record concise evidence in the task execution log.
8. Update examples immediately before a commit only when user-visible language
   support changes. Skip examples for internal-only changes.
9. Stage only task-scoped files and commit only after all required gates pass.

## Required Inputs

- The current user request or accepted task.
- This file.
- The task Authority Extract.
- The cited `docs/SPEC.md` sections and accepted ADRs.
- Relevant tests and validation commands.

Do not scan unrelated ADRs, tasks, or implementation files unless the bounded
context cannot answer a necessary question.

## Context, Parallelism, And Report Budget

Use the task Authority Extract as the default bounded context. Do not delegate
or parallelize repository work: all planning, testing, implementation, review,
and release work occurs in the main task. Keep routine task records concise;
persist detailed reports only for an ambiguity, a soundness finding, or an
explicit user request.

## Required Outputs

Every task record must state:

- Authority read.
- Files changed.
- Tests written before implementation and their expected pre-implementation
  failure.
- Validation commands and results.
- Open questions or `none`.
- Remaining risk and the next main-task action.

## Non-Negotiable Rules

- `docs/SPEC.md` and accepted ADRs are the source of truth.
- Do not weaken, delete, skip, or rewrite failing tests to pass CI without an
  explicit review record justified by accepted authority.
- Do not silently broaden task scope.
- Do not treat Kotlin, Rust, or existing behavior as a substitute for a project
  decision.
- Do not modify accepted ADRs except through an explicit revision or
  superseding ADR.
- Do not implement a semantic decision while an ambiguity report is unresolved.
- Keep abstractions proportional to accepted requirements.

## Review And Escalation

Review each implementation for task scope, source-of-truth compliance,
maintainability, test-first integrity, diagnostics, and unnecessary complexity.
For ownership, borrowing, lifetime, thread-safety, async, unsafe, FFI, or other
soundness boundaries, perform explicit adversarial checks after ordinary tests.

Escalate by filing an ambiguity report when:

- a semantic rule is missing or contradictory;
- a required diagnostic, recovery behavior, or source span is unspecified; or
- an architectural conflict cannot be settled by the authority hierarchy.

The report must quote the ambiguity, list competing interpretations, explain
why guessing is unsafe, and identify affected source-of-truth files.

## Commit And CI

Use branch names `codex/<topic>` unless the project owner requests another
name. Commit subjects use the existing project prefixes, such as `compiler:`,
`tests:`, `docs:`, `adr:`, `spec:`, `roadmap:`, or `build:`.

A task is complete only when its tests were created first, scope remains
bounded, required review and adversarial checks pass, and CI passes. Minimum CI
for implementation changes is formatting, linting, unit tests, relevant
negative tests, relevant diagnostic tests, compiler smoke tests, and
cross-target smoke tests when target behavior changes.
