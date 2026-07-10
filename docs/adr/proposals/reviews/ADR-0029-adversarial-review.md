# ADR-0029 Adversarial Review

Role: Adversarial Engineer

Target: Draft `ADR-0029`, replacing the immutable-local `val` spelling with
`const` while preserving an ordinary immutable local binding and ordinary
runtime initialization.

Initial verdict: **REVISE before acceptance.** The hard replacement is sound in
direction, but the draft leaves two paths by which `const` or recovered `val`
can acquire unintended semantics. These must be closed before approval.

## Inputs read

- `AGENTS.md`
- `.codex/agents/adversarial-engineer.toml`
- `.codex/agents/language-lawyer.toml`
- `docs/SPEC.md`
- `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`
- `docs/adr/ADR-0001-ownership-model.md`
- `docs/adr/ADR-0002-borrowing-semantics.md`
- `docs/adr/ADR-0003-lifetime-model.md`
- `docs/adr/ADR-0004-destruction-and-resource-finalization.md`
- `docs/adr/ADR-0005-copy-move-and-value-categories.md`
- `docs/adr/ADR-0008-structured-concurrency-semantics.md`
- `docs/adr/ADR-0009-async-suspension-and-borrowing.md`
- `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
- `docs/adr/ADR-0013-mutability-model.md`
- `docs/adr/ADR-0014-thread-safety-and-data-race-freedom.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0019-compile-time-evaluation-and-metaprogramming.md`
- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- `docs/adr/ADR-0026-name-resolution-policy.md`
- `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- `docs/syntax/grammar-authority-ledger.md`
- Current lexer, parser, name-resolution, type-checking, and related test
  references to `val`, inspected read-only to identify phase boundaries
- Concurrent ADR-0029 Diagnostics Engineer and Simplicity Guardian reviews,
  inspected read-only after they appeared in the workspace

There is no accepted ADR-0029 task, test change, or compiler implementation
diff to review. Existing implementation behavior was used only to construct
phase-consistency attacks, never as semantic authority.

## Invariants tested

1. `const` denotes only the existing immutable local-binding category.
2. A `const` initializer is an ordinary runtime expression; the spelling grants
   no const-evaluation, purity, literal, storage, type, or layout property.
3. `val` is not an accepted declaration alias, even when parser recovery emits
   a useful migration error.
4. Lexing, parsing, binding metadata, name resolution, assignment checks, flow
   typing, ownership, destruction, and concurrency checks agree on the same
   immutable/mutable distinction.
5. Binding immutability does not imply copyability, deep immutability,
   shareability, sendability, or exemption from deterministic destruction.
6. Smart casts continue to depend on binding identity and immutability, not on
   the `const` spelling or compile-time evaluation.

## Findings

### 1. Blocking: the draft leaves a future semantic-conflation escape hatch

Proposal lines 112-116 say that `const` initializers are ordinary runtime
expressions **unless** a separate accepted compile-time-evaluation rule says
otherwise. Line 125 weakens this again to an initializer "may remain" an
ordinary runtime expression. That is not equivalent to a spelling-only change.
It permits a later rule to reinterpret this declaration without explicitly
superseding ADR-0029, potentially making an effectful initializer illegal,
moving evaluation to another phase, or granting layout/type-level authority.

Concrete attack:

```text
const request = readRequest();
const nonce = randomNonce();
const handle = openResource();
```

An evaluator could classify these by the `const` token, reject them as impure,
attempt to execute them during compilation, or materialize them in static
storage. ADR-0019 does not authorize any of those results, and ADR-0024
explicitly defers compile-time-evaluation syntax.

Required revision: state categorically that this declaration form has ordinary
runtime initialization and no compile-time-constant semantics. Any future rule
that gives this spelling compile-time meaning must explicitly supersede the
relevant part of ADR-0029; merely accepting a separate const-evaluation feature
must not retroactively change immutable locals.

### 2. Blocking: recovery can become accidental `val` aliasing

Proposal lines 195-205 require a declaration-shaped `val` to recover "as though
an immutable local declaration had been recognized" while also requiring the
program to remain rejected. The draft does not say whether that recovered form
creates an ordinary local-binding node, enters scope and name-resolution
tables, becomes smart-cast eligible, or is marked invalid through later phases.

Concrete attack:

```text
val item: String? = readMaybe();
if (item != null) {
    consume(item);
}
```

