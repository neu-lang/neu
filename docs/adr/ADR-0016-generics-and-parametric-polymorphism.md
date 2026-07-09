# ADR-0016: Generics And Parametric Polymorphism

## Question

How expressive should generics be before implementation starts?

## Competing Designs

- Simple nominal generics with constraints.
- Higher-kinded types.
- Compile-time templates.
- Monomorphized generics only.

## Trade-offs

Simple constrained generics are understandable and sufficient for most APIs.

Higher-kinded types add abstraction but major complexity.

Templates are powerful but can damage diagnostics and compile times.

Monomorphization gives performance but can increase binary size.

## Recommended Choice

Constrained nominal generics with explicit capability bounds; allow static specialization without exposing template metaprogramming as the primary model.

## Downstream Consequences

- Ownership, copyability, send/share, and nullability need generic constraints.
- Standard collections depend on generic variance rules.
- Compile-time performance depends on specialization strategy.

## Dependencies

- ADR-0005
- ADR-0010
- ADR-0014
- ADR-0019

