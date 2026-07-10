# ADR-0029 Simplicity Review

Role: Simplicity Guardian
Target: `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`

Inputs read:

- `AGENTS.md`
- `docs/SPEC.md`
- `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`
- `docs/adr/ADR-0013-mutability-model.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0019-compile-time-evaluation-and-metaprogramming.md`
- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- `docs/adr/ADR-0026-name-resolution-policy.md`
- `docs/adr/ADR-0028-nullability-and-flow-typing.md`

Complexity under review:

The proposal replaces one local-binding spelling and explicitly attempts to
preserve the existing immutable-binding category. The relevant complexity is
not a new binding abstraction: it is the proposed migration-specific parser
recovery for `val`, plus the risk that `const` leaks into compile-time
evaluation, storage, ownership, or evaluator metadata.

Accepted requirement:

ADR-0013 requires immutable and mutable binding behavior to remain distinct;
ADR-0024 supplies the local declaration grammar; ADR-0026 supplies binding
identity and scope; and ADR-0028 keys flow refinement to immutability. ADR-0019
describes a separate, bounded compile-time-evaluation direction and does not
authorize concrete constant syntax. Therefore the accepted requirement forces
only a spelling change and coherent reclassification of the existing immutable
local binding. It does not force a new binding kind, evaluator path, storage
class, alias system, migration framework, or evaluator-facing `const` flag.

Simpler alternative:

Use one canonical `const` token in the accepted lexer and local-declaration
grammar, map it to the existing immutable-local representation, and let `val`
be an ordinary identifier everywhere. A declaration-shaped `val` then receives
the normal malformed/unexpected-token recovery already defined by ADR-0024.
If a migration hint is considered valuable, implement it as a bounded
diagnostic classification layered on that existing error, with no compatibility
parse, recovery-as-success path, alias table, deprecation state, or legacy AST
node. This keeps migration UX separate from language acceptance and avoids
making the parser recognize a removed keyword contextually.

Findings:

1. **Revise — the targeted `val` recovery is more machinery than the hard
   replacement requires.** The proposal says `val` must lex as an ordinary
   identifier and must not be an alias (lines 92–96), but then requires the
   parser to recognize `val` in a declaration-shaped context and recover as if
   an immutable declaration had been recognized (lines 195–205). That is a
   second, contextual legacy grammar with successful immutable-binding recovery.
   It creates hidden coupling between identifier parsing, statement recovery,
   and migration diagnostics, and risks accidentally allowing old syntax in
   later phases. Either remove this special case and use existing malformed
   declaration recovery, or constrain it explicitly to an error-only diagnostic
   wrapper that produces no accepted AST/binding and does not participate in
   name resolution or type checking.

2. **Revise — state the implementation invariant for binding metadata.** The
   proposal correctly rejects an alternate AST binding kind (lines 221–225),
   but “same kind” and “all phases” are not concrete enough to prevent a
   `const`-specific evaluator or semantic flag from appearing. Acceptance
   should require that `const` lowers to the existing immutable-local metadata
   and that no `const`-specific evaluator input, constant-value table, storage
   class, copyability, destruction, send/share, or type/layout capability is
   introduced. The evaluator implication is especially important because
   ADR-0019 uses “constants” in a distinct sense. No evaluator work is required
   by this spelling proposal.

3. **Non-blocking — the alias analysis is appropriately rejected.** Rejecting
   a transition alias is the simpler result for the owner-selected hard
   replacement (design 2, lines 60–67). The report should preserve the useful
   distinction between “migration suggestion” and “accepted alias”; it should
   not grow into a versioned compatibility or deprecation subsystem.

4. **Non-blocking — the preserved semantic boundaries are strong.** The
   proposal correctly keeps initializer execution ordinary, preserves optional
   initializer and type-annotation grammar, and keeps immutability separate
   from ownership, moves, borrowing, destruction, and thread safety (lines
   112–144). Its exclusion of globals, members, parameters, pattern bindings,
   and compile-time declarations avoids a broader binding redesign.

Decision:

**REVISE before acceptance.** The hard replacement direction is simple and
justified by the stated owner decision, and no new binding kind or evaluator is
needed. Acceptance should wait until the proposal removes or sharply limits the
contextual legacy parser path and makes the existing immutable-local metadata
invariant explicit. I do not block the keyword replacement itself.

Required changes:

