# ADR-0053: Bootstrap Unsupported Executable-Form Diagnostics

Status: Accepted

## Question

How do diagnostics for parsed forms outside the ADR-0042 executable subset
attach, recover, and avoid duplicate reporting before HIR?

## Competing Designs

1. Diagnose every unsupported AST node.
2. Diagnose the outermost unsupported form and suppress its unsupported
   descendants.
3. Reuse only existing deferred diagnostics.

## Trade-offs

Per-node diagnostics are mechanically simple but produce cascades for nested
forms. Existing deferred diagnostics do not cover every deferred declaration,
type, pattern, and runtime form. An outermost boundary gives deterministic
recovery while preserving more-specific accepted diagnostics.

## Recommended Choice

For every parsed form outside ADR-0042, emit `unsupported_executable_form`
unless a more-specific accepted diagnostic applies. Its primary location is
the source-file-qualified span of the outermost unsupported form. If an
unsupported form contains another unsupported form, report only the outermost
form for that nested region.

Existing `DirectCallDeferred`, `FunctionTypeApplicationDeferred`,
`MemberExpressionDeferred`, `BinaryExpressionDeferred`, `UnaryExpressionDeferred`,
and `IfValueDeferred` diagnostics remain the more-specific result for their
existing deferred forms. Parser diagnostics remain independent and are never
suppressed by this rule.

Recovery records no executable type, control-flow, ownership, or lowering fact
for the diagnosed form or its suppressed unsupported descendants. HIR must not
receive those forms. Unrelated sibling forms continue to be checked.

## Downstream Consequences

- The compiler can reject deferred declarations, types, patterns, and runtime forms
  deterministically before HIR.
- HIR and later stages may require an executable-form-clean frontend report.
- Diagnostics remain source-file-qualified without relying on arena-local IDs.

## Dependencies

- ADR-0015
- ADR-0024
- ADR-0042
