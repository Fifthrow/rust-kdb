use crate::any::KAny;
use crate::atoms::KItem;
use crate::error::ConversionError;
use crate::lists::KListItem;
use crate::raw::kapi;
use crate::raw::types::*;
use crate::unowned::Unowned;
use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;
use std::mem;
use std::ops;

#[repr(transparent)]
pub struct KMixedList(*const K);

impl KItem for KMixedList {
    const K_TYPE: KType = MIXED_LIST;
    fn as_k_ptr(&self) -> *const K {
        self.0
    }
}

impl Drop for KMixedList {
    fn drop(&mut self) {
        unsafe {
            kapi::r0(self.0);
        }
    }
}

impl KListItem for KMixedList {
    type Item = KAny;
}

impl KMixedList {
    pub fn iter(&self) -> impl Iterator<Item = &KAny> {
        unsafe { as_slice(self.0).iter() }
    }

    pub fn push(&mut self, value: impl Into<KAny>) {
        self.push_internal(value.into())
    }

    fn push_internal(&mut self, value: KAny) {
        self.0 = unsafe { kapi::jk(&mut (self.0 as *mut K), value.into_ptr() as *const _ as *const _) };
    }

    pub fn extend(&mut self, other: KMixedList) {
        self.0 = unsafe { kapi::jv(&mut (self.0 as *mut K), mem::ManuallyDrop::new(other).0) };
    }

    pub fn new() -> Self {
        unsafe { KMixedList(kapi::ktn(MIXED_LIST.into(), 0)) }
    }
}

impl fmt::Debug for KMixedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl Default for KMixedList {
    fn default() -> Self {
        Self::new()
    }
}

impl ops::Index<ops::RangeFrom<usize>> for KMixedList {
    type Output = [KAny];
    fn index(&self, i: ops::RangeFrom<usize>) -> &Self::Output {
        (**self).index(i)
    }
}

impl ops::Index<ops::RangeTo<usize>> for KMixedList {
    type Output = [KAny];
    fn index(&self, i: ops::RangeTo<usize>) -> &Self::Output {
        (**self).index(i)
    }
}

impl ops::Index<ops::Range<usize>> for KMixedList {
    type Output = [KAny];
    fn index(&self, i: ops::Range<usize>) -> &Self::Output {
        (**self).index(i)
    }
}

impl ops::Index<usize> for KMixedList {
    type Output = KAny;
    fn index(&self, i: usize) -> &Self::Output {
        (**self).index(i)
    }
}

impl ops::Index<ops::RangeFull> for KMixedList {
    type Output = [KAny];
    fn index(&self, _: ops::RangeFull) -> &Self::Output {
        &**self
    }
}

impl ops::Deref for KMixedList {
    type Target = [KAny];
    fn deref(&self) -> &[KAny] {
        unsafe { as_slice(self.0) }
    }
}

impl<T: Into<KAny>> FromIterator<T> for KMixedList {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter: I::IntoIter = iter.into_iter();
        let mut k = unsafe { kapi::ktn(MIXED_LIST.into(), iter.size_hint().0 as i64) as *mut K };
        let slice = unsafe { as_mut_slice(k) };
        iter.enumerate().for_each(|(i, item)| {
            if i < slice.len() {
                slice[i] = item.into().into_ptr();
            } else {
                k = unsafe { kapi::jk(&mut k, &item.into().into_ptr() as *const _ as *const _) as *mut K }
            }
        });
        KMixedList(k)
    }
}

/*impl<'a> FromIterator<&'a KAny> for KMixedList {
    fn from_iter<I: IntoIterator<Item = &'a KAny>>(iter: I) -> Self {

        KMixedList(list(MIXED_LIST, iter, |mut k, item| unsafe {
            kapi::jk(&mut k, item as *const _ as *const _) as *mut K
        }))
    }
}*/

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

impl From<Unowned<KMixedList>> for KMixedList {
    fn from(item: Unowned<KMixedList>) -> KMixedList {
        KMixedList(unsafe { item.clone_k_ptr() })
    }
}

impl From<Unowned<KMixedList>> for Unowned<KAny> {
    fn from(item: Unowned<KMixedList>) -> Unowned<KAny> {
        unsafe { mem::transmute(item) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atoms::KIntAtom;

    #[test]
    fn len_returns_number_of_elements() {
        let mut list = KMixedList::new();
        list.push(1i8);
        list.push(2i32);
        assert_eq!(2, list.len());
    }

    #[test]
    fn len_returns_0_for_new_list() {
        let list = KMixedList::new();
        assert_eq!(0, list.len());
    }

    #[test]
    fn iter_over_empty_returns_no_elements() {
        let list = KMixedList::new();
        assert_eq!(Vec::<&KAny>::new(), list.iter().collect::<Vec<_>>());
    }

    #[test]
    fn push_adds_a_single_element_to_the_list() {
        let mut list = KMixedList::new();
        list.push(2);
        assert_eq!((1, 2), (list.len(), **list[0].try_as_ref::<KIntAtom>().unwrap()));
    }

    #[test]
    fn iter_returns_all_elements() {
        let mut list = KMixedList::new();
        list.push(1);
        list.push(2);

        let v = (1..=2).map(KAny::from).collect::<Vec<_>>();
        let expected: Vec<_> = v.iter().collect();
        assert_eq!(expected, list.iter().collect::<Vec<_>>());
    }

    #[test]
    fn collect_creates_collection() {
        let list: KMixedList = (1..=10i32).collect();
        let v = (1..=10).map(KAny::from).collect::<Vec<_>>();
        let expected: Vec<_> = v.iter().collect();
        assert_eq!(expected, list.iter().collect::<Vec<_>>());
    }

    #[test]
    fn extend_merges_two_lists_together() {
        let mut list: KMixedList = (1..=5i32).collect();
        let list_2: KMixedList = (6..=10i32).collect();
        list.extend(list_2);

        let v = (1..=10).map(KAny::from).collect::<Vec<_>>();
        let expected: Vec<_> = v.iter().collect();
        assert_eq!(expected, list.iter().collect::<Vec<_>>());
    }

    #[test]
    fn index_rangefull_converts_mixed_list_to_slice() {
        let list: KMixedList = (1..=10i32).collect();
        let slice = &list[..];
        let expected: Vec<_> = (1..=10i32).map(KAny::from).collect();
        assert_eq!(&expected[..], slice);
    }

    #[test]
    fn index_range_converts_mixed_list_to_slice() {
        let list: KMixedList = (1..=10i32).collect();
        let slice = &list[1..5];
        let expected: Vec<_> = (1..=10i32).map(KAny::from).collect();
        assert_eq!(&expected[1..5], slice);
    }

    #[test]
    fn index_rangeto_converts_mixed_list_to_slice() {
        let list: KMixedList = (1..=10i32).collect();
        let slice = &list[..5];
        let expected: Vec<_> = (1..=10i32).map(KAny::from).collect();
        assert_eq!(&expected[..5], slice);
    }

    #[test]
    fn index_rangefrom_converts_mixed_list_to_slice() {
        let list: KMixedList = (1..=10i32).collect();
        let slice = &list[5..];
        let expected: Vec<_> = (1..=10i32).map(KAny::from).collect();
        assert_eq!(&expected[5..], slice);
    }

    #[test]
    fn index_usize_returns_item() {
        let list: KMixedList = (1..=10i32).collect();
        assert_eq!(6, **list[5].try_as_ref::<KIntAtom>().unwrap());
    }
}
