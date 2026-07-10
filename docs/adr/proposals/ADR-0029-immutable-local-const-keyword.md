# ADR-0029: Immutable Local `const` Keyword

Status: Draft proposal - not accepted source of truth

## Non-Authority Notice

This file is a draft proposal only. It is not accepted language syntax or
semantics, not an accepted ADR, and not a valid basis for implementation,
tests, tasks, milestones, examples, or `docs/SPEC.md` changes.

The project owner has selected the intended direction, but the change remains
non-authoritative until the required semantic-change reviews are complete and
the main task approves an accepted ADR. The existing accepted ADRs and
`docs/SPEC.md` remain authoritative in the meantime.

## Semantic Question

Should the immutable local declaration keyword `val` be replaced by `const`,
and, if so, does that spelling change also make immutable local bindings
compile-time constants or otherwise change their existing semantics?

## Existing Authority And Problem Statement

Accepted ADR-0021 reserves `val` as a fixed keyword and does not reserve
`const`. Accepted ADR-0024 defines local declaration syntax as:

```text
variable-declaration = (`val` | `var`) identifier type-annotation? initializer? `;`
```

ADR-0024 assigns no compile-time-evaluation meaning to `val`; it introduces a
local binding syntactically and leaves mutability, ownership, moves, and borrow
effects to semantic analysis. Accepted ADR-0026 identifies local `val` and
`var` statements as binding positions and preserves their lexical scope,
declaration-order, shadowing, and duplicate-name rules. Accepted ADR-0028 bases
smart-cast eligibility on whether a local binding is immutable, not on whether
its initializer is evaluated at compile time.

Separately, ADR-0019 describes deterministic bounded compile-time evaluation
for constants but does not accept concrete compile-time-evaluation syntax.
ADR-0024 and the grammar authority ledger explicitly defer that syntax.

The project owner's decision is therefore interpreted narrowly as a keyword
replacement. Local `const` is to behave exactly like the Kotlin-style `val`
binding already designed by this project: it denotes this project's existing
immutable-local semantic category and uses this project's existing initializer
rules. This wording imports no additional Kotlin rule. It does not supply
authority to merge immutable local bindings with the distinct, still-deferred
concept of compile-time constants.

## Competing Designs

### 1. Hard-replace `val` with `const`

Reserve `const`, remove `val` from the keyword set, and accept only `const` for
immutable local declarations. Preserve all existing local-binding semantics.

This directly implements the owner's selected spelling and leaves one canonical
immutable-local form. It creates intentional source incompatibilities in both
directions: `val` stops introducing an immutable local when it occupies that
declaration-introducer position, and the formerly ordinary identifier spelling
`const` becomes reserved. The spelling may also be associated with compile-time
evaluation, so the specification must distinguish those concepts
categorically.

### 2. Accept `const` and `val` during a transition period

Reserve both spellings and parse both as the same immutable local declaration,
possibly warning on `val` before removing it later.

This eases migration, but creates two accepted spellings, extends the migration
across multiple language versions, complicates fixtures and diagnostics, and
does not follow the requested hard replacement.

### 3. Keep `val` for immutable locals and reserve `const` for compile-time constants

Retain the current local declaration syntax and introduce `const` only after
compile-time constant semantics are designed.

This keeps the terms conceptually separate and avoids source breakage, but it
contradicts the owner's keyword decision and would require a later semantic
design for `const` rather than the requested replacement.

### 4. Replace `val` with a broader binding redesign

Use another spelling such as `let`, make immutable declarations keywordless, or
change whether immutability is the default.

This could avoid the conventional compile-time meaning of `const`, but it
reopens declaration grammar and mutability policy beyond the owner's decision.
It would also create unnecessary interactions with ownership, flow typing, and
name resolution.

## Recommended Draft Choice

Choose design 1: a hard replacement of `val` with `const`.

If accepted, the lexical keyword set will reserve `const` and will no longer
reserve `val`. Because ADR-0021 classifies an identifier solely by exact match
against the accepted keyword set, `val` will lex as an ordinary identifier. It
will not be accepted as an immutable-local declaration keyword or as a legacy
alias for `const`.

The exclusion applies only when `val` occupies the immutable-local
declaration-introducer position. `val` remains a valid ordinary identifier in
every identifier position permitted by the grammar, subject to the ordinary
rules for that position. In particular, both of these declarations use `val`
as the binding name:

