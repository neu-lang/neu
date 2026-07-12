# ADR-0095: Directory Packages And Import Aliases

Status: Accepted

## Question

How should the bootstrap compiler resolve multi-file packages and explicit
imports without making source files or host paths part of language identity?

## Competing Designs

1. Import individual source files.
2. Import directory packages whose direct source files form one namespace.
3. Require a manifest and package registry before imports can resolve.
4. Infer dependencies from unqualified references.

## Trade-offs

File imports expose implementation layout and complicate package identity.
Manifest resolution is stable but delays the useful bootstrap workflow.
Inferred dependencies are convenient but make diagnostics and cycles
non-deterministic. Directory packages provide a small, deterministic model and
leave manifests and registries for later.

## Decision

Imports resolve directories, never individual source files. The canonical form
is `import "./relative/directory"`, with an optional local alias:
`import "./relative/directory" as name`.

Every `.neu` file directly inside an imported directory belongs to that
package. Subdirectories are separate packages and are not loaded recursively.
The entry source directory is the project source root for relative resolution;
normalized imports may not escape it. Empty packages, missing directories,
file imports, malformed aliases, duplicate aliases, duplicate package identity,
header disagreement, and import cycles are deterministic diagnostics.

Package headers are optional during bootstrap. When present in one package,
all direct source files must declare the same header. When omitted, the package
identity is derived deterministically from its normalized directory path within
the virtual project. This path identity is compiler metadata, not a public
module identity or FFI name.

Files in the current package share an unqualified namespace. An imported
package is referenced only through its explicit alias or its deterministic
package qualifier when no alias is supplied. Imports do not alter visibility;
the accepted visibility policy remains ADR-0025 and its later superseding
decision.

The driver accepts an in-memory virtual source database consisting of normalized
paths and raw source strings. File IDs are assigned in deterministic path order
and source paths remain attached to diagnostics and intermediate source facts.
The compiler constructs the complete package graph before name resolution,
ownership analysis, HIR, MIR, object emission, or linking.

Cross-package calls use the existing compiler-private symbol and host-linking
contracts. No manifest, dependency download, package registry, precompiled
artifact format, dynamic loading, re-export, wildcard import, or public ABI is
introduced. Cyclic package dependencies are rejected rather than assigned
runtime semantics.

## Consequences

The frontend must retain import path and alias metadata, normalize virtual
paths, and validate the package graph before ordinary semantic analysis. The
source database becomes the authority for file IDs and paths in multi-file
compilations. Visibility enforcement is intentionally deferred to the
follow-up visibility decision.

## Dependencies

This ADR supersedes the import-syntax-only and package-loading portions of
ADR-0025 and depends on ADR-0017, ADR-0021, ADR-0022, and ADR-0025. It does not
resolve the `internal` replacement or any other visibility question reserved
for task 029.

## Required Diagnostics

The implementation must provide source-mapped diagnostics for missing or empty
packages, file imports, path traversal, malformed aliases, duplicate aliases,
header disagreement, duplicate package identity, ambiguous qualified names,
and import cycles.
