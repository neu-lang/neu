# Soundness Report: M0035-010 Primitive HIR Operators

## Result

Pass.

## Checks

- Logical/comparison operators cannot silently lower as arithmetic operations.
- `Not` cannot silently lower as bitwise complement.
- Unsupported MIR operation boundaries fail explicitly.
- The legacy M0028 checker does not classify logical `Not` as integer unary
  execution, preventing accidental acceptance before primitive integration.
