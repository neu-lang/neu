# ADR-0044: Bootstrap HIR Runtime Contract

Status: Accepted

## Question

What runtime-relevant facts must HIR preserve for the first executable subset?

## Competing Designs

1. Source-shaped typed HIR with semantic side-table references.
2. Desugared HIR with early control-flow lowering.
3. AST reuse as HIR.
4. Backend-specific HIR.

## Trade-offs

Source-shaped typed HIR preserves diagnostics and semantic facts while giving
MIR a cleaner input than raw AST.

Early control-flow lowering belongs in MIR for the current architecture.

AST reuse blurs representation boundaries and risks carrying parser recovery
nodes into backend work.

Backend-specific HIR ties the frontend to Cranelift too early.

## Recommended Choice

Bootstrap HIR is a typed, source-mapped, backend-independent representation of
the accepted executable subset. It must preserve:

- function identity, package/module identity, and `main` entry classification;
- parameter order, parameter types, and return type;
- local binding identity, mutability, declared or inferred type, and source
  span;
- expression type for every HIR expression;
- direct callee identity for call expressions;
- left-to-right argument and operand order;
- explicit return statements and their returned expression;
- source spans for diagnostics on functions, parameters, locals, expressions,
  calls, operators, and returns;
- ownership/move facts needed to confirm executable-subset values are valid;
- borrow, lifetime, thread, coroutine, unsafe, and FFI diagnostics already
  produced or proven absent for the executable subset; and
- unsupported-form markers for parsed constructs excluded by ADR-0042.

HIR must not invent runtime semantics. It may reject unchecked, unresolved, or
unsupported AST input rather than represent it as executable HIR.

## Downstream Consequences

- M0029 acceptance criteria must include preservation of executable-subset
  runtime facts.
- MIR lowering can depend on typed callee, local, return, and source-mapping
  information.
- Diagnostic source mapping remains available after lowering.

## Dependencies

- ADR-0015
- ADR-0025
- ADR-0026
- ADR-0027
- ADR-0035
- ADR-0036
- ADR-0037
- ADR-0038
- ADR-0039
- ADR-0040
- ADR-0041
- ADR-0042
- ADR-0043
