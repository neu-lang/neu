# Soundness Report: M0029-004

## Decision

Pass. Lowering requires an explicit clean frontend input and a type for every
lowered expression. Unsupported shapes, missing facts, and unclean input fail
before HIR construction; direct calls use the accepted direct-call result type.
