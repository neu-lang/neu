# ADR-0032: Generic Constraint Enforcement Sequencing

Status: Accepted

## Question

When may the compiler enforce generic capability bounds, given that M0020 is
scheduled before ownership, move, and thread-capability analyses?

## Competing Designs

1. Define a provisional built-in catalog (`Copy`, `Send`, `Share`) and
   provisional satisfaction rules in M0020.
2. Treat every bound as a nominal interface requirement and resolve it through
   general type/interface lookup.
3. Complete generic parameter and bound representation in M0020, but defer
   constraint enforcement until ownership and thread-capability semantics are
   accepted and implemented.

## Decision

Choose option 3. M0020 preserves generic parameter identity and explicit bound
occurrences only. It does not decide a capability catalog or resolution model,
type-satisfaction rules, substitution point, or bound-violation diagnostics.

A later accepted decision must define those semantics and schedule enforcement
after its ownership and thread-capability inputs. Generic parameter and bound
representation are valid M0020 outputs; enforcing bounds is not an M0020
acceptance condition.

## Consequences

M0020 may complete after representation work and explicit deferral coverage.
M0021 may depend on that representation-only M0020 boundary. Capability-bound
enforcement is intentionally deferred until a post-M0024 milestone. No parser
syntax, runtime behavior, ownership rule, or thread-safety rule changes here.

## Dependencies And Supersession

This narrows the implementation sequencing implied by ADR-0016 without
changing its constrained-generic direction. It relies on ADR-0005, ADR-0010,
ADR-0014, ADR-0016, ADR-0017, and ADR-0023. It resolves
`docs/ambiguities/M0020-generic-constraint-semantics.md` for milestone
sequencing only; a later semantic ADR remains required for enforcement.
