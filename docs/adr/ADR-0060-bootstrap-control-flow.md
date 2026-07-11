# ADR-0060: Bootstrap Control Flow

Status: Accepted

## Question

Which control-flow forms are supported by the first source-to-executable
pipeline, while keeping `while` unsupported?

## Decision

The bootstrap executable subset accepts:

- `if (condition) { ... } else { ... }` as a control-flow statement;
- `if (condition) { ... }` in a statement position, with `Unit` behavior;
- Boolean conditions only;
- value-producing conditional expressions remain deferred;
- `for (name in start..end) { ... }` over inclusive `Int` ranges;
- an immutable `Int` loop binding scoped to the loop body;
- `break` to leave the innermost `for`; and
- `continue` to advance the innermost `for`;
- structured loop back-edges with deterministic local cleanup.

`for` ranges use a unit step. `break` and `continue` are statements only, apply
to the innermost `for`, and do not carry values or labels. `while` remains
unsupported and must produce the existing unsupported-statement diagnostic. No
printing, standard library, scheduler, or new runtime service is introduced by
this decision.

Branches and loop bodies are ownership and borrowing regions. Values must be
initialized on every path before use, and borrows may not cross a loop back-edge
unless the existing ownership and borrow rules prove that they remain valid.

## Consequences

The parser records conditional and range-loop structure. HIR preserves branch,
loop, source-span, and safety facts. MIR represents branch joins, loop headers,
back-edges, exits, and cleanup boundaries. Cranelift lowers these CFGs directly.

## Dependencies

- ADR-0024
- ADR-0027
- ADR-0035
- ADR-0036
- ADR-0044
- ADR-0045
- ADR-0050
