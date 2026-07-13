# ADR-0029: Immutable Local `const` Keyword

Status: Superseded by ADR-0061

## Question

Should the immutable local declaration keyword `val` be replaced by `const`,
and does that spelling change make immutable local bindings compile-time
constants or otherwise change their existing semantics?

## Context

ADR-0021 originally reserved `val` as a fixed keyword and did not reserve
`const`. ADR-0024 originally defined local declaration syntax as:

```text
variable-declaration = (`val` | `var`) identifier type-annotation? initializer? `;`
```

ADR-0024 assigned no compile-time-evaluation meaning to `val`; it introduced a
local binding syntactically and left mutability, ownership, moves, and borrow
effects to semantic analysis. ADR-0026 identified local `val` and `var`
statements as binding positions and defined their lexical scope,
declaration-order, shadowing, and duplicate-name rules. ADR-0028 based
smart-cast eligibility on whether a local binding is immutable, not on whether
its initializer is evaluated at compile time.

Separately, ADR-0019 describes deterministic bounded compile-time evaluation
for constants but does not accept concrete compile-time-evaluation syntax.
ADR-0024 and the grammar authority ledger continue to defer that syntax.

## Competing Designs

1. Hard-replace `val` with `const` while preserving the existing immutable-local
   semantic category.
2. Accept `const` and `val` during a transition period.
3. Keep `val` for immutable locals and reserve `const` for future compile-time
   constants.
4. Replace `val` through a broader binding redesign, such as `let` or
   keywordless immutable declarations.

## Trade-offs

A hard replacement leaves one canonical immutable-local form and implements the
owner-selected spelling directly. It creates source incompatibilities in both
directions: `val` stops introducing an immutable local when it occupies that
declaration-introducer position, and the formerly ordinary identifier spelling
`const` becomes reserved. The spelling may also suggest compile-time evaluation
to users, so its lack of compile-time meaning must be categorical.

A transition period reduces immediate migration cost but creates two accepted
spellings, compatibility state, and temporary parser and diagnostic surface.

Keeping `val` and reserving `const` for compile-time constants separates the
concepts but contradicts the owner-selected replacement.

A broader binding redesign reopens declaration grammar and mutability policy
without an accepted need.

## Decision

Hard-replace the immutable-local declaration-introducer keyword `val` with
`const`.

The lexical keyword set reserves `const` and no longer reserves `val`. Under
ADR-0021's exact-match identifier rule, `val` therefore lexes as an ordinary
identifier. It is not accepted as an immutable-local declaration introducer or
as a legacy alias for `const`.

The exclusion applies only when `val` occupies the immutable-local
declaration-introducer position. `val` remains a valid ordinary identifier in
every identifier position permitted by the grammar, subject to the ordinary
rules for that position. In particular, both declarations below use `val` as
the binding name:

```text
const val: Int = 1;
var val = 1;
```

The local declaration grammar is:

```text
variable-declaration = (`const` | `var`) identifier type-annotation? initializer? `;`
```

Local `const` behaves exactly like the Kotlin-style `val` binding already
designed by this project. It denotes the existing immutable-local semantic
category and uses the existing initializer rules. This wording imports no
additional Kotlin rule.

Local `const` categorically has no compile-time-constant meaning. The keyword
does not request or guarantee compile-time evaluation and does not require a
literal initializer, purity, compile-time evaluability, static storage, global
scope, type-level availability, or compile-time materialization. Its
initializer follows whatever local-initializer and expression-execution rules
are separately accepted for the existing immutable-local category.

Any future decision that gives this declaration spelling compile-time meaning
must explicitly supersede this ADR. Accepting a separate compile-time-evaluation
feature does not reinterpret local `const` implicitly.

## Preserved Semantics And Deferrals

This ADR changes the immutable-local keyword spelling and the lexical
availability of the spellings `val` and `const`. It does not change the
semantic category represented by a valid immutable-local declaration:

- `const` introduces the same immutable-local binding category that `val`
  introduced under the superseded syntax.
