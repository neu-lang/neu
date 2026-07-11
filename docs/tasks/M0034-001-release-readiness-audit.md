# Task: M0034-001 Release Readiness Audit

## Task Metadata

- Task ID: `M0034-001`
- Milestone: `M0034`
- Milestone File: `docs/milestones/M0034-milestone-release-hardening.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Audit the implemented compiler milestone against the accepted specification,
ADR set, diagnostics contracts, build gates, examples, and target-pack smokes.

## Authority Extract

- `AGENTS.md` defines release CI gates and source-of-truth precedence.
- `docs/SPEC.md` and accepted `docs/adr/` define the implemented contracts.
- M0034 requires release readiness evidence and known limitations.
- M0001-M0033 task records provide implementation and validation evidence.

## Scope

- Produce spec-compliance, diagnostics, build/target-pack, and test-coverage
  audit sections.
- List known limitations and intentionally deferred decisions.
- Run the full release CI and all current example/target-pack validators.
- Identify blockers rather than silently accepting gaps.

## Out Of Scope

- New language features or semantic changes.
- Optimization, LLVM, or broad refactors.
- Changing accepted ADRs to make the audit pass.
- Executing foreign-architecture binaries.

## Tests

- Full formatter, Clippy, workspace test, diff, diagnostic, negative, example,
  host executable, and cross-target format smoke gates.
- Documentation checks for the audit reports and milestone checklist.

## Acceptance Criteria

- Every M0034 deliverable has an explicit report or a recorded blocker.
- Reports distinguish implemented bootstrap behavior from deferred semantics.
- No known limitation contradicts SPEC.md or accepted ADRs.
- All applicable CI gates pass, or a concrete blocking finding is recorded.
- The release checklist is objectively marked complete only after evidence is
  present.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0033 is
  complete and M0034 is the ordered release-hardening milestone. handoff=audit
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the release
  validator had no audit reports or completed release checklist to validate.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=five
  release audit reports document specification, diagnostics, build/target-pack,
  coverage, limitations, and deferred decisions. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=release
  documentation validator and focused target/example validators passed.
  handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=
  limitations are explicit, foreign execution is not claimed, and deferred
  decisions do not contradict accepted source of truth. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=the release
  reports cover every M0034 deliverable and identify residual risk. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter, release,
  example, target-pack, capability, Clippy, workspace tests, and diff checks
  passed. handoff=commit

## Required Outputs

- Authority read: AGENTS.md, SPEC.md, accepted ADRs, M0034, and completed task
  records for M0001-M0033.
- Files changed: this task, release audit reports, validators, and the M0034
  milestone checklist.
- Tests written before implementation and expected pre-implementation failure:
  release-audit validator must fail until all required reports exist.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/m0034-release-audit.sh`,
  `docs/tests/current-example-backend-surface.sh`,
  `docs/tests/m0033-target-pack-inventory.sh`,
  `docs/tests/m0033-cross-target-pack.sh`,
  `docs/tests/m0033-target-capability-ambiguity.sh`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, and
  `git diff --check` all passed.
- Open questions or `none`.
- Remaining risk: the release remains limited to the documented bootstrap and
  target-pack surfaces. Next main-task action: commit release evidence locally.
