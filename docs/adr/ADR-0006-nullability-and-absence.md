# ADR-0006: Nullability And Absence

## Question

How does the language represent missing values?

## Competing Designs

- Kotlin-style nullable types.
- Algebraic option type only.
- Both nullable references and option values.
- Sentinel values by convention.

## Trade-offs

Kotlin-style nullability is ergonomic and familiar.

Option types compose well and avoid special null rules.

Supporting both increases complexity.

Sentinels are unsafe and unsuitable for this language.

## Recommended Choice

Kotlin-style nullable types as surface syntax, semantically modeled as explicit optional values with no implicit null for non-nullable types.

## Downstream Consequences

- Smart casts and flow typing become part of the core language.
- FFI must define how platform nulls map into safe types.
- Generic nullability must be specified carefully.

## Dependencies

- ADR-0011
- ADR-0015
- ADR-0018

