# ADR-0032: Generic Constraint Enforcement Sequencing

Status: Accepted

## Question

When may the compiler enforce generic capability bounds, given that the compiler is
scheduled before ownership, move, and thread-capability analyses?

## Competing Designs

1. Define a provisional built-in catalog (`Copy`, `Send`, `Share`) and
   provisional satisfaction rules in this implementation.
2. Treat every bound as a nominal interface requirement and resolve it through
   general type/interface lookup.
3. Complete generic parameter and bound representation in this implementation, but defer
   constraint enforcement until ownership and thread-capability semantics are
   accepted and implemented.

## Decision

Choose option 3. The implementation preserves generic parameter identity and explicit bound
occurrences only. It does not decide a capability catalog or resolution model,
type-satisfaction rules, substitution point, or bound-violation diagnostics.

A later accepted decision must define those semantics and schedule enforcement
after its ownership and thread-capability inputs. Generic parameter and bound
representation are valid outputs; enforcing bounds is not an acceptance condition.

## Consequences

The compiler may complete after representation work and explicit deferral coverage.
The implementation may depend on that representation-only boundary. Capability-bound
enforcement is intentionally deferred until a later implementation phase. No parser
syntax, runtime behavior, ownership rule, or thread-safety rule changes here.

## Dependencies And Supersession

This narrows the implementation sequencing implied by ADR-0016 without
changing its constrained-generic direction. It relies on ADR-0005, ADR-0010,
ADR-0014, ADR-0016, ADR-0017, and ADR-0023. It resolves
the corresponding ambiguity report for sequencing only; a later semantic ADR
remains required for enforcement.
