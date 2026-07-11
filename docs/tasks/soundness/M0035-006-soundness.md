# Soundness Report: M0035-006 Primitive Cranelift Lowering

## Result

Pass.

## Checks

- Unit does not produce an ABI result or fabricated value.
- Float uses the target backend's f64 representation without integer
  reinterpretation.
- Bool and Byte use distinct i8 constant paths.
- Unsupported runtime types remain rejected.
