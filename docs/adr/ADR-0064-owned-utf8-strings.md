# ADR-0064: Owned UTF-8 Strings

Status: Accepted

## Question

What string semantics can Neu support without garbage collection, user-managed
deallocation, hidden copying, or a stable public runtime ABI?

## Decision

Neu supports immutable owned UTF-8 strings. Existing double-quoted literals
remain the only source spelling in this implementation phase and decode only the escape
forms already accepted by ADR-0021. Raw strings, interpolation, character
literals, Unicode escape syntax, and multiline strings remain deferred.

`String` is move-only. Read-only uses create implicit shared borrows and leave
the source usable. Storage, returns, and consuming parameters transfer
ownership. `clone(value)` creates an independent owned copy and leaves the
source usable. `val` and `var` bindings follow the existing binding rules;
`const` strings remain invalid compile-time constants under ADR-0061.

String length is the number of UTF-8 bytes and has type `Int`. Indexing uses
`value[index]`, requires an `Int` index, and returns `Byte`. Known negative or
out-of-range indices receive diagnostics; dynamic invalid indices trap and
never wrap. Equality and inequality compare byte sequences. `String + String`
allocates a new owned concatenation and leaves both operands usable.

`.length` and indexing are compiler-recognized built-ins, not general member
dispatch. `clone(value)` is a compiler-recognized built-in function. No public
allocator, deallocator, substring, slice, formatting, parsing, or Unicode text
API is introduced.

String literals use compiler-managed immutable storage. Clones and
concatenations use compiler-inserted allocation and destruction supplied by the
host compiler runtime. Allocation failure traps. The opaque internal value
contains sufficient pointer and byte-length information for the compiler but
has no stable public or FFI layout.

Existing ownership effects classify length, indexing, equality, and
concatenation as read-only uses; clone returns a new owned value; storage,
returns, and consuming calls preserve move-only transitions and diagnostics.
`String` remains `Send` but not `Share` under the existing capability model.

## Intermediate And ABI Contract

HIR preserves decoded bytes, literal source spans, exact String types, built-in
operations, ownership facts, and source mappings. MIR preserves owned string
values, byte loads, length reads, equality, concatenation, clone operations,
allocation-failure traps, and cleanup boundaries. Backend lowering may use an
internal target-specific representation only through the host compiler runtime;
it must not expose that representation through FFI or public symbols.

The host compiler runtime provides compiler-only allocation and deallocation
artifacts. The allocator is not a standard-library API and is not directly
callable by Neu source.

## Diagnostics And Deferrals

Diagnostics include malformed or invalid string literals, String type mismatch,
invalid String operations, static invalid byte indices, dynamic bounds traps,
allocation failure, and use-after-consumption with the existing ownership
provenance model.

Substrings, slices, formatting, parsing, Unicode classification, case
conversion, normalization, interpolation, FFI, general method dispatch,
standard-library modules, and alternate string encodings remain deferred.

## Dependencies

- ADR-0021
- ADR-0027
- ADR-0035
- ADR-0037
- ADR-0041
- ADR-0042
- ADR-0044
- ADR-0045
- ADR-0046
- ADR-0047
- ADR-0055
- ADR-0100
- ADR-0061
- ADR-0062
