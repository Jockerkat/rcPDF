# Coding Style Guide

## Table of Contents

- [Formatting](#formatting)
- [Imports](#imports)
- [Setters/Getters](#setters-and-getters)

## Formatting

The entire project is formatted using the default settings of *Rustfmt*, unless specified otherwise in `rustfmt.toml`.

## Imports

To import a struct `Foo` from the file `foo.rs`, found in directory `/src/bar/`, use the following syntax:

```rust
use crate::bar::foo::Foo;

// use struct `Foo` here
```

This applies to structs, traits, and static variables.
Exceptions to this rule can be made for clarity purposes.

## Setters and Getters

Use the following convention for a field `foo: T`:

- A method `foo(&self) -> &T` for **getting** the field's value.
- A method `set_foo(&self, value: T)` for **setting** the field's value (depending on the context, the `value` argument may take `&T` or some other type).

This convention does not apply to builder objects.
