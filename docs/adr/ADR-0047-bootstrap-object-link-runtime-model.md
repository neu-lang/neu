# ADR-0047: Bootstrap Object Link Runtime Model

Status: Superseded by ADR-0100

Superseded in part by ADR-0127: the `override` keyword is removed, and
replacements of inherited methods need no marker.

The first runnable program requires no Neu standard library and preserves the
compiler-owned runtime boundary for entry, exit status, and bootstrap traps.
ADR-0100 replaces the former bundled-startup design: Cranelift emits a host
object and the host system linker produces the executable. `NEU_LINKER` may
override the linker command. Non-host targets are rejected before lowering.

Printing, allocation APIs, CLI arguments, scheduling, panic formatting,
exceptions, FFI helpers, and platform APIs remain outside this contract.