- An initializer remains optional syntactically where ADR-0024 permits it.
- A `const` initializer uses the same project-defined initializer rules as the
  superseded `val` form and acquires no compile-time-evaluation rule.
- Type annotations and initializer grammar remain unchanged.
- `var` remains the mutable-local declaration keyword.
- Local scope, declaration order, visibility after the declaration, shadowing,
  duplicate-name behavior, and lookup remain those of ADR-0026.
- Binding identity remains unchanged as a semantic concept.
- Immutability remains distinct from copyability, ownership, move behavior,
  borrowing, lifetime, destruction, and thread-safety capabilities.
- Assignment and mutation rules do not become stricter or looser because the
  keyword is spelled `const`.
- ADR-0028 smart-cast eligibility continues to depend on an immutable local
  binding. A `const` local is eligible exactly where the former `val` local was
  eligible and gains no compile-time-constant status.
- Blocks remain the same syntactic and ownership scopes.
- No global, member, parameter, pattern-binding, type-level, or
  compile-time-constant declaration form is introduced.

The following matters remain exactly as specified or deferred by existing
authority and are not answered by this ADR:

- whether and how an initializer-free immutable local may later be initialized
- definite-initialization and one-time-initialization rules
- initializer and subexpression evaluation order
- execution count and effect semantics
- constant folding and other optimizations
- ownership transfer during initialization
- call typing and call effects
- HIR, MIR, backend lowering, storage placement, and code generation

Optional initializer syntax is not authority for accepting or rejecting a later
assignment to an initializer-free `const`. That question requires separate
accepted semantics before tests or implementation encode an answer.

This ADR does not alter the no-GC, no-manual-memory-management, compile-time
memory-safety, or compile-time thread-safety constraints. It adds no runtime
behavior and weakens no safety invariant.

## Cross-Phase Invariant

Every phase must preserve one normative mapping:

- source `const` in the accepted local-declaration grammar maps to the one
  existing immutable-local semantic category
- source `var` continues to map to the existing mutable-local category
- an identifier token spelled `val` in the immutable-local
  declaration-introducer position never enters the accepted immutable-local
  declaration path; uses of `val` in ordinary identifier positions continue
  through the normal path for those positions

Lexing, parsing, binding metadata, name resolution, type checking, flow typing,
ownership, borrowing, destruction, concurrency checking, and later lowering
must agree on that mapping. No phase may infer evaluator participation,
constant materialization, static or alternate storage, copyability, destruction
behavior, borrow or lifetime behavior, send/share capability, type-position
availability, or layout authority from the `const` spelling.

No `const`-specific semantic binding category, evaluator input, constant-value
table, storage-class metadata, ownership flag, copy/drop flag, send/share flag,
type capability, layout capability, legacy alias marker, or downstream semantic
record is authorized. Source spelling may be retained only as ordinary source
or syntax metadata where diagnostics require it; semantic phases consume the
existing immutable-local category.

## Diagnostics And Source Compatibility

This is a source-breaking hard replacement in both directions. Existing source
that declares immutable locals with `val` must migrate those declaration
introducers to `const`. Existing uses of `const` as an identifier must be
renamed because `const` is now reserved. No automatic replacement name is
specified because name-resolution context is required to choose one safely.

There is no special legacy-`val` migration diagnostic, recognition predicate,
fix-it, or recovery contract. When `val` appears where the old grammar used it
as the immutable-local declaration introducer, the source is parsed under the
new grammar and receives only the existing ordinary ADR-0024 parser diagnostics
and recovery applicable to its actual token sequence. Parser behavior must not
contextually recognize that removed declaration-introducer use or recover as if
a `const` declaration had succeeded.

The rejected use of `val` in the immutable-local declaration-introducer
position produces no alias, compatibility declaration, legacy AST node,
immutable binding, name-resolution entry, type-checking fact, flow fact,
ownership record, or other downstream semantic record. Valid uses of `val` as
an identifier, including the binding names in `const val: Int = 1;` and
`var val = 1;`, remain subject only to the ordinary grammar and semantic rules
for their actual positions.

Source broken because `const` is newly reserved receives the existing ordinary
lexical or parser diagnostic appropriate to the position. This ADR introduces
no new diagnostic category, rule identifier, severity, recovery artifact, or
snapshot contract.