If recovery produces the same unmarked semantic record as `const`, downstream
name resolution and ADR-0028 flow analysis can treat `val` as a valid immutable
binding. A driver, incremental query, IDE path, or later diagnostic-suppression
bug that fails to gate on the parser error then has a de facto compatibility
alias. Conversely, discarding the binding entirely can produce misleading
`unresolved_name` cascades. Both behaviors fit the current phrase.

Required revision: define the recovery result and its validity state. It must
be impossible for recovered `val` to yield a successful compilation or an
accepted declaration, while downstream recovery behavior and suppression of
spelling-induced cascades must be deterministic. The recognition predicate
must also distinguish legacy declarations from valid identifier uses such as
`val = 1;`, `val();`, and `var val = 0;`.

### 3. Major: phase consistency is required but not stated as a verifiable invariant

Proposal lines 161-176 identify inconsistent phase migration, but leave it as
an implementation-planning warning. The current frontend has spelling-bearing
boundaries in the keyword token, parser dispatch, local-binding kind, name
resolution, assignment checking, and flow eligibility. A partial migration can
therefore produce any of these failures:

- `const` lexes but does not parse as a declaration;
- `const` parses but is classified mutable or unknown later;
- assignment to `const` is accepted because only the old immutable category is
  checked;
- `const x: T?` loses ADR-0028 smart-cast eligibility;
- `var` accidentally gains refinement through a broad "not val" migration;
- `val` lexes as an identifier but is still accepted by a parser fallback;
- a const-evaluator, storage class, or send/share capability is keyed directly
  from the source token.

Required revision: make the cross-phase invariant normative. `const` must map
to the one existing semantic immutable-local category; `var` must remain the
mutable category; no semantic phase may infer evaluator, storage, copy, drop,
borrow, lifetime, send/share, type, or layout properties from the spelling; and
`val` must not enter the accepted declaration path. Internal representation
names are implementation details, but tests must observe this semantic matrix.

### 4. Major ambiguity: initializer-free immutable locals remain semantically unspecified

Proposal lines 124 and 133-134 preserve ADR-0024's optional initializer and say
assignment rules do not change. ADR-0024, however, accepts the syntax while
deferring binding mutability and later semantic checks; the accepted authority
read for this review does not define definite initialization or whether a
later assignment initializes an initializer-free immutable local exactly once.

Concrete attack:

```text
const resource: Resource;
resource = openResource();
resource = openResource();
```

The first assignment might be initialization, forbidden assignment, or still
deferred; the second must not be accepted merely because `const` was confused
with static storage or because no initializer appeared at the declaration.

Required revision: do not encode a new answer in ADR-0029. State that optional
initializer syntax is preserved, while definite-initialization and one-time
initialization semantics remain exactly as specified or deferred by existing
authority. Route any claim of exact later-initialization behavior to Language
Designer and Language Lawyer before tests encode it.

### 5. Major diagnostic/source-compatibility case: reserving `const` also invalidates old identifiers

The hard replacement makes `const` globally reserved under ADR-0021's
non-contextual keyword model. The migration section discusses old `val`
declarations but not programs that currently use `const` as an identifier.
This also makes an unconditional one-token fix unsafe:

```text
val const = value;
```

Replacing only `val` yields invalid `const const = value;`. A mandatory fix-it
would therefore violate the safe-suggestion obligation.

Required revision: acknowledge the identifier-collision side of the source
break and make the `val`-to-`const` fix conditional on the resulting declaration
being valid. This is primarily owned by Diagnostics Engineer, but it also
prevents recovery from pretending that an invalid rewritten declaration is an
accepted alias.

## Attacks attempted

- **Effectful/runtime initializer:** `const x = readRequest();`, random input,
  allocation, I/O, and resource acquisition. Result: intended behavior is
  stated, but Finding 1 must remove the future-rule escape hatch.
- **Compile-time/layout/static promotion:** use a local `const` as an array
  length, generic argument, layout value, global initializer, or static address.
  Result: correctly unauthorized by lines 112-116 and 165-169; retain this
  boundary.
- **Move-only value:** move a user-defined value out of a `const` binding, then
  use it again. Result: `const` must follow ADR-0001/0005 move and use-after-move
  rules; it is not implicitly copyable.
- **Deterministic destruction:** bind a resource-owning value to `const`, move it
  conditionally, and exit the block or cancel a future coroutine. Result: the
  keyword grants no drop exemption or static lifetime; ADR-0004 remains
  authoritative, with detailed drop/cancellation rules still deferred.
