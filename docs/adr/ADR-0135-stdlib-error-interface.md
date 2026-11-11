# ADR-0135: Stdlib Error Interface

Status: Proposed

## Question

How should `stdlib/core` define recoverable operation errors so that `Result<T, E>`
can carry more than one error type, callers can distinguish them without downcasts,
and collection operations (ADR-0110, ADR-0114) can report index and allocation
failures?

## Decision

### Declarations

`stdlib/core` declares a nominal interface and a closed kind enum (illustrative
syntax; the declaration grammar follows ADR-0022 and ADR-0065):

    enum ErrorKind {
        IndexOutOfBounds,
        AllocationFailed,
    }

    interface Error {
        func kind(): ErrorKind;
    }

Concrete error types are classes that implement `Error`:

    class IndexOutOfBoundsError: Error {
        func kind(): ErrorKind { return ErrorKind.IndexOutOfBounds; }
    }

    class AllocationFailedError: Error {
        func kind(): ErrorKind { return ErrorKind.AllocationFailed; }
    }

1. `Error` is an ordinary nominal core interface (ADR-0113). It has one required
   method, `kind`. It has no state (ADR-0133) and no default method.
2. `ErrorKind` is closed. Adding a variant changes the exhaustive `when` of every
   caller, so it requires a revision of this ADR.
3. Error values are classes. Enums cannot implement interfaces (ADR-0079), so an enum
   error cannot satisfy `Error`.
4. A class implementing `Error` is assignable to `Error` (static upcast, ADR-0065).
   There is no implicit conversion back. Downcasts and runtime type tests remain
   deferred (SPEC), so no caller tests the concrete type.

### Results

5. `Result<T, Error>` is a valid instantiation. An enum payload may have an interface
   type, and `Err` takes ownership of the interface value.
6. An `Error` value is move-only (ADR-0067), not copyable, and non-null. The nullable
   form `Error?` is written only where a nullable interface type is already allowed
   (ADR-0065).
7. Destroying an `Error` value destroys the concrete object exactly once, in the
   concrete class's field order (ADR-0067). Dispatch metadata stays compiler-private
   (ADR-0070).

### Matching

8. Callers branch on `err.kind()` with an exhaustive `when` over `ErrorKind`. They do
   not branch on concrete class.

### Send and Share (proposed, see O1)

9. An `Error` value carries `Send` and `Share` only if the interface declares them.
   Each implementing class must satisfy the declared capabilities (ADR-0014,
   ADR-0037). The proposed default is that `Error` declares both.

## Non-goals

This ADR does not add:

- Downcasts, runtime type tests, or type-tested pattern matching on errors.
- A `message` method or any `String`-producing method. Those depend on the accepted
  `String` representation (ADR-0064) and on formatting, which is out of scope.
- Error chaining, causes, or stack traces.
- A `?`/`try` operator or implicit propagation (ADR-0111).
- Implicit conversion between error types. Conversion requires an explicit library
  operation.
- Panic or exception handling (ADR-0111, ADR-0123).

## Consequences

- Callers gain a single error type for `Result`, while keeping exhaustive handling
  through `ErrorKind`.
- Concrete error classes can be added later without changing `Result` signatures, as
  long as `ErrorKind` is unchanged.
- Adding an `ErrorKind` variant is breaking for exhaustive callers. This is the
  deliberate cost of a closed kind.
- Every failing `Result` returns an object, so error construction allocates unless
  O2 is resolved.

## Open questions

- **O1. Send and Share.** Should `Error` declare both capabilities, or should interface
  values never carry them? Declaring them forbids error classes with non-`Send`
  fields. Not declaring them forbids sending any `Result<T, Error>` across tasks.
  Accepted ADR-0014 says explicit declaration is used "where necessary"; no accepted
  ADR gives the syntax for declaring capabilities on an interface.
- **O2. Allocation-free construction of `AllocationFailedError`.** Constructing the
  error object may itself need allocation, which is the failure being reported. Options:
  a zero-field class the target can construct without the heap, or a compiler-reserved
  instance. Neither is specified.
- **O3. Location of `ErrorKind` and the error classes.** Proposed: `stdlib/core`, so
  that `collections` and later packages share them. Alternative: per-package kinds.
- **O4. Interface-typed payload in a generic enum.** ADR-0033 defers generic enums and
  interfaces at bootstrap. Accepting decision 5 needs that restriction lifted.

## Required follow-up

Before implementation, add negative tests written first:

- a class that does not implement `Error` is rejected where `Error` is required;
- an enum that claims to implement `Error` is rejected (ADR-0079);
- `Result<T, Error>` is move-only and rejects copying;
- `kind()` exhaustive matching diagnoses a missing `ErrorKind` case;
- destroying an `Error` value runs the concrete class's destruction exactly once;
- `Send` and `Share` checks follow the outcome of O1;
- `AllocationFailedError` construction under simulated allocation failure (O2).

Accepting this ADR requires answers to O1 through O4 and an update to `docs/SPEC.md`
before implementation relies on it.

## Dependencies

ADR-0006, ADR-0014, ADR-0022, ADR-0023, ADR-0033, ADR-0037, ADR-0064, ADR-0065,
ADR-0066, ADR-0067, ADR-0070, ADR-0079, ADR-0107, ADR-0110, ADR-0111, ADR-0113,
ADR-0114, ADR-0123, ADR-0126, ADR-0133, ADR-0134.
