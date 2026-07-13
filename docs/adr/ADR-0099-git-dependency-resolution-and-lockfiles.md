# ADR-0099: Git Dependency Resolution And Lockfiles

Status: Accepted

## Decision

Neu supports `https://` Git dependencies with a non-empty tag. Authentication,
redirects, and credentials are delegated to the configured Git client; other
URL schemes, branches, archives, registries, and version ranges are rejected.

The cache root is `NEU_PATH` when set, otherwise `$HOME/.neu`. A repository URL
is stored below `pkg/<host>/<owner>/<repository>`; this path is storage only.
The dependency manifest at repository root supplies module identity through its
`name`. Git resolution never executes hooks, build scripts, or repository
programs.

Tags resolve to full commit hashes. A dependency repository is checked out at
the resolved commit in a detached immutable working tree. Recursive manifests
are resolved depth-first in deterministic URL/module order. Cycles, duplicate
module identities, conflicting URLs/tags, missing manifests, submodules,
symlink escapes, and unsupported dependency types are diagnostics.

When dependencies exist, the project root owns `neu.lock.json`. It contains one
sorted entry per resolved module with module name, URL, type, requested tag,
and full commit. Existing lock entries must match; moved tags never silently
change a locked build. Regeneration is an explicit resolver operation, and
lockfiles are written atomically only after the complete graph succeeds.

Offline resolution uses only cached repositories and locked commits; missing
objects are errors. Cache identity is independent of host linking because
source resolution is target-neutral.

Source imports may name a declared dependency with its repository URL followed
by a package subpath, for example `import "github.com/example/lib/core"`.
The resolver maps that stable URL-qualified name to the locked checkout and
selected package sources; cache paths never become package identity. Only
declared dependencies are eligible, and missing, escaping, ambiguous, or
otherwise inaccessible package paths are deterministic diagnostics. Explicit
local aliases use the same qualifier-collision rules as relative imports.

No registry, archive, binary artifact, submodule, branch dependency, feature
solver, package manager, or automatic update command is introduced.

## Dependencies

This ADR depends on ADR-0097 and ADR-0095 and supplies the resolver boundary
used by the project driver. It preserves manifest module identity, package
qualifiers, visibility, host-linking, and raw-source contracts.
