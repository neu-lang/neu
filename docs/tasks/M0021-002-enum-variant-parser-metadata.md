# Task: M0021-002 Enum Variant Parser Metadata

## Task Metadata

- Task ID: `M0021-002`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Parse ADR-0033 identifier-only enum variants and expose ordered metadata tied
to their enclosing enum declaration.

## Authority Extract

- `docs/SPEC.md`, “ADR-0033: Bootstrap Sealed Sums And Exhaustive Match”.
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`,
  “Decision” and “Diagnostics And Recovery”.
- `crates/newlang/src/ast.rs`: enum declaration AST conventions.
- `crates/newlang/src/parser.rs`: named-body declaration parsing and parsed
  side-table records.
- `crates/newlang/tests/parser.rs` and `crates/newlang/tests/ast.rs`.
- Validation: `cargo test -p newlang --test parser m0021_enum_variant`;
  `cargo test -p newlang --test ast`; `cargo fmt --all --check`.

## Scope

- Add an enum-variant AST node and parsed metadata record.
- Accept identifier-only variants separated by commas or semicolons.
- Preserve variant text, name span, variant node, and enclosing enum node in
  source order.
- Retain ordinary malformed parser diagnostics and recovery for unsupported
  enum-body content.

## Out Of Scope

- `when` parsing, variant resolution, duplicate semantic diagnostics,
  exhaustiveness, payloads, fields, constructors, nested declarations, generic
  enums, and type checking.

## Required Tests Before Implementation

- Enum variants separated by commas and semicolons record source-ordered
  metadata and enclosing enum identity.
- Empty enums record no variants.
- Payload-like and nested-declaration contents remain rejected or recover with
  ordinary parser diagnostics; they do not synthesize variants.

## Acceptance Criteria

- [x] Tests fail before the metadata API exists.
- [x] Only identifier-only variants are recorded.
- [x] Separators, spans, ordering, and enum identity are preserved.
- [x] No match, semantic resolution, or exhaustiveness behavior is added.
- [x] Focused tests, formatter, review, adversarial check, and CI pass.

## Execution Log

- 2026-07-10 agent=Main phase=create-task result=pass evidence=task bounded to ADR-0033 enum declaration syntax. handoff=Test-Engineer
- 2026-07-10 agent=Main phase=test-first result=fail evidence=`cargo test -p newlang --test parser m0021_enum_variants` failed because enum variant AST and parser metadata were absent. handoff=Implementer
- 2026-07-10 agent=Main phase=implementation result=pass evidence=identifier-only enum-body parser and ordered metadata added; focused parser and AST tests passed. handoff=Reviewer
- 2026-07-10 agent=Main phase=ordinary-tests result=pass evidence=focused enum tests, validator, formatting, strict clippy, and 220 workspace tests passed. handoff=Reviewer
- 2026-07-10 agent=Main phase=adversarial result=pass evidence=payload-shaped and nested declaration entries produce no variant metadata; `docs/tasks/soundness/M0021-002-soundness.md`. handoff=Reviewer
- 2026-07-10 agent=Main phase=review result=approve evidence=ADR-0033 scope and parser-only boundary verified; `docs/tasks/reviews/M0021-002-review.md`. handoff=none
