# ADR-0077: Value-Producing Conditional Expressions

Status: Accepted by the main-task Chief Architect

## Question

How should Neu's existing `if` syntax produce a value while retaining the
statement form, branch-local flow facts, and compiler-private CFG ABI?

## Competing Designs

1. Require a `Bool` condition and an `else` branch, require exact branch result
   types, and lower the expression to two branches joined by one typed result.
2. Allow truthy conditions and infer a common branch supertype.
3. Treat the first branch as the value and make `else` optional.
4. Add a separate ternary syntax with eager branch evaluation.

Design 1 preserves explicit Kotlin-like control flow and makes ownership,
nullability, and diagnostics deterministic. Designs 2 and 3 hide missing
values or conversions; design 4 duplicates syntax and violates branch
short-circuiting.

## Decision

An `if` is a value-producing conditional when it appears in an expression
position. Its condition must have exact type `Bool`; Neu has no truthiness
conversion. A value-producing conditional requires an `else` branch. The
existing statement form remains valid and does not require `else`.

The condition is evaluated exactly once, before either branch. Exactly one
branch executes. Branch blocks use the existing statement grammar and must
produce a final value expression in value context. The final expression is
evaluated as the branch result; a branch that ends in `return`, `break`, or
`continue` is terminating and contributes no result. A value conditional is
valid only when every reachable non-terminating branch produces a result and
at least one result-producing branch exists. Missing results and invalid
terminators are diagnostics. Expression-level `else if` has no separate
syntax; nested conditionals may be used where an expression is accepted.

Both reachable result branches must have the same exact type identity. No
common-supertype search, numeric conversion, boxing, nullable widening, or
class/interface conversion is performed for the result. `Unit` is valid only
when every reachable result branch has exact type `Unit`. The result can be
used in local initializers, assignments, returns, call arguments, indexing
expressions, and constructor arguments wherever the existing expression
grammar accepts it.

Branch-local nullability refinements are scoped to that branch and do not
become an unconditional refinement after the join. Ownership and definite
initialization states are joined conservatively: a move-only binding is
available after the expression only if every reachable branch leaves it
available, and a local is initialized after the expression only if every
reachable branch initializes it. Shared borrows end at the branch boundary
under existing borrow rules. Consuming an owned value in one branch and not
the other therefore produces a possible-use-after-consumption diagnostic at a
later use unless the value is restored by an accepted atomic rebind.

HIR records the condition, true and false branch identities, result type,
branch result mappings, reachability, cleanup boundaries, source spans, and
ownership/effect join facts. MIR represents the condition, branch blocks, one
typed result temporary, and a join block. The backend lowers these facts to
ordinary Cranelift CFG and existing value ABI forms; it does not evaluate both
branches or invent a runtime service. Aggregate and owned results use their
existing accepted ABI and cleanup contracts. `main(): Int` is unchanged.

Diagnostics identify the conditional or offending branch for non-`Bool`
conditions, missing `else`, incompatible results, missing branch results,
invalid terminators, incomplete initialization, and branch-dependent
consumption. A conditional that fails these checks never reaches HIR, MIR,
Cranelift, object emission, or linking.

Coroutines, suspension, value-carrying loop control, `while`, exceptions,
pattern matching changes, lambdas, closures, implicit conversions, new borrow
syntax, and public ABI changes remain deferred.

## Supersession

This ADR supersedes only the value-producing conditional deferrals in
ADR-0024, ADR-0045, ADR-0053, and ADR-0060. It preserves the existing statement
conditional, flow-typing, ownership, loop, diagnostics, and target-pack rules.

## Dependencies

ADR-0024, ADR-0028, ADR-0045, ADR-0053, ADR-0060, ADR-0062, ADR-0075,
and ADR-0076.
