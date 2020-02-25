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
use std::convert::TryFrom;
use std::ffi::CString;
use std::iter::FromIterator;
use std::mem;
use std::ops;

fn list<I, T>(kind: KType, iter: I, append: impl Fn(*mut K, T) -> *mut K) -> *const K
where
    I: IntoIterator<Item = T>,
{
    let iter: I::IntoIter = iter.into_iter();
    let mut k = unsafe { kapi::ktn(kind.into(), iter.size_hint().0 as i64) as *mut K };
    iter.for_each(|s| k = append(k, s));
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
    {$type:ident, ListType = $list_type:ident, Item = $item:ty, Joiner = $joiner:ident} => {
        pub struct $type(* const K);

        impl KItem for $type {
            fn as_k_ptr(&self) -> * const K { self.0 }
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
                unsafe{ kapi::$joiner(&mut (self.0 as *mut _), &value as * const _ as * const _); }
            }

            pub fn extend(&mut self, other: $type) {
                unsafe { kapi::jv(&mut (self.0 as *mut K), other.0); }
            }

            pub fn new() -> Self {
                unsafe{ $type(kapi::ktn($list_type.into(), 0)) }
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
                unsafe { as_slice(self.0) }.index(i)
            }
        }

        impl ops::Index<ops::RangeTo<usize>> for $type {
            type Output = [$item];
            fn index(&self, i: ops::RangeTo<usize>) -> &Self::Output {
                unsafe { as_slice(self.0) }.index(i)
            }
        }

        impl ops::Index<ops::Range<usize>> for $type {
            type Output = [$item];
            fn index(&self, i: ops::Range<usize>) -> &Self::Output {
                unsafe { as_slice(self.0) }.index(i)
            }
        }

        impl ops::Index<usize> for $type {
            type Output = $item;
            fn index(&self, i: usize) -> &Self::Output {
                unsafe { as_slice(self.0) }.index(i)
            }
        }

        impl ops::Deref for $type {
            type Target = [$item];
            fn deref(&self) -> &[$item] {
                unsafe { as_slice(self.0) }
            }
        }

        impl From<$type> for Vec<$item> {
            fn from(k_list: $type) -> Vec<$item> {
                k_list.iter().copied().collect()
            }
        }

        impl FromIterator<$item> for $type {
            fn from_iter<I: IntoIterator<Item=$item>>(iter: I) -> Self {
                $type(list($list_type, iter, |mut k, item| {
                    unsafe {
                        kapi::$joiner(&mut k, &item as *const _ as *const _)  as *mut K
                    }
                }))
            }
        }

        impl<'a> FromIterator<&'a $item> for $type {
            fn from_iter<I: IntoIterator<Item=&'a $item>>(iter: I) -> Self {
                $type(list($list_type, iter, |mut k, item| {
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
                    if t == $list_type {
                        Ok(unsafe { mem::transmute(any) })
                    } else {
                        Err(ConversionError::InvalidKCast{ from: t, to: $list_type })
                    }
            }
        }
    }
}

impl_klist! {KByteList, ListType = BYTE_LIST, Item = u8, Joiner = ja}
impl_klist! {KCharList, ListType = CHAR_LIST, Item = i8, Joiner = ja}
impl_klist! {KShortList, ListType = SHORT_LIST, Item = i16, Joiner = ja}
impl_klist! {KIntList, ListType = INT_LIST, Item = i32, Joiner = ja}
impl_klist! {KLongList, ListType = LONG_LIST, Item = i64, Joiner = ja}
impl_klist! {KRealList, ListType = REAL_LIST, Item = f32, Joiner = ja}
impl_klist! {KFloatList, ListType = FLOAT_LIST, Item = f64, Joiner = ja}
impl_klist! {KBooleanList, ListType = BOOLEAN_LIST, Item = bool, Joiner = ja}
impl_klist! {KSecondList, ListType = SECOND_LIST, Item = KSecond, Joiner = ja}
impl_klist! {KMinuteList, ListType = MINUTE_LIST, Item = KMinute, Joiner = ja}
impl_klist! {KMonthList, ListType = MONTH_LIST, Item = KMonth, Joiner = ja}
impl_klist! {KTimeList, ListType = TIME_LIST, Item = KTime, Joiner = ja}
impl_klist! {KDateList, ListType = DATE_LIST, Item = KDate, Joiner = ja}
impl_klist! {KDateTimeList, ListType = DATE_TIME_LIST, Item = KDateTime, Joiner = ja}
impl_klist! {KSymbolList, ListType = SYMBOL_LIST, Item = KSymbol, Joiner = js}
impl_klist! {KGuidList, ListType = GUID_LIST, Item = KGuid, Joiner = ja }

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

pub struct KMixedList(*const K);

impl KListItem for KMixedList {
    type Item = KAny;
}

impl KMixedList {
    pub fn iter(&self) -> impl Iterator<Item = &KAny> {
        unsafe { as_slice(self.0).into_iter() }
    }

    pub fn push(&mut self, item: impl Into<KAny>) {
        let k_any = mem::ManuallyDrop::new(item.into());

        unsafe {
            kapi::jk(&mut (self.0 as *mut K), k_any.as_k_ptr());
        }
    }
}

impl KItem for KMixedList {
    fn as_k_ptr(&self) -> *const K {
        self.0
    }
}

impl<T> FromIterator<T> for KMixedList
where
    T: KItem,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> KMixedList {
        KMixedList(list(MIXED_LIST, iter, |mut k, item| unsafe {
            kapi::jk(&mut k, item.clone_k_ptr()) as *mut K
        }))
    }
}

impl From<KMixedList> for KAny {
    fn from(item: KMixedList) -> KAny {
        unsafe { mem::transmute(item) }
    }
}

impl TryFrom<KAny> for KMixedList {
    type Error = ConversionError;

    fn try_from(any: KAny) -> Result<Self, Self::Error> {
        let t = any.k_type();
        if t == MIXED_LIST {
            Ok(unsafe { mem::transmute(any) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: t,
                to: MIXED_LIST,
            })
        }
    }
}
