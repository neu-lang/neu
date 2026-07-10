# Ambiguity Report: M0028 Direct Call Diagnostic Contract

## Metadata

- Report ID: `M0028-DIRECT-CALL-DIAGNOSTIC-CONTRACT`
- Related Task: `M0028-010`
- Related Milestone: `M0028`
- Filed By: `main-task diagnostics review`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`, ADR-0015 and ADR-0041 summaries.
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md`
- Milestone: `docs/milestones/M0028-executable-expression-frontend-completion.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0041 names invalid_call_target, argument_count_mismatch,
argument_type_mismatch, and recursive_call_unsupported but does not define
their primary/secondary locations, recovery, or safe suggestions.
```

## Competing Interpretations

1. Put every call diagnostic on the call expression.
2. Put invalid target on the callee, argument diagnostics on the applicable
   argument/call, and recursion diagnostics on the recursive call.
3. Defer all call validation pending HIR.

## Why Guessing Is Unsafe

- Diagnostics would highlight different source forms with different recovery.
- Cross-source function declarations require source-qualified provenance.
- Deferral would leave the first runnable source subset without its accepted
  helper-call semantics.

## Affected Work

- Tasks blocked: `M0028-010`.
- Milestones affected: `M0028`.
- Tests blocked: direct target, argument, and recursion cases.
- Implementation areas blocked: `crates/compiler/src/type_check.rs`.

## Recommended Resolution Path

- [ ] main-task semantic design defines diagnostic locations, recovery, and
  safe suggestions.
- [ ] main-task diagnostics/adversarial/simplicity reviews approve the scope.
- [ ] main task accepts an ADR bundle.

## Temporary Rule

Do not implement direct-call diagnostics until accepted authority defines their
contract.

## Resolution

- Decision: ADR-0051 defines direct-call primary locations, recovery, and safe
  suggestions.
- Source of truth updated: `docs/adr/ADR-0051-bootstrap-direct-call-diagnostics.md` and `docs/SPEC.md`.
- Date resolved: `2026-07-11`.
