use crate::atoms::*;
use crate::error::ConversionError;
use crate::lists::*;
use crate::mixed_list::KMixedList;
use crate::raw::kapi;
use crate::raw::types::*;
use std::fmt;
use std::mem;

/// KAny wraps the core K type safely. It can be converted into more specific wrappers
/// that offer more useful functionality using standard rust conversions (TryFrom) or
/// checked reference conversions (the try_as_ref method)
pub struct KAny(pub(crate) *const K);

impl KAny {
    pub(crate) fn into_ptr(self) -> *const K {
        mem::ManuallyDrop::new(self).as_k_ptr()
    }

    // TODO: Unsound - if someone implements their own KItem with a different mem representation, boom! or bang. Or woosh.
    // Okay, it's a niche case if someone is trying to do something silly, so not critical for now. How can I prevent this?
    // Using a TryAsRef trait is a pain too. And the end result for users is ugly. Argh references are complicated.
    pub fn try_as_ref<T: KItem>(&self) -> Result<&T, ConversionError> {
        if T::K_TYPE == self.k_type() && mem::size_of::<T>() == mem::size_of::<Self>() {
            Ok(unsafe { mem::transmute(self) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: self.k_type(),
                to: T::K_TYPE,
            })
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

impl fmt::Debug for KAny {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.k_type() {
            MIXED_LIST => fmt::Debug::fmt(self.try_as_ref::<KMixedList>().unwrap(), f),
            BOOLEAN_ATOM => fmt::Debug::fmt(self.try_as_ref::<KBoolAtom>().unwrap(), f),
            GUID_ATOM => fmt::Debug::fmt(self.try_as_ref::<KGuidAtom>().unwrap(), f),
            BYTE_ATOM => fmt::Debug::fmt(self.try_as_ref::<KByteAtom>().unwrap(), f),
            SHORT_ATOM => fmt::Debug::fmt(self.try_as_ref::<KShortAtom>().unwrap(), f),
            INT_ATOM => fmt::Debug::fmt(self.try_as_ref::<KIntAtom>().unwrap(), f),
            LONG_ATOM => fmt::Debug::fmt(self.try_as_ref::<KLongAtom>().unwrap(), f),
            REAL_ATOM => fmt::Debug::fmt(self.try_as_ref::<KRealAtom>().unwrap(), f),
            FLOAT_ATOM => fmt::Debug::fmt(self.try_as_ref::<KFloatAtom>().unwrap(), f),
            CHAR_ATOM => fmt::Debug::fmt(self.try_as_ref::<KCharAtom>().unwrap(), f),
            SYMBOL_ATOM => fmt::Debug::fmt(self.try_as_ref::<KSymbolAtom>().unwrap(), f),
            TIMESTAMP_ATOM => fmt::Debug::fmt(self.try_as_ref::<KTimestampAtom>().unwrap(), f),
            MONTH_ATOM => fmt::Debug::fmt(self.try_as_ref::<KMonthAtom>().unwrap(), f),
            DATE_ATOM => fmt::Debug::fmt(self.try_as_ref::<KDateAtom>().unwrap(), f),
            DATE_TIME_ATOM => fmt::Debug::fmt(self.try_as_ref::<KDateTimeAtom>().unwrap(), f),
            TIMESPAN_ATOM => fmt::Debug::fmt(self.try_as_ref::<KTimespanAtom>().unwrap(), f),
            MINUTE_ATOM => fmt::Debug::fmt(self.try_as_ref::<KMinuteAtom>().unwrap(), f),
            SECOND_ATOM => fmt::Debug::fmt(self.try_as_ref::<KSecondAtom>().unwrap(), f),
            TIME_ATOM => fmt::Debug::fmt(self.try_as_ref::<KBoolAtom>().unwrap(), f),
            BOOLEAN_LIST => fmt::Debug::fmt(self.try_as_ref::<KBoolList>().unwrap(), f),
            GUID_LIST => fmt::Debug::fmt(self.try_as_ref::<KGuidList>().unwrap(), f),
            BYTE_LIST => fmt::Debug::fmt(self.try_as_ref::<KByteList>().unwrap(), f),
            SHORT_LIST => fmt::Debug::fmt(self.try_as_ref::<KShortList>().unwrap(), f),
            INT_LIST => fmt::Debug::fmt(self.try_as_ref::<KIntList>().unwrap(), f),
            LONG_LIST => fmt::Debug::fmt(self.try_as_ref::<KLongList>().unwrap(), f),
            REAL_LIST => fmt::Debug::fmt(self.try_as_ref::<KRealList>().unwrap(), f),
            FLOAT_LIST => fmt::Debug::fmt(self.try_as_ref::<KFloatList>().unwrap(), f),
            CHAR_LIST => fmt::Debug::fmt(self.try_as_ref::<KCharList>().unwrap(), f),
            SYMBOL_LIST => fmt::Debug::fmt(self.try_as_ref::<KSymbolList>().unwrap(), f),
            TIMESTAMP_LIST => fmt::Debug::fmt(self.try_as_ref::<KTimestampList>().unwrap(), f),
            MONTH_LIST => fmt::Debug::fmt(self.try_as_ref::<KMonthList>().unwrap(), f),
            DATE_LIST => fmt::Debug::fmt(self.try_as_ref::<KDateList>().unwrap(), f),
            DATE_TIME_LIST => fmt::Debug::fmt(self.try_as_ref::<KDateTimeList>().unwrap(), f),
            TIMESPAN_LIST => fmt::Debug::fmt(self.try_as_ref::<KTimespanList>().unwrap(), f),
            MINUTE_LIST => fmt::Debug::fmt(self.try_as_ref::<KMinuteList>().unwrap(), f),
            SECOND_LIST => fmt::Debug::fmt(self.try_as_ref::<KSecondList>().unwrap(), f),
            TIME_LIST => fmt::Debug::fmt(self.try_as_ref::<KTimeList>().unwrap(), f),
            TABLE => write!(f, "table"),
            DICT => write!(f, "dict"),
            ERROR => fmt::Debug::fmt(self.try_as_ref::<KError>().unwrap(), f),
            _ => write!(f, "Unknown"),
        }
    }
}