```text
const val: Int = 1;
var val = 1;
```

Conversely, `const` is an ordinary identifier under current accepted authority.
Reserving it is a source break for every accepted identifier position, not only
local binding names. Existing declarations, local names, references,
qualified-name components, type names, package components, and any other
accepted identifier positions spelled `const` must be renamed as part of an
authorized migration. No automatic replacement name is specified because
name-resolution context is required to choose one safely.

The local declaration grammar will become:

```text
variable-declaration = (`const` | `var`) identifier type-annotation? initializer? `;`
```

For example, as a syntax illustration only:

```text
const request = readRequest();
const retries: Int = configuredRetries;
var attempts: Int = 0;
```

ADR-0027 still defers direct-call typing, so these examples do not establish
that either call type-checks. They illustrate only that `const` uses the same
initializer syntax accepted for the former `val` spelling.

Local `const` categorically has no compile-time-constant meaning. The keyword
does not request or guarantee compile-time evaluation and does not require a
literal initializer, purity, compile-time evaluability, static storage, global
scope, type-level availability, or compile-time materialization. Its
initializer follows whatever local-initializer and expression-execution rules
are separately accepted for the existing immutable-local category. Any future
decision that gives this declaration spelling compile-time meaning must
explicitly supersede the relevant rule in ADR-0029; accepting a separate
compile-time-evaluation feature does not reinterpret local `const` implicitly.

## Preserved Semantics

This proposal changes the immutable-local keyword spelling and the lexical
availability of the spellings `val` and `const`. It does not change the
semantic category represented by a valid immutable-local declaration. If
accepted:

- `const` introduces the same kind of immutable local binding that `val`
  introduced under the superseded syntax.
- An initializer remains optional syntactically where ADR-0024 currently
  permits it.
- A `const` initializer uses exactly the same project-defined initializer rules
  as the superseded `val` form and acquires no compile-time-evaluation rule.
- Type annotations and initializer grammar remain unchanged.
- `var` remains the mutable local declaration keyword.
- Local scope, declaration order, visibility after the declaration, shadowing,
  duplicate-name behavior, and lookup remain those of ADR-0026.
- Binding identity remains unchanged as a semantic concept.
- Immutability remains distinct from copyability, ownership, move behavior,
  borrowing, lifetime, destruction, and thread-safety capabilities.
- Assignment and mutation rules do not become stricter or looser merely because
  the keyword is spelled `const`.
- ADR-0028 smart-cast eligibility continues to depend on an immutable local
  binding. A `const` local is eligible in exactly the cases where the former
  `val` local was eligible; it gains no compile-time-constant status.
- Blocks remain the same syntactic and ownership scopes.
- No global, member, parameter, pattern-binding, type-level, or
  compile-time-constant declaration form is introduced.

The following matters remain exactly as specified or deferred by existing
authority and are not answered by ADR-0029:

- whether and how an initializer-free immutable local may later be initialized
- definite-initialization and one-time-initialization rules
- initializer and subexpression evaluation order
- execution count and effect semantics
- constant folding and other optimizations
- ownership transfer during initialization
- call typing and call effects
- HIR, MIR, backend lowering, storage placement, and code generation

In particular, optional initializer syntax must not be interpreted as authority
for accepting or rejecting a later assignment to an initializer-free `const`.
That question requires separate accepted semantics before tests or
implementation encode an answer.

This proposal does not alter the no-GC, no-manual-memory-management,
compile-time memory-safety, or compile-time thread-safety constraints. It adds
no runtime behavior and weakens no safety invariant.

## Rejected Alternatives

Design 2 is rejected because a transition alias conflicts with the requested
hard replacement and creates temporary language surface without changing the
eventual outcome.

Design 3 is rejected because it retains the spelling the project owner has
decided to replace. It also postpones the requested decision behind unrelated
compile-time-evaluation design work.

Design 4 is rejected because it turns a spelling decision into a broader
declaration and mutability redesign without accepted need.

## Soundness Impact

The keyword replacement has no direct memory-safety or thread-safety effect if
all phases continue to classify the resulting binding as the existing immutable
local binding kind.

The principal soundness risk is semantic conflation: an implementation might
infer that `const` values are compile-time evaluated, implicitly copyable,
promoted to static storage, usable in type or layout positions, exempt from
ordinary destruction, or safe to share across threads. None of those
consequences is authorized by this proposal.

