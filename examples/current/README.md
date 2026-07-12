# Current Language Examples

Only source that fits the current end-to-end Cranelift bootstrap boundary is
kept here. Every example has one top-level `main` with no parameters and an
`Int` return, and requires no runtime library, printing, or linker runtime
features. The examples cover integer arithmetic, structured control flow,
classes, class methods, interfaces, and the accepted `Bool`, `Unit`, `Float`,
and `Byte` primitive runtime forms, plus mutable `var` initialization and
reassignment.

Parser-only, type-checker-only, ownership, pattern, nullable, and unsupported
syntax examples are intentionally omitted until those forms are accepted by
the full backend and executable pipeline.

The `packages/` directory is a virtual project example: `main.neu` imports the
sibling `math` directory with an explicit alias, and every direct `.neu` file
in that directory belongs to the package.
