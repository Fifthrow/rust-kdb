//#![cfg(feature = "embedded")]
use crate::any::KAny;
use crate::atoms::KItem;
use crate::raw::kapi;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Debug)]
pub struct Unowned<T>(ManuallyDrop<T>);

impl<T> Deref for Unowned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<T> DerefMut for Unowned<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0.deref_mut()
    }
}

impl<T: KItem> Unowned<T> {
    pub fn into_owned(self) -> T {
        self.0.clone_k_ptr();
        ManuallyDrop::into_inner(self.0)
    }
}

impl Unowned<KAny> {
    pub fn into_owned(self) -> KAny {
        unsafe {
            kapi::r1(self.0.as_k_ptr());
        }
        ManuallyDrop::into_inner(self.0)
    }
}
