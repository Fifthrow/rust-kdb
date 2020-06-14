// run:  
/   cargo build --target=i686-unknown-linux-gnu && q src/load.q m32
//specify location of library
target:$[.z.x[0] ~ "m32"; "i686-unknown-linux-gnu/"; ""];
lib:hsym`$getenv[`PWD],"/target/",target,"debug/libso_utils"; /if executing `cargo run`
/ lib:`:libpipe_fd  //if lib copied to $QHOME

-1 "1. Loading lib:",string lib;

//load exported functions from shared library
add:            lib 2: (`add;2);
rdtsc:          lib 2: (`rdtsc;1);
cpus_freq:      lib 2: (`cpus_freq;1);
high_res_time:  lib 2: (`high_res_time;1);

//test
-1 "2. Executing imported functions:";
-1 "   * add[3;2]=5:", .Q.s1 5=add[3;2];
-1 "   * .[add;(`sym;1);(::)]~\"type\":", .Q.s1 .[add;(`sym;1);(::)]~"type";

-1 "   * rdtsc[] - rdtsc[]:", string rdtsc[]-rdtsc[];
-1 "   * cpus_freq[]:", .Q.s1 cpus_freq[];
-1 "   * (high_res_time[];.z.p):", .Q.s1 (high_res_time[];.z.p);

