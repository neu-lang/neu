# ADR-0136: Interface Capability Declarations and Derived Class Capabilities

Status: Accepted by the project owner. Revises the capability catalog in ADR-0037 for classes
and enums. Extends ADR-0023 and ADR-0084. Open questions Q1 and Q2 are resolved as recommended.

## Question

ADR-0135 requires `Error` to declare `Send` and `Share`, and every implementing class to
satisfy both. Two things block that:

1. No accepted syntax declares a capability on an interface. ADR-0023 defines
   `capability-bound-list` only for generic parameters.
2. ADR-0037 states that current-module nominal user-defined types "satisfy neither
   capability in this implementation," and defers user-declared capability
   implementations. Under that catalog, no error class can satisfy `Send` or `Share`.

## Decision

### Syntax

An interface may declare capability requirements using the ADR-0023
`capability-bound-list`, after the interface name:

    interface Error: Send & Share {
        func kind(): ErrorKind;
    }

Only `Send` and `Share` are accepted names. Any other name is a diagnostic.
Commas do not separate bounds (ADR-0023).

### Obligations and satisfaction

1. A declared capability on an interface is a requirement on every class that
   implements the interface. The requirement is checked at the class declaration,
   under the implementation check of ADR-0066, and a failure is a source-mapped
   constraint diagnostic (ADR-0084). A failing class is rejected.
2. An interface value satisfies a capability if and only if the interface declares it.
3. A class satisfies `Send` (and `Share`) if and only if every field's type satisfies it.
   A class with no fields satisfies both. This is derived by the compiler; it is not
   declared by the user.
4. A nullable field satisfies a capability when its non-null base type does (ADR-0037).
5. An interface-typed field satisfies a capability only when rule 2 holds for that
   interface.
6. A field whose type is an uninstantiated generic parameter keeps the proof obligation
   of ADR-0084. It satisfies neither capability in an uninstantiated class.
7. Enums follow the same rule over their payload types. A generic enum instance such as
   `Result<T, Error>` satisfies a capability when every type argument does, by the
   substitution rule of ADR-0084.
8. A cycle through fields (a recursive type) is conservative: the cyclic class does not
   satisfy a capability unless the cycle is resolved without it. Recursive derivation is
   not iterated to a fixed point.

### Scope of the ADR-0037 revision

This ADR replaces ADR-0037's statement that current-module nominal user-defined types
satisfy neither capability, for classes and enums only, with rules 3 and 7. Other
unsupported types keep ADR-0037's rule. ADR-0037's deferral of user-declared capability
implementations stays in force. A user cannot write `impl Send for X`.

## Non-goals

- User-declared capability implementations or overrides.
- Synchronization primitives, atomics, locks, or unsafe capability overrides (ADR-0037).
- Closures or task-spawn capture rules (ADR-0089).
- Any change to ownership classification (ADR-0001, ADR-0002).

## Consequences

- `Error` can be declared `Send & Share` and implemented by error classes with no fields
  (ADR-0137).
- `Result<T, Error>` is `Send` and `Share` when `T` is, which the current catalog does not
  allow.
- A class is no longer uniformly neither: its capability follows its fields. Programs that
  relied on the old catalog may now be accepted, and the ADR-0037 test expectations for
  user-defined types must be revised.

## Open questions

- **Q1. Derived or declared per class. Resolved: derived.** Capabilities follow fields
  (rule 3). Per-class declaration stays deferred under ADR-0037.
- **Q2. Recursive types. Resolved: conservative.** Rule 8 applies. Recursive classes are
  not rejected outright.

## Required follow-up

Negative and positive tests, written first:

- a class with a non-`Send` field is rejected where its interface declares `Send`;
- a class with no fields satisfies `Send` and `Share`;
- an interface value satisfies a capability only when it is declared;
- a recursive class does not satisfy a capability through the cycle;
- a generic class with an uninstantiated parameter does not satisfy a capability;
- `Result<Int, Error>` satisfies `Send` and `Share`, and `Result<Int, Unsupported>` does not;
- an unknown capability name on an interface is a diagnostic.

`docs/SPEC.md` needs the ADR-0037 revision and this ADR's section before acceptance.

## Dependencies

ADR-0001, ADR-0002, ADR-0014, ADR-0023, ADR-0032, ADR-0037, ADR-0066, ADR-0067,
ADR-0079, ADR-0084, ADR-0109, ADR-0135.
