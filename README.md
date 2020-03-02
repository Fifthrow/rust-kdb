rust-kdb is an idiomatic Rust wrapper around the C API for KDB+, the ultra fast time series database from KX Systems.

Check out the examples for more information on using it. Performance should be extremely good - 
there is little to no overhead over and above using the API directly.

# Compilation

In order to build the library  or run the tests or any of the examples, you'll need to have libkdb.a available somewhere in LIBRARY_PATH. If you'd rather not use library path, you can set the variable LKDB_LIB_DIR instead.

## Embedding
To use the library in an embedded context, compile with the the *embedded* feature. Make sure that you are compiling with the right architecture, and linking to the right version of libkdb.a for that architecture (either the 32-bit or 64-bit edition).

# Future plans

1. Table support!
2. There are a few API calls's that aren't supported yet, notably (de)serialization.
3. There are a couple of places where the API is not very clean, for example working with mixed lists. We are looking at providing nicer mechanisms to work with them.
4. Add better integration between chrono/std::time and the KDB time types.
5. Round out some of the edge cases and ensure it's fully sound.
