# ADR-0003: Lifetime Model

## Question

How are borrow lifetimes represented in the language?

## Competing Designs

- Mostly inferred lexical and non-lexical lifetimes, with explicit annotations only at API boundaries.
- Fully explicit lifetime parameters.
- Region names exposed as first-class language constructs.
- Lifetime elision with no user-visible lifetime syntax.

## Trade-offs

Inferred lifetimes preserve ergonomics, but need excellent diagnostics.

Fully explicit lifetimes are precise, but hostile to Kotlin-like usability.

Region names are powerful, but add conceptual weight.

No visible lifetimes improves readability until APIs become ambiguous.

## Recommended Choice

Inferred lifetimes by default, explicit lifetime parameters only where needed for public generic APIs and unsafe-adjacent abstractions.

## Downstream Consequences

- Public API design must define when lifetime annotations are required.
- Type inference and diagnostics become central language features.
- Borrow checking must be explainable without exposing too much internal machinery.

## Dependencies

- ADR-0001
- ADR-0002
- ADR-0015

