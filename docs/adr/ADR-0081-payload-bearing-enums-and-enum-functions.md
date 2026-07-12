# ADR-0081: Payload-Bearing Enums And Enum Functions

## Status

Accepted.

## Decision

This ADR supersedes the payload deferrals in ADR-0079 and the payload-pattern
deferrals in ADR-0080. Enums remain closed nominal types. An enum may declare
immutable payload fields in its header:

```text
enum Color(val rgb: Int) { RED(0xFF0000), GREEN(0x00FF00), BLUE(0x0000FF) }
```

Each variant supplies exactly one argument for each declared field, in field
declaration order. Arguments are evaluated left to right and are type-checked
exactly against the field types. A variant expression `Color.RED(value)`
constructs a new owned enum value. Zero-payload enums retain the
`Color.RED`-style form. Fields are immutable; enum values move unless every
field is copyable under the existing ownership rules. Destruction recursively
destroys payload fields in reverse declaration order.

Payload patterns use `Color.RED(binding)` with one binding per payload field.
The binding is scoped to the arm and has the declared field type. A payload
pattern must have exact arity. Expression-form `when` remains exhaustive over
all variants and requires exact arm result types; statement form follows the
same selection order and subject-once rule. Wildcard patterns may cover all
remaining variants but do not bind payloads.

Enum functions are declared inside the enum body after its variants. Ordinary
`func` declarations are instance functions with an implicit non-consuming
receiver named `this`; `static func` declarations are associated functions
without a receiver. Enum functions are final and non-overridable. Their
identity includes enum declaration, static/instance kind, name, and ordered
parameter types. Existing overload, ownership-effect, visibility, and
capability rules apply. Instance calls use `value.method(...)`; associated
calls use `Color.method(...)`.

Enum payloads and enum-function parameters/returns use the accepted internal
ABI type set. HIR and MIR preserve enum, variant, field, receiver, payload,
ownership, cleanup, and source-span facts. Cranelift and target packs may use a
compiler-private tagged aggregate representation; tag width, field offsets,
symbols, tables, and layout are not public ABI or FFI contracts. Separate
compilation exchanges semantic metadata, not raw layout.

Generic enum declarations and generic enum functions use generic semantics
only after the generic declaration and constraint tasks are accepted. Enum
inheritance, reflection, serialization, dynamic loading, FFI, implicit
conversions, and user allocation APIs remain deferred.

## Dependencies

ADR-0021, ADR-0027, ADR-0035, ADR-0062, ADR-0075, ADR-0079, ADR-0080.
