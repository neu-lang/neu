# ADR-0029 Language Lawyer Review

## Metadata

- Proposal: `ADR-0029`
- Reviewer: `Language Lawyer`
- Date: `2026-07-10`
- Initial verdict: `revise`

## Required Classification

Role: Language Lawyer

Question: Is draft ADR-0029 precise and complete enough to replace the project language's immutable-local `val` spelling with `const`, while preserving only the existing immutable-local semantics and conferring no compile-time-constant meaning?

Inputs read:

- `AGENTS.md`
- `.codex/agents/language-lawyer.toml`
- `docs/SPEC.md`
- `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`
- `docs/adr/ADR-0005-copy-move-and-value-categories.md`
- `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
- `docs/adr/ADR-0013-mutability-model.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0019-compile-time-evaluation-and-metaprogramming.md`
- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- `docs/adr/ADR-0026-name-resolution-policy.md`
- `docs/adr/ADR-0027-type-checking-core.md`
- `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- `docs/syntax/grammar-authority-ledger.md`

Classification: ambiguous

Controlling text:

- `docs/SPEC.md:5-6` makes the specification the record of accepted language-level decisions, and `AGENTS.md` places `docs/SPEC.md` above accepted ADRs in the authority hierarchy.
- `docs/SPEC.md:70-73` and ADR-0013 define immutable bindings and explicit mutable bindings without attaching compile-time evaluation to binding immutability.
- `docs/SPEC.md:103-107` and `docs/adr/ADR-0019-compile-time-evaluation-and-metaprogramming.md:24-32` accept bounded compile-time evaluation as a distinct direction but do not define its concrete syntax.
- `docs/adr/ADR-0021-lexical-grammar.md:56-70` classifies a spelling as a keyword only by exact membership in the accepted keyword set; `docs/adr/ADR-0021-lexical-grammar.md:73-108` currently reserves `val`, does not reserve `const`, and requires accepted authority for future keywords.
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md:77-94` defines local declaration syntax with `val` and `var`, permits an optional initializer syntactically, and leaves mutability, ownership, moves, and borrow effects to later semantic checks. Its `docs/adr/ADR-0024-expression-statement-pattern-syntax.md:241-262` defers compile-time-evaluation syntax.
- `docs/adr/ADR-0026-name-resolution-policy.md:57-100` defines local `val` and `var` binding positions, declaration order, scope, shadowing, and duplicate behavior.
- `docs/adr/ADR-0028-nullability-and-flow-typing.md:184-224` predicates refinement on immutable local-binding identity, not keyword spelling or compile-time evaluation.

Interpretation:

The narrow semantic direction is compatible with existing authority: a future accepted change may substitute `const` for `val` as the spelling of the same project-defined immutable-local binding category without thereby creating compile-time evaluation, static storage, implicit copyability, type-level availability, or any other constant-specific capability. That preservation is preservation of this project's accepted `val` semantics only. Kotlin is an ergonomic influence and cannot supply unspecified definite-assignment, evaluation, property, capture, or initialization rules.

The draft is nevertheless incomplete at acceptance boundaries. It does not fully account for the fact that `const` is currently an ordinary identifier, it does not prevent a higher-authority SPEC/ADR contradiction during acceptance, and its legacy diagnostic and runtime-expression wording leave behavior that implementations could decide inconsistently.

Non-authoritative assumptions:

- No assumption is made about Kotlin behavior beyond rules independently accepted by this project.
- No assumption is made about initializer evaluation order, execution count, effect semantics, constant folding, definite assignment of initializer-less immutable locals, or backend lowering.
- No implementation, fixture, example, task, or milestone behavior was treated as semantic authority.

Required escalation:

- Language Designer must revise the proposal to close the findings below.
- Diagnostics Engineer must approve the exact legacy-`val` recognition, diagnostic category, recovery artifact, and interaction with valid identifier uses.
- Chief Architect must require the accepted ADR and conforming `docs/SPEC.md` revision to become authoritative together, after the semantic reviews are complete.

## Actionable Findings

### 1. High: the compatibility analysis omits existing programs that use `const` as an identifier

ADR-0021 currently does not reserve `const`. Under its exact-match rule, `const` is therefore an ordinary identifier in every identifier position accepted by later grammar. The proposal correctly observes that removing `val` from the keyword set makes `val` an identifier (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:92-96`), but its migration discussion covers only source that declares immutable locals with `val` (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:189-209`). Reserving `const` creates a second and potentially broader source break: existing declaration names, local names, references, qualified-name components, type names, package components, and other accepted identifier positions spelled `const` cease to lex as identifiers.

Required revision:

- State explicitly that the hard replacement both unreserves `val` and reserves the formerly available identifier spelling `const`.
- Add existing `const` identifier uses to compatibility impact, downstream migration work, examples, and diagnostic review.
- Do not promise an automatic rename fix unless a safe replacement can be established under accepted name-resolution rules.
- Qualify claims such as “changes only the immutable-local keyword spelling” and “adds no runtime behavior” so they do not conceal the broader lexical source-acceptance change.

### 2. High: the supersession plan does not make the higher-authority SPEC update atomic with ADR acceptance

The proposal lists `docs/SPEC.md` as a dependency and a downstream update, but its supersession targets name only ADR-0021, ADR-0024, and ADR-0026 (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:254-278`). The current specification still accepts local `val` syntax at `docs/SPEC.md:149-154` and names local `val` binding positions at `docs/SPEC.md:192-198`. Under `AGENTS.md`, that text outranks an accepted ADR. The workflow at proposal lines 292-294 could therefore be read as allowing ADR acceptance before the corresponding SPEC revision, producing contradictory authority in which the SPEC continues to control.

