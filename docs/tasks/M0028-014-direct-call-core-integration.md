# Task: M0028-014 Direct Call Core Integration

## Task Metadata

- Task ID: `M0028-014`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Integrate successfully checked same-package direct calls into executable-core
typing so they no longer retain the deferred-call diagnostic.

## Authority Extract

- `docs/SPEC.md` executable subset and diagnostic recovery sections.
- ADR-0041 direct-call eligibility and result typing.
- ADR-0051 direct-call diagnostic recovery.
- ADR-0052 module-wide type identity.

## Dependencies

- M0028-013 completion.

## Scope

- Carry source-qualified successful direct-call result types into executable
  core typing.
- Suppress `DirectCallDeferred` only for those successful calls.
- Preserve direct-call diagnostics and deferred diagnostics for all other calls.

## Out Of Scope

- New call forms, overloads, function values, generic calls, HIR, MIR,
  backend, runtime, and linker work.

## Test-First Gate

- Test: a valid same-package direct call records its `Int` result type without
  `DirectCallDeferred`; an invalid call still has no result type.
- Expected initial result: `fail`; executable core still classifies every call
  expression as deferred.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0028-013 establishes direct-call diagnostics and successful result types. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=apply_m0028_direct_call_results is absent and executable core still defers calls. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=source-qualified successful call result types remove only matching DirectCallDeferred diagnostics. handoff=validation
- 2026-07-11 main_task=main phase=focused-validation result=pass evidence=valid calls type to Int; invalid calls remain deferred and untyped. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0028-direct-call-core-integration.sh. handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=invalid direct-call reports leave DirectCallDeferred and no result type; docs/tasks/soundness/M0028-014-soundness.md. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=source-qualified integration complies with ADR-0041, ADR-0051, and ADR-0052; docs/tasks/reviews/M0028-014-review.md. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; docs/tests/m0028-direct-call-core-integration.sh. handoff=commit
