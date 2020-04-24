//specify location of library
target:$[.z.x[0] ~ "m32"; "i686-unknown-linux-gnu/"; ""];
lib:hsym`$getenv[`PWD],"/target/",target,"debug/libembedded"; /if executing `cargo run`
/ lib:`:libembedded  //if lib copied to $QHOME

-1 "1. Loading lib:",string lib;

//load exported functions from shared library
identityConstK: lib 2: (`identityConstK;1)
identityKAny:   lib 2: (`identityKAny;1)
throw:          lib 2: (`throw;1)

//test
-1 "2. Running tests.";
`test1 ~ identityConstK `test1
1 2 3. ~ identityKAny 1 2 3.
"test_error" ~ @[throw;`;{x}]

-1 "3. Tests completed. Exiting.";
//exit ;-)
exit 0
