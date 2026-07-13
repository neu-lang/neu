# ADR-0097: Neu Project Manifest

Status: Accepted

## Question

How does a project identify its source root, entrypoint, target-independent
source set, and declared dependencies before compilation?

## Decision

The project root is the directory containing `neu.json`. A manifest may be
found at an explicit file path, in the current directory, or by searching
parent directories. The first discovered manifest wins; an explicit path must
name a file or an unambiguous project directory.

The user-facing manifest fields are `name`, optional `description`,
`entrypoint`, `srcs`, and `dependencies`. `name`, `entrypoint`, and `srcs` are
required. Unknown fields, duplicate keys, malformed JSON, wrong field types,
empty source patterns, and invalid paths are diagnostics with manifest
locations. `name` is the module identity and follows the existing dotted
identifier rules. `description` is informational metadata.

`entrypoint` is a normalized relative `.neu` path and must be in the expanded
authoritative `srcs` allowlist. Patterns support `*`, `**`, and `?`, use `/`
separators, match `.neu` files only, exclude hidden path components, reject
absolute paths and `..` escapes, and produce sorted, deduplicated paths.
Symlinks may not escape the manifest root. Empty matches are diagnostics.

The selected source set is loaded into the compiler's virtual source graph. A
directory package is loaded only when its direct files are selected by the
manifest or by an authorized dependency source set. Import paths remain
relative to source files and package identities remain distinct from the
manifest module name.

Dependencies are descriptor metadata at this stage. A dependency has `url`,
optional `type` defaulting to `git`, and `tag`; only `git` is accepted. Git
resolution, recursive manifests, `NEU_PATH`, and `neu.lock.json` belong to the
dependency-resolver boundary. A dependency repository must expose its own root
manifest and uses that manifest's `name` as module identity.

The project driver selects the host-only output and system-linker pipeline.
Non-host targets are rejected before compilation. Raw-source compilation remains available as a library API. No
registry, archive, binary artifact, workspace manifest, build script, or
conditional compilation semantics is introduced.

## Consequences

Manifest paths and expanded source paths must be deterministic and retained in
source diagnostics and graph metadata. Manifest validation is separate from
Git cache/lockfile resolution and from language semantics.

## Dependencies

This ADR supersedes the project-discovery and source-set portions of ADR-0025
and ADR-0095 and depends on ADR-0020, ADR-0025, ADR-0095, and the accepted
host-linking and entrypoint contracts. ADR-0098 will define qualifier collision
diagnostics; the dependency resolver defines Git resolution and lockfiles.
