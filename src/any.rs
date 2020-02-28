use crate::atoms::KItem;
use crate::raw::kapi;
use crate::raw::types::K;
use std::mem;

/// KAny wraps the core K type safely. It can be converted into more specific wrappers
/// that offer more useful functionality using standard rust conversions (TryFrom) or
/// checked reference conversions (the try_as_ref method)
pub struct KAny(pub(crate) *const K);

impl KAny {
    pub(crate) fn into_ptr(self) -> *const K {
        mem::ManuallyDrop::new(self).as_k_ptr()
    }
}

    pub fn as_k_ptr(&self) -> *const K {
        self.0
    }

    pub fn k_type(&self) -> KType {
        unsafe { (*self.as_k_ptr()).t }
    }
}

impl PartialEq for KAny {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { *self.0 == *other.0 }
    }
}

impl Drop for KAny {
    fn drop(&mut self) {
        unsafe {
            kapi::r0(self.0);
        }
    }
}
