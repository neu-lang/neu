# ADR-0036: Bootstrap Borrow And Lifetime Analysis

Status: Accepted

## Question

What smallest synchronous borrow and lifetime subset can M0023 implement without
assuming function calls, methods, member access, references as first-class
runtime values, async suspension, unsafe escape hatches, or HIR?

## Competing Designs

1. Add explicit Rust-like `&` and `&mut` borrow expressions immediately.
2. Infer all borrow behavior from ordinary local-name uses with no source-level
   borrow markers.
3. Define a metadata-only bootstrap model: accepted borrow records are compiler
   side-table facts over local bindings, with shared and exclusive borrow
   conflict checking and local lifetime escape diagnostics.
4. Defer all borrow analysis until function calls, HIR, and reference types
   exist.

## Trade-Offs

Design 1 is familiar for ownership-oriented languages but would add syntax not
yet accepted by the Kotlin-like grammar and could conflict with future surface
design.

Design 2 is ergonomic but too implicit for a first soundness milestone; it risks
treating ordinary reads or assignments as borrows without accepted rules.

Design 3 lets M0023 validate the core shared-or-exclusive invariant over
explicit compiler facts without inventing user syntax. It keeps the source
surface stable while producing diagnostics and data structures later syntax can
feed.

Design 4 avoids premature surface choices but delays a core memory-safety
invariant past the scheduled safety phase.

## Decision

Choose design 3 for M0023.

M0023 introduces a bootstrap borrow-analysis side-table model. It does not add
new source syntax. Parser-level borrow expressions, reference types, dereference
operators, function parameter passing by borrow, method receivers, member
borrows, closure captures, coroutine borrows, unsafe references, and FFI
references remain deferred.

## Borrow Records

An accepted borrow input record has:

- borrow node identity;
- borrowed local binding identity;
- borrow kind: `shared` or `exclusive`;
- region node identity.

The region node is the lexical scope in which the borrow is live. For M0023,
regions are existing AST nodes such as block nodes supplied by earlier parser
metadata or test-side synthetic metadata.

Shared borrows permit other shared borrows of the same local binding in the
same overlapping region. An exclusive borrow conflicts with any other shared or
exclusive borrow of the same local binding in the same overlapping region.

M0023 only checks equality of region identity for overlap. Nested, sibling,
non-lexical, loop, path-sensitive, and control-flow-sensitive lifetime overlap
rules are deferred.

## Lifetime Escape Records

An accepted lifetime escape input record has:

- escape node identity;
- borrowed local binding identity;
- borrow node identity;
- borrow region node identity;
- use region node identity.

For M0023, a lifetime escape is diagnosed when a borrowed local is used as a
borrow outside the exact region identity that owns the borrow. Equality of
region identity is the only accepted validity rule in this bootstrap subset.

## Diagnostics And Recovery

Diagnostic: `borrow_conflict`

- Primary span: the later borrow node that conflicts.
- Secondary span: the earlier conflicting borrow node.
- Recovery action: keep both borrow facts in the report and continue checking
  independent borrow records.
- Safe suggestion policy: identify the conflicting borrow kinds and source
  locations only. Do not suggest cloning, allocation, unsafe, refactoring,
  adding lifetime annotations, or changing mutability unless a future accepted
  rule defines that suggestion.

Diagnostic: `lifetime_escape`

- Primary span: the escape node.
- Secondary span: the borrow node whose region is being escaped.
- Recovery action: keep the escape diagnostic and continue checking independent
  records.
- Safe suggestion policy: state that the borrow cannot be used outside its
  region. Do not suggest lifetime annotations or heap allocation unless a future
  accepted rule defines that suggestion.

Unsupported source-level borrow syntax or reference types remain parser or
type-system deferrals until accepted by a later ADR.

## Consequences

M0023 can implement borrow conflict and lifetime escape checking over explicit
side-table facts. This validates the core shared-or-exclusive invariant without
committing the language to Rust-like borrow syntax or implicit Kotlin-like
borrowing rules.

Later ADRs may add syntax that lowers to these records, broaden region overlap
rules, and define borrow interaction with calls, returns, member access,
captures, async suspension, unsafe, and FFI.

## Dependencies And Supersession

Depends on ADR-0001, ADR-0002, ADR-0003, ADR-0011, ADR-0015, ADR-0024,
ADR-0026, ADR-0035, and M0023.

This resolves `docs/ambiguities/M0023-borrow-lifetime-semantics.md`. It does
not define source-level borrow syntax or reference types.
