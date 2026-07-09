# ADR-0007: Error Handling

## Question

How are recoverable errors represented?

## Competing Designs

- Result types with propagation syntax.
- Checked exceptions.
- Unchecked exceptions.
- Effect-typed errors.

## Trade-offs

Result types preserve explicit control flow and avoid hidden unwinding.

Checked exceptions become noisy and brittle.

Unchecked exceptions are ergonomic but undermine systems predictability.

Effect-typed errors are powerful but add major type-system complexity.

## Recommended Choice

Result-style error values with lightweight propagation syntax; panics reserved for unrecoverable programmer faults.

## Downstream Consequences

- Standard library APIs must distinguish recoverable errors from panics.
- Destructors must run during propagation.
- Async tasks need clear error aggregation rules.

## Dependencies

- ADR-0004
- ADR-0008
- ADR-0014