Required revision:

- Identify `docs/SPEC.md:149-154` and `docs/SPEC.md:192-198` as exact synchronized replacement targets.
- State that ADR-0029 must not become accepted source of truth before or separately from the conforming SPEC revision; both must become authoritative in one approved semantic-change step.
- State the controlling behavior if synchronization fails: existing SPEC and accepted ADR-0021/0024/0026 rules remain authoritative and implementation stays blocked.

### 3. High: the legacy-`val` diagnostic recognizer and recovery semantics are not defined precisely enough

After the lexical change, `val` is an identifier and can participate in name expressions and assignment statements under ADR-0024. The proposal requires a migration diagnostic only in “the syntactic position and shape of a local declaration” and recovery “as though an immutable local declaration had been recognized” (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:195-209`), but it does not define that shape, diagnostic category, precedence over ordinary statement parsing, or the semantic status of the recovered binding. Implementations could consequently diagnose valid uses of the identifier `val`, accept an old declaration accidentally, or inject a recovery-only binding into name resolution and flow typing as if it were valid source.

ADR-0024's declaration-starting-keyword recovery boundaries at `docs/adr/ADR-0024-expression-statement-pattern-syntax.md:165-187` are also spelling-dependent and are not explicitly covered by the proposal's ADR-0024 supersession boundary.

Required revision:

- Define the exact token/AST shape that selects the migration diagnostic and distinguish it from valid expression and assignment statements involving an identifier named `val`.
- Assign a stable diagnostic category and specify its relationship to `malformed_variable_declaration`.
- Specify that recovery does not make the rejected source accepted and whether the synthetic declaration/binding is marked invalid for name resolution, type checking, and flow analysis.
- Extend the ADR-0024 supersession boundary to every spelling-dependent recovery rule: `const` replaces `val` as the immutable-local statement starter, while ordinary identifier `val` is not a general declaration-starting recovery boundary.

### 4. Medium: “ordinary runtime expression” must be bounded so it does not invent deferred execution semantics

The proposal's negative rule is sound: the spelling `const` must not require compile-time evaluability or confer constant-specific status (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:112-116`). However, accepted ADR-0024 specifies initializer syntax, not evaluation order or runtime execution semantics, and defers compile-time-evaluation syntax. ADR-0027 also defers calls, ownership analysis, HIR/MIR, and backend behavior. The examples and phrases “ordinary runtime expressions” and “adds no runtime behavior” must therefore not be read as deciding when, how often, or with what effects an initializer executes.

