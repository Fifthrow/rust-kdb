use crate::k::K;
use crate::k_type::*;

/// Represents a known type that can be stored in and retrieved from a list or an atom.
pub trait KValue: private::Sealed {
    ///TYPE_CODE is *not* the K 't' field. it's a value than can be
    /// Converted into a list or an atom code.
    const TYPE_CODE: TypeCode;

    unsafe fn from_k(k: &K) -> Self;
    unsafe fn as_mutable(k: &mut K) -> &mut Self;
    fn into_k(self) -> *const K;
}

/// Indicates a type that wraps a `K` object. This trait is sealed and can't be implemented
/// from other crates.
pub trait KObject: private::Sealed {
    /// Returns a pointer to the underlying K type
    fn k_ptr(&self) -> *const K;
    /// Returns a mutable pointer to the underlying K type
    fn k_ptr_mut(&mut self) -> *mut K;
}

/// Indicates something that can be stored in a List. Basically this is
/// All KValues and the Any type. Used to provide list concatenation functions.
// This trait is sealed and can't be implemented from other crates.
pub trait KListable: private::Sealed {
    type ListItem; //: std::fmt::Debug;
    const LIST_TYPE_CODE: KTypeCode;
    unsafe fn join_to(item: Self::ListItem, k: *mut K) -> *mut K;
}

pub(crate) mod private {
    /// Dummy trait used to prevent the other traits from being implemented
    /// for types outside of this crate.
    pub trait Sealed {}
}

mod k_type_impls {
    use super::*;
    use crate::k_error::KError;
    use crate::kapi;
    use crate::symbol::Symbol;
    use crate::{any::Any, dictionary::Dictionary};
    use crate::{date_time_types::*, table::Table};

    #[cfg(feature = "uuid")]
    use uuid::Uuid;

    macro_rules! impl_k_value {
        ($type:ident, Code = $typecode: ident, Ctor = $ctor:ident, Accessor = $accessor:ident) => {
            impl_k_value!(
                $type,
                Code = $typecode,
                Ctor = $ctor,
                Accessor = $accessor,
                Joiner = ja
            );
        };
        ($type:ident, Code = $typecode: ident, Ctor = $ctor:ident, Accessor = $accessor: ident, Joiner = $joiner: ident) => {
            impl KValue for $type {
                const TYPE_CODE: TypeCode = TypeCode::$typecode;

                unsafe fn from_k(k: &K) -> Self {
                    k.union.$accessor.into()
                }

                unsafe fn as_mutable(k: &mut K) -> &mut Self {
                    (&mut k.union.$accessor).into()
                }

                fn into_k(self) -> *const K {
                    unsafe { kapi::$ctor(self.into()) }
                }
            }

            impl KListable for $type {
                type ListItem = $type;
                const LIST_TYPE_CODE: KTypeCode = TypeCode::$typecode.as_list();

                unsafe fn join_to(item: Self::ListItem, mut k: *mut K) -> *mut K {
                    kapi::$joiner(&mut k, &item as *const _ as *const _)
                }
            }

            impl private::Sealed for $type {}
        };
    }

    impl_k_value! {u8, Code = BYTE, Ctor = kg, Accessor = g }
    impl_k_value! {i8, Code = CHAR, Ctor = kc, Accessor = c }
    impl_k_value! {i16, Code = SHORT, Ctor = kh, Accessor = h }
    impl_k_value! {i32, Code = INT, Ctor = ki, Accessor = i }
    impl_k_value! {i64, Code = LONG, Ctor = kj, Accessor = j }

    impl_k_value! {f32, Code = REAL, Ctor = ke, Accessor = e }
    impl_k_value! {f64, Code = FLOAT, Ctor = kf, Accessor = f }
    impl_k_value! {bool, Code = BOOLEAN, Ctor = kb, Accessor = bl }

    impl_k_value! {Second, Code = SECOND, Ctor = ksec, Accessor = sec }
    impl_k_value! {Minute, Code = MINUTE, Ctor = kmin, Accessor = min }
    impl_k_value! {Month, Code = MONTH, Ctor = kmonth, Accessor = m }
    impl_k_value! {Time, Code = TIME, Ctor = kt, Accessor = t }
    impl_k_value! {Date, Code = DATE, Ctor = kd, Accessor = d }
    impl_k_value! {DateTime, Code = DATE_TIME, Ctor = kz, Accessor = dt }
    impl_k_value! {Symbol, Code = SYMBOL, Ctor = ks, Accessor = sym, Joiner = js }
    #[cfg(feature = "uuid")]
    impl_k_value! {Uuid, Code = GUID, Ctor = ku, Accessor = u }
    impl_k_value! {Timestamp, Code = TIMESTAMP, Ctor = tst, Accessor = tst }
    impl_k_value! {Timespan, Code = TIMESPAN, Ctor = tsp, Accessor = ts }

    impl private::Sealed for Any {}

    impl private::Sealed for Dictionary {}

    impl private::Sealed for Table {}

    impl private::Sealed for KError {}
}
