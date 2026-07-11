# Neu Compiler Operating Rules

Work in the main task. Do not delegate repository work or rely on role-specific
configuration.

## Authority

1. Project owner instructions.
2. `docs/SPEC.md`.
3. Accepted ADRs in `docs/adr/`.
4. This file.
5. The current task note.
6. Existing implementation behavior.

Existing behavior never overrides the specification or accepted ADRs. Do not
invent language semantics. If the authority is ambiguous or contradictory,
file an ambiguity report and stop the affected implementation.

## Normal Workflow

For ordinary implementation work:

1. Write one concise task note in the main task, referencing exactly one
   milestone.
2. Read only the note's cited specification, ADRs, source paths, and tests.
3. Write focused tests first and verify the expected failure.
4. Implement the smallest complete change justified by accepted authority.
5. Run focused tests, Clippy, and relevant full CI gates.
6. Perform a brief scope and source-of-truth review in the main task note.
7. Update examples only when the runnable language surface changes.
8. Stage scoped files and commit locally.
9. Do not push.

Create or revise an ADR or lengthy review document only when the
change introduces new language semantics, ownership/thread-safety rules, ABI
behavior, or another irreversible architectural decision. Accepted ADRs and
`docs/SPEC.md` must be updated before implementation relies on such a decision.

## Required Task Contents

The concise task note must state:

- authority read and bounded context;
- goal, scope, and out of scope;
- tests and objectively testable acceptance criteria;
- the expected pre-implementation test failure;
- files changed;
- validation commands and results;
- a brief scope/spec review;
- open questions, remaining risk, and next action.

Keep the note concise. Do not create separate task, review, soundness, or
validator files under `docs/` for ordinary work. Persist a separate ambiguity
report only when an ambiguity blocks safe implementation, or a soundness report
when an explicit soundness finding requires durable tracking.

## Non-Negotiable Rules

- `docs/SPEC.md` and accepted ADRs are the source of truth.
- Do not modify accepted ADRs except through an explicit revision or
  superseding ADR.
- Do not weaken, delete, skip, or rewrite failing tests to pass CI without a
  written review record justified by accepted authority.
- Do not silently broaden task scope.
- Do not treat Kotlin, Rust, or existing behavior as a substitute for a project
  decision.
- Keep abstractions proportional to accepted requirements.

## Ambiguity And Escalation

Record an ambiguity in the main task note when a semantic rule, required
diagnostic, recovery behavior, source span, or architectural boundary is
missing or contradictory. Quote the ambiguity, list competing interpretations,
explain why guessing is unsafe, and identify affected source-of-truth files.

Do not implement a semantic decision while its ambiguity remains unresolved.

## Commit And CI

Use branch names `codex/<topic>` unless the project owner requests another
name. Use concise subjects with existing prefixes such as `compiler:`,
`tests:`, `docs:`, `adr:`, `spec:`, `roadmap:`, or `build:`.

A task is complete only when its tests were created first, scope remains
bounded, the brief source-of-truth review passes, and CI passes. Minimum CI for
implementation changes is formatting, Clippy with warnings denied, workspace
tests, relevant negative/diagnostic tests, compiler smoke tests, and
cross-target smoke tests when target behavior changes.
