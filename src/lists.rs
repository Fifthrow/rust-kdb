//! lists are roughly the KDB equivalent of `Vec`. As such, you can do the most common operations using the same syntax as vectors.
//! For example, to populate a vector, you can use
//!
//! or `collect` from an iterator:
//!
//! lists can also be converted into slices, again in the same way as a `Vec`.
//!

use crate::any::KAny;
use crate::atoms::*;
use crate::error::ConversionError;
use crate::raw::kapi;
use crate::raw::types::*;
use crate::unowned::Unowned;
use std::convert::TryFrom;
use std::ffi::CString;
use std::fmt;
use std::iter::FromIterator;
use std::mem;
use std::ops;

pub(crate) fn list<I, T>(kind: KType, iter: I, append: impl Fn(*mut K, T) -> *mut K) -> *const K
where
    I: IntoIterator<Item = T>,
{
    let iter: I::IntoIter = iter.into_iter();
    let mut k = unsafe { kapi::ktn(kind.into(), iter.size_hint().0 as i64) as *mut K };
    let slice = unsafe { as_mut_slice(k) };
    iter.enumerate().for_each(|(i, s)| {
        if i < slice.len() {
            slice[i] = s;
        } else {
            k = append(k, s);
        }
    });
    k
}

pub trait KListItem: KItem {
    type Item;

    fn len(&self) -> usize {
        unsafe { (*self.as_k_ptr()).union.list.n as usize }
    }

    fn get(&self, index: usize) -> Option<&Self::Item> {
        if index >= self.len() {
            return None;
        }

        unsafe {
            let list_ptr = (*self.as_k_ptr()).union.list.g0 as *const Self::Item;
            Some(&*list_ptr.offset(index as isize))
        }
    }
}

macro_rules! impl_klist {
    {$type:ident, KType = $k_type:ident, Item = $item:ty, Joiner = $joiner:ident} => {
        #[repr(transparent)]
        pub struct $type(*const K);

        impl KItem for $type {
            const K_TYPE: KType = $k_type;
            fn as_k_ptr(&self) -> *const K { self.0 }
        }

        impl Drop for $type {
            fn drop(&mut self) {
                unsafe {
                    kapi::r0(self.0);
                }
            }
        }

        impl KListItem for $type {
            type Item = $item;
        }

        impl $type {
            pub fn iter<'a>(&self) -> impl Iterator<Item = &$item> {
                unsafe { as_slice(self.0).into_iter() }
            }

            pub fn push(&mut self, value: $item) {
                self.0 = unsafe{ kapi::$joiner(&mut (self.0 as *mut K), &value as *const _ as *const _) };
            }

            pub fn extend(&mut self, other: $type) {
                self.0 = unsafe { kapi::jv(&mut (self.0 as *mut K), mem::ManuallyDrop::new(other).0) };
            }

            pub fn new() -> Self {
                unsafe{ $type(kapi::ktn($k_type.into(), 0)) }
            }
        }

        impl fmt::Debug for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(&**self, f)
            }
        }

        impl Default for $type {
            fn default() -> Self {
                Self::new()
            }
        }

        impl ops::Index<ops::RangeFrom<usize>> for $type {
            type Output = [$item];
            fn index(&self, i: ops::RangeFrom<usize>) -> &Self::Output {
                (**self).index(i)
            }
        }

        impl ops::Index<ops::RangeTo<usize>> for $type {
            type Output = [$item];
            fn index(&self, i: ops::RangeTo<usize>) -> &Self::Output {
                (**self).index(i)
            }
        }

        impl ops::Index<ops::Range<usize>> for $type {
            type Output = [$item];
            fn index(&self, i: ops::Range<usize>) -> &Self::Output {
                (**self).index(i)
            }
        }

        impl ops::Index<usize> for $type {
            type Output = $item;
            fn index(&self, i: usize) -> &Self::Output {
                (**self).index(i)
            }
        }

        impl ops::Index<ops::RangeFull> for $type {
            type Output = [$item];
            fn index(&self, _: ops::RangeFull) -> &Self::Output {
                &**self
            }
        }

        impl ops::Deref for $type {
            type Target = [$item];
            fn deref(&self) -> &[$item] {
                unsafe { as_slice(self.0) }
            }
        }

        impl FromIterator<$item> for $type {
            fn from_iter<I: IntoIterator<Item=$item>>(iter: I) -> Self {
                $type(list($k_type, iter, |mut k, item| {
                    unsafe {
                        kapi::$joiner(&mut k, &item as *const _ as *const _)  as *mut K
                    }
                }))
            }
        }

        impl<'a> FromIterator<&'a $item> for $type {
            fn from_iter<I: IntoIterator<Item=&'a $item>>(iter: I) -> Self {
                $type(list($k_type, iter, |mut k, item| {
                    unsafe {
                        kapi::$joiner(&mut k, item as *const _ as *const _)  as *mut K
                    }
                }))
            }
        }

        impl From<$type> for KAny
        {
            fn from(item: $type) -> KAny {
                unsafe { mem::transmute(item) }
            }
        }

        impl TryFrom<KAny> for $type
        {
            type Error = ConversionError;

            fn try_from(any: KAny) -> Result<Self, Self::Error> {
                    let t = any.k_type();
                    if t == $k_type {
                        Ok(unsafe { mem::transmute(any) })
                    } else {
                        Err(ConversionError::InvalidKCast{ from: t, to: $k_type })
                    }
            }
        }

        impl From<Unowned<$type>> for $type {
            fn from(item: Unowned<$type>) -> $type {
                $type(unsafe { item.clone_k_ptr() })
            }
        }

        impl From<Unowned<$type>> for Unowned<KAny> {
            fn from(item: Unowned<$type>) -> Unowned<KAny> {
                unsafe { mem::transmute(item) }
            }
        }

        impl TryFrom<Unowned<KAny>> for Unowned<$type>
        {
            type Error = ConversionError;

            fn try_from(any: Unowned<KAny>) -> Result<Self, Self::Error> {
                    let t = any.k_type();
                    if t == $k_type {
                        Ok(unsafe { mem::transmute(any) })
                    } else {
                        Err(ConversionError::InvalidKCast{ from: t, to: $k_type })
                    }
            }
        }
    }
}