- Replace the proposed recovery-as-immutable-declaration behavior with ordinary
  error recovery, or specify an error-only migration hint that cannot produce an
  accepted declaration, alias, binding, or downstream semantic record.
- Add an explicit invariant that `const` uses the existing immutable-local
  binding representation and has no evaluator, constant-materialization,
  storage, ownership, copyability, destruction, thread-sharing, type-position,
  or layout consequences.
- Keep migration support one bounded diagnostic; do not introduce aliases,
  compatibility modes, deprecation state, version negotiation, or a general
  legacy-syntax framework.
- Preserve the proposal's explicit distinction between an ordinary runtime
  initializer and ADR-0019's future compile-time-evaluation mechanism.

Open questions:

- Whether the project owner considers a migration hint worth the parser
  complexity of declaration-shaped recognition. This is a UX choice, not a
  semantic requirement.
- If retained, which existing ADR-0024 recovery diagnostic owns the error-only
  migration classification and guarantees that no invalid binding enters later
  phases?

Blockers:

- No semantic blocker to replacing `val` with `const`.
- Simplicity blocker to acceptance until the migration recovery and metadata
  invariants above are clarified.

Handoff:

Return to the Language Designer for proposal revision, then to the Chief
Architect for the acceptance decision. Diagnostics Engineer should separately
decide whether the migration hint justifies its bounded parser integration.

## Revised-Round Review

Date: `2026-07-10`

Inputs read for revised round:

- `AGENTS.md`
- `.codex/agents/simplicity-guardian.toml`
- `docs/SPEC.md`
- revised `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`
- this prior simplicity review
- accepted ADR-0013, ADR-0015, ADR-0019, ADR-0021, ADR-0024, ADR-0026,
  and ADR-0028

Revised findings:

1. **Resolved — no contextual legacy `val` mechanism remains.** Revised lines
   257–269 categorically remove the migration diagnostic, recognition
   predicate, fix-it, recovery contract, compatibility declaration, legacy AST
   node, binding, and downstream semantic records. Old declaration-shaped
   source now follows ordinary ADR-0024 diagnostics and recovery. This satisfies
   the prior requirement and leaves `val` an ordinary identifier consistently.

2. **Resolved — the immutable-binding metadata invariant is explicit.** The
   cross-phase invariant at revised lines 214–235 maps source `const` directly
   to the one existing immutable-local category and forbids const-specific
   evaluator inputs, constant tables, storage classes, ownership/copy/drop
   flags, send/share flags, type or layout capabilities, alias markers, and
   downstream semantic records. This is sufficiently concrete to prevent a
   spelling-only change from becoming a new binding kind or evaluator feature.

3. **Resolved — runtime initializer and compile-time evaluation remain
   separate.** Revised lines 129–137 categorically deny compile-time-constant
   meaning and require an explicit future supersession before that declaration
   spelling could acquire such meaning. Lines 168–183 also preserve unresolved
   initializer semantics as deferrals instead of guessing. No evaluator work or
   speculative extension point is authorized.

4. **Resolved — aliases and migration machinery remain rejected.** Revised
   lines 65–72 and 286–293 retain one canonical spelling and reject aliases,
   compatibility modes, deprecation state, contextual legacy grammar, and
   recovery-as-success paths.

5. **Non-blocking — the ordered migration gates are process constraints, not a
   new subsystem.** Revised lines 318–340 are more detailed than the semantic
   change itself, but they address an identified active-task sequencing risk and
   require coherent tests-first migration. They do not authorize reusable
   migration infrastructure, version negotiation, evaluator behavior, or new
   semantic metadata. No simplification change is required.

Final decision:

**APPROVE.** The revised proposal satisfies all prior required simplicity
changes. It specifies a hard lexical replacement, reuses the existing
immutable-local semantic category, uses ordinary parser diagnostics for removed
syntax, and excludes aliases, migration frameworks, new binding kinds,
evaluator implications, storage semantics, and speculative extension points.

Required changes after revised round:

- None from Simplicity Guardian.

Open questions after revised round:

- None blocking simplicity approval.

Blockers after revised round:

- None.

Files changed:

- `docs/adr/proposals/reviews/ADR-0029-simplicity-review.md`

Handoff after revised round:

Chief Architect for the final acceptance decision after the remaining required
role reviews and atomic authority-update requirements are satisfied.
