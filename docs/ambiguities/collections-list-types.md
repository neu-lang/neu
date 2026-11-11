# Ambiguity Report: `List<T>`, `ArrayList<T>`, `LinkedList<T>`

Status: Decided by the project owner; implementation still gated on the superseding
ADRs listed under "Resolutions". No code has been written.

## Resolutions

- A1 (growth failure): `Result`. Amend ADR-0073 and ADR-0114 so that growth-requiring
  `add` on `T[]` returns a recoverable `Result` and leaves the list unchanged on failure.
  Requires a compiler growth primitive that reports failure.
- A2 (move-only removal): move-out `remove(index)` returns the owned element as `T`
  inside a bounds `Result` (ADR-0110). Revises ADR-0073's `Unit`-returning `remove`.
- A3 (generic collection fields): ADR-0126 supersedes the ADR-0119 clause deferring
  generic collection fields.
- A4 (`LinkedList`): deferred. Not implemented in this phase.
- A5 (sequence type): retain only `ArrayList<T>`. `Vector<T>` is removed from the package.
  This supersedes the `Vector<T>` naming in ADR-0108, ADR-0110, ADR-0118, and ADR-0126,
  and in issue #20's sequence scope. Those ADRs need a superseding revision before
  `stdlib/collections/vector.neu` is deleted.
- A6 (`List<T>` method set): proposed, not yet approved. See "Proposed List<T> set" below.

## Proposed `List<T>` set (pending approval)

Required interface methods:
- `len(): Int`
- `get(index: Int): Option<T>`. Checked; returns an owned copy. Available only when `T` is copyable. See open point B1.
- `set(index: Int, value: T): Result<Unit, E>`. Replaces the element at a valid index; the old element is destroyed.
- `push(value: T): Result<Unit, E>`. Growth failure returns `Err` and leaves the list unchanged (A1).
- `insert(index: Int, value: T): Result<Unit, E>`. Valid indices are `0..=len`.
- `remove(index: Int): Result<T, E>`. Returns the owned element (A2).
- `pop(): Option<T>`. Returns the owned last element, or `None` when empty.
- `clear(): Unit`. Removal and clear report no allocation failure (ADR-0114).

Default methods (ADR-0134):
- `is_empty(): Bool` defined as `len() == 0`.
- `first(): Option<T>` and `last(): Option<T>`, defined through `get`.

