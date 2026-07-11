# M0034 Release Readiness

## Status

Ready for the implemented bootstrap milestone, with the limitations below.

## Evidence

- Specification compliance: `M0034-spec-compliance.md`.
- Diagnostics: `M0034-diagnostics.md`.
- Build and target packs: `M0034-build-and-target-packs.md`.
- Test coverage: `M0034-test-coverage.md`.
- Full validation is recorded in `M0034-001` after the release gates pass.

## Known Limitations

- The runnable language subset is the accepted bootstrap subset only.
- The host pack is executable on the host; the x86-64 Linux pack is format and
  link validated but is not executed by the arm64 host.
- Printing, standard library, CLI arguments, heap allocation, coroutines,
  scheduler/runtime services, FFI, and stable public ABI remain deferred by
  accepted source-of-truth decisions.
- The target matrix is limited to the checked-in host and x86-64 Linux packs.

These limitations are consistent with SPEC.md and accepted ADRs and are not
presented as completed language features.

## Deferred Decisions

- Broader target capability profiles and target-specific ABI/layout semantics.
- Additional target packs and distribution automation.
- Standard library and platform API contracts.
- Stable external ABI and FFI symbol policy.
- LLVM backend support.

## Release Decision

The implemented bootstrap milestone may be released only with these
limitations visible and with the full CI evidence recorded in the task log.
