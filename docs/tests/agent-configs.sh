#!/usr/bin/env sh
set -eu

fail() {
  echo "agent-configs: $*" >&2
  exit 1
}

[ -f AGENTS.md ] || fail "AGENTS.md must be at the repository root"
[ ! -e docs/AGENTS.md ] || fail "docs/AGENTS.md must not exist"
[ -f .codex/config.toml ] || fail "missing .codex/config.toml"
grep -Eq '^max_threads = 6$' .codex/config.toml || fail "max_threads must be 6"
grep -Fq '## Context, Parallelism, And Report Budget' AGENTS.md ||
  fail "AGENTS.md must define the context and report budget"
grep -Fq '## Review Routing' docs/TASK_TEMPLATE.md ||
  fail "task template must define conditional review routing"
grep -Fq '## Authority Extract' docs/TASK_TEMPLATE.md ||
  fail "task template must define a bounded authority extract"
grep -Fq 'Successful reviews should be no more than 150 words' docs/REVIEW_TEMPLATE.md ||
  fail "review template must define concise successful reviews"

set -- .codex/agents/*.toml
[ "$#" -eq 13 ] || fail "expected 13 TOML agent definitions, found $#"

if find .codex/agents -maxdepth 1 -type f -name '*.md' | grep -q .; then
  fail "Markdown agent definitions remain under .codex/agents"
fi

for file in "$@"; do
  grep -Eq '^name = ".+"$' "$file" || fail "missing name in $file"
  grep -Eq '^description = ".+"$' "$file" || fail "missing description in $file"
  grep -Eq '^model = "gpt-5\.6-(sol|terra|luna)"$' "$file" ||
    fail "model must use the GPT-5.6 family in $file"
  grep -Eq '^model_reasoning_effort = "(low|medium|high)"$' "$file" ||
    fail "reasoning effort must be low, medium, or high in $file"
  grep -Eq '^sandbox_mode = "workspace-write"$' "$file" ||
    fail "sandbox mode must be workspace-write in $file"
  grep -Eq '^developer_instructions = """$' "$file" ||
    fail "missing developer instructions in $file"
  grep -Fq '## Efficiency Rules' "$file" ||
    fail "missing efficiency rules in $file"
done

if grep -R -Eq 'model_reasoning_effort = "xhigh"|model = "gpt-5\.[0-5]' .codex/agents; then
  fail "unsupported model or reasoning effort found"
fi

if grep -R -Eq '\.codex/agents/[A-Za-z*_-]+\.md' \
  AGENTS.md docs/TASK_TEMPLATE.md .codex/agents; then
  fail "active agent-system files still reference Markdown role definitions"
fi

echo "agent-configs: 13 GPT-5.6 TOML agent definitions validated"
