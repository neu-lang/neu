# Spec Compliance Review: M0019-014

Role: main-task specification check

Target: Implementation and full current working-tree diff for
`docs/tasks/M0019-014-immutable-local-const-migration.md`.

## Findings First

1. **Block — stale lexer expectation contradicts the accepted lexical
   replacement.**
   `tests/fixtures/lexer/comments.fixture.toml:17-20` now supplies
   `const /* open` but still expects `KW_VAL`. ADR-0029 and SPEC ADR-0021
   require `const` to be reserved and `val` to lex as an identifier. The
   expected token must be `KW_CONST`. The current M0007 fixture validator
   passes because it does not inspect this expectation, so that passing result
   is not compliance evidence.

2. **Block — the added positive parser fixture is invalid and no longer
   expresses the required cases.**
   `tests/fixtures/parser/statements/positive.fixture.toml:12-22` starts the
   `block_with_trailing_expression` table, then starts the new
   `const_and_var_accept_val_as_binding_name` table before completing the
   former. The latter consequently contains duplicate `expected_forms` and
   `expected_diagnostics` keys (lines 19-22). A strict TOML parse fails with
   `Cannot overwrite a value` at line 21. This invalidates the fixture and
   loses the required expectations for the block case. Restore each case as a
   separate complete TOML table; retain the spec-backed `const val` and
   `var val` coverage.

3. **Block — a downstream M0019 test validator still asserts an
   unsupported spelling-specific semantic category.**
   `docs/tests/m0019-null-test-eligibility.sh:37` requires
   `LocalBindingKind::Val`, and therefore currently fails. ADR-0029's
   Cross-Phase Invariant permits only the existing source-independent
   immutable-local category downstream of a valid `const` declaration; it
   expressly forbids a spelling-specific semantic record. Update the validator
   to assert `LocalBindingKind::Immutable` (and retain its immutable-versus-var
   eligibility check). Do not restore `Val` merely to satisfy this validator.

4. **Revise — the diagnostics tests do not verify the required primary spans.**
   `crates/newlang/tests/parser.rs:438-463` verifies diagnostic kinds, normal
   recovery, and no `val` binding, but not the primary span. ADR-0029 requires
   ordinary source-level diagnostics for the actual sequence: the removed
   introducer case must identify `val`, and the newly reserved identifier case
   must identify `const`. The implementation appears to use ordinary parser
   spans, but that behavior is a diagnostic obligation under ADR-0015 and
   ADR-0029 and is presently unprotected. Add span-text or byte-range
   assertions without introducing a migration-specific diagnostic.

## Inputs Read

- `main task rules` and `main task rules`
- `docs/SPEC.md` (ADR-0021, ADR-0024, ADR-0026, ADR-0028, and ADR-0029
  summaries)
- Accepted ADRs: `ADR-0015`, `ADR-0019`, `ADR-0021`, `ADR-0024`, `ADR-0026`,
  `ADR-0028`, and `ADR-0029`
- `docs/syntax/grammar-authority-ledger.md`
- Task and recorded test-first evidence:
  `docs/tasks/M0019-014-immutable-local-const-migration.md`
- Full current diff and untracked concurrent task/review artifacts; concurrent
  changes were read but not modified
- Changed implementation, tests, fixtures, and validators, including
  `docs/tests/m0019-immutable-local-const-migration.sh` and the relevant
  pre-existing M0019 eligibility validator

## Compliance Matrix

| Requirement | Evidence | Classification |
| --- | --- | --- |
| Reserve `const`; `val` lexes as an identifier | `lexer.rs` replaces `KwVal` with `KwConst`; lexer tests and keyword fixture cover `val` as `Identifier`. | Compliant |
| Only `const` introduces immutable locals | Parser dispatch accepts `KwConst` and `KwVar`; removed `val` introducer uses ordinary statement recovery and produces no binding. | Compliant |
| `val` remains usable as an identifier | Parser test and positive fixture add `const val` and `var val`. | Compliant, but fixture must be repaired (Finding 2). |
| Preserve one source-independent immutable category | `LocalBindingKind::Val` is renamed to `Immutable`; parser, resolution, and flow eligibility consume that category. | Compliant; stale validator blocks completion (Finding 3). |
| No compile-time or compatibility semantics | No const evaluator/value/storage/capability/legacy-alias mechanism appears in the changed implementation; the migration validator also guards selected prohibited markers. | Compliant on inspected implementation paths. |
| Ordinary diagnostics and recovery | Removed `val` receives `UnexpectedTokenInStatement`, with no alias/binding; reserved `const` receives an ordinary parser diagnostic. | Revise: primary-span coverage is missing (Finding 4). |
| Fixture expectations remain spec-backed and usable | Unterminated-comment expected token is stale; positive parser fixture is invalid TOML. | Non-compliant (Findings 1-2). |

## Non-Compliance Findings

- Findings 1 and 2 are direct conflicts with the required lexical and grammar
  contract and make the changed fixture set unreliable.

## Unsupported Expectations

- `docs/tests/m0019-null-test-eligibility.sh:37` encodes the superseded
  spelling-specific `LocalBindingKind::Val` category. ADR-0029 authorizes the
  existing immutable category, not a legacy spelling category.

## Ambiguities

None. ADR-0029 explicitly resolves the lexical spelling, parser recovery,
semantic-category preservation, and absence of compile-time semantics.

