# ADR-0059: Bootstrap Primitive Runtime Support

Status: Accepted

## Question

Which semantics and runtime contracts make `Bool`, `Unit`, `Float`, and `Byte`
real supported primitive values beyond the existing `Int` bootstrap subset?

## Competing Designs

1. Add only type-checking identities and defer runtime representation.
2. Represent all primitives as machine integers and emulate floating-point and
   unit behavior in compiler conventions.
3. Add explicit fixed representations and typed operations for each primitive,
   preserving exact types and rejecting implicit numeric conversion.
4. Add a boxed or runtime-tagged primitive value model.

## Trade-offs

Type-only support cannot reach HIR, MIR, Cranelift, or executable tests.
Machine-integer emulation misrepresents floating-point behavior and creates
target-dependent ABI risks. Boxing adds allocation and runtime work that the
bootstrap language explicitly does not provide.

Explicit fixed representations preserve systems-language predictability and
allow each primitive to be lowered directly while keeping the implementation
incremental.

## Recommended Choice

The bootstrap runtime supports four additional copyable primitives:

- `Bool` has values `true` and `false`, is represented as a one-byte value with
  only `0` and `1` valid, and supports logical `!`, short-circuit `&&` and
  `||`, and equality/inequality. Logical operators preserve left-to-right
  short-circuit evaluation.
- `Unit` has exactly one value written as `()`. It has no runtime payload,
  consumes no storage in local slots, and functions returning `Unit` have no
  ABI return value. Unit expressions are limited to the unit literal, unit
  locals, unit parameters, calls, and explicit returns.
- `Float` is IEEE 754 binary64 (`f64`). Decimal and exponent literals are
  accepted as `Float` literals; integer literals do not implicitly convert.
  Arithmetic `+`, `-`, `*`, `/`, unary `+` and unary `-`, comparisons, and
  equality use IEEE 754 behavior. Division by zero and NaN do not trap; NaN is
  unequal to every value, including itself, and ordered comparisons with NaN
  are false.
- `Byte` is an unsigned eight-bit value in `0..255`. Integer literals may
  initialize `Byte` only when the literal is in range and the expected type is
  `Byte`; there is no implicit `Int`/`Byte` conversion. Equality, bitwise
  `&`, `|`, `^`, shifts with counts in `0..7`, and checked `+`, `-`, `*`, `/`,
  and `%` are supported. Byte overflow, division by zero, and invalid shifts
  are compile-time diagnostics when provable and runtime traps otherwise.

Primitive operations require exact operand types. There is no implicit numeric
promotion or conversion. `Bool`, `Unit`, `Float`, and `Byte` are copyable and
do not introduce allocation, destruction, standard-library, or FFI behavior.

The accepted executable subset remains unchanged for entry points: `main`
continues to return `Int`. Additional primitive values are supported in helper
functions, locals, parameters, returns, calls, and backend smoke tests. A
future ADR may expand control-flow or conversion facilities without changing
these representations.

## ABI And Intermediate Representation Contract

- `Bool` lowers to Cranelift `i8` and must be normalized to `0` or `1` at every
  value-producing boundary.
- `Byte` lowers to Cranelift `i8` with unsigned operation semantics.
- `Float` lowers to Cranelift `f64`.
- `Unit` has no value-producing ABI result; HIR and MIR preserve its type and
  source mapping while lowering calls and returns without a result operand.

HIR preserves literal kind/value, exact primitive type, source span, operand
  order, and existing ownership/safety facts. MIR preserves typed constants,
  local slots, temporaries, primitive operations, traps where applicable, and
  unit no-result boundaries. Backend lowering rejects unsupported primitive
  combinations rather than reinterpreting values.

## Diagnostics

Diagnostics include `float_literal_malformed`, `float_literal_non_finite` when
the accepted literal syntax cannot represent a finite source literal,
`byte_literal_out_of_range`, `primitive_type_mismatch`,
`byte_overflow`, `byte_division_by_zero`, and `byte_invalid_shift_count`.
Existing integer diagnostics remain unchanged. Runtime float NaN/infinity
behavior is not diagnosed as an arithmetic failure.

## Downstream Consequences

- Lexer and parser must preserve decimal/exponent literals and unit literals.
- Type checking must represent all four primitive identities and exact
  operation rules.
- HIR, MIR, and Cranelift must carry and lower non-`Int` primitive values.
- Target packs must validate that the selected target supports the required
  `i8` and `f64` bootstrap representations.
- Examples and executable smoke tests continue to use `Int`-returning `main`
  while exercising additional primitive helper functions.

## Dependencies

- ADR-0027
- ADR-0042
- ADR-0043
- ADR-0044
- ADR-0045
- ADR-0046
- ADR-0058
