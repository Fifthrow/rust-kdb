//#![cfg(feature = "embedded")]
use crate::atoms::KItem;
use crate::lists::KByteList;
use crate::raw::kapi;
use crate::{any::KAny, ERROR};
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

use crate::atoms::KError;
use crate::raw::types::K;

/// Describes how to perform serialization when using `b9_serialize`.
pub enum SerializationMode {
    /// Valid for V3.0+ for serializing/deserializing within the same process.
    InProc = -1,
    /// unenumerate, block serialization of timespan and timestamp (for working with versions prior to V2.6).
    Unenumerate = 0,
    /// Retain enumerations, allow serialization of timespan and timestamp: Useful for passing data between threads.
    Enumerate = 1,
    /// Unenumerate, allow serialization of timespan and timestamp.
    UnenumerateWithTimestamps = 2,
    /// Unenumerate, compress, allow serialization of timespan and timestamp.
    Compress = 3,
}

/// Serialize a K object using KDB serialization.
#[inline]
pub fn b9_serialize(mode: SerializationMode, k: impl Into<KAny>) -> Result<KByteList, KError> {
    b9_serialize_any(mode, &k.into())
}

unsafe fn ee(k: *const K) -> Result<KAny, KError> {
    let r = kapi::ee(k);
    if (*r).t == ERROR {
        Err(KError(k))
    } else {
        Ok(KAny(k))
    }
}

fn b9_serialize_any(mode: SerializationMode, k: &KAny) -> Result<KByteList, KError> {
    unsafe { std::mem::transmute(ee(kapi::b9(mode as i32, k.as_k_ptr()))) }
}

/// Decode a serialized K object.
#[inline]
pub fn d9_deserialize(k: &KByteList) -> Result<KAny, KError> {
    unsafe { ee(kapi::d9(k.as_k_ptr())) }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;
    use crate::{mixed_list, KMixedList};

    #[test]
    fn b9_d9_roundtrips() {
        let bytes = b9_serialize(SerializationMode::InProc, mixed_list![1, 2, 3]).unwrap();
        let v: KMixedList = d9_deserialize(&bytes).unwrap().try_into().unwrap();
        assert_eq!(v.len(), 3);

        let bytes = b9_serialize(SerializationMode::Enumerate, mixed_list![1, 2, 3]).unwrap();
        let v: KMixedList = d9_deserialize(&bytes).unwrap().try_into().unwrap();
        assert_eq!(v.len(), 3);

        let bytes = b9_serialize(SerializationMode::Unenumerate, mixed_list![1, 2, 3]).unwrap();
        let v: KMixedList = d9_deserialize(&bytes).unwrap().try_into().unwrap();
        assert_eq!(v.len(), 3);

        let bytes = b9_serialize(SerializationMode::Compress, mixed_list![1, 2, 3]).unwrap();
        let v: KMixedList = d9_deserialize(&bytes).unwrap().try_into().unwrap();
        assert_eq!(v.len(), 3);
    }

    #[cfg(feature = "remote-test")]
    #[test]
    fn b9_d9_roundtrips_remote() {
        use crate::symbol;
        let kdb = crate::Connection::connect("127.0.0.1", 4200, "", None).unwrap();

        let bytes = b9_serialize(SerializationMode::Enumerate, mixed_list![1, 2, 3]).unwrap();
        kdb.eval_2("set", symbol("b9_a"), bytes).unwrap();

        let bytes = b9_serialize(SerializationMode::Unenumerate, mixed_list![1, 2, 3]).unwrap();
        kdb.eval_2("set", symbol("b9_b"), bytes).unwrap();

        let bytes = b9_serialize(SerializationMode::Compress, mixed_list![1, 2, 3]).unwrap();
        kdb.eval_2("set", symbol("b9_c"), bytes).unwrap();
    }
}
