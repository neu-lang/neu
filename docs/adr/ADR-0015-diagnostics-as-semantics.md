# ADR-0015: Diagnostics As Semantics

## Question

What diagnostic obligations are part of the language design?

## Competing Designs

- Diagnostics are implementation quality only.
- Define required diagnostic classes for ownership, borrowing, nullability, and concurrency.
- Provide machine-readable explanations as part of the compiler contract.
- Require fix-its for most static errors.

## Trade-offs

Treating diagnostics as implementation-only risks a technically correct but hostile language.

Required diagnostic classes make the language teachable.

Machine-readable diagnostics help tooling but constrain compiler design.

Mandatory fix-its are unrealistic for deep semantic errors.

## Recommended Choice

Define diagnostic obligations for all core safety systems, including ownership, borrowing, lifetime, nullability, move, and concurrency errors.

## Downstream Consequences

- Language rules should be designed for explainability.
- Compiler phases must preserve source-level intent.
- Some theoretically elegant rules may be rejected if they cannot be diagnosed clearly.

## Dependencies

- ADR-0002
- ADR-0003
- ADR-0006
- ADR-0008
- ADR-0014

