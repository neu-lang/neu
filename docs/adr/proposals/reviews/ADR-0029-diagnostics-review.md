# ADR-0029 Diagnostics Review

Role: main-task diagnostics check

Diagnostic area: Hard migration from the immutable-local `val` spelling to
`const`, including parser recovery and source-level suggestions.

Inputs read:

- `main task rules`
- `main task rules`
- `docs/SPEC.md`
- `docs/adr/proposals/ADR-0029-immutable-local-const-keyword.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- `docs/adr/ADR-0026-name-resolution-policy.md`
- `docs/adr/ADR-0028-nullability-and-flow-typing.md`
- `docs/adr/ADR-0013-mutability-model.md`
- `docs/adr/ADR-0019-compile-time-evaluation-and-metaprogramming.md`
- `docs/diagnostics.md`
- `tests/fixtures/parser/statements/{diagnostics,negative}.fixture.toml`
- existing parser diagnostic and recovery implementation, read only to assess
  the currently accepted recovery contract

Expected error condition:

After acceptance, report one migration **error** only when an identifier token
spelled `val` begins an otherwise recognizable local-declaration attempt. The
primary span must be that exact token. The message should say: `` `val` is no
longer the immutable-local declaration keyword; use `const` ``. A short note
may say that `const` retains an ordinary runtime initializer and is not a
compile-time constant. No secondary span is needed for a well-formed legacy
declaration.

Required message qualities:

- Source-level wording only: "immutable local binding" is appropriate;
  compiler phase names and AST terminology are not.
- The message must not imply literal-only initializers, purity,
  compile-time evaluation, static storage, copyability, or changed ownership
  behavior.
- The suggestion is an edit of the three-byte `val` token only, with
  replacement text `const`, and is offered only when that edit leaves the
  entire recognized declaration lexically and syntactically valid under the
  new grammar.
- A declaration whose binding name is `const`, for example
  `val const = value;`, must receive the migration error but **no** single-token
  `val`-to-`const` fix-it. `const` becomes reserved, so that suggested edit
  produces invalid `const const = value;`. The diagnostic may explain that the
  binding name also needs a separate source edit, but must not invent a rename.
- Outside the defined declaration shape, `val` remains an identifier and must
  use ordinary expression, assignment, or malformed-statement diagnostics.

Tests/snapshots:

The draft requires diagnostic snapshots but does not yet define a diagnostic
identifier, exact recognition predicate, recovery output, or fixture-facing
cases. The accepted ADR must require stable snapshots covering at least:

1. `val item = readRequest();` and `val item: Int = configuredRetries;`:
   one migration error, primary span `val`, one safe `const` replacement, and
   no compile-time-constant wording.
2. `val item;`: one migration error and recovery as an immutable declaration;
   no unrelated statement error.
3. `val : Int = value;`: migration error plus the existing
   `malformed_variable_declaration` behavior, with no duplicate generic
   statement diagnostic.
4. `val item = ;`: migration error plus the applicable existing malformed
   declaration or expression diagnostic, with the statement boundary preserved.
5. `val const = value;`: migration error with no unsafe fix-it.
6. Identifier uses such as `var val = 0;`, `val = 1;`, and `val();`: no
   migration error.
7. A legacy declaration followed by an independent valid declaration after a
   semicolon or right brace: the later construct is parsed normally.

Findings:

1. **Blocking — the required fix-it is not universally safe.** Proposal lines
   199-202 require replacing exactly the first token in every
   declaration-shaped legacy use. Once `const` is reserved, the old valid
   binding name `const` cannot occupy the name position in the new grammar.
   Therefore `val const = value;` cannot receive the required one-token edit.
   This contradicts the project diagnostic contract's requirement that a safe
   suggestion preserve accepted semantics. Revise the proposal to make the
   fix-it conditional on the resulting declaration being lexically and
   syntactically valid, and explicitly require no fix-it otherwise.

2. **Blocking — "syntactic position and shape" and recovery are underspecified.**
   The proposal does not state which lookahead forms select the migration
   diagnostic, whether a terminating semicolon is required, or how a malformed
   suffix is handled. ADR-0024 instead requires
   `malformed_variable_declaration` to span the declaration range and recover
   at the statement boundary. The accepted text must define: (a) the minimal
   legacy-declaration prefix, (b) which malformed tails also emit the existing
   malformed-declaration diagnostic, (c) diagnostic ordering, and (d) that the
   migration diagnostic replaces, rather than accompanies, generic
   `missing_statement` or `unexpected_token_in_statement` for that attempt.
   Without this, implementations can validly emit one error, two errors, or
   consume different amounts of following source.

3. **Blocking — recovered semantic output is ambiguous.** Proposal line 202
   says to recover "as though an immutable local declaration had been
   recognized" while line 204 says the program is not accepted. It must say
   whether a complete legacy-shaped declaration produces an error-marked
   immutable-local node that participates in local scope/name resolution and
   permits independent initializer diagnostics, or whether it is discarded.
   It must also require suppression of downstream diagnostics that are solely
   artifacts of the rejected spelling. This matters to ADR-0026 declaration
   order and duplicate-name reporting and ADR-0028 immutable-binding flow
   eligibility. Parser recovery must still stop at ADR-0024's semicolon, right
   brace, declaration-start, or EOF boundary; it must not consume a following
   independent statement.

4. **Revision required — the diagnostic contract is incomplete for snapshots.**
   ADR-0015 and the project diagnostic contract require a severity, message,
   primary span, recovery action, source-of-truth citation, and safe-suggestion
   policy. The proposal supplies only partial wording, span, and broad recovery
   intent. Add a stable diagnostic name/rule identifier, `error` severity,
   exact message or wording requirements, citation to the accepted superseding
   ADR plus ADR-0015/ADR-0024, and the conditional suggestion policy above.

5. **Non-blocking — wording direction is sound.** Proposal lines 211-215
   correctly prevent later diagnostics from calling the binding a
   compile-time constant. Retain this requirement and add the runtime-
   initializer note only to this migration diagnostic; unrelated immutable-
   binding errors should remain concise.

Decision: revise

The proposed primary span and source-level distinction are good, but the
mandatory one-token fix-it is unsafe in a valid migration case and the special
parser recovery path is not defined tightly enough to coexist with
`malformed_variable_declaration` and statement recovery. These are diagnostic
contract blockers for acceptance; no semantic accept/reject rule is changed by
this review.

Handoff: main-task semantic design to revise ADR-0029's diagnostic section; then
main-task diagnostics check to verify the revised recognition predicate, recovery
matrix, and fixture/snapshot contract before main task approval.

## Revised Review Round

Re-reviewed against the revised proposal, especially lines 250-284. The
proposal now removes the special legacy-`val` diagnostic, fix-it, recognition
predicate, recovery-as-declaration path, semantic recovery artifact, and new
snapshot contract. This materially changes the diagnostic design reviewed
above; the earlier `revise` decision is historical and is superseded by the
final verdict below.

### Disposition Of Prior Findings

1. The unsafe one-token fix-it finding is resolved. No `val`-to-`const` fix-it
   is prescribed, and newly reserved identifier uses of `const` receive no
   invented automatic rename.
2. The underspecified recognition and diagnostic-order finding is resolved.
   There is no contextual legacy-declaration recognition. A token sequence
   beginning with identifier `val` is parsed solely under the new grammar and
   receives the ordinary ADR-0024 diagnostic for that sequence.
3. The recovered-output ambiguity is resolved. Rejected old syntax explicitly
   creates no declaration node, immutable binding, name-resolution entry,
   type-checking fact, flow fact, ownership record, or other downstream
   semantic record.
4. The incomplete new-diagnostic contract finding is no longer applicable.
   ADR-0029 introduces no diagnostic category, severity, rule identifier,
   suggestion, recovery artifact, or snapshot contract requiring definition.
5. The sound wording requirement is retained: diagnostics may call a valid
   `const` binding an "immutable local binding" but must not call it a
   compile-time constant.

### Malformed Declaration Recovery

The revised boundary is coherent with ADR-0024:

- source beginning with removed `val` does not enter variable-declaration
  parsing and therefore does not receive `malformed_variable_declaration`
  merely because it resembles the old grammar; ordinary expression or
  statement diagnostics and their existing recovery boundaries apply
- source beginning with accepted `const` does enter variable-declaration
  parsing, so malformed names, annotations, initializers, or terminators
  continue to use `malformed_variable_declaration` and ADR-0024 statement
  recovery
- neither path recovers rejected `val` source as a successful immutable local,
  so no compatibility alias or downstream cascade-suppression contract is
  implied

No diagnostic-specific blocker remains. Follow-up tests should verify the
ordinary lexer/parser behavior after the keyword-set migration, but they must
not snapshot a special legacy diagnostic or semantic recovery artifact.

Final verdict: **approve**

Handoff: main task for proposal decision. main-task test work should define the
ordinary keyword and parser expectations during the separately authorized,
tests-first migration.
