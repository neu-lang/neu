# Ambiguity Report: M0032 Function Symbol Identity

## Metadata

- Report ID: `M0032-FUNCTION-SYMBOL-IDENTITY`
- Related Task: `M0032-001`
- Related Milestone: `M0032`
- Filed By: `main-task architecture review`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- `docs/SPEC.md`, ADR-0044 through ADR-0047, and the M0032 milestone.
- `HirFunction` preserves module and package identity, but not the source
  function name.
- `MirFunction` preserves only a MIR-local numeric function ID.

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0046 requires bootstrap symbols to be derived from module identity, package
namespace, and function name, while the HIR-to-MIR data available to M0032
does not preserve the source function name.
```

## Competing Interpretations

1. Use the MIR-local numeric ID as the object symbol identity.
2. Carry one already-flattened symbol string from the frontend.
3. Preserve structured module, package, and source-function identity through
   HIR and MIR, and derive the backend symbol at object emission.
4. Maintain a separate backend symbol table keyed by MIR IDs.

## Why Guessing Is Unsafe

- Numeric IDs are local implementation identities and are not collision-safe
  across modules or stable under declaration-order changes.
- A side table can drift from the intermediate representations and make object
  emission depend on hidden state.
- Flattening before HIR loses structured identity needed by future diagnostics
  and target policies.

## Affected Work

- Tasks blocked: `M0032-001` object emission until identity transport is
  resolved.
- Milestones affected: M0030 and M0032.
- Implementation areas affected: HIR function identity, MIR function identity,
  and object symbol construction.

## Recommended Resolution Path

- [x] main-task language review confirms this is compiler identity, not new
  language semantics.
- [x] main-task semantic design drafts ADR-0056.
- [x] main-task adversarial check rejects ID-only symbol derivation.
- [x] main-task diagnostics check requires explicit missing-identity failure.
- [x] main-task simplicity check rejects a parallel backend side table.
- [x] main-task specification check confirms ADR-0056 matches ADR-0046.
- [x] main task approves the final resolution under delegated Chief Architect
  authority.

## Resolution

ADR-0056 requires structured module, package, and source-function identity to
be preserved through HIR and MIR and consumed by object emission. The exact
backend symbol encoding remains an implementation detail. No source-language
semantics changed.
