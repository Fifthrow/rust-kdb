`cargo run` will build test application and shared objects.
Test application will:
 -  `exec` to `q`
 -  load exported functions 
 -  run simple q tests

Make sure:
 - `q/kdb` is properly installed, means
 - to export libraries in `LD_LIBRARY_PATH` or `LKDB_LIB_DIR`

> Test app designed only to run `debug` targets (due to paths to build).

```console
 #64bit
 root$ export LKDB_LIB_DIR=/home/user/path/to/lib/kdb/l64
 root$ cargo run

 #32bit
 root$ export LKDB_LIB_DIR=/home/user/path/to/lib/kdb/l32
 root$ cargo run --target=i686-unknown-linux-gnu -- m32
```
                                                                          
