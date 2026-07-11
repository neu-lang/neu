# ADR-0058: Bootstrap Target Capability Profile

Status: Accepted

## Question

How must bundled target packs declare the target capabilities that are relevant
to compilation without silently deriving ABI facts from the host?

## Competing Designs

1. Store all target capability values in each pack manifest and validate them
   against every compiler subsystem.
2. Derive capabilities entirely from `target_lexicon` and keep manifests
   limited to linker artifacts.
3. Store an explicit, typed bootstrap capability profile in each pack, with
   unsupported capability categories represented as deferred rather than
   inferred.
4. Store only a profile identifier and keep the values in a central compiler
   table.

## Trade-offs

Fully manifest-owned ABI data is reproducible but duplicates knowledge across
compiler and pack boundaries. Triple-derived data hides pack-specific policy
and violates the no-hidden-host-dependency direction. A profile identifier is
compact but makes the pack dependent on a compiler-side table.

An explicit typed bootstrap profile keeps the selected pack authoritative,
supports incremental target support, and makes unsupported capability
categories observable without pretending they are available.

## Recommended Choice

Each target-pack manifest declares a `[capabilities]` table with these fields:

- `int_width_bits`: the runtime width of bootstrap `Int`;
- `pointer_width_bits`: the target pointer width;
- `endianness`: `little` or `big`;
- `alignment_model`: a declared model or `deferred` when the current compiler
  does not expose layout-dependent forms;
- `calling_convention`: `platform-default` for the bootstrap ABI, with no
  stable public ABI implied;
- `atomic_model`: `deferred` until atomic language and runtime semantics are
  accepted; and
- `platform_apis`: an explicit list, empty for the no-stdlib bootstrap.

The compiler validates the profile against the accepted bootstrap contract:
`Int` is signed 64-bit, the initial host profile is 64-bit little-endian,
`platform-default` is the only bootstrap calling convention, deferred models
cannot be consumed by executable language forms, and an empty platform API
list means no platform API is available. Values are never inferred from the
host or silently substituted from `target_lexicon`.

The profile is target-pack metadata, not a new source-language feature. Future
target packs may provide non-deferred values only after the corresponding ABI,
layout, atomic, or platform API semantics are accepted.

## Downstream Consequences

- Target-pack resolution must parse and validate capability metadata.
- Backend target selection consumes the validated profile rather than host
  defaults for supported bootstrap facts.
- Cross-target packs remain independently reviewable and must declare their
  own profiles.
- Layout, atomics, and platform APIs remain unavailable where the profile says
  `deferred` or lists no APIs.

## Dependencies

- ADR-0020
- ADR-0043
- ADR-0046
- ADR-0057