impl_klist! {KBoolList, KType = BOOLEAN_LIST, Item = bool, Joiner = ja}
impl_klist! {KByteList, KType = BYTE_LIST, Item = u8, Joiner = ja}
impl_klist! {KCharList, KType = CHAR_LIST, Item = i8, Joiner = ja}
impl_klist! {KShortList, KType = SHORT_LIST, Item = i16, Joiner = ja}
impl_klist! {KIntList, KType = INT_LIST, Item = i32, Joiner = ja}
impl_klist! {KLongList, KType = LONG_LIST, Item = i64, Joiner = ja}
impl_klist! {KRealList, KType = REAL_LIST, Item = f32, Joiner = ja}
impl_klist! {KFloatList, KType = FLOAT_LIST, Item = f64, Joiner = ja}
impl_klist! {KBooleanList, KType = BOOLEAN_LIST, Item = bool, Joiner = ja}
impl_klist! {KSecondList, KType = SECOND_LIST, Item = Second, Joiner = ja}
impl_klist! {KMinuteList, KType = MINUTE_LIST, Item = Minute, Joiner = ja}
impl_klist! {KMonthList, KType = MONTH_LIST, Item = Month, Joiner = ja}
impl_klist! {KTimeList, KType = TIME_LIST, Item = Time, Joiner = ja}
impl_klist! {KDateList, KType = DATE_LIST, Item = Date, Joiner = ja}
impl_klist! {KDateTimeList, KType = DATE_TIME_LIST, Item = DateTime, Joiner = ja}
impl_klist! {KSymbolList, KType = SYMBOL_LIST, Item = Symbol, Joiner = js}
impl_klist! {KGuidList, KType = GUID_LIST, Item = Guid, Joiner = ja }
impl_klist! {KTimestampList, KType = TIMESTAMP_LIST, Item = Timestamp, Joiner = ja}
impl_klist! {KTimespanList, KType = TIMESPAN_LIST, Item = Timespan, Joiner = ja}

impl FromIterator<String> for KSymbolList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        KSymbolList(list(SYMBOL_LIST, iter, |mut k, item| {
            let s = CString::new(&item[..]).unwrap();
            unsafe { kapi::js(&mut k, kapi::ss(s.as_ptr())) as *mut K }
        }))
    }
}

impl<'a> FromIterator<&'a str> for KSymbolList {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        KSymbolList(list(SYMBOL_LIST, iter, |mut k, item| {
            let s = CString::new(item).unwrap();
            unsafe { kapi::js(&mut k, kapi::ss(s.as_ptr())) as *mut K }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_returns_number_of_elements() {
        let mut list = KIntList::new();
        list.push(1);
        list.push(2);
        assert_eq!(2, list.len());
    }

    #[test]
    fn len_returns_0_for_new_list() {
        let list = KIntList::new();
        assert_eq!(0, list.len());
    }

    #[test]
    fn iter_over_empty_returns_no_elements() {
        let list = KIntList::new();
        assert_eq!(Vec::<&i32>::new(), list.iter().collect::<Vec<_>>());
    }

    #[test]
    fn push_adds_a_single_element_to_the_list() {
        let mut list = KIntList::new();
        list.push(2);
        assert_eq!((1, 2), (list.len(), list[0]));
    }

    #[test]
    fn iter_returns_all_elements() {
        let mut list = KIntList::new();
        list.push(1);
        list.push(2);
        assert_eq!(vec![&1, &2], list.iter().collect::<Vec<&i32>>());
    }

    #[test]
    fn collect_creates_collection() {
        let list: KIntList = (1..=10i32).collect();
        assert_eq!((1..=10).collect::<Vec<_>>(), list.iter().copied().collect::<Vec<_>>());
    }

    #[test]
    fn extend_merges_two_lists_together() {
        let mut list: KIntList = (1..=5i32).collect();
        let list_2: KIntList = (6..=10i32).collect();
        list.extend(list_2);
        assert_eq!((1..=10).collect::<Vec<_>>(), list.iter().copied().collect::<Vec<_>>());
    }

    #[test]
    fn index_rangefull_converts_list_to_slice() {
        let list: KIntList = (1..=10i32).collect();
        let slice = &list[..];
        let expected: Vec<_> = (1..=10i32).collect();
        assert_eq!(&expected[..], slice);
    }

    #[test]
    fn index_range_converts_list_to_slice() {
        let list: KIntList = (1..=10i32).collect();
        let slice = &list[1..5];
        let expected: Vec<_> = (1..=10i32).collect();
        assert_eq!(&expected[1..5], slice);
    }

    #[test]
    fn index_rangeto_converts_list_to_slice() {
        let list: KIntList = (1..=10i32).collect();
        let slice = &list[..5];
        let expected: Vec<_> = (1..=10i32).collect();
        assert_eq!(&expected[..5], slice);
    }

    #[test]
    fn index_rangefrom_converts_list_to_slice() {
        let list: KIntList = (1..=10i32).collect();
        let slice = &list[5..];
        let expected: Vec<_> = (1..=10i32).collect();
        assert_eq!(&expected[5..], slice);
    }

    #[test]
    fn index_usize_returns_item() {
        let list: KIntList = (1..=10i32).collect();
        assert_eq!(6, list[5]);
    }
}
