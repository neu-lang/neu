# M0001: Source of Truth Alignment

## Title

M0001: Source of Truth Alignment

## Identifier

M0001

## Goal

Align project documentation paths and authority references so every agent and future task can locate the language source of truth.

## Motivation

The agent rules and execution templates must consistently refer to the canonical specification path, `docs/SPEC.md`.

## Background

`docs/AGENTS.md` states that the specification and ADRs are authoritative. The current repository contains `docs/SPEC.md` and `docs/adr/`.

## Prerequisites

None.

## Inputs

- `AGENTS.md`
- `docs/SPEC.md`
- `docs/adr/`
- `.codex/agents/`

## Outputs

- A documented, consistent source-of-truth path.
- An ambiguity report only if a future relocation reintroduces multiple candidate spec paths.

## Scope

- Documentation alignment only.
- No semantic changes.

## Out of Scope

- Compiler code.
- ADR content changes.
- Language specification changes.

## Deliverables

- Updated references or a recorded ambiguity report.
- Validation that all agent references point to the canonical spec path.

## Acceptance Criteria

- A search for spec path references shows `docs/SPEC.md` as the canonical convention or an explicit ambiguity report.
- `docs/adr/` remains the ADR directory.
- No language semantic text is changed.

## Test Strategy

- Run text searches for stale bare `SPEC.md` references and canonical `docs/SPEC.md` references.
- Review changed documentation manually.

## Risks

- Accidentally changing semantic wording while fixing paths.
- Leaving agents with inconsistent required inputs.

## Estimated Effort

1 working day.

## Expected Files Changed

- `AGENTS.md`
- `.codex/agents/*.md`
- Possibly `docs/ambiguities/*`

## Completion Checklist

- [x] Canonical spec path is decided or ambiguity is recorded.
- [x] Agent references are consistent.
- [x] No semantic rules are changed.
