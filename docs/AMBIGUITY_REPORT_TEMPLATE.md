# Ambiguity Report: <ID>

## Metadata

- Report ID: `<AMBIGUITY_ID>`
- Related Task: `<TASK_ID or none>`
- Related Milestone: `<MILESTONE_ID>`
- Filed By: `<agent>`
- Date: `YYYY-MM-DD`
- Status: `open | resolved | superseded`
- Required Owner: `Language Lawyer | Language Designer | Chief Architect`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/<ADR>.md`
- Milestone:
  - `docs/milestones/<MILESTONE_FILE>.md`

## Exact Ambiguous Text Or Missing Rule

```text
<quote exact text, or state the missing rule>
```

## Competing Interpretations

1. <Interpretation A>
2. <Interpretation B>
3. <Interpretation C, if applicable>

## Why Guessing Is Unsafe

- <soundness risk, diagnostic risk, architecture risk, or roadmap risk>

## Affected Work

- Tasks blocked:
  - `<TASK_ID>`
- Milestones affected:
  - `<MILESTONE_ID>`
- Tests blocked:
  - `<path or fixture category>`
- Implementation areas blocked:
  - `<path or subsystem>`

## Recommended Resolution Path

- [ ] Language Lawyer determines whether existing text resolves it.
- [ ] Language Designer drafts ADR or spec revision if new semantics are required.
- [ ] Adversarial Engineer reviews soundness risk.
- [ ] Diagnostics Engineer reviews diagnostic consequences.
- [ ] Simplicity Guardian reviews complexity.
- [ ] Chief Architect approves final resolution.

## Temporary Rule

No implementation may proceed on the ambiguous behavior until the source of truth is updated or the ambiguity is ruled non-blocking by Chief Architect.

## Resolution

- Decision:
  - <decision>
- Source of truth updated:
  - `<path>`
- Date resolved:
  - `YYYY-MM-DD`