## Validation Observed

- Test-first evidence is recorded in the task: pre-implementation focused
  tests failed because `KwConst` and `Immutable` did not exist.
- Passed: focused Rust suites (163 tests), `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets` (200 tests), the new M0019 validator,
  M0007 fixture validator, M0013 fixture validator, and `git diff --check`.
- Failed: strict TOML parsing of the changed positive parser fixture; and
  `sh docs/tests/m0019-null-test-eligibility.sh`, which still requires
  `LocalBindingKind::Val`.

Decision: block

Required handoff: main-task test work and main-task implementation must repair the spec-backed
fixtures, replace the obsolete validator expectation without restoring legacy
semantics, and add ordinary diagnostic-span coverage. Return to Spec Compliance
Auditor after focused, fixture, M0019 eligibility, and workspace validation
passes.

## Closure Re-review

Date: `2026-07-10`

### Prior Finding Disposition

1. **Resolved.** `tests/fixtures/lexer/comments.fixture.toml:19` now expects
   `KW_CONST` for source `const /* open`.
2. **Resolved.** `tests/fixtures/parser/statements/positive.fixture.toml:12-22`
   now contains two complete, separate TOML case tables. Strict TOML parsing of
   all changed fixtures passes.
3. **Resolved.** `docs/tests/m0019-null-test-eligibility.sh:37` now requires
   `LocalBindingKind::Immutable`, and the validator passes.
4. **Partially resolved, but the reserved-`const` span change introduces a new
   blocking non-compliance.** The removed-introducer test now correctly proves
   that `unexpected_token_in_statement` spans `val`, as required by ADR-0024's
   unexpected-token primary-span rule. However, the test for `fun const();`
   now requires `missing_declaration_name` to span `const`, and
   `crates/newlang/src/parser.rs:345-353` was broadened to produce that span for
   every missing function name.

### New Blocking Finding

**Block — `missing_declaration_name` violates accepted ADR-0022's primary-span
contract.** Accepted ADR-0022 § Declaration Diagnostics specifies the primary
span of `missing_declaration_name` as the declaration keyword. For
`fun const();`, that span is `fun`, not the reserved token `const`. ADR-0029
requires an ordinary parser diagnostic for `const` in a formerly valid
identifier position, but does not supersede ADR-0022's diagnostic category or
primary-span obligation. The changed implementation also alters all malformed
function-name cases from the declaration-keyword span to the current-token span,
which exceeds this spelling migration and conflicts with the existing
`missing_function_name` fixture in
`tests/fixtures/parser/declarations/diagnostics.fixture.toml:27-32`.

Required fix: preserve the ordinary `missing_declaration_name` diagnostic and
recovery, restore its ADR-0022 declaration-keyword primary span, and make the
`fun const();` regression test assert `fun`. Do not introduce a special
reserved-`const` or migration diagnostic.

### Re-review Compliance Matrix

| Requirement | Re-review classification |
| --- | --- |
| `const` reserved and `val` ordinary identifier | Compliant |
| `const` sole immutable-local introducer | Compliant |
| `val` accepted in ordinary binding-name positions | Compliant |
| Source-independent immutable semantic category | Compliant |
| No compile-time, storage, ownership, capability, or compatibility semantics | Compliant |
| Removed `val` receives ordinary recovery with no binding or alias | Compliant |
| Newly reserved `const` receives an ordinary diagnostic consistent with accepted diagnostic obligations | Non-compliant: category and recovery are ordinary, but the primary span conflicts with ADR-0022 |

### Re-review Validation

- Passed: focused lexer/parser/name-resolution/type-check suites (163 tests).
- Passed: M0019 immutable-local migration validator, M0019 null-test eligibility
  validator, M0007 fixture validator, and M0013 fixture validator.
- Passed: strict TOML parsing of all changed fixtures.
- Passed: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets` (200 tests), and `git diff --check`.
- Compliance failure remains despite green tests because the new parser test
  encodes a primary span contradicted by accepted ADR-0022.

Ambiguities: none. ADR-0022 controls the diagnostic span, and ADR-0029 does not
supersede it.

Final decision: block

Final handoff: main-task test work and main-task implementation for the single diagnostic-span
correction, then return to main-task specification check for closure.

## Final Closure

Role: main-task specification check

Inputs read: task execution log and Authority Extract; SPEC ADR-0021, ADR-0022,
ADR-0024, ADR-0026, and ADR-0029 summaries; accepted ADR-0022 declaration
diagnostics; accepted ADR-0029 cross-phase and diagnostic sections; final
parser/test diff, fixtures, validators, and examples.

Finding disposition: **resolved.** `MissingDeclarationName` now spans the full
`fun` token in implementation and regression coverage, satisfying ADR-0022.
All earlier stale fixture and validator findings remain resolved. The final
diff preserves one immutable-local category, ordinary `val` identifier use,
ordinary removed-introducer recovery, and introduces no compile-time or
compatibility semantics.

Validation: 207 workspace tests, 15 task-relevant validators, strict parsing of
four changed TOML fixtures, formatting, Clippy, and `git diff --check` pass.

Files changed by this re-review: this report only.

Open questions: none. Blockers: none. Residual risk: none identified in the
bounded M0019-014 behavior.

Final decision: **APPROVE**

Handoff: main-task review for final task closure.
