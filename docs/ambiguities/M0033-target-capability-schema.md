# Ambiguity Report: M0033 Target Capability Schema

## Metadata

- Report ID: `M0033-TARGET-CAPABILITY-SCHEMA`
- Related Task: `M0033-002`
- Related Milestone: `M0033`
- Filed By: `main-task architecture review`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md` requires
  platform capability declarations per target.
- `docs/adr/ADR-0057-bootstrap-target-pack-linker-contract.md` defines the
  bootstrap manifest fields but does not define capability fields or their
  validation policy.
- `docs/milestones/M0033-target-packs-and-cross-compilation-smoke.md` requires
  target capabilities to be read from bundled metadata.

## Exact Ambiguous Text Or Missing Rule

```text
The source of truth requires per-target declarations for integer widths,
pointer sizes, alignment, atomics, calling conventions, and platform APIs, but
does not define the target-pack capability schema, whether values are derived
from the target triple or stored in the manifest, or which compiler phase
consumes and validates those declarations.
```

## Competing Interpretations

1. Store all capability values in each target-pack manifest and validate them
   against backend and ABI requirements.
2. Derive capabilities entirely from `target_lexicon` and use target-pack
   metadata only for artifacts.
3. Store a small explicit capability profile in the pack and defer unsupported
   capability categories until their language features are implemented.
4. Use a central compiler capability table keyed by target triple, with packs
   declaring only a profile identifier.

## Trade-offs

- Fully manifest-owned values are reproducible but duplicate ABI knowledge and
  require validation rules immediately.
- Triple-derived values reduce metadata but hide pack-specific ABI and runtime
  constraints.
- A profile is compact and incremental but requires a stable profile identity
  and a defined compatibility contract.
- A central table is simple for the compiler but weakens the self-contained
  target-pack guarantee.

## Why Guessing Is Unsafe

- Integer layout and calling-convention assumptions affect type checking,
  ownership-relevant layout, ABI lowering, and object compatibility.
- A target pack that builds with silently inferred capabilities violates
  ADR-0020's no-hidden-target-dependency requirement.
- Different capability interpretations could make the same source compile to
  incompatible binaries without a diagnostic.

## Affected Work

- Tasks blocked: `M0033-002` and any task adding a non-host target pack.
- Milestones affected: `M0033` and later cross-target release hardening.
- Implementation areas affected: target-pack manifest, target selection,
  backend target configuration, and future target-aware diagnostics.

## Required Resolution

- Decide the authoritative owner of capability values.
- Define the minimum bootstrap capability fields and their types.
- Define validation and diagnostic behavior for unsupported or inconsistent
  values.
- Update the accepted ADR/specification before implementation relies on the
  decision.

## Temporary Rule

Do not add a non-host target pack or infer target capabilities from an
unaccepted source. The explicit target-pack registry may resolve existing packs
without claiming that cross-target capability semantics are complete.

## Resolution

ADR-0058 accepts an explicit typed bootstrap capability profile in each target
pack. It defines the bootstrap values, represents unsupported capability
categories as `deferred`, and requires target-pack validation without host
inference. `docs/SPEC.md` now records the accepted rule.

- Date resolved: `2026-07-11`
- Resolved by: delegated main-task Chief Architect decision
- Implementation task unblocked: `M0033-002`
