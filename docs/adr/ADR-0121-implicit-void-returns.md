# ADR-0121: Implicit Void Returns

Status: Accepted

## Decision

Neu uses an omitted return annotation for functions that produce no result:

```neu
func do_something() {
    return;
}
```

The compiler normalizes this declaration to an internal `Void` result. `Void`
is compiler-owned and is not a user-facing type name. The former public `Unit`
type and `()` literal are removed and must be diagnosed if encountered.

No-result functions may fall through or use a bare `return;`. A value return is
invalid in a no-result function. Functions with an explicit non-void return
annotation must return a value on every reachable path; bare returns and
fallthrough are invalid there.

Function types may omit their result after the arrow (`() ->`); this is
normalized to the same internal `Void` result. Calls, methods, interfaces,
generic signatures, conditionals, HIR, MIR, and native test functions use the
normalized result and do not expose a separate Unit value.

Expression statements, `if` statements without a value, `assert`, and `fail`
are no-result operations. Native lowering emits no result register or ABI
payload for them. This convention does not add a `void` keyword, a standard
library type, or an I/O API.

## Consequences

The parser accepts omitted declaration returns and omitted function-type
returns, while rejecting `: Unit` and `()`. Type checking and lowering remain
type-complete by using the compiler-private `Void` identity, preserving the
existing zero-result native ABI.
