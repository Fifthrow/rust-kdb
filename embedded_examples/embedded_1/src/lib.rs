//  root$ export LKDB_LIB_DIR=/home/user/path/to/lib/kdb/(l32|l64)
//  root$ cargo run
//  root$ cargo run --target=i686-unknown-linux-gnu -- m32

use kdb::raw::types::*;
use kdb::{c_api, KAny, KError, KSymbolAtom, Unowned};

#[no_mangle]
pub extern "C" fn identityConstK(x: Unowned<KAny>) -> *const K {
    unsafe { c_api::r1(x.as_k_ptr()) }
}

#[no_mangle]
pub extern "C" fn identityKAny(x: Unowned<KAny>) -> KAny {
    x.to_owned()
}

#[no_mangle]
pub extern "C" fn throw(_qname: Unowned<KAny>) -> KAny {
    KError::new("test_error").unwrap().into()
}
