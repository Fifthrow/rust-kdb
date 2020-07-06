use crate::atoms::*;
use crate::error::ConversionError;
use crate::lists::*;
use crate::mixed_list::KMixedList;
use crate::raw::kapi;
use crate::raw::types::*;
use std::fmt;
use std::mem;
use std::convert::TryFrom;

/// KAny wraps the core K type safely. It can be converted into more specific wrappers
/// that offer more useful functionality using standard rust conversions (TryFrom) or
/// checked reference conversions (the try_as_ref method)
#[repr(transparent)]
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
            Ok(unsafe { &*(self as *const KAny as *const T) })
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
