//  root$ export LKDB_LIB_DIR=/home/user/path/to/lib/kdb/(l32|l64)
//  root$ cargo run 
//  root$ cargo run --target=i686-unknown-linux-gnu -- m32

use kdb::raw::types::*;
use kdb::{KAny, KError, KSymbolAtom};

#[no_mangle]
pub extern "C" fn identityConstK(x: KAny) -> *const K {
    std::mem::ManuallyDrop::new(x).as_k_ptr()
}

#[no_mangle]
pub extern "C" fn identityKAny(x: KAny) -> KAny {
    x
}

#[no_mangle]
pub extern "C" fn throw(_qname: KAny) -> KAny {
    KError::new("test_error").into()
}
