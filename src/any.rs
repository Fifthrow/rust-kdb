use crate::atom::Atom;
use crate::error::ConversionError;
use crate::k::K;
use crate::kapi;
use crate::kbox::KBox;
use crate::list::List;
use crate::type_traits::*;
use crate::{k_type::KTypeCode, k_type::MIXED_LIST};
use std::convert::TryFrom;
use std::fmt;
use std::mem;

/// Any represents any KDB value regardless of type.
/// Unlike atoms or lists you can't do anything with it, except for convert it into an atom or a list.
/// It is ABI compatible with a K object, so it can be safely used as a parameter or return type for a function.
/// See the chapter on embedded functions for more information.
#[repr(transparent)]
pub struct Any {
    k: K,
}

impl fmt::Debug for Any {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Any(Type={})", self.k.t)
    }
}

impl<T: KValue> AsRef<Any> for Atom<T> {
    fn as_ref(&self) -> &Any {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl<T> TryFrom<&Any> for &Atom<T>
where
    T: KValue,
{
    type Error = ConversionError;

    fn try_from(any: &Any) -> Result<Self, Self::Error> {
        if any.k.t == T::TYPE_CODE.as_atom() {
            Ok(unsafe { &*(any as *const _ as *const _) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.k.t,
                to: T::TYPE_CODE.as_atom(),
            })
        }
    }
}

impl<T> TryFrom<KBox<Any>> for KBox<Atom<T>>
where
    T: KValue,
{
    type Error = ConversionError;

    fn try_from(any: KBox<Any>) -> Result<Self, Self::Error> {
        if any.as_ref().k.t == T::TYPE_CODE.as_atom() {
            Ok(unsafe { mem::transmute(any) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.as_ref().k.t,
                to: T::TYPE_CODE.as_atom(),
            })
        }
    }
}

impl<T> TryFrom<&Any> for &List<T>
where
    T: KValue,
{
    type Error = ConversionError;

    fn try_from(any: &Any) -> Result<Self, Self::Error> {
        if any.k.t == T::TYPE_CODE.as_list() {
            Ok(unsafe { &*(any as *const _ as *const _) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.k.t,
                to: T::TYPE_CODE.as_list(),
            })
        }
    }
}

impl<T> TryFrom<KBox<Any>> for KBox<List<T>>
where
    T: KValue,
{
    type Error = ConversionError;

    fn try_from(any: KBox<Any>) -> Result<Self, Self::Error> {
        if any.as_ref().k.t == T::TYPE_CODE.as_list() {
            Ok(unsafe { mem::transmute(any) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.as_ref().k.t,
                to: T::TYPE_CODE.as_list(),
            })
        }
    }
}

impl<T: KValue> From<KBox<List<T>>> for KBox<Any> {
    fn from(value: KBox<List<T>>) -> Self {
        unsafe { mem::transmute(value) }
    }
}
impl<T: KValue> From<KBox<Atom<T>>> for KBox<Any> {
    fn from(value: KBox<Atom<T>>) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl<T: KValue> From<T> for KBox<Any> {
    fn from(value: T) -> Self {
        unsafe { mem::transmute(KBox::new_atom(value)) }
    }
}

// macro implementations for conversion of types
// into any types
macro_rules! impl_into_any {
    ($ty:ident) => {};
}

impl_into_any!(u8);
impl_into_any!(i8);
impl_into_any!(i16);
impl_into_any!(i32);
impl_into_any!(i64);

impl_into_any!(f32);
impl_into_any!(f64);
impl_into_any!(bool);

impl_into_any!(Second);
impl_into_any!(Minute);
impl_into_any!(Month);
impl_into_any!(Time);
impl_into_any!(Date);
impl_into_any!(DateTime);
impl_into_any!(Symbol);
impl_into_any!(Guid);
impl_into_any!(Timestamp);
impl_into_any!(Timespan);

impl KObject for Any {
    fn k_ptr(&self) -> *const K {
        &self.k
    }

    fn k_ptr_mut(&mut self) -> *mut K {
        &mut self.k
    }
}

impl KListable for Any {
    const LIST_TYPE_CODE: KTypeCode = MIXED_LIST;
    type ListItem = KBox<Any>;

    unsafe fn join_to(item: Self::ListItem, mut k: *mut K) -> *mut K {
        // don't r0 this - it's owned by the list now.
        kapi::jk(&mut k, mem::ManuallyDrop::new(item).k_ptr())
    }
}

/*
impl fmt::Debug for KAny {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.k_type() {
            MIXED_LIST => fmt::Debug::fmt(<&KMixedList>::try_from(self).unwrap(), f),
            BOOLEAN_ATOM => fmt::Debug::fmt(<&KBoolAtom>::try_from(self).unwrap(), f),
            GUID_ATOM => fmt::Debug::fmt(<&KGuidAtom>::try_from(self).unwrap(), f),
            BYTE_ATOM => fmt::Debug::fmt(<&KByteAtom>::try_from(self).unwrap(), f),
            SHORT_ATOM => fmt::Debug::fmt(<&KShortAtom>::try_from(self).unwrap(), f),
            INT_ATOM => fmt::Debug::fmt(<&KIntAtom>::try_from(self).unwrap(), f),
            LONG_ATOM => fmt::Debug::fmt(<&KLongAtom>::try_from(self).unwrap(), f),
            REAL_ATOM => fmt::Debug::fmt(<&KRealAtom>::try_from(self).unwrap(), f),
            FLOAT_ATOM => fmt::Debug::fmt(<&KFloatAtom>::try_from(self).unwrap(), f),
            CHAR_ATOM => fmt::Debug::fmt(<&KCharAtom>::try_from(self).unwrap(), f),
            SYMBOL_ATOM => fmt::Debug::fmt(<&KSymbolAtom>::try_from(self).unwrap(), f),
            TIMESTAMP_ATOM => fmt::Debug::fmt(<&KTimestampAtom>::try_from(self).unwrap(), f),
            MONTH_ATOM => fmt::Debug::fmt(<&KMonthAtom>::try_from(self).unwrap(), f),
            DATE_ATOM => fmt::Debug::fmt(<&KDateAtom>::try_from(self).unwrap(), f),
            DATE_TIME_ATOM => fmt::Debug::fmt(<&KDateTimeAtom>::try_from(self).unwrap(), f),
            TIMESPAN_ATOM => fmt::Debug::fmt(<&KTimespanAtom>::try_from(self).unwrap(), f),
            MINUTE_ATOM => fmt::Debug::fmt(<&KMinuteAtom>::try_from(self).unwrap(), f),
            SECOND_ATOM => fmt::Debug::fmt(<&KSecondAtom>::try_from(self).unwrap(), f),
            TIME_ATOM => fmt::Debug::fmt(<&KBoolAtom>::try_from(self).unwrap(), f),
            BOOLEAN_LIST => fmt::Debug::fmt(<&KBoolList>::try_from(self).unwrap(), f),
            GUID_LIST => fmt::Debug::fmt(<&KGuidList>::try_from(self).unwrap(), f),
            BYTE_LIST => fmt::Debug::fmt(<&KByteList>::try_from(self).unwrap(), f),
            SHORT_LIST => fmt::Debug::fmt(<&KShortList>::try_from(self).unwrap(), f),
            INT_LIST => fmt::Debug::fmt(<&KIntList>::try_from(self).unwrap(), f),
            LONG_LIST => fmt::Debug::fmt(<&KLongList>::try_from(self).unwrap(), f),
            REAL_LIST => fmt::Debug::fmt(<&KRealList>::try_from(self).unwrap(), f),
            FLOAT_LIST => fmt::Debug::fmt(<&KFloatList>::try_from(self).unwrap(), f),
            CHAR_LIST => fmt::Debug::fmt(<&KCharList>::try_from(self).unwrap(), f),
            SYMBOL_LIST => fmt::Debug::fmt(<&KSymbolList>::try_from(self).unwrap(), f),
            TIMESTAMP_LIST => fmt::Debug::fmt(<&KTimestampList>::try_from(self).unwrap(), f),
            MONTH_LIST => fmt::Debug::fmt(<&KMonthList>::try_from(self).unwrap(), f),
            DATE_LIST => fmt::Debug::fmt(<&KDateList>::try_from(self).unwrap(), f),
            DATE_TIME_LIST => fmt::Debug::fmt(<&KDateTimeList>::try_from(self).unwrap(), f),
            TIMESPAN_LIST => fmt::Debug::fmt(<&KTimespanList>::try_from(self).unwrap(), f),
            MINUTE_LIST => fmt::Debug::fmt(<&KMinuteList>::try_from(self).unwrap(), f),
            SECOND_LIST => fmt::Debug::fmt(<&KSecondList>::try_from(self).unwrap(), f),
            TIME_LIST => fmt::Debug::fmt(<&KTimeList>::try_from(self).unwrap(), f),
            TABLE => write!(f, "table"),
            DICT => write!(f, "dict"),
            ERROR => fmt::Debug::fmt(<&KError>::try_from(self).unwrap(), f),
            _ => write!(f, "Unknown"),
        }
    }
}
*/
