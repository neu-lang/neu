# ADR-0019: Compile-Time Evaluation And Metaprogramming

## Question

What compile-time computation is allowed?

## Competing Designs

- Constants and simple pure evaluation only.
- Full compile-time function execution.
- Macro system.
- Reflection-based code generation.

## Trade-offs

Simple constant evaluation preserves fast compilation and diagnostics.

Full compile-time execution is powerful but can slow and complicate builds.

Macros can damage readability and error quality.

Reflection generation helps frameworks but risks hidden complexity.

## Recommended Choice

Start with deterministic, bounded compile-time evaluation for constants, layout-relevant values, and simple generic support; defer macros until the core language proves insufficient.

## Downstream Consequences

- The language remains easier to compile quickly.
- Some framework ergonomics may require external code generation initially.
- Const evaluation must obey ownership and purity restrictions.

## Dependencies

- ADR-0015
- ADR-0016
- ADR-0020

