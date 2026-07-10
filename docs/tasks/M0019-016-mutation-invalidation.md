# Task: M0019-016 Mutation Invalidation

- Task ID: `M0019-016`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Test and implement one current-subset trigger: a simple unqualified name resolving to the same eligible immutable `T?` local, used later in the enclosing block after its guarded `if` branch closes, where `T` is required.

## Authority Extract

- `docs/SPEC.md`, “ADR-0028: Nullability And Flow Typing” (M0019 summary): exact region-exit diagnostic and deferred forms.
- `docs/adr/ADR-0031-region-exit-refinement-invalidation.md`, “Decision”: `invalidated_refinement`, rule `region_exit_invalidated_refinement`, primary span on the later bare name, no secondary span, recovery as original `T?`, independent-check continuation, and precedence over ADR-0030 for matching initializers.
- `docs/adr/ADR-0028-nullability-and-flow-typing.md`, “Branch Region Boundaries” and “Flow Diagnostics”.
- `docs/adr/ADR-0030-local-initializer-nullable-diagnostic.md`: mapping remains applicable outside ADR-0031’s exact precedence case.

## Required Tests (before implementation)

Fixtures and snapshots must cover the post-region trigger, non-triggers (inside/sibling/else/before/missing refinement/shadowing), spans, recovery, and initializer precedence. Expected pre-implementation result: failure.

Commands: `cargo test -p newlang --test type_check m0019_mutation_invalidation_classifies_only_exact_post_region_bare_name_initializer`; `sh docs/tests/m0019-region-exit-refinement-invalidation.sh`; `cargo fmt --all --check`.

## Scope / Non-scope

Implement only corresponding flow classification/checking. Defer assignment, mutable treatment, exclusive borrows, aliases, calls, suspension, member mutation, unsafe, FFI, and other effects. No parser, backend, examples, SPEC, ADR, or milestone edits.

## Reviews / Handoff

Required: Reviewer, Spec Compliance Auditor, Diagnostics Engineer, Adversarial Engineer, Build Engineer. Handoff: Test Engineer → Implementer → required reviewers; no unsupported trigger may be inferred.

## Execution Log

- 2026-07-10 `Task Decomposer` — ADR-0031 accepted in `e5adf69`; blocker cleared, task revalidated, tests-first handoff to `Test Engineer`.
- 2026-07-10 agent=Test-Engineer phase=test-first result=fail evidence=`cargo test -p newlang --test type_check m0019_mutation_invalidation_classifies_only_exact_post_region_bare_name_initializer` fails only on unresolved import `type_m0019_region_exit_refinement_invalidations`; `sh docs/tests/m0019-region-exit-refinement-invalidation.sh` fails on the same missing API. next=Implementer
- 2026-07-10 agent=Implementer phase=implementation result=pass evidence=`cargo fmt --all --check`; `cargo test -p newlang --test type_check m0019_mutation_invalidation_classifies_only_exact_post_region_bare_name_initializer` (1 passed); `sh docs/tests/m0019-region-exit-refinement-invalidation.sh`; `cargo test --workspace` (215 passed); `cargo clippy --workspace --all-targets -- -D warnings` passed. next=Reviewer, Spec-Compliance-Auditor, Diagnostics-Engineer, Adversarial-Engineer, Build-Engineer
- 2026-07-10 agent=Test-Engineer phase=focused-validation result=pass evidence=`cargo test -p newlang --test type_check m0019_mutation_invalidation_classifies_only_exact_post_region_bare_name_initializer` (1 passed, 71 filtered); `sh docs/tests/m0019-region-exit-refinement-invalidation.sh`; `cargo fmt --all --check`; `git diff --check`. next=Reviewer
- 2026-07-10 agent=Test-Engineer phase=adversarial-regression result=fail evidence=production-shaped condition metadata exposed multiple-region and if-condition matching gaps; focused test failed with `InvalidNullableUse` where `InvalidatedRefinement` was expected. next=Implementer
- 2026-07-10 agent=Implementer phase=repair result=pass evidence=classifier now accepts any applicable matching preceding region and matches refinement origins to `ParsedIfExpression.condition`; focused test, validator, formatting, clippy, and 215 workspace tests passed. next=Reviewer
- 2026-07-10 agent=Main phase=ordinary-tests result=pass notes=focused region-exit test, task validator, formatting, strict clippy, and 215 workspace tests passed after the production-metadata repair.
- 2026-07-10 agent=Test-Engineer phase=test-contract-repair result=pass evidence=added no-refinement resolved immutable nullable bare-name non-trigger and explicit region-exit nullable recovery assertion; `cargo test -p newlang --test type_check m0019_mutation_invalidation_classifies_only_exact_post_region_bare_name_initializer` (1 passed, 71 filtered); validator and `cargo fmt --all --check` passed; `git diff --check` passed. next=Reviewer
- 2026-07-10 agent=Test-Engineer phase=adversarial-test-repair result=fail evidence=added a second sequential refinement of the same immutable nullable binding and later direct enclosing-block bare-name initializer; focused test fails at the first post-region initializer with `InvalidNullableUse`/`NullableAssignmentWithoutRefinement`, expected `InvalidatedRefinement`/`RegionExitInvalidatedRefinement`. next=Implementer
- 2026-07-10 agent=Implementer phase=multiple-refinement-regression-fix result=pass evidence=region-exit classification now selects any matching preceding guarded region for the later direct bare name; focused test, validator, fmt, workspace tests (215 passed), and clippy passed. next=Reviewer, Spec-Compliance-Auditor, Diagnostics-Engineer, Adversarial-Engineer, Build-Engineer
- 2026-07-10 agent=Test-Engineer phase=final-review-test-first-repair result=fail evidence=aligned both refinement records with production `ParsedIfExpression.condition` provenance; focused test now fails at the first post-region initializer with `InvalidNullableUse`/`NullableAssignmentWithoutRefinement`, expected `InvalidatedRefinement`/`RegionExitInvalidatedRefinement`. next=Implementer
- 2026-07-10 agent=Main phase=adversarial-review result=pass evidence=two sequential regions, branch boundaries, shadowing, missing refinement, direct enclosing-block placement, recovery, and production condition provenance verified; `docs/tasks/soundness/M0019-016-soundness.md`. next=Reviewer
- 2026-07-10 agent=Main phase=review result=approve evidence=scope, test-first history, ADR-0031 diagnostic contract, and deferred orchestration boundary verified; `docs/tasks/reviews/M0019-016-review.md`. next=Milestone-Release
