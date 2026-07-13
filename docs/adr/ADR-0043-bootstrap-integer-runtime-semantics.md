# ADR-0043: Bootstrap Integer Runtime Semantics

Status: Accepted

## Question

What runtime meaning does `Int` have for the first executable subset?

## Competing Designs

1. Fixed signed 64-bit `Int`.
2. Target-pointer-sized `Int`.
3. Arbitrary precision `Int`.
4. Leave integer representation to the backend.

## Trade-offs

A fixed signed 64-bit representation is portable and simple for HIR, MIR, and
Cranelift lowering.

Pointer-sized integers are common in systems work, but would make the first
smoke result target-dependent.

Arbitrary precision requires allocation and runtime support.

Backend-defined representation would violate source-of-truth semantics.

## Recommended Choice

For the bootstrap executable subset, `Int` is a signed 64-bit two's-complement
integer with value range `-9223372036854775808..9223372036854775807`.

Integer literals in executable code must fit this range. A literal outside the
range reports `integer_literal_out_of_range`.

Unary `+` on `Int` evaluates its operand and returns that value. Unary `-` on
`Int` evaluates its operand and produces the arithmetic negation. Negating
`-9223372036854775808` overflows. Unary `~` on `Int` evaluates its operand and
produces the bitwise complement of the two's-complement representation.

Binary arithmetic operations `+`, `-`, `*`, `/`, `%`, and `**` on `Int`
evaluate the left operand before the right operand and produce `Int`.
Division truncates toward zero. Remainder has the same sign as the dividend
and satisfies `a == (a / b) * b + (a % b)` when `b != 0`.

Exponentiation `a ** b` requires a non-negative exponent `b`. Negative
exponents report `negative_exponent` when statically known and trap at runtime
otherwise. Exponentiation overflow follows the same overflow rules as other
arithmetic.

Bitwise operations `&`, `|`, and `^` operate on the signed 64-bit
two's-complement representation and produce `Int`.

Shift operations `<<` and `>>` operate on the signed 64-bit two's-complement
representation and produce `Int`. The right operand is the shift count. A shift
count outside `0..63` reports `invalid_shift_count` when statically known and
traps at runtime otherwise. `>>` is arithmetic right shift.

If the compiler can prove overflow for a constant expression in the bootstrap
subset, it reports `integer_overflow`. Division or modulo by a statically known
zero reports `division_by_zero`. Runtime overflow, runtime division/modulo by
zero, runtime negative exponent, or runtime invalid shift count traps. Trap
presentation is backend/runtime-defined by the implementation, but it must not
silently wrap or continue.

The first executable smoke test must avoid overflow and division/modulo by
zero, negative exponent, and invalid shift counts. Wrapping, saturating,
unchecked arithmetic operators, numeric casts, unsigned integers, fixed-width
integer aliases, floats, machine-word integers, rotate operations, and
population-count or bit-scan intrinsics are deferred.

## Downstream Consequences

- HIR and MIR may use a stable `Int64` runtime type for bootstrap `Int`.
- Cranelift lowering must emit checked or trapping arithmetic, exponentiation,
  bitwise, and shift behavior for the executable subset.
- The compiler can treat a small non-negative `Int` from `main` as the process exit
  code through ADR-0047.

## Dependencies

- ADR-0021
- ADR-0024
- ADR-0027
- ADR-0035
- ADR-0047
