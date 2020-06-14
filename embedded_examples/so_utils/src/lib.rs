//32bit:  root$ cargo build --target=i686-unknown-linux-gnu && q src/load.q m32
//64bit:  root$ cargo build && q src/load.q

use kdb::raw::types::*;
use kdb::{KAny, KError, KFloatList, KLongAtom, KTimestamp, Unowned};
use std::{
    fs::File,
    io::{self, BufRead},
    time::SystemTime,
};

#[no_mangle]
pub extern "C" fn add(x: Unowned<KAny>, y: Unowned<KAny>) -> KAny {
    if (LONG_ATOM != (*x).k_type()) || (LONG_ATOM != (*y).k_type()) {
        KError::new("type").unwrap().into()
    } else {
        let xx: i64 = **(x.try_as_ref::<KLongAtom>().unwrap());
        let yy: i64 = **(y.try_as_ref::<KLongAtom>().unwrap());
        (xx + yy).into()
    }
}

#[no_mangle]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub extern "C" fn rdtsc(_x: Unowned<KAny>) -> KLongAtom {
    (unsafe { core::arch::x86::_rdtsc() as i64 }).into()
}

// Inline asm is not yet supported in stable. As of 2020 one can experiment with nightly builds
//#![feature(asm)] // should go to the top of the module
/* #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn get_rdtsc() -> u64 {
  let a:u64; let d:u64;
  unsafe { asm!("rdtsc": "=a" (a), "=d"(d));}
  a|(d<<32)
}
*/

#[no_mangle]
pub extern "C" fn cpus_freq(_x: Unowned<KAny>) -> KAny {
    let cpu_file = match File::open("/proc/cpuinfo") {
        Ok(f) => f,
        Err(e) => {
            return KError::new(format!("Can't open 'cpuinfo' file, due:{:?}", e).as_str())
                .unwrap()
                .into()
        }
    };
    let cpu_file = io::BufReader::new(cpu_file);
    let cpus_speed: KFloatList = cpu_file
        .lines()
        .filter(|l| l.as_ref().unwrap().contains("cpu MHz"))
        .map(|l| {
            let a = l.unwrap();
            let a = a.split(":").nth(1).unwrap();
            a.trim().parse::<f64>().unwrap()
        })
        .collect();
    cpus_speed.into()
}

#[no_mangle]
pub extern "C" fn high_res_time(_x: Unowned<KAny>) -> KAny {
    let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    KTimestamp::from((((ts.as_secs() - 10957 * 86400) * 1_000_000_000) + ts.subsec_nanos() as u64) as i64).into()
}
