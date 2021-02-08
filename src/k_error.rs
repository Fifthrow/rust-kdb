use std::{ffi::CStr, fmt};

use crate::k_type::ERROR;
use crate::kbox::KBox;
use crate::symbol::symbol;
use crate::type_traits::KObject;
use crate::{error::Error, k::K};

/// Represents an error in KDB.
pub struct KError {
    k: K,
}

impl KObject for KError {
    fn k_ptr(&self) -> *const K {
        &self.k
    }

    fn k_ptr_mut(&mut self) -> *mut K {
        &mut self.k
    }
}

impl KBox<KError> {
    /// Create a new KDB error from the specified string.
    pub fn new_error(msg: &str) -> Self {
        let mut err = KBox::new_atom(symbol(msg)).into_raw() as *mut K;
        unsafe {
            (*err).t = ERROR;
            KBox::<KError>::from_raw(err)
        }
    }
}

impl fmt::Display for KError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cs = unsafe { CStr::from_ptr(self.k.union.s) };
        write!(f, "{}", String::from_utf8_lossy(cs.to_bytes()))
    }
}

impl fmt::Debug for KError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cs = unsafe { CStr::from_ptr(self.k.union.s) };
        write!(f, "{}", String::from_utf8_lossy(cs.to_bytes()))
    }
}
impl From<KBox<KError>> for Error {
    fn from(ke: KBox<KError>) -> Self {
        Error::QError(ke.to_string())
    }
}
