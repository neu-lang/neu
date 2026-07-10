# Task: M0019-012 Record Refined Expression Types

## Task Metadata

- Task ID: `M0019-012`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner Agent: `Implementer`
- Created By: `Task Decomposer`
- Created Date: `2026-07-10`
- Branch: `task/M0019-012-refined-expression-type-records`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Milestone: `docs/milestones/M0019-nullability-and-flow-typing.md`

## Goal

Record per-use non-null expression type views for exact local binding uses inside active M0019 refinement regions.

## Motivation

M0019-010 records branch-scoped refinement facts, and M0019-011 preserves exact local binding identity for resolved uses. ADR-0028 requires these facts to produce per-use refined expression types without mutating the binding's original nullable type.

## Scope

- Enrich an existing `TypeCheckReport` from its recorded refinement facts.
- Match resolved local name uses to refinements by exact `LocalBinding` identity.
- Require name uses to be contained within the refinement's parser block region.
- Preserve resolved-use order in refined expression type output.
- Inherit active refinements into nested blocks inside the branch.
- Skip uses in conditions, sibling branches, after the branch, or resolving to shadowing bindings.
- Report `ambiguous_flow_rule` with `ambiguous_null_test_region` when one use matches multiple active refinements.
- Add focused Rust tests and a docs validator.

## Out Of Scope

- Nullable-use diagnostics when no active refinement exists.
- Assignment compatibility using refined types.
- Mutation invalidation.
- Changing declaration signatures or original expression type entries.
- Boolean-combination, negated-condition, member, parameter, mutable-binding, or exclusive-borrow refinements.
- Updating examples; this task exposes already accepted flow semantics only through internal type-check side tables.

## Required Inputs

- Branch refinement records from M0019-010.
- Exact resolved local binding identities from M0019-011.
- Parser block and name-expression spans.
- ADR-0028 branch, nested scope, shadowing, and ambiguity rules.

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - Exact binding uses directly inside a refined branch receive the refined non-null type.
  - Uses in nested blocks inside the refined branch inherit the refinement.
  - Refined records preserve region, original nullable type, refined non-null type, and resolved-use order.
- Negative tests:
  - The condition use, a same-name shadowing binding use, and a use after the branch are not refined.
  - Uses outside any active refinement produce no new flow diagnostic in this slice.
- Diagnostic tests:
  - A use matching multiple active refinement regions reports `AmbiguousFlowRule` with `AmbiguousNullTestRegion` and receives no refined type.
- Adversarial tests:
  - Matching must use exact binding identity and AST region containment, not symbol text alone.

## Test-First Gate

- Test files to create before implementation:
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-refined-expression-type-records.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - Per-use M0019 refined expression type recording API does not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add a focused flow-report enrichment function that evaluates resolved local name uses against existing refinement records, records unambiguous per-use views, and emits the accepted ambiguity diagnostic for overlapping applicable regions.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Reviewer compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Examples decision is recorded.

## Execution Commands

- Generate tests: `edit crates/newlang/tests/type_check.rs and create docs/tests/m0019-refined-expression-type-records.sh`
- Verify tests fail: `cargo test -p newlang --test type_check m0019_refined_expression_type_records`
- Ordinary tests: `cargo test -p newlang --test type_check m0019_refined_expression_type_records && sh docs/tests/m0019-refined-expression-type-records.sh`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-012-refined-expression-type-records.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-012-refined-expression-type-records.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && sh docs/tests/m0019-refined-expression-type-records.sh && sh docs/tests/m0019-local-binding-resolution-identity.sh && sh docs/tests/m0019-branch-refinement-records.sh && sh docs/tests/m0019-null-test-eligibility.sh && sh docs/tests/m0019-null-test-recognition.sh && sh docs/tests/m0019-parser-flow-metadata.sh && sh docs/tests/m0019-flow-output-data-model.sh && sh docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-refined-expression-type-records.sh`
- Implementation files:
  - `crates/newlang/src/type_check.rs`
- Documentation or checklist files:
  - `docs/tasks/M0019-012-refined-expression-type-records.md`
  - `docs/tasks/reviews/M0019-012-review.md`
  - `docs/tasks/soundness/M0019-012-soundness.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md` or `docs/adr/`.
- Do not weaken or delete failing tests without reviewer approval.
- Do not match refinements by textual name or symbol alone.
- Do not refine uses outside the recorded branch region.
- Do not mutate original declaration signatures or expression types.
- Do not implement nullable-use checking, assignment integration, or invalidation.

## Ambiguities And Dependencies

- Later M0019 tasks must consume refined expression views during non-null-required use checking and implement invalidation diagnostics.

## Execution Log

- 2026-07-10 agent=Task-Decomposer phase=create-task result=pass notes=Created task for binding-identity-aware per-use refined expression type records.
- 2026-07-10 agent=Test-Engineer phase=generate-tests result=pass notes=Added active-region, nested-shadowing, region-boundary, overlap-ambiguity tests and a docs validator before implementation.
- 2026-07-10 agent=Test-Engineer phase=verify-tests-fail result=pass notes=Focused Rust tests failed on the missing `record_m0019_refined_expression_types` import and the validator failed on the absent function.
- 2026-07-10 agent=Implementer phase=implement result=pass notes=Added report enrichment using exact local binding identity, AST name/block containment, source-order output, and overlap ambiguity diagnostics.
- 2026-07-10 agent=Test-Engineer phase=ordinary-tests result=pass notes=Focused tests, all 55 type-check tests, docs validator, formatting, and `git diff --check` passed.
- 2026-07-10 agent=Adversarial-Engineer phase=add-attacks result=pass notes=Added forged non-name resolution and cross-source-file containment attacks after ordinary tests.
- 2026-07-10 agent=Adversarial-Engineer phase=adversarial-check result=pass notes=All four focused attacks passed and the harness produced a concrete soundness report after ordinary tests.
- 2026-07-10 agent=Reviewer phase=review result=pass notes=Compared implementation against SPEC, ADR-0028, and M0019; no scope, compliance, architecture, or maintainability findings; approved pending final CI.
- 2026-07-10 agent=Build-Engineer phase=ci result=pass notes=Formatting, workspace clippy with warnings denied, all 195 workspace tests, all listed M0019 validators, and the M0002 baseline CI gate passed.
- 2026-07-10 agent=Task-Decomposer phase=milestone-checklist result=pass notes=No M0019 completion item changed; per-use views are now recorded, but nullable-use integration and mutation invalidation remain incomplete.
- 2026-07-10 agent=Implementer phase=examples-decision result=pass notes=No examples update; this task adds internal type-check side-table output without changing accepted source forms or compiler accept/reject behavior.

## Handoff

- Next Agent: `Task Decomposer`
- Reason: `Create the next M0019 task for nullable-use checking with active refinements.`
- Required Context:
  - This task file
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/src/name_resolution.rs`
