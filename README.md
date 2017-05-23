# `gumdrop`

Option parser with custom derive support

[Documentation](https://docs.rs/gumdrop/)

## Building

To include `gumdrop` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
gumdrop = "0.1"
gumdrop_derive = "0.1"
```

And the following to your crate root:

```rust
extern crate gumdrop;
#[macro_use] extern crate gumdrop_derive;
```

## License

`gumdrop` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