Deferred, not in the set:
- `capacity`, `reserve`, `shrink`: ADR-0073 exposes no capacity, so the issue's capacity scope needs a decision (B2).
- `contains`, `index_of`, `deduplicate`: need `Eq` (ADR-0113).
- `clone`: waits on the `Clone` protocol (#62).
- `append`, `extend`, `retain`, `reverse`: bulk operations, deferred to a later phase.
- Iterators and slices: gated on ADR-0118's borrow and iterator wiring.

Open points for approval:
- B1. **Decided by the project owner:** `E` is `stdlib` `Error`, an interface. The concrete
  errors are classes that implement it, starting with `IndexOutOfBoundsError` and
  `AllocationFailedError`. This replaces the earlier proposed `enum CollectionError`.
  Drafted in `docs/adr/ADR-0135-stdlib-error-interface.md` (Proposed). Implementation is blocked
  until the following are accepted in that ADR:
  1. **Interface-typed `Err` payloads.** `Result<T, Error>` holds an interface value. Nothing
     accepted covers an interface-typed payload in a generic enum (ADR-0033 defers generic
     enums and interfaces at bootstrap).
  2. **How callers tell errors apart.** ADR-0079 forbids enums from implementing interfaces,
     and SPEC defers downcasts and runtime type tests. A caller holding `Error` therefore
     cannot branch on `IndexOutOfBoundsError` versus `AllocationFailedError`. Proposed fix:
     `Error` declares `func kind(): ErrorKind`, where `ErrorKind` is a closed enum, so
     callers can use exhaustive `when`.
  3. **`Send`/`Share` of interface values.** ADR-0111 says `Err(E)` carries the properties of
     `E`. Accepted ADRs don't say whether an interface value is `Send` or `Share`
     (ADR-0014).
- B2. Capacity. Keep it hidden (ADR-0073), or expose a query under a new ADR?
- B3. `get` on move-only `T`. Option (a): `get` requires a copy bound. Option (b): defer `get`, `first`, and `last` until a borrowed-return rule exists.

Requested in chat (not named in issue #20). Authority touching these names:
ADR-0132 (`ArrayList<T>` is ordinary Neu code over `T[]`, private fields), ADR-0134
(`List<T>` as a generic interface with default bodies, example `first()`). No accepted
ADR defines `LinkedList<T>`, and no accepted ADR defines the required method set of `List<T>`.

## A1. Growth failure: trap or recoverable `Result`?

Quoted:
- ADR-0073: "Initial capacity is private and growth is compiler-managed; reaching ... allocation failure traps deterministically." Operations return `Unit`.
- ADR-0067: "Runtime allocation failure traps non-successfully under the existing bootstrap trap contract."
- ADR-0114: "A request that cannot be satisfied returns a recoverable `Result` ... An insertion that requires growth ... either completes ... or returns an error and leaves the prior sequence unchanged."

Competing interpretations:
- (a) ADR-0073/0067 govern `T[]`, so `ArrayList.add` inherits the trap. ADR-0114 applies only to `Vector<T>`.
- (b) ADR-0114 governs `ArrayList.add`, which must return `Result`. This needs a growth primitive that reports failure, which ADR-0073 does not provide.

Why guessing is unsafe: `add`'s signature and failure behavior differ between the two readings. ADR-0114's own follow-up requires tests for failed growth before any growth-sensitive operation is implemented.

## A2. Removing a move-only element

Quoted:
- ADR-0073: `remove(index)` "return[s] `Unit`". ADR-0132 keeps "the operations of ADR-0073 unchanged".
- ADR-0110: "removal ... return[s] an explicit recoverable result."

Problem: returning the removed element (`pop`, `remove -> T`) requires moving an element out of a `T[]`. No accepted rule permits this. Element reads are mentioned in ADR-0132, but moving out of an indexed slot is not defined for move-only `T`.

Competing interpretations:
- (a) Add a move-out removal (`take`/`remove -> T`) as an ADR-0073 revision.
- (b) Keep `remove` as `Unit` and offer the value only through a separate, separately specified read/move rule.
- (c) Restrict value-returning removal to copyable `T`.

## A3. Generic collection fields

Quoted:
- ADR-0119 (Accepted): "Generic collection fields, nominal element destruction, slices, and specialization inference remain deferred."
- ADR-0126 (Accepted, later): "A generic class may declare a `private` field whose type is any collection, including `Array<T>`." It revises ADR-0118's deferral and does not mention ADR-0119.

Likely reading: ADR-0126 supersedes the ADR-0119 clause. ADR-0130 and the `ArrayList` private-storage design depend on it. Needs explicit confirmation before implementing a generic class that owns `T[]`.

## A4. `LinkedList<T>` representation

No accepted ADR covers linked nodes. Options:
- (a) `class Node<T>` with private `value: T` and `next: Node<T>?`. The postfix nullable class form is accepted (SPEC, class/interface nullability). Open points: ADR-0067 defines recursive field destruction in reverse declaration order but states no depth bound, so destroying a long list recurses without limit. Also, destruction of a move-only generic `T` field is not specified for generic classes (ADR-0119 defers nominal element destruction).
- (b) Index-based arena over two `T[]` and an `Int[]` of links with a free list. This avoids nominal elements (ADR-0073 defers them) but depends on A2 (moving elements out) and on free-list behavior that no ADR specifies.

## A5. Relationship between `Vector<T>` and `ArrayList<T>`

ADR-0126 makes `Vector<T>` a generic class over private storage. ADR-0132 makes `ArrayList<T>` ordinary code over `T[]`. Issue #20 names only `Vector`. Competing readings:
- (a) They are one type; `ArrayList` is the name of `Vector`.
- (b) Two separate sequence types with overlapping APIs.
- (c) `ArrayList` replaces `Vector`.

## A6. Required method set of `List<T>`

No ADR lists the methods of `List<T>`. ADR-0134 allows default bodies in generic interfaces, and its example (`first()`) is only illustrative. A surface could be derived from issue #20's `Vector` operations, but the return types of removal depend on A1 and A2.

## Decisions requested

1. A1: trap (ADR-0073) or `Result` (ADR-0114) for growth in `ArrayList.add`?
2. A2: how a removed element is returned to the caller for move-only `T`?
3. A3: confirm that ADR-0126 supersedes the ADR-0119 generic-field deferral.
4. A4: node-based or arena-based `LinkedList`, and the destruction-depth rule for long chains.
5. A5: one sequence type or two?
6. A6: approve the `List<T>` method set once A1 and A2 are settled.

## Affected source of truth

- docs/adr/ADR-0067-object-lifecycle-and-abi.md
- docs/adr/ADR-0073-dynamic-array-bootstrap.md
- docs/adr/ADR-0110-collection-iteration-and-failure-contracts.md
- docs/adr/ADR-0114-collection-capacity-and-allocation-failure.md
- docs/adr/ADR-0119-generic-dynamic-array-elements.md
- docs/adr/ADR-0126-collection-storage-embedding.md
- docs/adr/ADR-0130-private-constructor-parameters.md
- docs/adr/ADR-0132-dynamic-array-type-syntax.md
- docs/adr/ADR-0134-generic-interface-default-methods.md
- docs/SPEC.md (collection and ADR-0118 sections)
- stdlib/collections/ (existing `vector.neu`, `slice.neu`, `deque.neu` still use the pre-ADR-0126 enum shape)
