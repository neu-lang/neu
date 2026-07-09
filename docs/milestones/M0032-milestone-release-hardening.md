# M0032: Milestone Release Hardening

## Title

M0032: Milestone Release Hardening

## Identifier

M0032

## Goal

Harden the first compiler milestone release candidate against roadmap, spec, diagnostic, test, and build requirements.

## Motivation

Before declaring a milestone release, the compiler must be audited against the specification and CI gates rather than implementation momentum.

## Background

`AGENTS.md` defines release workflow and CI gate requirements. Spec Compliance Auditor compares implementation against `docs/SPEC.md` and accepted ADRs.

## Prerequisites

- M0031

## Inputs

- All completed milestones M0001-M0031.
- `docs/SPEC.md`
- `docs/adr/`
- `AGENTS.md`
- CI results.

## Outputs

- Release readiness report.
- Known limitations list.
- Deferred decisions list.
- Passing release CI gates.

## Scope

- Audit and hardening only.
- Release documentation.

## Out of Scope

- New language features.
- Optimization.
- LLVM.
- Broad refactors not required for release gates.

## Deliverables

- Release checklist.
- Spec compliance report.
- Diagnostic quality report.
- Test coverage report.
- Build and target-pack report.

## Acceptance Criteria

- CI gates from `AGENTS.md` pass for implemented areas.
- Spec Compliance Auditor signs off or lists blocking non-compliance.
- Diagnostics Engineer signs off on implemented diagnostic areas.
- Build Engineer signs off on host and cross-target smoke gates.
- Known limitations do not contradict `docs/SPEC.md`.

## Test Strategy

- Full CI.
- Release smoke tests.
- Negative test suite.
- Diagnostic snapshot suite.

## Risks

- Earlier ambiguity reports may block release.
- Cross-target smoke may expose packaging gaps.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- Release documentation.
- CI configuration if gaps are found.
- Tests only for missing release coverage.

## Completion Checklist

- [ ] Full CI passes.
- [ ] Spec compliance report is complete.
- [ ] Diagnostic report is complete.
- [ ] Build report is complete.
- [ ] Deferred decisions are listed.

