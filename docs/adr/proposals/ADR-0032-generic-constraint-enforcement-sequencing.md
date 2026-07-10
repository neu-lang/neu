# ADR-0032: Generic Constraint Enforcement Sequencing

Status: Accepted as `docs/adr/ADR-0032-generic-constraint-enforcement-sequencing.md`

## Non-Authority Notice

This proposal is retained as design history. The accepted ADR is the only
source of authority; this file does not define capability semantics.

## Question

When may the compiler enforce generic capability bounds, given that M0020 is
scheduled before ownership, move, and thread-capability analyses?

## Competing Designs

1. Define a provisional built-in catalog (`Copy`, `Send`, `Share`) and
   provisional satisfaction rules in M0020.
2. Treat every bound as a nominal interface requirement and resolve it through
   general type/interface lookup.
3. Complete generic parameter and bound representation in M0020, but defer
   constraint enforcement until ownership and thread-capability semantics are
   accepted and implemented.

## Trade-offs

The provisional-catalog option produces early diagnostics, but would invent
copyability and send/share derivation rules before their owning milestones.

Nominal-interface constraints are extensible, but require type/interface
resolution and a declared relation between interfaces and capabilities that is
not yet accepted.

Deferring enforcement preserves the selected constrained-generic direction
without committing to unsound or incomplete satisfaction rules. It requires a
roadmap adjustment and leaves generic bound violations unchecked temporarily.

## Recommended Draft Choice

Choose option 3. M0020 should preserve generic parameter identity and explicit
bound occurrences only. A later accepted decision must define the capability
catalog or resolution model, type-satisfaction rules, substitution point,
diagnostic identifiers/spans/recovery, and the relationship to ownership and
thread-safety analyses. That decision should schedule enforcement after the
required semantic inputs rather than infer them from syntax.

## Consequences

M0020 cannot meet its current bound-violation acceptance criterion until the
accepted roadmap is revised. M0020-001 through M0020-003 remain valid
representation work. No parser syntax, runtime behavior, ownership rule, or
thread-safety rule changes under this proposal.

## Dependencies

ADR-0005, ADR-0010, ADR-0014, ADR-0016, ADR-0017, ADR-0023, M0020, M0022,
and M0024.

## Handoff

Language Designer should review the alternatives; Roadmap Planner should
propose the resulting milestone sequencing; Chief Architect must approve any
accepted revision. No implementation follows until `docs/SPEC.md` changes.
