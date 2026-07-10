# Task: M0021-004 Enum Variant Identity Records

## Task Metadata

- Task ID: `M0021-004`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Build deterministic metadata records for each parser-accepted bootstrap enum
variant, preserving its declaring enum, module/package identity, spelling, and
source order. This is an analysis input only; it does not yet resolve `when`
arms or issue ADR-0033 diagnostics.

## Authority Extract

- `docs/SPEC.md`, “ADR-0033: Bootstrap Sealed Sums And Exhaustive Match”.
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`,
  “Decision” and “Diagnostics And Recovery”.
- `crates/newlang/src/parser.rs`: `ParsedEnumVariant`.
- `crates/newlang/src/module.rs`: module/package metadata conventions.
- `crates/newlang/src/name_resolution.rs`: stable source-ordered index patterns.
- Validation: `cargo test -p newlang --test name_resolution m0021_enum_variant_identity`;
  `cargo fmt --all --check`; `git diff --check`.

## Scope

- Add an analysis-side enum variant identity record and source-ordered index.
- Build records only from parser-accepted enum variant metadata and module
  metadata.
- Preserve the declaring enum node, variant node, interned spelling, and
  module/package identity required by later resolution.

## Out Of Scope

- `duplicate_enum_variant`, type records, matching, variant lookup from a
  `when` arm, subject validation, exhaustive coverage, and all new diagnostics.

## Required Tests Before Implementation

- Two enums with like-named variants retain distinct enum identities.
- Variant records preserve parser source order and node identity.
- Empty parser metadata produces an empty index.

## Acceptance Criteria

- [x] Tests fail before the index API exists.
- [x] Every record is traceable to one parser-accepted variant and module/package.
- [x] The index does not impose duplicate or match semantics.
- [x] Focused tests, formatter, review, adversarial check, and CI pass.

## Execution Log

- 2026-07-10 agent=Main phase=create-task result=pass evidence=identity metadata isolated from ADR-0033 diagnostics and coverage. handoff=Test-Engineer
- 2026-07-10 agent=Main phase=test-first result=fail evidence=`cargo test -p newlang --test name_resolution m0021_enum_variant_identity` failed because `build_enum_variant_index` was absent. handoff=Implementer
- 2026-07-10 agent=Main phase=implementation result=pass evidence=source-ordered records preserve enum/variant nodes, symbols, module, and package without diagnostics. handoff=Reviewer
- 2026-07-10 agent=Main phase=ordinary-tests result=pass evidence=focused identity tests, formatter, strict clippy, and full workspace tests passed. handoff=Adversarial-Engineer
- 2026-07-10 agent=Main phase=adversarial-check result=pass evidence=like-named variants remain separated by declaring enum; no safety or semantic enforcement is bypassed. handoff=Reviewer
- 2026-07-10 agent=Main phase=review result=approve evidence=ADR-0033 and M0021 parser-analysis scope verified; required validations passed. handoff=none
