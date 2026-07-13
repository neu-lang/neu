# ADR-0049: Bootstrap Entry-Point Diagnostic Provenance

Status: Accepted

## Question

Where do ADR-0040 entry-point diagnostics attach in a package-scoped,
multi-source compilation?

## Competing Designs

1. Use an arbitrary source-file node for every entry diagnostic.
2. Use candidate declaration locations when candidates exist and the explicit
   entry-package invocation input when none exists.
3. Validate entry points independently per source file.

## Trade-offs

Arbitrary source locations misattribute package-level errors. Per-file
validation contradicts the one-program entry contract. Candidate locations and
an explicit invocation location are deterministic without host-path inference.

## Recommended Choice

Entry-point diagnostic provenance is defined as follows:

- `missing_entry_point` has the explicit selected entry-package invocation
  input as its primary location; recovery records no selected entry point.
- Every top-level `main` candidate in the selected package receives
  `duplicate_entry_point` when more than one candidate exists. Its primary
  location is that candidate declaration; recovery records no selected entry.
- `invalid_entry_point_signature` has the candidate function declaration as
  its primary location; recovery records no selected entry.

Each diagnostic carries either its source-file-qualified source span or the
explicit invocation input location. It must not identify source locations using
host paths or arena-local node identity alone. Safe suggestions are: add the
accepted `main` form, remove or rename duplicate candidates, or use the exact
accepted no-parameter `Int` signature with a body.

## Downstream Consequences

- The compiler entry checking may aggregate candidates across source files in the
  selected package.
- Later diagnostics may use the same source-or-external-input provenance
  contract where no source declaration exists.
- HIR and backend stages receive no entry declaration after an entry diagnostic.

## Dependencies

- ADR-0015
- ADR-0025
- ADR-0040
