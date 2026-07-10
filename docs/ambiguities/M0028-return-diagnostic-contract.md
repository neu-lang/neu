# Ambiguity Report: M0028 Return Diagnostic Contract

## Metadata

- Report ID: `M0028-RETURN-DIAGNOSTIC-CONTRACT`
- Related Task: `M0028-009`
- Related Milestone: `M0028`
- Filed By: `main-task diagnostics review`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`, ADR-0015, ADR-0041, and ADR-0042 summaries.
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md`
  - `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`
- Milestone:
  - `docs/milestones/M0028-executable-expression-frontend-completion.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0041 names missing_return and unreachable_return, but does not define their
primary spans, recovery, safe suggestions, or the exact direct-return rule for
ADR-0042's straight-line subset.
```

## Competing Interpretations

1. Treat any return nested anywhere in a function body as satisfying its path.
2. Only a return directly contained by the function body block satisfies the
   straight-line path; a later direct return is unreachable.
3. Delay all return diagnostics until general branch analysis exists.

## Why Guessing Is Unsafe

- Nested branch returns would be misrepresented as unconditional returns.
- Different primary nodes change editor highlighting and recovery behavior.
- Deferring the diagnostic undermines ADR-0040's required executable entry
  validation.

## Affected Work

- Tasks blocked: `M0028-009`.
- Milestones affected: `M0028`.
- Tests blocked: direct, missing, unreachable, and nested-block return tests.
- Implementation areas blocked: `crates/compiler/src/type_check.rs`.

## Recommended Resolution Path

- [ ] main-task semantic design defines the straight-line direct-return rule,
  diagnostic locations, recovery, and safe suggestions.
- [ ] main-task diagnostics check reviews the contract.
- [ ] main-task adversarial check reviews nested-block cases.
- [ ] main-task simplicity check prevents premature branch analysis.
- [ ] main task accepts an ADR bundle.

## Temporary Rule

Do not implement return diagnostics until the accepted source of truth defines
the direct-return boundary and diagnostic contract.

## Resolution

- Decision: ADR-0050 defines direct-return analysis, primary spans, recovery,
  and safe suggestions.
- Source of truth updated: `docs/adr/ADR-0050-bootstrap-straight-line-return-diagnostics.md` and `docs/SPEC.md`.
- Date resolved: `2026-07-11`.