- **Borrow/lifetime escape:** take a borrow from a `const` local, move the owner,
  leave the block, or let a future suspended frame outlive it. Result: the
  spelling grants no lifetime extension; ADR-0002/0003/0009 checks still apply.
- **Assignment and interior mutation:** assign a new value to `const`, mutate
  through an alias, or call an interior-mutability abstraction. Result: binding
  immutability must not be mistaken for deep immutability; exclusive mutation
  and safe abstraction rules remain separate.
- **Smart casts:** compare `const x: T?` with null, shadow it, and compare a
  `var` control case. Result: ADR-0028 correctly keys refinement to resolved
  binding identity plus immutability. `const` must remain eligible exactly
  where the old immutable local was; `var` must not gain eligibility.
- **Thread crossing:** capture `const x` in concurrent work where `x` lacks
  send/share capability, and share interior mutable state without a safe
  synchronization abstraction. Result: ADR-0014, not the spelling or binding
  immutability, controls acceptance.
- **Legacy alias:** lex `val` as an identifier, parse declaration-shaped and
  non-declaration-shaped uses, and inspect recovery flow. Result: Finding 2.
- **Phase split:** migrate lexer only, parser only, semantic binding metadata
  only, or flow checks only. Result: Finding 3.

## Negative tests added/proposed

No tests were added because the user prohibited test edits and the proposal is
not accepted authority. After revision and acceptance, Test Engineer should add
or update tests covering:

1. `const x = runtimeEffect();` is accepted without const evaluation.
2. Literal, purity, static-storage, layout, and type-position restrictions are
   not inferred from local `const`.
3. Move-only `const` values move once, reject use-after-move, and are destroyed
   under the ordinary ownership rules.
4. Borrows from `const` do not outlive the binding and gain no suspension or
   thread-crossing exemption.
5. Assignment to initialized `const` is rejected; `var` assignment behavior is
   unchanged.
6. `const x: T?` is eligible for the exact ADR-0028 refinement cases, while
   `var x: T?` is not; shadowing remains identity-based.
7. A `const` capture lacking send/share capability is rejected across a
   thread/task boundary.
8. `val item = value;` always produces a migration error and cannot compile;
   `val = 1;`, `val();`, and `var val = 0;` use `val` as an identifier and do
   not receive the migration diagnostic.
9. `val const = value;` receives no unsafe one-token fix-it.
10. Lexer, parser, name-resolution, type-check, flow, ownership, and later
    lowering tests all distinguish source spelling from semantic immutability.

## Ambiguities

- The semantic treatment of initializer-free immutable locals and later
  initialization is not resolved by the accepted authority read here. ADR-0029
  must preserve that status rather than claiming or testing a guessed rule.
- The recovery representation and downstream participation of rejected
  declaration-shaped `val` source are unspecified in the draft.
- The phrase allowing a separate future compile-time rule to say otherwise is
  ambiguous about whether explicit supersession of ADR-0029 would be required.

## Decision or action taken

Request revision before acceptance. No compiler, test, task, milestone,
example, proposal, SPEC, or accepted ADR file was changed. This review report
is the only file added.

## Files changed or proposed for change

- Changed: `docs/adr/proposals/reviews/ADR-0029-adversarial-review.md`
- Proposed for later Language Designer revision:
  `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`
- Proposed after acceptance only: phase-spanning positive, negative, recovery,
  diagnostic, ownership, flow, and thread-safety tests listed above

## Open questions

1. Will the migration diagnostic recover an error-marked binding for cascade
   control, or remain a classification of ordinary statement recovery with no
   binding record?
2. Which accepted future authority will define initializer-free immutable
   local initialization, and how will it distinguish initialization from
   reassignment?
3. Must a future compile-time declaration use a different spelling, or may it
   supersede ADR-0029 explicitly? This proposal need only require explicit
   supersession, not choose that future syntax.

## Blockers

- Remove the implicit permission for a separate rule to retroactively give
  ordinary local `const` declarations compile-time semantics.
- Define recovery so rejected `val` cannot become an accepted alias through
  downstream semantic records or alternate compilation paths.
- State a normative, testable cross-phase mapping from source `const` to the
  existing immutable-local semantic category.
- Preserve, rather than guess, the unresolved semantics of initializer-free
  immutable locals.

## Handoff

