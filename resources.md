# Resources

out of 200 tabs i had open, these were the useful ones

- [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180): The CSV RFC
- [Wikipedia - CSV](https://en.wikipedia.org/wiki/Comma-separated_values): An overview of everything CSV, including the things that the RFC didn't want to cover... ;)
- [Serde Docs - Writing a Data Format](https://serde.rs/data-format.html): Docs
- [`csv` crate's `serializer.rs` file](https://github.com/BurntSushi/rust-csv/blob/master/src/serializer.rs): Decent implementation of CSV serialization. Nice reference for header generation.
- [`csv` crate PR to add `#[serde(flatten)]`](https://github.com/BurntSushi/rust-csv/pull/223): PR to add flattening support to the `csv` crate
- [`serde::Serializer` trait](https://docs.rs/serde/1.0.203/serde/trait.Serializer.html): An explanation of Serde's data model
- [`std::io::BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html): We'll want to use this later on
- [`flexstr`](https://docs.rs/flexstr/latest/flexstr/): Enables static string creation
- [`ufmt`](https://docs.rs/ufmt/latest/ufmt/): An alternative to core::fmt with speed and safety improvements
- [Compile-Time Reflection Report](https://soasis.org/posts/a-mirror-for-rust-a-plan-for-generic-compile-time-introspection-in-rust/): Some thoughts on reflection, which is useful in crates like these
- [`std::compile_error!`](https://doc.rust-lang.org/std/macro.compile_error.html): A nice tool for macro_rules! macros
- [Nine Rules For Creating Procedural Macros in Rust](https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff): Nice article on writing proc macros
-
