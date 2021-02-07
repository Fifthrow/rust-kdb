use crate::{k::K, k_type::ERROR, kapi, type_traits::KObject, Any, KBox, KError, List};

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
pub fn b9_serialize(mode: SerializationMode, k: impl AsRef<Any>) -> Result<KBox<Any>, KBox<KError>> {
    b9_serialize_any(mode, k.as_ref())
}

unsafe fn wrap_ee<T: KObject>(k: *mut K) -> Result<KBox<T>, KBox<KError>> {
    let r = kapi::ee(k);
    if (*r).t == ERROR {
        Err(KBox::from_raw(r))
    } else {
        Ok(KBox::from_raw(r))
    }
}

fn b9_serialize_any(mode: SerializationMode, k: &Any) -> Result<KBox<Any>, KBox<KError>> {
    unsafe { wrap_ee(kapi::b9(mode as i32, k.k_ptr())) }
}

/// Decode a serialized K object.
#[inline]
pub fn d9_deserialize(k: impl AsRef<List<u8>>) -> Result<KBox<Any>, KBox<KError>> {
    unsafe { wrap_ee(kapi::d9(k.as_ref().k_ptr() as *mut _) as *mut _) }
}
