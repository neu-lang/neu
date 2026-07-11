#!/usr/bin/env sh
set -eu

rg -q 'ADR-0053: Bootstrap Unsupported Executable-Form Diagnostics' docs/SPEC.md
rg -q 'outermost unsupported form' docs/adr/ADR-0053-bootstrap-unsupported-executable-form-diagnostics.md
rg -q 'Status: `resolved`' docs/ambiguities/M0028-unsupported-executable-form-diagnostics.md
printf '%s\n' 'm0028 unsupported executable-form semantics accepted'
