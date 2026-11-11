# ADR-0124: Compile-Time Precondition Violations

Status: Accepted; decides the compile-time follow-up of ADR-0123

## Question

Should the compiler diagnose, at compile time, a call whose precondition fails
for constant arguments, such as `clamp(3, 5, 0)` under ADR-0112?

## Decision

A call is diagnosed as `precondition_violation`, at the call expression, when all
of the following hold:

- The callee's first statement is `precondition(condition, message)`. No
  statement starts before it in the callee's body.
- The call is a plain call. Member calls are not decided.
- The condition uses only the callee's parameters, and the arguments it uses are
  constants: integer literals, `Bool` literals, and negated, grouped, or
  arithmetic combinations of them. Arithmetic that overflows is not decided.
- The condition, evaluated with those constants, is `false`.

Any other call is not diagnosed at compile time, and the run-time check of
ADR-0123 applies. The compile-time diagnostic does not replace that check.

## Consequences

An invalid `clamp` range with constant bounds is a compile-time error. With bounds
computed at run time it traps, as ADR-0123 specifies.

## Required follow-up

Preconditions after another statement, preconditions over non-parameter names,
and member-call preconditions remain run-time checks until a separate decision
covers them.