Required revision:

- Formulate the controlling rule negatively and narrowly: `const` by itself does not request or guarantee compile-time evaluation, and its initializer follows whatever initializer/expression execution rules are separately accepted.
- Explicitly preserve current deferrals for evaluation order, execution count, effects, constant folding, definite assignment, ownership transfer, and lowering.
- Mark call-based examples as syntax illustrations only while ADR-0027 direct-call typing remains deferred.
- If runtime execution semantics are intended to be newly fixed here, expand the semantic proposal and review surface rather than relying on “ordinary” as an undefined term.

## Supersession Audit

The following boundaries are substantively correct once the required revisions above are made:

- ADR-0021: replace reserved keyword `val` with reserved keyword `const`; all other token rules remain unchanged.
- ADR-0024: replace only the immutable-local declaration token and all directly spelling-dependent recovery references; retain optional-initializer syntax, statement structure, expression grammar, ownership-scope statements, and explicit deferrals.
- ADR-0026: replace references to local `val` binding positions with local `const` binding positions; retain binding identity, scope, declaration order, lookup, shadowing, duplicate, and ambiguity rules.
- ADR-0013 and ADR-0028: no semantic supersession; the resulting binding remains the same immutable local category.
- ADR-0019: no semantic or syntactic supersession; compile-time evaluation remains a distinct concept whose concrete syntax is still deferred.
- ADR-0027: no type-checking expansion; unsupported expression, ownership, lowering, and backend rules remain deferred.
- Grammar authority ledger: not itself semantic authority, but its statement-grammar and maintenance records must be updated in the same acceptance task as required by `docs/syntax/grammar-authority-ledger.md:65-67`.

## Decision And Handoff

Decision: Revise before acceptance. The selected hard replacement is not blocked as a design, and no accepted semantic text requires the spelling `val` permanently. Acceptance is blocked until the compatibility surface, synchronized SPEC supersession, legacy diagnostic/recovery behavior, and runtime-initializer boundary are made precise.

Files changed: only `docs/adr/proposals/reviews/ADR-0029-language-lawyer-review.md`.

Files proposed for change: the draft proposal, by the Language Designer; no direct edits are authorized by this review.

Open questions:

- What exact rejected token sequence triggers the legacy-`val` migration diagnostic without capturing valid identifier uses?
- What stable diagnostic category and invalid recovery representation will be used?
- Will the Chief Architect require ADR acceptance and SPEC synchronization in one change, as the authority hierarchy requires?

Blockers: ADR-0029 acceptance, tests, compiler implementation, and downstream syntax migration must remain blocked until the required revisions are accepted.

Validation performed: cross-checked every controlling claim against the cited SPEC and accepted ADR text; checked the grammar authority ledger's maintenance rule; inspected repository status before and after writing the report. No compiler or test validation was appropriate for this documentation-only semantic review.

Handoff target: Language Designer for proposal revision, then Diagnostics Engineer for the migration contract, then Chief Architect for synchronized acceptance or further direction.

## Revised-Round Review

### Round Metadata

- Re-review date: `2026-07-10`
- Revised proposal: `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`
- Prior findings re-evaluated: `4`
- Final classification: `ambiguous`
- Final verdict: `revise`

### Prior Finding Disposition

#### 1. Resolved: compatibility impact of reserving the former identifier `const`

The revised proposal now states that the change is source-breaking in both directions (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:53-63`), identifies the accepted identifier positions affected by reserving `const`, and declines to prescribe an unsafe context-free rename (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:97-109`). It carries that impact into diagnostics, migration, and downstream work at proposal lines 250-274 and 295-310. This satisfies the first required revision.