Diagnostics should use the source-level term "immutable local binding" where
the distinction matters. They may quote the `const` spelling but must not
describe the binding as a compile-time constant unless accepted authority
explicitly supersedes this ADR for that declaration form.

## Soundness Impact

The keyword replacement has no direct memory-safety or thread-safety effect
because every phase continues to classify the resulting binding as the existing
immutable-local category.

The principal soundness risk is semantic conflation: treating `const` values as
compile-time evaluated, implicitly copyable, promoted to static storage, usable
in type or layout positions, exempt from ordinary destruction, or safe to share
across threads. The decision and cross-phase invariant forbid those
consequences.

## Simplicity Impact

The hard replacement keeps one immutable-local spelling and introduces no
compatibility mode, deprecation state, alternate AST binding kind, constant
evaluator, storage class, new ownership rule, contextual legacy grammar,
migration framework, recovery-as-success path, or semantic alias.

## Downstream Consequences

Downstream migration must update the lexical keyword set, local declaration
grammar, spelling-dependent parser dispatch and ordinary recovery boundaries,
binding-position terminology, frontend metadata, semantic fixtures, and
examples while preserving the cross-phase invariant.

ADR-0028's semantic rules require no behavioral change. Any edit is
terminology-only unless needed to state that a `const` local denotes the same
immutable-binding category.

### Implementation Hard Gate

The associated task remains paused through all of these ordered gates:

1. The complete ADR-0029 authority bundle becomes authoritative.
2. main-task test work performs the lexical, parser, binding-category, diagnostic,
   name-resolution, type-check, and flow tests-first migration and records the
   expected pre-implementation failures.
3. main-task implementation performs one coherent frontend migration of lexing, parsing,
   syntax metadata, and local-binding classification.
4. main-task test work and required specialty reviewers verify semantic fixtures
   across name resolution, type checking, nullability, flow typing, and
   ownership-facing behavior, including proof that no const-specific semantics
   or metadata were introduced.
5. Examples and related documentation migrate only after the semantic fixtures
   pass.

After all five gates pass, project planning must revalidate
The associated task must be checked against this authority before returning it to the main-task test work and
main-task implementation workflow. Existing task text, implementation, fixtures, or examples
are not semantic precedent.

## Supersession

This ADR supersedes only these portions of earlier authority:

- ADR-0021's keyword set, replacing reserved `val` with reserved `const`
- ADR-0024's immutable-local declaration spelling, replacing `val` with `const`
- ADR-0024's directly spelling-dependent statement dispatch and recovery
  boundaries, making `const` the immutable-local statement starter and treating
  `val` only as an identifier
- ADR-0026's references to local `val` statements as binding positions,
  replacing them with local `const` statements

This ADR does not supersede ADR-0019's compile-time-evaluation direction,
ADR-0024's remaining declaration grammar or ownership-scope rules, ADR-0026's
scope and lookup semantics, or ADR-0028's immutable-binding and flow-typing
semantics.

## Acceptance Bundle

This ADR, the conforming `docs/SPEC.md` changes, and the grammar authority ledger
update are authoritative as one semantic-change transaction. None of those
artifacts may be interpreted as authorizing a partial or contradictory version
of the decision.

The specification records the keyword change in ADR-0021's summary, the local
declaration and recovery change in ADR-0024's summary, the binding-position
change in ADR-0026's summary, and the complete semantic boundary in its new
ADR-0029 summary. The grammar authority ledger cites ADR-0029 for the
superseded token-spelling and statement-grammar portions while leaving
compile-time-evaluation syntax deferred.

## Dependencies

- ADR-0013, mutability model
- ADR-0015, diagnostics as semantics
- ADR-0019, compile-time evaluation and metaprogramming
- ADR-0021, lexical grammar
- ADR-0024, expression, statement, and pattern syntax
- ADR-0026, name-resolution policy
- ADR-0028, nullability and flow typing
- `docs/SPEC.md`
- `docs/syntax/grammar-authority-ledger.md`
