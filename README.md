# Rust KDB

rust-kdb is an idiomatic Rust wrapper around the C API for KDB+, the ultra fast time series database from KX Systems.

[![Docs.rs][docs-badge]][docs-url]
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[docs-badge]: https://docs.rs/kdb/badge.svg
[docs-url]: https://docs.rs/kdb
[crates-badge]: https://img.shields.io/crates/v/kdb.svg
[crates-url]: https://crates.io/crates/kdb
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

Check out the examples for more information on using it. Performance should be extremely good -
there is little to no overhead over and above using the API directly.

## Compilation

In order to build the library or run the tests or any of the examples, you'll need to have `libkdb.a` available somewhere in `LIBRARY_PATH`. If you'd rather not use library path, you can set the variable `LKDB_LIB_DIR` instead.

## Embedding

To use the library in an embedded context, compile with the the `embedded` feature. Make sure that you are compiling with the right architecture, and linking to the right version of `libkdb.a` for that architecture (either the 32-bit or 64-bit edition).

## Future plans

1. Table support!
2. There are a few API calls's that aren't supported yet.
3. Add better integration between chrono/std::time and the KDB time types.
