# Diagnostic Contract

This document defines the compiler-wide diagnostic shape required by ADR-0015 before any compiler stage emits real errors. It is a contract for diagnostics infrastructure and tests, not a language semantic rule.

## Source Of Truth

Diagnostic behavior must cite:

- `docs/SPEC.md`
- the relevant file under `docs/adr/`
- the task or milestone that authorizes harness-only behavior

If the source of truth is ambiguous, agents must file an ambiguity report instead of creating a diagnostic expectation.

## Required Diagnostic Shape

Every user-facing diagnostic must provide:

- severity
- message
- primary span

Every diagnostic may provide:

- secondary span entries
- notes
- safe suggestion entries

The primary span identifies the user's most actionable source location. A secondary span explains related source locations only when it improves the diagnostic. Notes provide short supporting explanations. A safe suggestion must preserve the accepted semantics from `docs/SPEC.md` and `docs/adr/`.

## Severity

The initial severity model is:

- error
- warning
- note

Compiler stages may use only severities documented here unless a later milestone extends the contract.

## User-Facing Wording

Diagnostics must describe source-level concepts. They must not expose internal compiler jargon such as pass names, internal representation names, implementation IDs, or backend internals unless the user explicitly requested an internal diagnostic mode in a future milestone.

## Snapshot Expectations

Diagnostic snapshots must be stable enough for review. A snapshot should record:

- severity
- message
- primary span
- secondary span entries, when present
- notes, when present
- safe suggestion entries, when present
- source-of-truth citation

Snapshots must not be used to encode guessed language behavior. If a compiler rule is not specified, the expected diagnostic is blocked until the specification or ADRs resolve it.

