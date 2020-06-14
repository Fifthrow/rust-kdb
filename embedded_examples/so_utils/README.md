Simple shared object functions written using rust-kdb.
This repo implements few [examples from code.kx.com](https://code.kx.com/q/interfaces/using-c-functions/#portable-example):
 - `add`: add to long numbers, throws `type` error on missmatched tipes
 - `rdtsc`: reads Intel x86 CPU high resolution Time-Stamp-Counter 
 - `cpus_freq`: parse `/proc/cpuinfo` file to extract all CPUs clock speed
 - `high_res_time`: use realtime system clock to get high precision/resolution timestamp


Make sure:
 - `q/kdb` is properly installed, means

```console
 #64bit
 root$ cargo build && q src/load.q

 #32bit
 root$ cargo build --target=i686-unknown-linux-gnu && q src/load.q m32
```

