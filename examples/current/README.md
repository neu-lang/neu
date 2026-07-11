# Current Language Examples

Only source that fits the current end-to-end Cranelift bootstrap boundary is
kept here. The example has one top-level `main`, no parameters, an `Int`
return, one block, and an expression composed of integer literals and checked
arithmetic. It requires no locals, calls, runtime library, printing, or linker
runtime features.

Parser-only, type-checker-only, ownership, pattern, nullable, and unsupported
syntax examples are intentionally omitted until those forms are accepted by
the full backend and executable pipeline.
