# ADR-0029 Chief Architect Decision

Role: Chief Architect

Inputs read:

- `AGENTS.md`
- `.codex/agents/chief-architect.toml`
- `.codex/agents/roadmap-planner.toml`
- `docs/SPEC.md`
- `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`
- `docs/adr/proposals/reviews/ADR-0029-language-lawyer-review.md`, including its revised-round verdict
- `docs/adr/proposals/reviews/ADR-0029-simplicity-review.md`, including its revised-round verdict
- `docs/adr/proposals/reviews/ADR-0029-adversarial-review.md`, including its revised-round verdict
- `docs/adr/proposals/reviews/ADR-0029-diagnostics-review.md`, including its revised-round verdict
- `docs/adr/ADR-0013-mutability-model.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0019-compile-time-evaluation-and-metaprogramming.md`
- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- `docs/adr/ADR-0026-name-resolution-policy.md`
- `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- `docs/syntax/grammar-authority-ledger.md`
- `docs/milestones/M0019-nullability-and-flow-typing.md`
- `docs/tasks/M0019-014-refinement-aware-local-initializers.md`

Decision:

**APPROVE for atomic acceptance.** The revised ADR-0029 may become accepted authority only through the exact atomic acceptance bundle below. This decision does not by itself make the proposal authoritative and does not authorize a partial ADR-only, SPEC-only, or ledger-only update.

The accepted rule is a hard spelling replacement: reserve `const`, stop reserving `val`, and use `const` as the sole immutable-local declaration introducer. A valid local `const` denotes the project's existing immutable-local binding category, follows the ordinary initializer rules already applicable to that category, and has no compile-time-constant, evaluator, storage, copyability, ownership, destruction, borrow, lifetime, send/share, type-position, or layout meaning merely because of its spelling.

Rationale:

- The owner clarification selects the spelling while expressly preserving this project's Kotlin-style `val` behavior. That is consistent with the immutable-binding model in `docs/SPEC.md` and ADR-0013 and does not import unaccepted Kotlin semantics.
- ADR-0019 treats bounded compile-time evaluation as a separate direction, while ADR-0024 defers compile-time-evaluation syntax. The proposal therefore correctly requires explicit future supersession before local `const` could acquire compile-time meaning (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:141-149`).
- ADR-0024 supplies the existing local-declaration initializer grammar; ADR-0026 supplies binding identity, scope, declaration order, shadowing, duplicate handling, and lookup; ADR-0028 predicates flow refinement on immutable local-binding identity. The proposal preserves those rules and changes only the relevant lexical spelling and spelling-dependent parser dispatch/recovery boundaries (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:151-193`, `375-388`).
- The revised Simplicity, Diagnostics, and Adversarial verdicts approve. Their former blockers are closed by removal of contextual legacy recovery, preservation of ordinary diagnostics, the one-category cross-phase invariant, and the categorical exclusion of const-specific semantic metadata.
- The Language Lawyer's revised-round `revise` verdict identified one remaining risk: an overbroad statement that could reject `val` even in a valid binding-name position. The final proposal corrects that issue exactly. It limits exclusion to the immutable-local declaration-introducer position, explicitly accepts `const val: Int = 1;` and `var val = 1;`, repeats the positional restriction in the cross-phase invariant, and preserves ordinary identifier treatment in the diagnostics and migration rules (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:98-113`, `228-250`, `273-288`). All other downstream references to removed `val` syntax are likewise qualified by declaration-introducer or binding-position context. The Language Lawyer's sole remaining blocker is therefore resolved; no semantic choice remains open.
- The design preserves memory safety and thread safety because every phase must consume the existing immutable-local category and may not derive ownership, borrowing, destruction, or concurrency capabilities from `const` (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:215-250`).
- One canonical spelling, ordinary parser recovery, and no alias or migration subsystem are the minimum complexity needed for the owner's selected hard replacement.

Exact atomic acceptance bundle:

1. Accepted ADR: create `docs/adr/ADR-0029-immutable-local-const-keyword.md` from the approved revised proposal, mark it `Status: Accepted`, remove the draft-only non-authority framing, and preserve the approved semantic boundaries, positional `val` correction, cross-phase invariant, diagnostics rules, atomicity rule, and Roadmap hard gate without semantic alteration.
2. SPEC target `ADR-0021: Lexical Grammar`: state that the fixed reserved keyword set reserves `const` and does not reserve `val`; `val` consequently lexes as an ordinary identifier under the accepted identifier rule.
3. SPEC target `ADR-0024: Expression Statement And Pattern Syntax`: in the first summary paragraph replace local `val`/`var` declaration statements with local `const`/`var` declaration statements, with ADR-0029 controlling the corresponding immutable-local statement starter and directly spelling-dependent parser dispatch/recovery.
4. SPEC target `ADR-0026: Name Resolution Policy`: in the paragraph beginning “Function declaration names,” replace the local `val` binding position with the local `const` binding position; retain all existing binding identity, scope, order, lookup, shadowing, duplicate, and ambiguity rules.
5. New SPEC target `ADR-0029: Immutable Local const Keyword`: record the hard lexical replacement, the mapping to the existing immutable-local category, ordinary initializer rules, the categorical absence of compile-time-constant meaning, ordinary identifier treatment of `val`, the source break caused by reserving `const`, and the requirement that future compile-time meaning explicitly supersede ADR-0029.
6. Grammar authority ledger:
   - update the `Token spellings` row to cite ADR-0021 and ADR-0029 and state that ADR-0029 replaces reserved `val` with reserved `const`;
   - update the `Statement grammar` row to cite ADR-0024 and ADR-0029 and state that `const` is the immutable-local introducer while `var` and the remaining statement grammar are unchanged;
   - keep `Compile-time evaluation syntax` classified `deferred` and state that local `const` is not compile-time-evaluation syntax and gains no such semantics from ADR-0029;
   - update the Parser Unblock List and Parser Block List references that assert ADR-0021 token-spelling or ADR-0024 statement-grammar authority so they cite ADR-0029 for the superseded portions;
   - leave unaffected expression, pattern, coroutine, unsafe, and other grammar classifications unchanged.

The six items above are one semantic-change transaction. If any item is absent or lands separately, ADR-0029 remains non-authoritative; the current `docs/SPEC.md`, ADR-0021, ADR-0024, and ADR-0026 continue to control.

Affected files:

- Changed by this decision: `docs/adr/proposals/reviews/ADR-0029-chief-architect-decision.md` only.
- Approved for a later atomic acceptance change: `docs/adr/ADR-0029-immutable-local-const-keyword.md`, `docs/SPEC.md`, and `docs/syntax/grammar-authority-ledger.md` only as specified in the atomic bundle.
- No proposal, accepted ADR, compiler, test, task, milestone, example, or other file is changed or authorized by this decision alone.

Required follow-up:

1. Language Designer prepares the atomic acceptance bundle exactly as approved; Spec Compliance Auditor verifies that the accepted ADR, all four SPEC targets, and ledger records agree before the bundle becomes authoritative.
2. Roadmap Planner keeps `M0019-014` paused and enforces these ordered gates from the proposal (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:337-359`):
   1. the complete atomic authority bundle becomes authoritative;
   2. Test Engineer performs the lexical, parser, binding-category, diagnostic, name-resolution, type-check, and flow tests-first migration and records the expected pre-implementation failures;
   3. Implementer performs one coherent frontend migration of lexing, parsing, syntax metadata, and local-binding classification;
   4. Test Engineer and required specialty reviewers verify semantic fixtures across name resolution, type checking, nullability, flow typing, and ownership-facing behavior, including proof that no const-specific semantics or metadata were introduced;
   5. examples and related documentation migrate only after the semantic fixtures pass.
3. After all five gates pass, Task Decomposer and Roadmap Planner revalidate `M0019-014` against the new authority before returning it to the Test Engineer/Implementer workflow.

Open questions:

- None. Initializer-free immutable-local assignment, definite initialization, evaluation order, effects, ownership transfer during initialization, and lowering remain explicitly deferred; this decision does not answer them.

Blocked work:

- Until the atomic acceptance bundle is complete, `val` remains the accepted immutable-local keyword, `const` remains an ordinary identifier, and all ADR-0029-driven tests, compiler migration, task rewrites, milestone rewrites, and example rewrites remain blocked.
- After atomic acceptance, implementation remains blocked until the tests-first gate is complete.
- `M0019-014` remains blocked until every ordered gate is complete and the task is revalidated. Existing task text, implementation, fixtures, or examples are not semantic precedent.

Handoff:

Language Designer for the exact atomic acceptance bundle, with Spec Compliance Auditor verification; Roadmap Planner owns enforcement of the downstream gate sequence.
