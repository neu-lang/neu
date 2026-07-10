#!/usr/bin/env sh
set -eu

fail() {
  echo "m0027-executable-semantics-accepted: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing required file: $1"
}

require_text() {
  file="$1"
  pattern="$2"
  grep -Eq "$pattern" "$file" || fail "missing expected pattern in $file: $pattern"
}

for adr in 0040 0041 0042 0043 0044 0045 0046 0047; do
  require_file "docs/adr/ADR-${adr}-"*
done

require_text docs/adr/ADR-0040-bootstrap-program-entry-point.md '^Status: Accepted$'
require_text docs/adr/ADR-0040-bootstrap-program-entry-point.md 'top-level function named `main`'
require_text docs/adr/ADR-0040-bootstrap-program-entry-point.md 'with no parameters'
require_text docs/adr/ADR-0040-bootstrap-program-entry-point.md 'declared return type `Int`'
require_text docs/adr/ADR-0040-bootstrap-program-entry-point.md 'CLI arguments are deferred'

require_text docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md '^Status: Accepted$'
require_text docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md 'Arguments are evaluated left-to-right'
require_text docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md 'Only explicit `return expression;`'
require_text docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md '`missing_return`'

require_text docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md '^Status: Accepted$'
require_text docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md '`\*\*`'
require_text docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md '`<<` and `>>`'
require_text docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md 'binary bitwise operations'
require_text docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md 'M0028 must add parser and type-checker support'

require_text docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md '^Status: Accepted$'
require_text docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md 'signed 64-bit two.s-complement'
require_text docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md '`division_by_zero`'
require_text docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md '`negative_exponent`'
require_text docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md '`invalid_shift_count`'

require_text docs/adr/ADR-0044-bootstrap-hir-runtime-contract.md '^Status: Accepted$'
require_text docs/adr/ADR-0044-bootstrap-hir-runtime-contract.md 'M0029 acceptance criteria'
require_text docs/adr/ADR-0045-bootstrap-mir-runtime-contract.md '^Status: Accepted$'
require_text docs/adr/ADR-0045-bootstrap-mir-runtime-contract.md 'exponentiation, bitwise, and shift'
require_text docs/adr/ADR-0046-bootstrap-abi-and-calling-convention.md '^Status: Accepted$'
require_text docs/adr/ADR-0046-bootstrap-abi-and-calling-convention.md 'M0033 responsible for cross-target packs'
require_text docs/adr/ADR-0047-bootstrap-object-link-runtime-model.md '^Status: Accepted$'
require_text docs/adr/ADR-0047-bootstrap-object-link-runtime-model.md 'requires no Neu standard library'

require_text docs/SPEC.md '^## ADR-0040: Bootstrap Program Entry Point$'
require_text docs/SPEC.md '^## ADR-0047: Bootstrap Object Link Runtime Model$'
require_text docs/SPEC.md '`negative_exponent`'
require_text docs/SPEC.md '`invalid_shift_count`'

echo "m0027 executable semantics accepted validation passed"
