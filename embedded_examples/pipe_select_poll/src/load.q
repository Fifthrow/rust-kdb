// run:  
/   cargo build --target=i686-unknown-linux-gnu && q src/load.q m32
//specify location of library
target:$[.z.x[0] ~ "m32"; "i686-unknown-linux-gnu/"; ""];
lib:hsym`$getenv[`PWD],"/target/",target,"debug/libpipe_fd"; /if executing `cargo run`
/ lib:`:libpipe_fd  //if lib copied to $QHOME

-1 "1. Loading lib:",string lib;

//load exported functions from shared library
start: lib 2: (`start;1)
callback:   lib 2: (`callback;1)
close_fd:   lib 2: (`close_fd;1)
upd:{[T;D] 0N!(T;D); };

//test
-1 "2. Running tests.";
-1 "   - created pipe FD pair:", .Q.s1 start[];

-1 "3. You can try to close read side of pipe with: close_fd 3i";

