# Local patches to third-party crates

## cranelift-object 0.133.1

Upstream `cranelift-object` panics with
`Aarch64AdrPrelPgHi21 is not supported for this file format` when it writes a
Windows (COFF) object for `aarch64-pc-windows-msvc`. Every reference to a data
symbol on that target uses an `ADRP` page relocation, so the compiler could not
emit any program with data on Windows arm64.

The local copy maps the two relocations used for these references to the
standard COFF ARM64 types:

- `Aarch64AdrPrelPgHi21` -> `IMAGE_REL_ARM64_PAGEBASE_REL21`
- `Aarch64AddAbsLo12Nc` -> `IMAGE_REL_ARM64_PAGEOFFSET_12A`

COFF has no addend field for these relocations, so a non-zero addend asserts
instead of producing a wrong address. Everything else is unchanged from the
published crate (see `Cargo.toml.orig` in the upstream release).

Remove this patch when an upstream `cranelift-object` release supports the
relocations on COFF.
