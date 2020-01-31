feature_macros
---

This crate allows for controling how safe/unsafe other crates are.

## Example

This macro _can_ (but should not necessarily) be used identically to
`unreachable!` or `panic!`.

```rust

    match x {
        Foo::Bar => bar(&x),
        Foo::Baz => baz(&x),
        _ => inconceivable!(),
    } 

```

## Developer Controllable Options

* `ub_inconceivable`: This controls the semantics of the `inconceivable!` macro. When this options is not supplied (or when this options is supplied, and the crate is compiled with `rustc --version < 1.27`) `inconceivable!` will simply alias `unreachable!`. When this option is supplied (and the crate is compiled with `rustc --version >= 1.27`) this will instead emit `unreachable_uncheck()` which is UB.

## Developer Uncontrollable Options

* `RUSTC_VERSION_GE_1_27`: States if `rustc --version >= 1.27` this is used as a feature check.

