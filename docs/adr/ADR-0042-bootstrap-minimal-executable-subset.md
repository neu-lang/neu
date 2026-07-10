# ADR-0042: Bootstrap Minimal Executable Subset

Status: Accepted

## Question

What exact source forms are supported end-to-end for the first runnable smoke
program?

## Competing Designs

1. A minimal integer function subset.
2. All currently parsed expressions and statements.
3. A backend-only synthetic MIR program.
4. A standard-library-based "hello world".

## Trade-offs

A minimal integer function subset is small enough to validate honestly and
large enough to exercise frontend, HIR, MIR, backend, object, link, and process
exit behavior.

All parsed syntax would pull many deferred semantics into backend work.

Synthetic MIR avoids parser/type-checker risk but does not prove source-to-run
execution.

Hello-world requires strings, I/O, standard library, and runtime support.

## Recommended Choice

The first executable smoke subset includes only:

- package declarations already accepted by the parser;
- top-level function declarations with explicit parameter types, explicit
  `Int` return types, and bodies;
- the accepted `main` form from ADR-0040;
- `Int` parameters and `Int` local bindings;
- local `const` and `var` declarations whose initializers are executable-subset
  expressions;
- assignments to local `var` bindings with executable-subset expressions;
- integer literals in the ADR-0043 bootstrap `Int` range;
- bare local-name expressions;
- parenthesized executable-subset expressions;
- unary arithmetic and bitwise operations `+`, `-`, and `~` on `Int`;
- binary arithmetic operations `+`, `-`, `*`, `/`, `%`, and `**` on `Int`;
- binary bitwise operations `&`, `|`, and `^` on `Int`;
- binary shift operations `<<` and `>>` on `Int`;
- direct same-module top-level function calls from ADR-0041; and
- explicit `return expression;`.

The first required runnable program shape is:

- helper functions that exercise `Int` arithmetic and bitwise operations; and
- `main` returning the helper result as the process exit code.

The subset explicitly defers classes, structs at runtime, interfaces, enums at
runtime, generics, nullable runtime representation, heap allocation,
destructuring, pattern matching, loops, branches beyond straight-line return
analysis, coroutines, unsafe, FFI, printing, strings, standard library calls,
scheduler/runtime work, exceptions/panics as language constructs, closures,
methods, member access, arrays, and target-pack APIs.

This ADR extends the executable subset beyond ADR-0024 where needed for
operators not previously parsed. M0028 must add parser and type-checker support
for `**`, `~`, `^`, `<<`, and `>>` before HIR lowering may consume executable
fixtures. Operator precedence for the executable subset is, from highest to
lowest: unary `+`, unary `-`, unary `~`; exponentiation `**` as
right-associative; `*`, `/`, `%`; `+`, `-`; `<<`, `>>`; `&`; `^`; `|`.

Unsupported parsed forms must fail before backend lowering with an
`unsupported_executable_form` diagnostic or a more specific existing diagnostic
when one already applies.

## Downstream Consequences

- M0029 through M0032 have an exact source-backed executable target.
- Backend work must not accept broader source semantics by accident.
- Examples may show the first runnable shape only after implementation reaches
  the matching milestone.

## Dependencies

- ADR-0022
- ADR-0024
- ADR-0027
- ADR-0029
- ADR-0040
- ADR-0041
- ADR-0043
