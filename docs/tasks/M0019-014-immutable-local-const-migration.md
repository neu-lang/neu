# Task: M0019-014 Migrate Immutable Local Spelling To `const`

## Task Metadata

- Task ID: `M0019-014`
- Milestone: `M0019`
- Milestone File: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Status: `complete`
- Owner main task: `main-task test work` then `main-task implementation`
- Created By: `main-task task planning`
- Created Date: `2026-07-10`
- Branch: `task/M0019-014-immutable-local-const-migration`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0029-immutable-local-const-keyword.md`
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Project Rules: `main task rules`
- main task Prompts: `main task rules`, `main task rules`, `main task rules`, `main task rules`, `main task rules`, `main task rules`, `main task rules`, `main task rules`

## Goal

Complete the coherent source spelling migration from immutable-local `val` to immutable-local `const`, preserving the existing immutable binding category and adding no compile-time-constant semantics.

## Motivation

ADR-0029 is a hard replacement and its cross-phase invariant requires lexer, parser, syntax metadata, binding classification, resolution, typing, flow typing, ownership-facing fixtures, diagnostics, and terminology to agree. Partial migration would create contradictory frontend behavior.

## Scope

- Tests first for lexer tokens, parser declaration introducers and ordinary parser recovery, source-independent immutable binding classification, resolution, type checking, flow typing, and ownership-facing behavior.
- Migrate the lexer/parser declaration introducer and syntax metadata from `val` to `const`; treat `val` as an ordinary identifier except in the removed introducer position.
- Preserve existing parser recovery; do not add a special legacy-`val` diagnostic, alias, fix-it, recovery-success path, or legacy AST/semantic record.
- Preserve the existing immutable-local semantic category independent of source spelling, including smart-cast eligibility and all existing initializer, ownership, borrowing, destruction, and thread-safety boundaries.
- Update implemented-state documentation terminology only where needed to describe immutable local bindings and the `const` spelling accurately.
- Add or update examples only after all semantic fixtures pass.
- Run adversarial checks proving no const-specific evaluator, storage, copyability, ownership, capability, or type-position metadata is introduced.

## Out Of Scope

- Any change to `docs/SPEC.md`, accepted ADRs, milestone order, or semantic authority.
- Compile-time evaluation, constant folding, static storage, new binding categories, or new initializer/assignment rules.
- Changes to global, member, parameter, pattern, type-level, or future constant syntax.
- Refinement-aware local initializer behavior; that is M0019-015 and remains paused behind this task and its diagnostic-identifier ambiguity.

## Required Tests

Tests must be created before implementation.

- Positive: `const` declarations lex and parse; `const val: Int = 1;` and `var val = 1;` retain `val` as a binding identifier; resolution, type, flow, and ownership-facing fixtures classify `const` exactly as the former immutable local.
- Negative: `val` in the removed introducer position follows ordinary new-grammar diagnostics and recovery; `const` in formerly ordinary identifier positions follows ordinary reserved-keyword behavior; no legacy alias or immutable binding is produced.
- Diagnostic: assert existing ordinary token/parser diagnostics and source-level “immutable local binding” terminology; do not introduce a migration rule identifier or special legacy snapshot.
- Adversarial: scan metadata and fixtures for const-specific semantic categories, evaluator inputs, storage classes, constant values, legacy markers, or altered capability behavior.

## Test-First Gate

- Test files to create or update before implementation: existing lexer, parser, name-resolution, type-check, flow, and ownership-facing Rust fixture files identified by the main-task test work, plus the relevant task/docs validator.
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation: the current frontend still uses the superseded `val` spelling and does not satisfy ADR-0029’s cross-phase migration contract.
- main-task review approval required to modify/delete failing tests: `yes`

## Roadmap Hard Gate

This task is paused until the complete ADR-0029 authority bundle is confirmed authoritative. Execute the gates in order: main-task test work writes and records failing migration tests; main-task implementation performs one coherent frontend migration; main-task test work and specialty reviewers verify semantic fixtures and absence of const-specific semantics; examples and implemented-state documentation migrate only after fixtures pass. main-task task planning and main-task roadmap planning must revalidate this task before handoff to implementation.

## Implementation Plan

After the tests-first gate, perform one narrowly scoped frontend migration covering lexical reservation, parser declaration dispatch/recovery boundaries, syntax metadata, and source-independent immutable binding classification. Update only implementation-facing terminology and downstream fixtures required by the accepted mapping, without adding compatibility state or semantic meaning.

## Diagnostics, Build, And Reviews

- main-task diagnostics check: verify ordinary recovery, spans, wording, and absence of a legacy-`val` diagnostic.
- main-task specification check: verify every phase against ADR-0029 and the cross-phase invariant.
- main-task adversarial check: verify no semantic conflation or forged metadata path.
- main-task review and main-task simplicity check: verify scope and lack of compatibility abstractions.
- main-task build check: verify formatting, lint, workspace tests, and relevant frontend gates.

## Acceptance Criteria

- [x] Tests and expected failures are recorded before implementation.
- [x] `const` is the sole immutable-local introducer and `val` is ordinary outside that position.
- [x] Existing recovery is preserved with no special legacy diagnostic or alias.
- [x] All semantic fixtures preserve the existing immutable-local category.
- [x] No const-specific semantic metadata or compile-time meaning exists.
- [x] Examples/documentation are updated only after semantic fixtures pass.
- [x] Required specialty reviews and CI pass.

## Execution Commands

- Generate tests: `cargo test -p newlang --test lexer --test parser --test name_resolution --test type_check`
- Verify tests fail: `cargo test -p newlang --test lexer --test parser --test name_resolution --test type_check`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `docs/scripts/adversarial-check.sh docs/tasks/M0019-014-immutable-local-const-migration.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0019-014-immutable-local-const-migration.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets`

## Files Expected To Change

- Test files: existing lexer/parser/name-resolution/type-check/flow/ownership fixture files and their task validator, as identified by main-task test work.
- Implementation files: lexer, parser, syntax metadata, and source-independent local-binding classification files only.
- Documentation/examples: implemented-state terminology and examples, only after semantic fixtures pass.

## Forbidden Changes

- Do not edit `docs/SPEC.md`, `docs/adr/`, milestones, or build files.
- Do not implement initializer refinement, compile-time constants, aliases, special migration diagnostics, or new semantics.
- Do not weaken or delete failing tests without main-task review approval.

## Ambiguities And Dependencies

- Depends on accepted ADR-0029 commit `583af9e` and its complete authority bundle.
- M0019-015 remains paused until this migration and the diagnostic-identifier ambiguity recorded there are resolved.

## Execution Log

```text
2026-07-10 main_task=Task-Decomposer/Roadmap-Planner phase=revalidation result=pass notes=ADR-0029 authority gate confirmed satisfied; M0019-014 revalidated and entered the tests-first workflow.
2026-07-10 main_task=main-task test work phase=test-first result=pass notes=Updated lexer/parser/name-resolution/type-check and flow-facing tests plus lexer/parser fixtures from immutable-local introducer val to const; preserved val only as an ordinary identifier, including const val, var val, declaration-body identifier, and removed-introducer negative cases. Tests require TokenKind::KwConst and source-independent LocalBindingKind::Immutable, assert ordinary UnexpectedTokenInStatement recovery with no val alias/binding, and add docs/tests/m0019-immutable-local-const-migration.sh to scan the cross-phase contract and forbid stale spelling-specific semantic metadata or legacy diagnostics.
2026-07-10 main_task=main-task test work phase=pre-implementation-validation result=expected-fail command="cargo test -p newlang --test lexer --test parser --test name_resolution --test type_check" evidence="33 E0599 errors: TokenKind::KwConst is absent (lexer.rs:56, :89, :93) and LocalBindingKind::Immutable is absent (parser.rs:404 and semantic tests); no assertion or fixture parse failure occurred."
2026-07-10 main_task=main-task test work phase=pre-implementation-validation result=expected-fail command="sh docs/tests/m0019-immutable-local-const-migration.sh" evidence="missing expected pattern in crates/newlang/src/lexer.rs: KwConst; confirms lexer/parser migration has not started."
2026-07-10 main_task=main-task test work phase=pre-implementation-validation result=pass command="cargo fmt --all --check && git diff --check" evidence="test sources are formatted and whitespace-clean."
2026-07-10 main_task=main-task implementation phase=ordinary-tests result=pass command="cargo test -p newlang --test lexer --test parser --test name_resolution --test type_check && cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets && git diff --check" evidence="focused suites: 163 passed; workspace all targets: 200 passed; formatting, Clippy, and whitespace checks passed."
2026-07-10 main_task=main-task review/main-task test work phase=post-implementation-verification result=block commands="cargo test -p newlang --test lexer --test parser --test name_resolution --test type_check; sh docs/tests/m0019-immutable-local-const-migration.sh; sh docs/tests/m0007-lexer-fixtures.sh; sh docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh; sh docs/tests/m0019-null-test-eligibility.sh; cargo fmt --all --check; git diff --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets" evidence="focused suites: 163 passed; M0019 migration, M0007, and M0013 validators passed; docs/tests/m0019-null-test-eligibility.sh failed because it still requires removed LocalBindingKind::Val; formatting, diff check, Clippy, and workspace all-targets (200 passed) passed. Blocked because tests/fixtures/parser/statements/positive.fixture.toml is invalid TOML (duplicate expected_forms/expected_diagnostics at the new const/var val case), so the M0013 grep validator passed without validating the fixture. No compiler or test files edited by verifier."
2026-07-10 main_task=main-task review/main-task test work phase=final-independent-verification result=approve commands="cargo test -p newlang --test lexer --test parser --test name_resolution --test type_check; cargo test --workspace --all-targets; strict TOML parse of 4 changed fixtures; sh docs/tests/m0007-lexer-fixtures.sh; sh docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh; sh docs/tests/m0019-flow-output-data-model.sh; sh docs/tests/m0019-parser-flow-metadata.sh; sh docs/tests/m0019-null-test-recognition.sh; sh docs/tests/m0019-null-test-eligibility.sh; sh docs/tests/m0019-branch-refinement-records.sh; sh docs/tests/m0019-local-binding-resolution-identity.sh; sh docs/tests/m0019-refined-expression-type-records.sh; sh docs/tests/m0019-refinement-aware-assignment-checking.sh; sh docs/tests/m0019-immutable-local-const-migration.sh; cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; git diff --check" evidence="focused suites: 163 passed; workspace all targets: 200 passed; 4 changed TOML fixtures strict-parsed; 11 relevant validators passed; formatting, Clippy, and whitespace checks passed. Diff inspection found no weakened tests: existing semantic cases remain, const/val positive and negative boundaries are added, ordinary diagnostic primary spans are asserted, fixtures are complete, and validators require the source-independent Immutable category without production-only helper dependencies. No compiler, test, validator, report, example, SPEC/ADR, or M0019-015 file was edited by this verifier."
2026-07-10 main_task=main-task test work phase=final-ordinary-tests result=pass commands="cargo test -p newlang --test lexer --test parser --test name_resolution --test type_check; cargo test --workspace --all-targets" evidence="focused suites: 163 passed (lexer 7, parser 35, name_resolution 57, type_check 64); workspace all targets: 207 passed (lib 5, ast 7, lexer 7, module 12, name_resolution 57, parser 35, symbol 8, type_check 64, types 12), 0 failed, 0 ignored" next="main-task review final closure"
2026-07-10 main_task=main-task test work phase=examples-gate result=pass commands="strict TOML parse of 4 changed fixtures; sh docs/tests/m0007-lexer-fixtures.sh; sh docs/tests/m0007-lexer-implementation.sh; sh docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh; sh docs/tests/m0013-expression-statement-pattern-parser-implementation.sh; sh docs/tests/m0016-name-resolution-data-model.sh; sh docs/tests/m0018-type-checking-core-complete.sh; sh docs/tests/m0019-flow-output-data-model.sh; sh docs/tests/m0019-parser-flow-metadata.sh; sh docs/tests/m0019-null-test-recognition.sh; sh docs/tests/m0019-null-test-eligibility.sh; sh docs/tests/m0019-branch-refinement-records.sh; sh docs/tests/m0019-local-binding-resolution-identity.sh; sh docs/tests/m0019-refined-expression-type-records.sh; sh docs/tests/m0019-refinement-aware-assignment-checking.sh; sh docs/tests/m0019-immutable-local-const-migration.sh; cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; git diff --check" evidence="15 task-relevant validators passed; 4 changed TOML fixtures strict-parsed; examples gate requires local const declarations in type_checked.nl, accepted_nullability_flow.nl, and parsed_surface.nl, permits member public val size: Int, and rejects stale indented local val lines; formatting, Clippy, and whitespace checks passed; review and CI status unchanged" next="main-task review final closure"
2026-07-10 main_task=Build-Engineer phase=final-ci result=pass evidence="207 tests and 15 validators passed; migration validator, cargo fmt --all --check, cargo clippy --workspace --all-targets -- -D warnings, and git diff --check succeeded" next="M0019-015 remains blocked on its diagnostic-identifier ambiguity"
```

## Handoff

- Next main task: `M0019-015`
- Reason: this migration is complete; M0019-015 remains blocked until its diagnostic-identifier ambiguity is resolved by accepted authority.
- Required Context: M0019-015, `docs/SPEC.md`, ADR-0027, ADR-0028, ADR-0029, and this completed migration task.
