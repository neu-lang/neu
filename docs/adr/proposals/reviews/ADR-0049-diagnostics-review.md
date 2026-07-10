# ADR-0049 Diagnostics Review

## Metadata

- Proposal: `ADR-0049`
- Milestone: `M0028`
- Review: `main-task diagnostics review`
- Date: `2026-07-11`
- Decision: `approve`

## Review

The ADR gives all ADR-0040 identifiers a deterministic primary location,
recovery, and safe suggestion. It avoids the unsafe use of arena-local node
IDs as cross-source provenance and satisfies ADR-0015's external-input option.
