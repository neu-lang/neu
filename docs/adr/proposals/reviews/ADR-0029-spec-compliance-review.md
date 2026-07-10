# ADR-0029 Spec Compliance Review

Role: Spec Compliance Auditor

Target: The accepted ADR-0029 authority bundle: accepted ADR, `docs/SPEC.md`,
and grammar-authority-ledger updates, checked against the final ADR-0029
proposal, specialty reviews, Chief Architect decision, and accepted surrounding
ADRs. Current compiler, fixtures, tasks, and examples were deliberately not
treated as authority.

Inputs read:

- `AGENTS.md` and `.codex/agents/spec-compliance-auditor.toml`
- `docs/SPEC.md`
- `docs/adr/ADR-0013-mutability-model.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0019-compile-time-evaluation-and-metaprogramming.md`
- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- `docs/adr/ADR-0026-name-resolution-policy.md`
- `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- `docs/adr/ADR-0029-immutable-local-const-keyword.md`
- `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`
- All ADR-0029 specialty reviews and
  `docs/adr/proposals/reviews/ADR-0029-chief-architect-decision.md`
- `docs/syntax/grammar-authority-ledger.md` and the documentation diff under
  audit

## Blocking findings

None.

## Compliance matrix

| Authority requirement | Bundle evidence | Classification |
| --- | --- | --- |
| Atomic acceptance of ADR, SPEC, and ledger | Accepted ADR-0029 §§Acceptance Bundle and Dependencies; all four SPEC targets and the prescribed ledger rows are present. | Compliant |
| Hard lexical replacement | ADR-0029 §§Decision and Supersession; SPEC ADR-0021 reserve `const`, release `val`; ledger token-spellings row cites both authorities. | Compliant |
| `val` remains an ordinary identifier outside the removed introducer position | ADR-0029 §§Decision, Cross-Phase Invariant, and Diagnostics; SPEC ADR-0024 and ADR-0029 retain `const val` and `var val` examples. | Compliant |
| No `const`-specific compile-time, storage, ownership, capability, or layout meaning | ADR-0029 §§Decision, Cross-Phase Invariant, and Soundness Impact; SPEC ADR-0029; ledger keeps compile-time-evaluation syntax deferred. | Compliant |
| Preserve initializer, binding identity, scope, lookup, and flow semantics | ADR-0029 §Preserved Semantics And Deferrals; SPEC ADR-0026; ADR-0024, ADR-0026, and ADR-0028 remain otherwise unsuperseded. | Compliant |
| Ordinary diagnostics and recovery for old introducer-shaped `val` source | ADR-0029 §Diagnostics And Source Compatibility; SPEC ADR-0024; aligns with the final Diagnostics and Adversarial reviews. | Compliant |
| M0019-014 remains test-first and gated | ADR-0029 §Roadmap Hard Gate For M0019-014; matches the Chief Architect’s ordered gates. | Compliant |

## Non-compliance findings

None. The bundle preserves the final proposal’s positional correction: `val`
is excluded only as the immutable-local declaration introducer, so it does not
overrule valid binding-name uses. It also preserves the accepted deferrals for
initializer-free immutable locals, evaluation order, effects, ownership
transfer, and lowering.

## Unsupported expectations

None in the audited authority bundle. No compiler behavior, fixture,
diagnostic snapshot, example, or task text was used to establish semantics.

## Ambiguities

None introduced by the bundle. The previously identified ambiguity about
initializer-free immutable locals remains explicitly deferred rather than being
resolved by the `const` spelling change.

Decision: approve

Handoff target: Test Engineer for the required tests-first migration, then
Implementer, subject to ADR-0029’s ordered M0019-014 gates.
