# Task: M0022-005 Ownership Analysis Report

## Task Metadata

- Task ID: `M0022-005`
- Milestone: `M0022`
- Milestone File: `docs/milestones/M0022-ownership-and-move-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Provide the M0022 ownership analysis pass as a single report-producing entry
point over parsed ownership sites, resolved local bindings, and type-check
outputs.

## Authority Extract

- `docs/SPEC.md`, “ADR-0035: Bootstrap Ownership And Move Analysis”.
- `docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md`,
  “Move-State Model” and “Diagnostics And Recovery”.
- `docs/milestones/M0022-ownership-and-move-analysis.md`, completion
  checklist.
- `crates/newlang/src/ownership.rs`: value categories, transfer records, and
  use-after-move diagnostics.
- `crates/newlang/src/type_check.rs`: `DeclarationSignature` and `TypeArena`
  as type-checking outputs.

## Scope

- Add an ownership report containing transfer records and diagnostics.
- Add a single ownership-analysis entry point that runs transfer collection
  and use-after-move analysis.
- Require type-checking outputs as inputs to the entry point.
- Update the M0022 milestone checklist when acceptance criteria are satisfied.

## Out Of Scope

- Parser/type-check driver orchestration beyond the report function.
- Diagnostic rendering or snapshots.
- Branch joins, loops, calls, returns, captures, `when` ownership, borrowing,
  destructors, generic copyability, and user-declared copy.

## Required Tests Before Implementation

- The report entry point records transfer sites.
- The report entry point records use-after-move diagnostics.
- Copyable source values produce neither transfer nor diagnostic.

## Acceptance Criteria

- [x] Tests fail before the report entry point exists.
- [x] The report exposes ownership transfers.
- [x] The report exposes use-after-move diagnostics.
- [x] The entry point consumes type-check declaration signatures and `TypeArena`.
- [x] M0022 checklist reflects completed copyability, move analysis, and
  use-after-move diagnostics.

## Execution Log

- 2026-07-11 agent=Main phase=task-created result=pass evidence=M0022-002 through M0022-004 provide report inputs. handoff=Test-Engineer
- 2026-07-11 agent=Main phase=test-first result=fail evidence=cargo test -p newlang --test ownership failed with unresolved analyze_ownership import. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=OwnershipReport and analyze_ownership compose transfer and diagnostic passes after type-check inputs. handoff=Reviewer
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets. handoff=Adversarial-Engineer
- 2026-07-11 agent=Main phase=adversarial-check result=pass evidence=docs/tasks/soundness/M0022-005-soundness.md. handoff=Reviewer
- 2026-07-11 agent=Main phase=review result=approve evidence=docs/tasks/reviews/M0022-005-review.md. handoff=Commit
