# ADR-0055 Simplicity Review

Decision: approve. An explicit companion arena reuses the existing TypeArena
and avoids copied arenas, global primitive IDs, or a premature runtime-type
abstraction.
