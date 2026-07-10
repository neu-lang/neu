# ADR-0047: Bootstrap Object Link Runtime Model

Status: Accepted

## Question

What object, linking, and minimal runtime model is accepted for the first
runnable executable?

## Competing Designs

1. Freestanding no-stdlib program with a tiny bootstrap runtime shim.
2. Link against the host C runtime.
3. Require a Neu standard library.
4. Produce object files only and defer running.

## Trade-offs

A freestanding no-stdlib program plus tiny shim validates executable production
without inventing standard-library semantics.

Host C runtime linking may be expedient but conflicts with the no-hidden-host
dependency direction in ADR-0020.

Requiring a Neu standard library pulls a large future milestone into the first
backend smoke.

Object-only output would not satisfy the runnable-program requirement.

## Recommended Choice

The first object/link pipeline targets the current host object's native format
only. Cross-object formats and target packs are deferred to M0033.

The first runnable program requires no Neu standard library. It may use a tiny
bootstrap runtime shim whose only responsibilities are:

- provide or participate in the executable entry path for the initial host
  target;
- call the compiled language `main`;
- map a non-negative `Int` in `0..255` to the process exit status; and
- trap on bootstrap runtime traps such as checked integer overflow,
  division/modulo by zero, negative exponent, or invalid shift count.

The shim is an implementation artifact, not a language standard library. It
must not provide printing, allocation, CLI arguments, scheduling, panics as a
language feature, exceptions, FFI helpers, or platform APIs.

M0032 must use the planned bundled linker path for the initial host target. Any
temporary host-tool dependency must be documented as a blocker or explicit
limitation and must not be presented as satisfying Go-like target-pack
semantics.

Bootstrap trap behavior must fail the process non-successfully and preserve a
diagnosable internal reason in tests. User-facing panic formatting is deferred.

## Downstream Consequences

- M0032 can honestly run a minimal arithmetic program without stdlib or
  printing.
- M0033 remains responsible for cross-target packs and hidden-host-dependency
  removal beyond the initial host smoke.
- Runtime scheduler, allocator, panic formatting, CLI args, and standard
  library are deferred.

## Dependencies

- ADR-0020
- ADR-0040
- ADR-0043
- ADR-0046
