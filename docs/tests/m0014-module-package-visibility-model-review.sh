#!/usr/bin/env sh
set -eu

fail() {
  echo "m0014-review: $*" >&2
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

require_absent_text() {
  file="$1"
  pattern="$2"
  if grep -Eq "$pattern" "$file"; then
    fail "unexpected pattern in $file: $pattern"
  fi
}

proposal=docs/adr/proposals/ADR-0025-module-package-visibility-model.md
ambiguity=docs/ambiguities/M0014-module-package-visibility-model.md
review_dir=docs/adr/proposals/reviews
task=docs/tasks/M0014-003-module-package-visibility-model-proposal-review.md

require_file "$proposal"
require_file "$ambiguity"
require_file "$task"
require_file "$review_dir/ADR-0025-language-lawyer-review.md"
require_file "$review_dir/ADR-0025-build-engineer-review.md"
require_file "$review_dir/ADR-0025-spec-compliance-review.md"
require_file "$review_dir/ADR-0025-simplicity-review.md"
require_file "$review_dir/ADR-0025-chief-architect-decision.md"

require_text "$proposal" '^Status: Draft proposal - not accepted source of truth$'
require_text "$ambiguity" 'Status: `open`'
require_text "$ambiguity" 'Blocking milestone: `M0014`'

require_text "$review_dir/ADR-0025-language-lawyer-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0025-language-lawyer-review.md" 'private'
require_text "$review_dir/ADR-0025-language-lawyer-review.md" 'default visibility'
require_text "$review_dir/ADR-0025-language-lawyer-review.md" 'module identity'

require_text "$review_dir/ADR-0025-build-engineer-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0025-build-engineer-review.md" 'compiler invocation'
require_text "$review_dir/ADR-0025-build-engineer-review.md" 'target packs'
require_text "$review_dir/ADR-0025-build-engineer-review.md" 'artifact compatibility'

require_text "$review_dir/ADR-0025-spec-compliance-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0025-spec-compliance-review.md" 'accepted source of truth'
require_text "$review_dir/ADR-0025-spec-compliance-review.md" 'diagnostic'
require_text "$review_dir/ADR-0025-spec-compliance-review.md" 'primary span or external input location'

require_text "$review_dir/ADR-0025-simplicity-review.md" '^Decision: request-revision-before-acceptance$'
require_text "$review_dir/ADR-0025-simplicity-review.md" 'small frontend module model'
require_text "$review_dir/ADR-0025-simplicity-review.md" 'defer'

require_text "$review_dir/ADR-0025-chief-architect-decision.md" '^Decision: pending-revision$'
require_text "$review_dir/ADR-0025-chief-architect-decision.md" 'not accepted'
require_text "$review_dir/ADR-0025-chief-architect-decision.md" 'M0014 remains blocked'

require_text "$task" 'Status: `complete`'

require_absent_text docs/SPEC.md '^## ADR-0025: Module Package And Visibility Model$'
require_absent_text crates/newlang/src/lib.rs 'pub mod module|pub mod modules|pub mod name_resolution'
require_absent_text crates/newlang/src/parser.rs 'ModuleId|PackageId|VisibilityModel|DefaultVisibility|InternalVisibility|ModuleDependency'

echo "m0014-review: module package and visibility model proposal review validation passed"
