use crate::kbox::KBox;
use crate::list::List;
use crate::{any::Any, k::K};
use crate::{k_type::MIXED_LIST, kapi, type_traits::KObject};
use std::{mem, ops::Index};

/// A key value based dictionary.
#[repr(transparent)]
pub struct Dictionary {
    k: K,
}

impl Dictionary {
    fn raw_key_value_lists(&self) -> &[KBox<List<Any>>; 2] {
        unsafe {
            assert_eq!(self.k.union.list.n, 2);
            ((&self.k.union.list.g0) as *const _ as *const [KBox<List<Any>>; 2])
                .as_ref()
                .unwrap()
        }
    }

    fn raw_key_value_lists_mut(&mut self) -> &mut [KBox<List<Any>>; 2] {
        unsafe {
            ((&self.k.union.list.g0) as *const _ as *mut [KBox<List<Any>>; 2])
                .as_mut()
                .unwrap()
        }
    }

    fn key_list_mut(&mut self) -> &mut KBox<List<Any>> {
        &mut self.raw_key_value_lists_mut()[0]
    }

    fn value_list_mut(&mut self) -> &mut KBox<List<Any>> {
        &mut self.raw_key_value_lists_mut()[1]
    }

    fn key_list(&self) -> &KBox<List<Any>> {
        &self.raw_key_value_lists()[0]
    }

    fn value_list(&self) -> &KBox<List<Any>> {
        &self.raw_key_value_lists()[1]
    }

    /// The number of items in the dictionary.
    #[inline]
    pub fn len(&self) -> usize {
        self.key_list().len()
    }

    /// Returns true if the dictionary has no items.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets a slice containing all the keys in this dictionary.
    #[inline]
    pub fn keys(&self) -> &[Any] {
        unsafe { mem::transmute(&self.key_list()[..]) }
    }

    /// Gets a slice containing all the values in this dictionary.
    #[inline]
    pub fn values(&self) -> &[Any] {
        unsafe { mem::transmute(&self.value_list()[..]) }
    }

    /// Insert a specified key and value at the end of the dictionary.
    /// No checks are done on uniqueness so duplicates are possible.
    #[inline]
    pub fn insert(&mut self, key: impl Into<KBox<Any>>, value: impl Into<KBox<Any>>) {
        self.key_list_mut().push(key.into());
        self.value_list_mut().push(value.into());
    }

    /// Gets a value by key. Note that KDB dictionaries are treated as unordered and hence this is an O(n) operation.
    #[inline]
    pub fn get<T: Into<KBox<Any>>>(&self, key: T) -> Option<&Any> {
        let key = key.into();
        let index = self
            .keys()
            .iter()
            .enumerate()
            .find(|(_, k2)| unsafe { **k2 == *key.k })
            .map(|(i, _)| i)?;
        self.values().get(index)
    }

    /// An iterator through every value in the KDB object
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&Any, &Any)> {
        self.keys().iter().zip(self.values().iter())
    }
}

impl<T> Index<T> for Dictionary
where
    for<'a> T: Into<KBox<Any>>,
{
    type Output = Any;

    fn index(&self, index: T) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl KObject for Dictionary {
    #[inline]
    fn k_ptr(&self) -> *const K {
        &self.k
    }

    #[inline]
    fn k_ptr_mut(&mut self) -> *mut K {
        &mut self.k
    }
}

impl KBox<Dictionary> {
    /// Create a new empty dictionary.
    pub fn new_dict() -> Self {
        unsafe {
            let keys = kapi::ktn(MIXED_LIST.into(), 0) as *mut K;
            let values = kapi::ktn(MIXED_LIST.into(), 0) as *mut K;
            mem::transmute(kapi::xD(keys, values))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::symbol;

    use super::*;

    #[test]
    fn insert_appends_items_to_dictionary() {
        let mut dict = KBox::new_dict();
        dict.insert(symbol("Hello"), symbol("World"));

        assert_eq!(dict.len(), 1);
    }

    #[test]
    fn get_retrieves_items_by_key() {
        let mut dict = KBox::new_dict();
        dict.insert(symbol("Hello"), symbol("World"));

        let val = dict.get(symbol("Hello")).unwrap();

        assert_eq!(*val, *KBox::<Any>::from(symbol("World")).as_ref());
    }
}