Language Designer should revise ADR-0029 for the four blockers above.
Diagnostics Engineer should own the exact migration recognition, recovery,
severity, span, and conditional-fix contract. Language Lawyer should verify the
initializer-free-binding deferral and supersession wording. Reviewer and Chief
Architect should not approve the proposal until those revisions are complete.

## Revised-round review

Date: `2026-07-10`

Re-review target: the revised
`docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`, checked against
each finding and blocker in this report. The proposal remains a draft; no test,
task, compiler, SPEC, accepted ADR, milestone, or example change was treated as
authority.

### Prior finding closure

1. **Compile-time semantic conflation — closed.** Revised proposal lines
   129-137 state categorically that local `const` has no
   compile-time-constant meaning, follows separately accepted initializer
   rules, and can gain compile-time meaning only through explicit supersession
   of ADR-0029. A separate compile-time-evaluation feature cannot reinterpret
   this declaration implicitly.
2. **Accidental `val` alias through recovery — closed.** Lines 257-269 remove
   the special legacy diagnostic and recovery-as-declaration path. `val` is an
   ordinary identifier, old declaration-shaped source receives only ordinary
   ADR-0024 parsing and recovery, and rejected old syntax creates no binding,
   AST alias, name-resolution entry, type fact, flow fact, ownership record, or
   other downstream semantic record.
3. **Cross-phase inconsistency — closed.** Lines 214-235 establish a normative
   mapping from source `const` to the existing immutable-local category, keep
   `var` mapped to the mutable category, exclude `val` from the accepted
   declaration path, enumerate all affected semantic phases, and prohibit
   const-specific evaluator, storage, ownership, copy/drop, send/share, type,
   layout, or legacy metadata. Lines 318-340 add ordered authority, tests-first,
   frontend, semantic-fixture, and example gates before dependent work resumes.
4. **Initializer-free local ambiguity — closed by preservation of deferral.**
   Lines 168-183 explicitly leave later initialization, definite
   initialization, one-time initialization, evaluation order, effects,
   ownership transfer, and lowering as specified or deferred by existing
   authority. Optional initializer syntax is not authority to accept or reject
   a later assignment.
5. **`const` identifier source break and unsafe fix-it — closed.** Lines
   103-109 and 252-274 recognize that globally reserving `const` invalidates all
   accepted identifier positions using that spelling. The proposal specifies
   no context-free rename, special `val` fix-it, or new diagnostic contract.

### Attacks re-run

- Effectful, runtime-only, move-only, resource-owning, and borrowed
  initializers gain no evaluator, static-storage, copyability, lifetime, or
  destruction property from `const`.
- A local `const` cannot serve as layout, type-level, static-storage, send, or
  share authority; accepted ownership, borrowing, destruction, and concurrency
  rules remain controlling.
- ADR-0028 smart-cast eligibility still consumes immutable binding identity,
  making valid `const` locals eligible only in the existing cases and leaving
  `var` ineligible.
- Partial lexer/parser/semantic migration is forbidden by the cross-phase
  invariant and ordered migration gates.
- Declaration-shaped `val` cannot enter semantic analysis as a recovered
  immutable binding, while valid identifier uses of `val` remain governed by
  ordinary grammar.
- Initializer-free `const` does not acquire a guessed definite-assignment or
  one-time-assignment rule.

No new ambiguity, safety regression, thread-safety authority leak, ownership
exception, smart-cast widening, or hidden compatibility alias was found in the
revised proposal.

### Revised-round action and validation

- Files changed: only
  `docs/adr/proposals/reviews/ADR-0029-adversarial-review.md`.
- Negative tests added: none; the proposal remains non-authoritative and the
  user prohibited test edits.
- Proposed post-acceptance coverage remains the phase-spanning test matrix in
  this report, now subject to the revised proposal's tests-first migration gate.
- Validation: compared every original finding, ambiguity, blocker, and attack
  against the revised normative text and checked repository status to preserve
  concurrent work.

## Final verdict

**APPROVE.** The revised ADR-0029 proposal resolves all prior adversarial
findings and is sufficiently precise for Chief Architect consideration. This
approval covers the proposal's safety, soundness, phase-consistency, ownership,
smart-cast, and thread-safety boundaries; it does not itself accept the ADR or
authorize downstream changes. Acceptance must still follow the proposal's
atomic ADR/SPEC authority update and tests-first migration gates.

Final handoff: Chief Architect for the acceptance decision, with Reviewer,
Language Lawyer, Diagnostics Engineer, Simplicity Guardian, and Roadmap Planner
confirming their respective revised-round obligations.
