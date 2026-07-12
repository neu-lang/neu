# ADR-0088: Owned Lambdas And Closures

## Status

Accepted.

## Decision

Lambda expressions use Kotlin-like braces with an arrow:
`{ value: Int -> value + 1 }`. Parameters may have explicit types; inferred
parameter types are permitted only when an expected function type supplies
them. The body is an expression or an existing block form and must have the
expected return type when context requires one.

Captures are inferred from lexical name use. Copyable captures are copied;
move-only captures are transferred into the closure. Mutable captures require
an exclusive inferred effect and are represented as compiler-managed mutable
environment state. A borrowed capture may not outlive its source or cross a
suspension/transfer boundary. Closures are immutable values, non-comparable,
compiler-managed, and have compiler-private identity and environment layout.

Closure construction, invocation, destruction, and capture cleanup are
source-mapped HIR/MIR facts. Closure values use the accepted function-value
capabilities after capture analysis: `Send`/`Share` require all captured values
to satisfy the corresponding existing predicates. Recursive closures require
an explicit later decision and are rejected in this stage.

No public environment layout, allocation API, FFI closure ABI, reflection,
coroutine suspension, detached execution, or user pointer syntax is introduced.

## Consequences

The parser and type checker must preserve lambda parameters, body, captures,
and source spans. Backend lowering may use compiler-private environments and
indirect calls; invalid ownership or lifetime facts are rejected before it.

## Dependencies

ADR-0023, ADR-0035, ADR-0037, ADR-0062, ADR-0086, ADR-0087.
