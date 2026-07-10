# Soundness Report: M0019-010

## Metadata

- Task ID: `M0019-010`
- Milestone: `M0019`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-010-branch-refinement-records.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
  - `docs/tests/m0019-branch-refinement-records.sh`
- Ordinary test results:
  - `cargo test -p newlang --test type_check m0019_branch_refinement`: pass
  - `cargo test -p newlang --test type_check`: pass
  - `sh docs/tests/m0019-branch-refinement-records.sh`: pass

## Safety Invariants Checked

- [x] `!= null` eligible tests attach only to the then branch block.
- [x] `== null` eligible tests attach only to the else branch block.
- [x] Missing else branches do not create else refinements.
- [x] Eligible tests not used as an `if` condition do not create branch refinements.
- [x] Branch attachment uses parser node identity, not source text.
- [x] This slice does not create per-use refined expression types or nullable-use diagnostics.

## Attacks Attempted

```text
Attack: Eligible `!= null` test with both then and else blocks.
Expected result: Refinement region is the then block only.
Actual result: Refinement region is the then block.
Source of truth: ADR-0028 branch region boundaries.
Outcome: pass
```

```text
Attack: Eligible `== null` test with both then and else blocks.
Expected result: Refinement region is the else block only.
Actual result: Refinement region is the else block.
Source of truth: ADR-0028 branch region boundaries.
Outcome: pass
```

```text
Attack: Eligible `== null` test in an `if` without an else branch.
Expected result: No refinement.
Actual result: No refinement.
Source of truth: ADR-0028 says equality refinements apply in the else branch when an else branch exists.
Outcome: pass
```

```text
Attack: Eligible test whose null-test expression is not the parser condition node.
Expected result: No refinement.
Actual result: No refinement.
Source of truth: SPEC.md says refinements apply to recognized null tests in `if` conditions.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/type_check.rs`
- Tests run:
  - `cargo test -p newlang --test type_check m0019_branch_refinement`
  - `cargo test -p newlang --test type_check`
  - `sh docs/tests/m0019-branch-refinement-records.sh`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-010-branch-refinement-records.md`
- Result:
  - pass

## Findings

No blocking, high, medium, or low soundness findings.

## Ambiguities

None. Per-use refined expression typing, nested scope inheritance, shadowing, nullable-use diagnostics, and mutation invalidation remain later tasks.

## Decision

Pass. Branch refinement records are attached only to the ADR-0028 branch regions and do not yet affect expression typing.
