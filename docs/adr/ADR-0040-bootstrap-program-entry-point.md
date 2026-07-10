# ADR-0040: Bootstrap Program Entry Point

Status: Accepted

## Question

What program entry point is accepted for the first runnable executable smoke
test?

## Competing Designs

1. `fun main(): Int`
2. `fun main()`
3. `fun main(args: Array<String>): Int`
4. Platform-specific entry declarations.

## Trade-offs

An explicit `Int` return gives the object/link pipeline a measurable exit-code
result without requiring printing, a standard library, or command-line
argument types.

Unit-returning `main` is ergonomic, but it cannot validate integer return
lowering in the first executable.

Argument-taking `main` requires arrays, heap allocation, string runtime
ownership, and CLI ABI rules that are not yet specified.

Platform-specific entry declarations expose ABI concerns before the bootstrap
language entry contract exists.

## Recommended Choice

The first executable subset accepts exactly one program entry point:

- a top-level function named `main`;
- in the root module entry package selected by the compiler invocation;
- with no parameters;
- with declared return type `Int`;
- with a body that returns an `Int` value on every reachable path.

The language-level result of `main` maps to the process exit code for the
bootstrap executable. The first executable smoke test must use a non-negative
`Int` in the host platform exit-code range `0..255`; behavior for other values
is deferred to the ABI/runtime roadmap.

CLI arguments are deferred. Multiple candidate `main` declarations, missing
`main`, parameterized `main`, non-`Int` `main`, declaration-only `main`, and
unreachable or missing `main` return paths are rejected for the executable
subset.

Diagnostics:

- `missing_entry_point` for no accepted `main`;
- `duplicate_entry_point` for multiple accepted-name candidates;
- `invalid_entry_point_signature` for parameters, missing body, or non-`Int`
  return type;
- `missing_return` for a reachable path in `main` without an `Int` return.

## Downstream Consequences

- M0029 can validate executable result through process exit status.
- M0030 does not need printing, CLI args, arrays, heap allocation, or a standard
  library for the first runnable program.
- Later user-facing entry forms must explicitly supersede or extend this ADR.

## Dependencies

- ADR-0022
- ADR-0024
- ADR-0027
- ADR-0041
- ADR-0047
