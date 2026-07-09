#!/usr/bin/env sh
set -eu

usage() {
  echo "usage: docs/scripts/review-task.sh docs/tasks/M####-NNN-slug.md" >&2
  exit 2
}

[ "$#" -eq 1 ] || usage

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
repo_root="$(CDPATH= cd -- "${script_dir}/../.." && pwd)"
docs_dir="${repo_root}/docs"
cd "$repo_root"

task_file="$1"
[ -f "$task_file" ] || { echo "error: task file not found: $task_file" >&2; exit 1; }
[ -f "${docs_dir}/REVIEW_TEMPLATE.md" ] || { echo "error: docs/REVIEW_TEMPLATE.md not found" >&2; exit 1; }

task_base="$(basename "$task_file" .md)"
task_id="$(printf "%s" "$task_base" | sed 's/^\(M[0-9][0-9][0-9][0-9]-[0-9][0-9][0-9]\).*/\1/')"
milestone_id="$(grep -E '^- Milestone: `M[0-9][0-9][0-9][0-9]`$' "$task_file" | sed 's/^- Milestone: `//; s/`$//' | head -n 1)"

[ -n "$milestone_id" ] || { echo "error: milestone metadata missing" >&2; exit 1; }

milestone_count="$(grep -E '^- Milestone: `M[0-9][0-9][0-9][0-9]`$' "$task_file" | wc -l | tr -d ' ')"
[ "$milestone_count" = "1" ] || { echo "error: task must reference exactly one milestone" >&2; exit 1; }

milestone_file="$(find docs/milestones -maxdepth 1 -type f -name "${milestone_id}-*.md" | sort | head -n 1)"
[ -n "$milestone_file" ] || { echo "error: milestone file not found for $milestone_id" >&2; exit 1; }

grep -Eq '^- Specification: `docs/SPEC\.md`$' "$task_file" || {
  echo "error: review requires task to reference docs/SPEC.md" >&2
  exit 1
}

mkdir -p "docs/tasks/reviews"
review_file="docs/tasks/reviews/${task_id}-review.md"
today="$(date +%Y-%m-%d)"
milestone_base="$(basename "$milestone_file")"

sed \
  -e "s|<TASK_ID>|${task_id}|g" \
  -e "s|<MILESTONE_ID>|${milestone_id}|g" \
  -e "s|tasks/<TASK_ID>-<slug>.md|${task_file}|g" \
  -e "s|docs/milestones/<MILESTONE_FILE>.md|${milestone_file}|g" \
  -e "s|<MILESTONE_FILE>|${milestone_base}|g" \
  -e "s|YYYY-MM-DD|${today}|g" \
  "${docs_dir}/REVIEW_TEMPLATE.md" > "$review_file"

echo "created $review_file"
echo "review must compare task output against docs/SPEC.md and $milestone_file"