### Cross-Phase Invariant

The accepted migration must preserve one normative mapping across every phase:

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
or syntax metadata where diagnostics require it; semantic phases must consume
the existing immutable-local category.

Required main-task adversarial check review should attack at least:

- runtime-only and effectful initializers under `const`
- move-only and deterministically destroyed values bound by `const`
- borrow and lifetime behavior of `const` locals
- assignment attempts to `const` locals
- smart-cast eligibility for `const` and ineligibility for `var`
- attempts to use `const` locals as compile-time, layout, static-storage, or
  cross-thread authority
- accidental continued acceptance of `val` as the immutable-local
  declaration introducer
- partial migration in any one frontend or semantic phase
- introduction of `const`-specific evaluator, storage, or capability metadata

## Diagnostics And Migration Consequences

This is a source-breaking hard replacement in both directions. Existing source,
fixtures, and examples that declare immutable locals with `val` must migrate to
`const`. Existing uses of `const` as an identifier must be renamed before the
new keyword set can accept that source.

There is no special legacy-`val` migration diagnostic, recognition predicate,
fix-it, or recovery contract. After acceptance, `val` is an ordinary
identifier. When `val` appears where the old grammar used it as the
immutable-local declaration introducer, the source is parsed under the new
grammar and receives only the existing ordinary ADR-0024 parser diagnostics and
recovery applicable to its actual token sequence. Parser behavior must not
contextually recognize that removed declaration-introducer use or recover as if
a `const` declaration had succeeded.

The rejected use of `val` in the immutable-local declaration-introducer
position produces no alias, compatibility declaration, legacy AST node,
immutable binding, name-resolution entry, type-checking fact, flow fact,
ownership record, or other downstream semantic record. Valid uses of `val` as
an identifier—including the binding names in `const val: Int = 1;` and
`var val = 1;`—remain subject only to the ordinary grammar and semantic rules
for their actual positions.

Likewise, source broken because `const` is newly reserved receives the existing
ordinary lexical or parser diagnostic appropriate to the position. This ADR
does not prescribe an automatic rename, because no context-free replacement is
guaranteed to preserve name resolution or program meaning.

Diagnostics for assigning to an immutable local, ownership and move errors,
borrowing, nullability, and flow typing should use the source-level term
"immutable local binding" where the distinction matters. They may quote the
`const` spelling, but must not describe the binding as a compile-time constant
unless accepted authority explicitly supersedes ADR-0029 for that declaration
form.

No new diagnostic category, rule identifier, severity, recovery artifact, or
snapshot contract is introduced by this spelling proposal.

## Simplicity Impact

The hard replacement keeps one immutable-local spelling and introduces no
compatibility mode, deprecation state, alternate AST binding kind, constant
evaluator, storage class, or new ownership rule.

Using existing parser diagnostics and recovery avoids a contextual legacy
grammar, migration framework, recovery-as-success path, or semantic alias.

## Downstream Consequences

After acceptance, separately authorized follow-up work will need to update:

- `docs/SPEC.md` summaries for ADR-0021, ADR-0024, and ADR-0026
- the accepted lexical keyword set and local declaration grammar
- name-resolution terminology that names local `val` binding positions
- syntax authority records that cite the superseded lexical and statement rules
- lexer and parser token/binding metadata while preserving the semantic
  immutable-binding classification
- spelling-dependent parser dispatch and ordinary recovery boundaries
- local-binding, name-resolution, type-checking, and flow-typing fixtures whose
  source uses `val` as the immutable-local declaration introducer
- fixtures and source that currently use `const` as an identifier
- milestones and tasks that name `val` as the immutable binding spelling
- current examples that use `val` as the immutable-local declaration introducer

ADR-0028's semantic rules need no behavioral change. Any follow-up edit should
be terminology-only unless it is needed to state that a `const` local denotes
the same immutable binding category.

No downstream file is changed or authorized for implementation by this draft.

### Roadmap Hard Gate For M0019-014

Task `M0019-014` must remain paused through all of the following ordered gates:

1. Authority gate: the accepted ADR-0029 and conforming `docs/SPEC.md` revision
   become authoritative atomically.
2. Tests-first migration gate: main-task test work updates or adds the required
   lexical, parser, binding-category, diagnostic, name-resolution, type-check,
   and flow expectations before implementation changes, with failures showing
   the intended authority transition.
