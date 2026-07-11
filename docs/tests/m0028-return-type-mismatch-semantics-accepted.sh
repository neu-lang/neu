#!/usr/bin/env sh
set -eu

rg -q 'ADR-0054: Bootstrap Return-Type Mismatch Diagnostics' docs/SPEC.md
rg -q 'attaches to the explicit return expression' docs/adr/ADR-0054-bootstrap-return-type-mismatch-diagnostics.md
rg -q 'Status: `resolved`' docs/ambiguities/M0028-return-type-mismatch-diagnostic-contract.md
printf '%s\n' 'm0028 return type mismatch semantics accepted'
