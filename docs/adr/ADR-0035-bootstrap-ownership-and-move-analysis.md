# ADR-0035: Bootstrap Ownership And Move Analysis

Status: Accepted

## Question

What smallest ownership and move-analysis subset can the compiler implement without
assuming full constructors, calls, destructors, copy traits, borrow checking, or
runtime representation?

## Competing Designs

1. Define a complete value-category system now, including primitive layout,
   user-defined copy declarations, destructors, partial moves, calls, captures,
   and returns.
2. Classify the existing primitive type identities into copyable trivial
   identities and move-only resource-like identities, classify nominal
   user-defined type identities as move-only, then analyze local name uses and
   local initializer transfers only.
3. Defer all ownership analysis until HIR exists.
4. Implement diagnostics only, without a semantic move-state model.

## Trade-Offs

Design 1 gives a more complete ownership story but would require decisions
that the compiler does not need and that later borrow, destructor, coroutine, and FFI
future work must still refine.

Design 2 creates a testable bootstrap safety pass from existing typed and name
metadata. It remains conservative, but it can diagnose accidental use after a
local ownership transfer without deciding calls, destructors, or layout.

Design 3 avoids premature choices but blocks the project plan from validating the
core no-GC/no-manual-memory-management discipline before borrow checking.

Design 4 could produce user-visible errors, but without a move-state model it
would not give later analysis passes reliable safety inputs.

## Decision

Choose design 2 for this implementation.

The bootstrap ownership pass classifies values by accepted type identity:

- Copyable primitive identities: `Bool`, `Int`, `Unit`, and `Null`.
- Move-only primitive identities: `String`.
- Move-only nominal identities: all user-defined types in the current module,
  including bootstrap enums.
- Explicitly copyable user-defined types: deferred.

This classification is a language-semantic bootstrap rule only. It does not
define ABI layout, destructor behavior, heap representation, traits, interfaces,
or generic capability satisfaction. Owned string storage, cloning, cleanup, and
runtime allocation are defined by ADR-0064.

## Move Sites

The compiler recognizes only these ownership transfer sites:

- a local `val` or `var` initializer whose initializer expression is a bare
  resolved local name of move-only type;
- an assignment statement whose value expression is a bare resolved local name
  of move-only type.

At a transfer site, the source binding becomes moved from the transfer
expression onward. A later bare local-name expression referring to that binding
is invalid unless a new binding with the same name shadows it in a nested or
later scope.

Copyable values do not enter the moved state at these sites.

## Non-Move Sites And Deferrals

The compiler does not treat these as move sites:

- local initializer or assignment values that are literals, grouped
  expressions, `if` expressions, `when` expressions, block expressions, member
  expressions, call expressions, or qualified names;
- `when` subject evaluation;
- `return` expressions;
- function argument passing;
- capture by lambdas, closures, coroutine frames, or child tasks;
- pattern binding, destructuring, member projection, partial moves, or field
  moves.

Unsupported or unanalyzable ownership forms are not accepted as safe move
behavior. They either produce an ownership diagnostic only when an accepted
diagnostic rule exists, or are ignored by this implementation and left to a later accepted
later accepted rule when no ownership rule exists.

## Move-State Model

The pass produces side-table metadata and diagnostics. It does not rewrite the
AST and does not lower to HIR.

For each analyzed local binding, the pass records:

- whether the binding's type is copyable or move-only;
- transfer sites where a move from the binding occurs;
- invalid later uses of the moved binding.

The pass uses lexical statement order and existing resolved local-binding
identity. It does not perform control-flow joins, loop analysis, path-sensitive
initialization, destructor scheduling, borrow checking, or alias analysis.

For an `if` or `when` branch, moves inside the branch are not propagated out of
the branch by this bootstrap subset. A later ADR must define path-sensitive
ownership joins before branch-local moves affect following statements.

## Diagnostics And Recovery

Diagnostic: `use_after_move`

- Primary span: the later local-name expression that attempts to use a moved
  binding.
- Secondary span: the transfer expression that moved the value.
- Recovery action: treat the later expression as still having its original
  type for cascading type recovery, but do not clear the moved state.
- Safe suggestion policy: say that the value was moved and identify the move
  origin. Do not suggest cloning, borrowing, copying, adding `copy`, changing
  mutability, inserting allocation, or changing function signatures unless a
  future accepted rule defines that suggestion.

Diagnostic: `unsupported_ownership_rule`

- Primary span: the expression or statement requiring ownership behavior not
  accepted by this ADR.
- Recovery action: omit ownership facts for the unsupported construct and
  continue checking independent local transfers.
- Required stable rule identifier examples:
  - `move_from_non_name_deferred`
  - `branch_move_join_deferred`
  - `call_move_deferred`
  - `return_move_deferred`
  - `when_subject_move_deferred`

The compiler prefers no ownership diagnostic over an unsupported diagnostic when the
construct is outside the accepted ownership surface and no later use would
otherwise be proven invalid.

## Consequences

The compiler can implement a real ownership side table, copyability check, and
use-after-move diagnostic using existing local binding and type metadata.

The compiler borrow checking can depend on the fact that a moved binding is not usable
through later bare-name local expressions in the analyzed subset.

`String` remains move-only, while its cloning, borrowing, cleanup, and opaque
runtime representation are now defined by ADR-0064. That ADR supersedes this
ADR's string-storage deferral without changing the move-only classification.

## Dependencies And Supersession

Depends on ADR-0001, ADR-0004, ADR-0005, ADR-0010, ADR-0015, ADR-0024,
ADR-0026, ADR-0027, ADR-0029, ADR-0032, ADR-0033, and ADR-0034.

This resolves the corresponding ambiguity report. It does
not supersede generic copyability or capability-bound deferral in ADR-0032.