3. Frontend migration gate: lexer, parser, syntax metadata, and local-binding
   classification migrate coherently to the cross-phase invariant.
4. Semantic fixture migration gate: name-resolution, type-checking,
   nullability, flow-typing, ownership-facing, and other semantic fixtures use
   `const` for the existing immutable-local category and prove that no
   const-specific semantics were introduced.
5. Examples gate: current examples and related documentation migrate only after
   the preceding semantic fixtures pass.

`M0019-014` may resume only after every gate above is complete and the task is
revalidated against the new accepted authority. Work already present in the
task file or implementation does not override this pause and is not semantic
precedent.

## Dependencies And Supersession Targets

Dependencies:

- `docs/SPEC.md`
- ADR-0013, mutability model
- ADR-0015, diagnostics as semantics
- ADR-0019, compile-time evaluation and metaprogramming
- ADR-0021, lexical grammar
- ADR-0024, expression, statement, and pattern syntax
- ADR-0026, name-resolution policy
- ADR-0028, nullability and flow typing
- `docs/syntax/grammar-authority-ledger.md`

If accepted, ADR-0029 will supersede only these portions of earlier authority:

- ADR-0021's keyword set, replacing reserved `val` with reserved `const`
- ADR-0024's immutable-local declaration spelling, replacing `val` with `const`
- ADR-0024's directly spelling-dependent statement dispatch and recovery
  boundaries, making `const` the immutable-local statement starter and treating
  `val` only as an identifier
- ADR-0026's references to local `val` statements as binding positions,
  replacing them with local `const` statements

ADR-0029 will not supersede ADR-0019's compile-time-evaluation direction,
ADR-0024's remaining declaration grammar or ownership-scope rules, ADR-0026's
scope and lookup semantics, or ADR-0028's immutable-binding and flow-typing
semantics.

### Atomic Acceptance And Exact Specification Targets

ADR-0029 must not become accepted before or separately from its conforming
`docs/SPEC.md` revision. main task approval must make the accepted ADR and
the specification changes authoritative in one semantic-change step.

That atomic specification revision must update these exact targets:

- `docs/SPEC.md` section `ADR-0021: Lexical Grammar`, so its fixed reserved
  keyword-set summary records that `const` is reserved and `val` is not
- `docs/SPEC.md` section `ADR-0024: Expression Statement And Pattern Syntax`,
  first summary paragraph, replacing “local `val` and `var` declaration
  statements” with “local `const` and `var` declaration statements”
- `docs/SPEC.md` section `ADR-0026: Name Resolution Policy`, paragraph beginning
  “Function declaration names,” replacing local `val` binding positions with
  local `const` binding positions
- a new `docs/SPEC.md` ADR-0029 summary recording the hard lexical replacement,
  preservation of the existing immutable-local category, absence of
  compile-time-constant meaning, ordinary treatment of `val` as an identifier,
  and the source break caused by reserving `const`

The same acceptance step must update the grammar authority ledger as required
by its maintenance rule, but the ledger does not itself create semantics.

If the accepted ADR, specification targets, and required authority-ledger
maintenance are not synchronized, ADR-0029 remains non-authoritative. The
existing `docs/SPEC.md` text and accepted ADR-0021, ADR-0024, and ADR-0026
continue to control: `val` remains the immutable-local keyword, `const` remains
an ordinary identifier, downstream migration and implementation remain
blocked, and `M0019-014` remains paused.

## Required Review And Approval Workflow

Before this proposal can become authoritative:

1. main-task language review must verify the preserved deferrals, exact supersession
   boundaries, and atomic authority update.
2. main-task adversarial check must verify the cross-phase invariant and absence of
   compile-time, evaluator, storage, ownership, and capability leakage.
3. main-task diagnostics check must verify that ordinary existing diagnostics and
   recovery are sufficient and that no legacy recognition path remains.
4. main-task simplicity check must verify that no alias, compatibility mode, legacy
   syntax mechanism, or const-specific semantic metadata is introduced.
5. main-task roadmap planning must enforce the ordered `M0019-014` hard gates above.
6. main task must approve, reject, or request revision.
7. On approval, the accepted ADR, exact `docs/SPEC.md` targets, and authority
   ledger maintenance must become authoritative atomically. No partial
   acceptance or stale-authority interval is permitted.
