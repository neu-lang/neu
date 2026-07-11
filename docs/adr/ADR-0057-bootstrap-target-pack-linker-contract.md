# ADR-0057: Bootstrap Target-Pack Linker Contract

Status: Accepted

## Question

What concrete target-pack contract lets M0032 link the bootstrap host object
without silently depending on the host `PATH`?

## Competing Designs

1. Invoke `ld` or `clang` discovered from the host environment.
2. Ship a pinned `lld` linker and target-specific startup shim in each target
   pack.
3. Implement a Neu-owned native linker and startup writer.
4. Defer linking and release object files only.

## Trade-offs

Host discovery is easy locally but contradicts ADR-0020 and makes builds
environment-dependent. A Neu-owned linker avoids external artifacts but creates
large platform-format maintenance before target packs exist. Object-only output
cannot satisfy ADR-0047's runnable smoke requirement.

A pack-owned `lld` artifact plus a pack-owned startup object keeps linker
behavior versioned with the target, preserves a small compiler integration
surface, and leaves cross-target packaging to M0033.

## Recommended Choice

The initial host target pack owns a pinned, executable `lld` linker artifact and
a target-specific startup-shim object. A pack manifest identifies the exact
target triple, native object/executable formats, linker path, startup-shim
path, platform entry symbol, canonical language-entry symbol, and the
test-visible non-success trap status. The paths are relative to the pack root;
the resolver rejects absolute paths, traversal, missing artifacts, and target
mismatches.

The compiler receives an explicit target-pack root and never searches `PATH` or
falls back to `ld`, `clang`, or another host tool. Link invocation and argument
construction consume only the validated manifest and generated object inputs.
The exact manifest serialization and Rust API are implementation details, but
the logical fields and no-fallback rule are normative.

The startup shim is not a standard library. It calls the canonical symbol for
the selected language `main`, maps a returned `Int` in `0..255` to the process
exit status, and exits unsuccessfully for bootstrap traps or an unsupported
exit value. The selected entry object may retain its ADR-0056 identity symbol
and additionally expose the canonical entry symbol required by the pack. The
shim provides no printing, allocation, scheduling, CLI arguments, or panic
formatting.

M0032 covers one current-host pack. M0033 covers additional target packs,
cross-target distribution, and removal of any host-specific assumptions.

## Downstream Consequences

- M0032 can implement a deterministic pack resolver and link adapter without a
  host-tool fallback.
- M0032 must provide or consume the canonical language-entry symbol and test
  the shim's exit/trap behavior.
- Target-pack artifact distribution and licensing must be recorded with each
  pack.
- Public ABI, FFI symbol policy, and cross-target layout remain deferred.

## Dependencies

- ADR-0020
- ADR-0040
- ADR-0043
- ADR-0046
- ADR-0047
- ADR-0056
