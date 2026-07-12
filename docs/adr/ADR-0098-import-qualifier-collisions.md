# ADR-0098: Import Qualifier Collisions

Status: Accepted

## Decision

After relative path normalization and package-header resolution, each import
has a local qualifier. An explicit `as` alias is the qualifier; otherwise the
resolved package identity is the default qualifier. Distinct packages may not
share a qualifier in one source file. The compiler reports
`import_qualifier_collision` with both import origins and suggests adding an
explicit alias.

Equivalent normalized paths resolving to the same package are one package and
do not create a collision. Explicit aliases must be unique and may not shadow
another import qualifier, a current-package qualifier, or a declaration in the
importing source. Aliases are local to the importing source file and do not
change package or module identity.

The rule applies before qualified function/type lookup and before ownership,
HIR, MIR, object emission, or linking. Wildcards, re-exports, friend packages,
and implicit import discovery remain deferred.

## Dependencies

This ADR depends on ADR-0095 and ADR-0097 and preserves ADR-0025 module
identity and ADR-0096 visibility rules.
