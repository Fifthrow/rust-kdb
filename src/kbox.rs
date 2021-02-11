use crate::type_traits::KObject;
use crate::{k::K, kapi};
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::{fmt, ops::DerefMut};

/// Represents a memory managed K pointer. They are the
/// KDB equivalent of a Rust Box, a zero overhead wrapper
/// around a K pointer. It will call `r0` to decrement the reference
/// count when it is dropped.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KBox<T: KObject> {
    pub(crate) k: *mut T,
}

impl<T: KObject> KBox<T> {
    /// Converts a box into a raw unmanged K pointer.
    /// Note that into raw will consume the KBox, and not call
    /// r0, so it's possible to leak memory by doing this.
    pub fn into_raw(self) -> *mut T {
        ManuallyDrop::new(self).k
    }

    /// Converts a raw K pointer into a boxed K object.
    /// This is the reciprocal of into_raw
    ///
    /// # Safety
    ///
    /// The type of the k pointer must match the type of the KBox being used.
    /// Do not use this to take ownership of a kdb callback function parameter,
    /// use from_shared instead.
    pub unsafe fn from_raw(k: *mut K) -> Self {
        KBox { k: k as *mut T }
    }

    /// Takes a reference and calls r1, incrementing the reference count
    /// and "re-boxing" it. Typically this is a bad thing as you have multiple
    /// owned references and kdb does not provide equivalent guarantees to rust
    /// about what happens to shared references (especially when reallocating a list for example)
    ///
    /// However in the embedded case, where you do not own the parameter and you wish to manipulate it
    /// without copying the data, then you need this functionality.
    ///
    /// # Safety
    ///
    /// A reference should not be owned by more than one `KBox` instance.
    pub unsafe fn from_shared(t: &mut T) -> Self {
        KBox {
            k: kapi::r1(t.k_ptr_mut()) as *mut T,
        }
    }
}

impl<T: KObject> Drop for KBox<T> {
    fn drop(&mut self) {
        unsafe {
            kapi::r0((*self.k).k_ptr_mut());
        }
    }
}

impl<T: KObject> Deref for KBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.k }
    }
}

impl<T: KObject> DerefMut for KBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.k }
    }
}

impl<T: KObject> AsRef<T> for KBox<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.k }
    }
}

impl<T: KObject + fmt::Debug> fmt::Debug for KBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KBox({:?})", *self)
    }
}

impl<T: KObject + fmt::Display> fmt::Display for KBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self)
    }
}