#### 2. Resolved: atomic synchronization with the higher-authority SPEC

The revised proposal identifies the exact affected SPEC sections, requires the accepted ADR and conforming SPEC revision to become authoritative atomically, requires the grammar authority ledger update in the same acceptance step, and states that existing authority continues to control if synchronization fails (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:371-400`). The approval workflow repeats the no-partial-acceptance rule at proposal lines 402-418. This satisfies the second required revision.

#### 3. Resolved: legacy-`val` diagnostic and recovery ambiguity

The revised proposal removes the special migration recognizer and recovery-as-success design. It now requires `val` to be treated only as an ordinary identifier, old declaration-shaped source to receive the ordinary diagnostics applicable to its actual token sequence, and rejected old syntax to create no binding or downstream semantic record (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:250-269`). It also extends the ADR-0024 supersession boundary to spelling-dependent statement dispatch and recovery at proposal lines 356-364. This is a precise alternative to defining the previously proposed special diagnostic and satisfies the third required revision.

#### 4. Resolved: runtime-initializer wording and preserved deferrals

The revised proposal makes the controlling rule negative and narrow: local `const` does not request or guarantee compile-time evaluation, and initializer execution follows separately accepted rules (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:129-137`). It marks call examples as syntax-only under ADR-0027 (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:117-127`) and explicitly preserves definite-initialization, evaluation-order, effect, optimization, ownership-transfer, call, and lowering deferrals (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:168-183`). This satisfies the fourth required revision without importing Kotlin behavior.

### Remaining Actionable Finding

#### Medium: the cross-phase invariant overstates the exclusion of the spelling `val`

The proposal correctly establishes that `val` becomes an ordinary identifier and that valid uses remain governed by their actual grammar positions (`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:97-101` and `257-269`). However, the new invariant says “source `val` never enters the accepted local-declaration path” at proposal line 221. Read literally, that sentence also covers a valid declaration whose binding identifier is spelled `val`:

```text
const val: Int = 1;
```

Under the proposal's own lexical rule, the second token is an ordinary identifier and the declaration matches the proposed grammar. The invariant appears intended to exclude `val` only from the declaration-keyword or statement-starter position, but it does not say so. Leaving the broader wording could cause parser, fixture, or name-resolution work to reject a valid identifier use and would conflict with the proposal's ordinary-identifier rule.

Required revision:

- Replace the invariant bullet with wording scoped to the declaration introducer, for example: “source `val` in the immutable-local declaration-keyword position never enters the accepted local-declaration path.”
- State explicitly that `val` remains permitted as the binding identifier in a declaration introduced by `const` or `var`, subject to ordinary duplicate, shadowing, type, and other accepted rules.
- Apply the same positional qualification anywhere downstream instructions use “source `val`” as shorthand for removed declaration syntax.

### Final Decision And Handoff

Final verdict: Revise before acceptance. All four prior findings are resolved in substance, and the proposal's semantic and supersession design is otherwise precise. Acceptance remains blocked only on the narrow cross-phase-invariant wording above; no new semantic design choice is required.

Files changed in this round: only `docs/adr/proposals/reviews/ADR-0029-language-lawyer-review.md`.

Files proposed for change: the draft proposal, by the Language Designer; this review authorizes no direct proposal edit.

Open question: Does the Language Designer confirm that `const val: Int = 1;` is intended to remain syntactically valid because `val` is an ordinary identifier in binding-name position?

Blocker: ADR-0029 acceptance and downstream migration remain blocked until the invariant is positionally qualified.

Validation performed: re-read the complete revised proposal, mapped each prior finding to its revised text, and re-checked the controlling SPEC, ADR-0021 lexical classification, ADR-0024 statement and recovery grammar, ADR-0026 binding positions, and ADR-0028 immutable-binding eligibility.

Handoff target: Language Designer for the one-line semantic-precision correction, then Chief Architect for final approval after confirming synchronized acceptance.

## Final Sign-Off

### Sign-Off Metadata

- Sign-off date: `2026-07-10`
- Final classification: `specified`
- Final verdict: `approve`

Role: Language Lawyer

Question: Did the Language Designer resolve the sole remaining positional-`val` ambiguity, and do the final proposal, accepted ADR-0029, synchronized specification, and grammar authority ledger state one precise semantic rule?

Inputs read:

- `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`
- `docs/adr/ADR-0029-immutable-local-const-keyword.md`
- `docs/SPEC.md`
- `docs/syntax/grammar-authority-ledger.md`
- this review's initial and revised-round findings

Controlling text:

- The final proposal scopes the exclusion of `val` to the immutable-local declaration-introducer position and expressly accepts `const val: Int = 1;` and `var val = 1;` at `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md:98-113`, `233-236`, and `282-288`.
- Accepted ADR-0029 states the same positional rule and examples at `docs/adr/ADR-0029-immutable-local-const-keyword.md:60-101`, preserves that distinction across phases at lines 150-174, and applies it to diagnostics and semantic records at lines 176-208.
- `docs/SPEC.md:125-127`, `157-169`, `205-210`, and `250-269` synchronizes the lexical, statement, binding-position, and semantic summaries. It reserves `const`, treats `val` as an ordinary identifier, and excludes `val` only as the immutable-local declaration introducer.
- `docs/syntax/grammar-authority-ledger.md:19`, `34`, `39`, `45`, and `51-57` cites ADR-0029 for the superseded token and statement rules while retaining the deferral of compile-time-evaluation syntax.

Interpretation:

The sole remaining finding is resolved. `val` is not an immutable-local declaration introducer, alias, or compatibility syntax, but it is an ordinary identifier in every otherwise accepted identifier position. Therefore `const val: Int = 1;` and `var val = 1;` are syntactically valid uses of the binding name `val`, subject to ordinary semantic checks.

The accepted rule remains a spelling replacement only. Local `const` maps to the existing immutable-local semantic category and does not request or guarantee compile-time evaluation or confer storage, copyability, ownership, destruction, borrowing, lifetime, send/share, type-position, or layout meaning. Existing initializer and execution deferrals remain intact.

Synchronization and supersession check:

- The accepted ADR is marked `Status: Accepted`.
- The SPEC records all required ADR-0021, ADR-0024, ADR-0026, and ADR-0029 changes without retaining a conflicting `val` declaration rule.
- The ledger cites ADR-0029 for token spellings, statement grammar, parser dispatch, and recovery while leaving compile-time-evaluation syntax deferred.
- The accepted ADR's supersession boundary remains limited to spelling-dependent portions of ADR-0021, ADR-0024, and ADR-0026; ADR-0019 compile-time evaluation, remaining ADR-0024 grammar, ADR-0026 scope and lookup, and ADR-0028 flow semantics remain unsuperseded.

Non-authoritative assumptions: none. No Kotlin rule, implementation behavior, fixture, task, milestone, or example outside the accepted authority bundle was used to fill a semantic gap.

Required escalation: none for semantic precision or source-of-truth consistency.

Final verdict: **Approve.** The final proposal is precise, the accepted ADR faithfully carries the decision, the SPEC and ledger are synchronized, all prior Language Lawyer findings are closed, and no semantic blocker remains.

Files changed in this sign-off: only `docs/adr/proposals/reviews/ADR-0029-language-lawyer-review.md`.

Open questions: none within Language Lawyer scope.

Blockers: none within Language Lawyer scope. Downstream work remains subject to the ordered gates in accepted ADR-0029 rather than to this review.

Validation performed: read the complete final proposal and accepted ADR, cross-checked every affected SPEC summary, verified each relevant grammar-ledger authority row and deferral, and confirmed the positional `val` rule and examples agree across all four artifacts.

Handoff target: Chief Architect and downstream role owners may proceed under the accepted ADR-0029 authority bundle and its explicit roadmap gates.
