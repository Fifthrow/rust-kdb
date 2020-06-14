`cargo build` will build shared objects library which once loaded will: 
 - create a pipe in `start` function
 - spawn a thread with write end of pipe,
 - register pipe read callback on q `select` poll,
 - push simple string each 1000uS to a pipe,
 - callback will read bytes from pipe,
 - convert payload to symbol and push to `upd` function declared in `load.q`

Make sure:
 - `q/kdb` is properly installed, means

```console
 #64bit
 root$ cargo build && q src/load.q

 #32bit
 root$ cargo build --target=i686-unknown-linux-gnu && q src/load.q m32
```
                                                                          
