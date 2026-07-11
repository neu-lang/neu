# Ambiguity: Class And Field Surface Syntax

Status: Open

## Affected Work

Task-008, class and interface parser, AST metadata, field type checking, and
member access lowering.

## Authority Gap

ADR-0065 accepts nominal classes, typed `val`/`var` fields, `fun` methods,
implicit `this`, and member visibility, but it does not define the concrete
source grammar needed by the implementation. ADR-0022 and ADR-0024 defer these
forms. In particular, the authority does not state:

- the class declaration keyword and header grammar;
- superclass and interface list syntax;
- field declaration syntax and whether fields require initializers in the
  declaration or constructor;
- member-body separators and whether method declarations may appear beside
  fields;
- the AST identity and receiver syntax for field access; or
- the object-creation expression available before constructor semantics.

## Competing Interpretations

1. Kotlin-like `class Name : Base(), Interface { val field: Int = 0 }` with
   `receiver.field` access.
2. Existing `struct` syntax extended with `val`/`var` fields and nominal class
   semantics.
3. Header-only class declarations with fields deferred until constructor
   syntax is accepted.

## Why Guessing Is Unsafe

Choosing between these forms changes reserved keywords, declaration identity,
inheritance parsing, initialization obligations, name resolution, and future
ABI contracts. Task-008 cannot satisfy its acceptance criteria without also
inventing object creation and field-initialization behavior that ADR-0067
explicitly assigns to the constructor task.

## Required Resolution

Superseding or revising authority must specify the class and field grammar,
inheritance/interface header syntax, field initialization placement, and the
minimum object-creation form permitted before task-009. Until then, class and
field implementation remains stopped; existing primitive, array, and String
work is unaffected.
