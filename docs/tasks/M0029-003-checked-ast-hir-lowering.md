# Task: M0029-003 Executable AST Traversal Metadata

## Task Metadata

- Task ID: `M0029-003`
- Milestone: `M0029`
- Milestone File: `docs/milestones/M0029-hir-design-and-lowering.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Preserve executable function-body statement order and containment metadata so a
later checked AST-to-HIR lowering pass can traverse source without guessing.

## Authority Extract

- ADR-0041 direct call and return ordering.
- ADR-0042 executable source subset.
- ADR-0044 HIR runtime contract and rejection boundary.
- ADR-0052 module-wide type identity.
- ADR-0053 unsupported-form recovery.
- ADR-0054 return mismatch recovery.

## Scope

- Record ordered direct body statements for parsed executable functions.
- Associate local declarations, assignments, and return statements with their
  containing function body.
- Preserve source spans and exclude malformed or deferred forms from records.

## Out Of Scope

- Checked AST-to-HIR lowering, resolution, type, ownership, borrow, coroutine,
  unsafe, FFI, MIR, and backend work.

## Test-First Gate

- Test: an executable helper body preserves local, assignment, expression, and
  return statement order with its owning function.
- Expected initial result: `fail`; parser output has no executable-body
  traversal metadata.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0029-001 and M0029-002 provide HIR representation forms. handoff=test
- 2026-07-11 main_task=main phase=architecture-refinement result=pass evidence=AST nodes carry spans but no body child order; traversal metadata is required before lowering. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=parser output has no executable_body_statements metadata. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=successful direct body statements retain owning function and source order. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=only successful direct statements become traversal metadata; docs/tasks/soundness/M0029-003-soundness.md. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0044 lowering prerequisite confirmed; docs/tasks/reviews/M0029-003-review.md. handoff=commit
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0029-executable-ast-traversal-metadata.sh. handoff=commit
