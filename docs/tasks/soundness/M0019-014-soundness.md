# Soundness Report: M0019-014

Role: main-task adversarial check

Target: `M0019-014` immutable-local spelling migration from `val` to `const`.

## Metadata

- Task ID: `M0019-014`
- Milestone: `M0019`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- `main task rules`
- `main task rules`
- `main task rules`
- `docs/SPEC.md`
- `docs/adr/ADR-0019-compile-time-evaluation-and-metaprogramming.md`
- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- `docs/adr/ADR-0026-name-resolution-policy.md`
- `docs/adr/ADR-0027-type-checking-core.md`
- `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- `docs/adr/ADR-0029-immutable-local-const-keyword.md`
- `docs/syntax/grammar-authority-ledger.md`
- `docs/milestones/M0019-nullability-and-flow-typing.md`
- `docs/tasks/M0019-014-immutable-local-const-migration.md`, including its
  recorded tests-first expected-failure evidence
- The full current working-tree diff, changed fixtures and validators, and
  concurrent untracked task/review artifacts; concurrent files were not edited
- Relevant lexer, parser, name-resolution, type-check, and flow implementation
  and tests

The task records that the main-task test work changed tests first and observed 33
missing-variant compilation errors for `KwConst` and `Immutable`, followed by a
failure of the new migration validator before implementation. That is specific,
plausible tests-first evidence. Because tests and implementation remain in one
uncommitted working-tree diff, authoring order cannot be reconstructed
independently beyond the task log.

## Invariants Tested

- Source `const` in local-declaration position must map only to the existing,
  source-independent immutable-local category.
- Source `var` must continue to map only to the mutable-local category.
- `val` must lex and parse through ordinary identifier paths except that the
  removed declaration-introducer sequence must not create a declaration,
  binding, alias, or downstream fact.
- Parser recovery must resume at the ordinary statement boundary and its
  diagnostic must identify the actionable source token.
- Binding identity, shadowing, declaration order, lookup, and refinement region
  checks must remain independent of source spelling.
- Smart-cast eligibility must accept `Immutable` and reject the `Var` control.
- No spelling-specific evaluator, value table, storage class, copy/drop rule,
  ownership or borrow flag, lifetime rule, send/share capability, type/layout
  capability, or compatibility marker may be introduced.
- Fixture and validation phases must agree with the same mapping; a passing
  focused Rust suite cannot conceal stale shell or fixture contracts.
- Async suspension, cancellation, unsafe, FFI, ownership, borrowing, lowering,
  and thread-capability behavior must not acquire a new `const` path. These
  systems remain deferred or outside the changed implementation, so the attack
  here was absence-of-new-path inspection rather than a runtime safety test.

## Attacks Attempted

```text
Attack: Lex `const val var` and attempt both `const val: Int = 1;` and
`var val = 2;`.
Expected result: const is reserved, val is an Identifier, and the two bindings
are respectively Immutable and Var with the source name "val".
Actual result: The focused lexer and parser tests pass with exactly that token
and metadata mapping.
Source of truth: SPEC lines 125-127 and 259-264; ADR-0029 lines 65-85.
Outcome: pass
```

```text
Attack: Parse `val removed = 1; var retained = 2;` to force the removed
introducer through ordinary expression/statement recovery.
Expected result: One ordinary unexpected-token diagnostic selecting the
actionable `val`, no declaration or binding for `removed`, and successful
recovery to mutable binding `retained`.
Actual result: Recovery and absence of a legacy binding pass, but the diagnostic
primary span selects `removed`, not `val`.
Source of truth: ADR-0029 Diagnostics And Source Compatibility; ADR-0015
actionable primary-span obligation.
Outcome: fail
```

```text
Attack: Put newly reserved `const` in a former identifier position with
`fun const();`.
Expected result: An ordinary missing-declaration-name diagnostic whose primary
span selects `const`, with no migration-specific diagnostic or fix-it.
Actual result: The ordinary diagnostic kind is correct, but its primary span is
empty and does not select `const`.
Source of truth: SPEC lines 266-269; ADR-0029 Diagnostics And Source
Compatibility.
Outcome: fail
```

```text
Attack: Follow a valid immutable declaration through parser metadata, local
binding indexing, exact binding resolution, nested shadowing, and flow-region
recording.
Expected result: Every downstream phase consumes LocalBindingKind::Immutable;
textual aliases and shadowing must not transfer refinements to another binding.
Actual result: Migrated resolution and flow tests pass. The nested-shadowing
test refines only uses resolving to the original binding and excludes the
shadowing declaration and post-region use.
Source of truth: ADR-0029 Cross-Phase Invariant; ADR-0026 binding identity;
ADR-0028 shadowing and region rules.
Outcome: pass
```

```text
Attack: Present the same nullable null-test shape once with Immutable metadata
and once with Var metadata.
Expected result: Immutable is eligible; Var produces
MutableLocalRefinementDeferred and no eligible refinement.
Actual result: Both focused tests pass, preserving the mutable control case.
Source of truth: ADR-0028 Smart-Cast Eligibility; ADR-0029 lines 123-125.
Outcome: pass
```

```text
Attack: Search implementation and semantic tests for KwVal,
LocalBindingKind::Val, const-specific evaluator/value/storage/capability
records, static materialization, copy/drop flags, and legacy aliases.
Expected result: None may remain or be introduced; syntax KwConst must lower
directly to the source-independent Immutable category.
Actual result: No forbidden semantic or compatibility mechanism was found.
The only new semantic enum member is Immutable, and type checking matches it
against the existing Var control.
Source of truth: SPEC lines 252-269; ADR-0029 Cross-Phase Invariant.
Outcome: pass
```

```text
Attack: Use effectful/call-shaped initializers such as
`const answer = compute();` and inspect type, storage, and evaluator output.
Expected result: Parsing remains ordinary local-initializer parsing; no
compile-time evaluation, static storage, constant table, or purity gate appears.
Actual result: Parser fixtures and tests accept the ordinary initializer shape;
the implementation diff introduces no evaluator or storage output. Call typing
remains independently deferred by ADR-0027.
Source of truth: ADR-0029 lines 87-101 and 109-128.
Outcome: pass
```

```text
Attack: Run older downstream validators after the semantic enum rename.
Expected result: Every phase validator must require the source-independent
Immutable category and continue checking the Var control.
Actual result: `docs/tests/m0016-name-resolution-data-model.sh` still requires
`Val`, and `docs/tests/m0019-null-test-eligibility.sh` still requires
`LocalBindingKind::Val`; both fail.
Source of truth: ADR-0029 Cross-Phase Invariant and Roadmap Hard Gate.
Outcome: fail
```

```text
Attack: Parse all changed TOML fixtures strictly and compare token expectations
with lexer behavior.
Expected result: Fixtures must be valid TOML and must encode KW_CONST for source
`const`.
Actual result: the positive statement fixture is invalid TOML because it
duplicates expected_forms and expected_diagnostics in one table, and the
unterminated-comment fixture still expects KW_VAL for `const /* open`.
Source of truth: ADR-0029 lexical replacement and the task's coherent fixture
migration requirement.
Outcome: fail
```

## Findings

### 1. Blocker: both required ordinary diagnostics have misleading primary spans

Invariant: Diagnostics must identify the actionable source token while using
ordinary, non-migration-specific parser categories.

Evidence:

- The new assertion in `crates/compiler/tests/parser.rs` for
  `val removed = 1;` receives span text `removed` instead of `val`.
- The new assertion for `fun const();` receives an empty span instead of
  `const`.
- The diagnostic kinds and recovery behavior are otherwise ordinary and
  correct; no alias or compatibility binding is created.

Why this blocks: ADR-0015 makes diagnostic quality semantic, and ADR-0029
requires ordinary diagnostics for the actual token sequence. Pointing at a
valid following identifier or an empty location misstates which token made the
source invalid.

Required fix: main-task implementation must correct only the existing ordinary parser
primary-span selection. Do not add a legacy-`val` diagnostic, rule identifier,
recognizer, alias, fix-it, or recovery-as-success path. Retain the adversarial
span assertions and return to main-task diagnostics check and main-task adversarial check.

### 2. High: two pre-existing phase validators retain the superseded category

Invariant: The migration must be coherent across parser, resolution, type, and
flow validation, and no downstream contract may require spelling-specific
`Val` metadata.

Evidence:

- `docs/tests/m0016-name-resolution-data-model.sh:86` requires `Val` and fails.
- `docs/tests/m0019-null-test-eligibility.sh:37` requires
  `LocalBindingKind::Val` and fails.
- The new `docs/tests/m0019-immutable-local-const-migration.sh` passes, so its
  selected textual scan does not detect these older broken gates.

Required fix: main-task test work must migrate those validator expectations to the
source-independent `Immutable` category while retaining their mutable `Var`
controls. Do not restore `Val` to satisfy the scripts.

### 3. High: the changed positive parser fixture is invalid TOML

Invariant: Positive fixture cases must remain independently parseable and must
not allow grep-only validators to report false success.

Evidence:

- `tests/fixtures/parser/statements/positive.fixture.toml:13-22` starts the
  `block_with_trailing_expression` table, opens another `[[cases]]`, then writes
  duplicate `expected_forms` and `expected_diagnostics` keys into the second
  table.
- Python `tomllib` rejects the file with `Cannot overwrite a value (at line 21,
  column 74)`.
- The existing M0013 shell validator still passes because it only searches for
  selected text and never parses the fixture.

Required fix: main-task test work must restore one complete expectation set per case
and validate the fixture with a strict TOML parser.

### 4. High: a lexer fixture still claims `const` produces `KW_VAL`

Invariant: Lexical fixtures must agree that `const` is reserved and `val` is an
ordinary identifier.

Evidence: `tests/fixtures/lexer/comments.fixture.toml:17-20` uses source
`const /* open` but expects `KW_VAL`. The current M0007 fixture validator passes
because it does not inspect that expected token.

Required fix: main-task test work must change the expected token to `KW_CONST` and
strengthen the fixture validation enough to catch this contradiction.

## Negative Tests Added Or Proposed

- Added narrowly scoped assertions to the existing parser migration tests in
  `crates/compiler/tests/parser.rs`:
  - removed-introducer recovery must span `val`
  - newly reserved identifier-position rejection must span `const`
- Both assertions currently fail and are intentionally retained as regression
  tests for Finding 1.
- Proposed, but not added here: strict TOML parsing in the fixture validation
  path so malformed or duplicate fixture fields cannot pass grep-only gates.

## Validation

- Pass: `sh docs/tests/m0019-immutable-local-const-migration.sh`
- Pass: focused existing immutable/mutable eligibility and nested-shadowing
  tests
- Pass: `cargo fmt --all --check`
- Pass: `git diff --check`
- Fail: `sh docs/tests/m0016-name-resolution-data-model.sh`
- Fail: `sh docs/tests/m0019-null-test-eligibility.sh`
- Fail: strict TOML parse of
  `tests/fixtures/parser/statements/positive.fixture.toml`
- Fail as intended: the two new parser primary-span assertions
- The task records the previously run ordinary focused suites, formatting,
  Clippy, workspace all-target tests, and whitespace checks as passing before
  this adversarial test addition.

## Ambiguities

None. ADR-0029 explicitly resolves the spelling, source compatibility,
semantic-category mapping, diagnostic boundary, and absence of compile-time or
compatibility semantics. Ownership, borrowing, async suspension, cancellation,
unsafe/FFI, storage, lowering, and thread-capability details remain unchanged or
deferred; this migration supplies no authority to implement them.

## Decision And Handoff

Decision: fail. The core lexer/parser/semantic mapping contains no discovered
const-specific safety semantics, but the change is not ready because actionable
diagnostic spans fail, two downstream phase validators are partially migrated,
and two changed fixtures are contradictory or invalid.

Handoff:

1. main-task test work repairs the validators and fixtures without restoring any
   spelling-specific semantic category.
2. main-task implementation fixes only ordinary parser span selection for the two failing
   adversarial assertions.
3. main-task diagnostics check, main-task specification check, main-task review, main-task build check, and
   main-task adversarial check re-run their gates.
4. Examples may migrate only after these semantic fixtures and specialty gates
   pass, as required by ADR-0029; current example deferral is not treated as a
   finding in this review.

Files changed by this execution:

- `crates/compiler/tests/parser.rs`
- `docs/tasks/soundness/M0019-014-soundness.md`

## Re-review Closure: 2026-07-10

This section supersedes the earlier validation status and handoff for the
current tree. The historical attack evidence above is retained to show what was
found and how closure was tested.

### Finding closure

1. **Closed — parser diagnostic primary spans.** The ordinary recovery path now
   reports the parsed `val` expression span, and missing function names use the
   current token span when one exists. Both retained adversarial assertions
   pass:
   - `removed_val_introducer_uses_ordinary_recovery_without_a_binding_alias`
   - `const_is_rejected_in_prior_identifier_positions_by_ordinary_parser_diagnostics`
2. **Partially closed; still blocking — stale phase validators.** The M0019
   eligibility validator now requires `LocalBindingKind::Immutable` and passes.
   However, `docs/tests/m0016-name-resolution-data-model.sh:86` still requires
   the superseded text `Val`. Running the validator fails with:
   `m0016-data-model: missing expected pattern in
   crates/compiler/src/name_resolution.rs: Val`. The validator was therefore not
   fixed and the prior coherent-migration finding remains open.
3. **Closed — positive statement fixture.** Every case again has one complete
   expectation set. Strict `tomllib` parsing succeeds for all four changed
   lexer/parser fixtures.
4. **Closed — unterminated-comment lexer fixture.** Source `const /* open` now
   expects `KW_CONST`.
5. **Closed — unused helper.** `LocalBindingKind::is_immutable` was removed; no
   implementation or test reference remains.

### Re-review validation

- Pass: both retained parser diagnostic-span tests
- Pass: strict TOML parsing of the four changed lexer/parser fixtures
- Pass: `cargo test --workspace --all-targets` — 207 tests across 9 suites
- Pass: `sh docs/tests/m0019-null-test-eligibility.sh`
- Pass: `sh docs/tests/m0019-immutable-local-const-migration.sh`
- Pass: `sh docs/tests/m0007-lexer-fixtures.sh`
- Pass: `sh docs/tests/m0013-expression-statement-pattern-parser-fixtures.sh`
- Pass: `cargo fmt --all --check`
- Pass: `git diff --check`
- Pass: renewed scan found no const-specific evaluator, storage, ownership,
  borrow, capability, layout, or compatibility mechanism
- **Fail:** `sh docs/tests/m0016-name-resolution-data-model.sh` because line 86
  still requires `Val`

### Final decision

**Fail.** The implementation behavior, adversarial span tests, semantic mapping,
fixtures, and all 207 workspace tests now pass. No compile-time or compatibility
semantics were found. Nevertheless, M0019-014's required coherent migration is
not complete while the pre-existing M0016 data-model validator still encodes
and requires the removed `Val` category and fails against
`LocalBindingKind::Immutable`.

Required final fix: main-task test work must change only that stale validator
expectation to the source-independent immutable category, retain the `Var`
control, and rerun the validator. Do not restore `Val` in compiler metadata.
After that validator passes, main-task adversarial check can change the final decision
to pass without reopening the already closed findings.

Files changed by this re-review:

- `docs/tasks/soundness/M0019-014-soundness.md`

## Final Closure: 2026-07-10

This is the authoritative final status and supersedes both earlier fail
decisions while retaining their attack history.

### Closure evidence

- The last open blocker is closed:
  `docs/tests/m0016-name-resolution-data-model.sh` now requires the exact
  source-independent enum member `Immutable` and passes while retaining its
  `Var` control.
- The main-task language review-confirmed ordinary function-name diagnostic span is
  implemented and protected by the passing `fun const();` assertion.
- Removed-introducer recovery selects `val`, creates no immutable alias or
  binding, and recovers to the following `var` declaration.
- The lexer and parser fixtures strict-parse as TOML and encode `KW_CONST`,
  ordinary identifier `val`, and complete independent case expectations.
- The examples gate passes: implemented local declarations use `const`; the
  remaining member `public val size: Int` is intentionally outside ADR-0029's
  local-declaration replacement.
- The source-independent `Immutable` category continues through resolution,
  typing, flow eligibility, exact binding identity, shadowing, and refinement
  records. The mutable `Var` control remains ineligible.
- Renewed scans find no `KwVal`, `LocalBindingKind::Val`, legacy alias,
  const-specific evaluator/value/storage metadata, ownership or borrow flag,
  send/share capability, type/layout authority, or unused `is_immutable`
  helper in implementation or semantic tests.

### Final validation

- Pass: `cargo test --workspace --all-targets` — 207 tests across 9 suites
- Pass: both retained adversarial parser span tests
- Pass: strict TOML parse of all 4 changed lexer/parser fixtures
- Pass: all 15 task-relevant validators, including M0007 lexer, M0013 parser,
  M0016 name-resolution data model, M0018 type-check core, all affected M0019
  flow gates, and the M0019-014 cross-phase migration gate
- Pass: `cargo fmt --all --check`
- Pass: `git diff --check`

### Final decision

**PASS.** All adversarial findings are closed. `const` is only the immutable
local introducer, `val` remains an ordinary identifier outside removed
introducer use, and every implemented semantic phase consumes the existing
source-independent immutable-local category. No compile-time, storage,
copyability, ownership, borrowing, capability, type-position, layout, or
compatibility semantics were introduced.

Residual risk is limited to ownership, borrowing, async suspension,
cancellation, unsafe/FFI, lowering, and thread-capability behavior that remains
explicitly deferred and unchanged by this spelling migration. No open question
or blocker remains for M0019-014.

Handoff: main-task review for final task closure.

Files changed by this final re-review:

- `docs/tasks/soundness/M0019-014-soundness.md`
