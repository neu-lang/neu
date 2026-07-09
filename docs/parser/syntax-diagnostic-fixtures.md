# Syntax Diagnostic Fixture Format

Status: M0010 fixture convention

## Fixture Kind

Synthetic parser diagnostic fixtures use:

```toml
kind = "synthetic-parser-diagnostic-fixture"
compiler_behavior = "synthetic-parser-diagnostic-only"
```

These fixtures validate diagnostic shape only. They must not encode declaration, type, expression, statement, or pattern grammar.

## Required Fields

Each fixture must include:

- `kind`
- `milestone`
- `compiler_behavior`
- `source`
- `synthetic_input`
- `synthetic_error`
- `primary_span`
- `recovery_action`

## Golden Diagnostic Fields

Each paired golden diagnostic must include:

- `severity`
- `message`
- `primary_span`
- `source_of_truth`
- `recovery_action`

## Prohibited Content

Until accepted syntax authority exists, parser diagnostic fixtures must not include source programs using concrete declaration, type, expression, statement, or pattern grammar.
