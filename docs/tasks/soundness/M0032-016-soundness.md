# Soundness Report: M0032-016

Decision: pass.

The startup shim cannot resolve the language entry unless the compiler emits
the selected entry symbol with external linkage; the smoke proves that exact
boundary. Unsupported exit values trap through the pack-defined status, and
the pack linker path is LFS-backed and explicit rather than a host fallback.
