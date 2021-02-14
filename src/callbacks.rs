use crate::{kapi, Any, Atom, Error, KBox};
use std::mem;

/// Callback for using in the `set_callback` function.
pub type Callback = extern "C" fn(i32) -> Option<KBox<Any>>;

/// Registers a callback function to be called when data is available on a particular file descriptor.
/// Equivalent to calling `sd1(fd, cb)` in the C API.
pub fn register_callback(fd: i32, cb: Callback) -> Result<KBox<Atom<i32>>, Error> {
    unsafe {
        let r = kapi::sd1(fd, mem::transmute(cb));
        if r.is_null() {
            Err(Error::Callback)
        } else {
            Ok(KBox::from_raw(r))
        }
    }
}

/// Removes a callback registered on the specified file descriptor with the `register_callback` function
pub fn remove_callback(fd: i32) {
    unsafe { kapi::sd0x(fd, 0) };
}
