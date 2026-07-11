# Ambiguity Report: M0032 Bundled Linker Contract

## Metadata

- Report ID: `M0032-BUNDLED-LINKER-CONTRACT`
- Related Task: `M0032-003`
- Related Milestone: `M0032`
- Filed By: `main-task architecture review`
- Date: `2026-07-11`
- Status: `open`
- Required Owner: `main-task semantic design and build architecture`

## Ambiguous Or Missing Authority

- `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
- `docs/adr/ADR-0046-bootstrap-abi-and-calling-convention.md`
- `docs/adr/ADR-0047-bootstrap-object-link-runtime-model.md`
- `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0047 requires M0032 to use a planned bundled linker path for the initial
host target and permits a tiny bootstrap runtime shim, but it does not define
which linker artifact a target pack owns, how that artifact is located or
invoked, what startup-shim object is supplied, or how the initial host target
maps language main and runtime traps to the process.
```

## Competing Designs

1. Invoke the host `ld` or `clang` driver.
   - Simple locally, but directly violates ADR-0020's no-hidden-host-tool
     direction and is not a bundled target pack.
2. Bundle a pinned `lld` executable plus a target-specific startup shim in the
   target pack.
   - Matches the portability direction, but requires accepted artifact,
     licensing, distribution, and target-pack layout decisions.
3. Implement a Neu-owned native linker and startup writer.
   - Avoids external linker distribution, but creates a large new systems
     component and platform-format maintenance burden before M0033.
4. Stop at object files and defer linking.
   - Reduces immediate work, but fails ADR-0047 and the M0032 executable smoke
     acceptance criterion.

## Why Guessing Is Unsafe

- Falling back to a host linker would produce a false claim of bundled
  cross-compilation support.
- A linker choice changes target-pack contents, build reproducibility, license
  obligations, and the executable entry contract.
- A startup shim cannot be invented from the language `main` ABI alone because
  platform entry and trap behavior are target-specific.
- Object emission is now available, so the unresolved boundary is isolated and
  should remain explicit rather than leaking host assumptions into the
  compiler.

## Affected Work

- Blocked task: `M0032-003` and all executable-link tasks after it.
- Affected milestone gates: bundled linker path and hidden-host dependency.
- Affected files: linker integration, target-pack manifest, startup shim, and
  executable smoke harness.

## Required Resolution

An accepted ADR or SPEC revision must define:

1. the initial host target identity and native object/executable formats;
2. the linker artifact and version ownership of the target pack;
3. how the compiler locates and invokes that artifact without host `PATH`
   fallback;
4. the startup-shim object/input and language-main symbol contract;
5. mapping of `Int` return values in `0..255` to process exit status; and
6. process failure behavior and test-visible identity for bootstrap traps.

No executable-link implementation may begin until these requirements are
accepted. The object emitter remains valid and independently testable.
