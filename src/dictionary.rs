use crate::kbox::KBox;
use crate::list::List;
use crate::{any::Any, k::K};
use crate::{k_type::MIXED_LIST, kapi, type_traits::KObject};
use std::{mem, ops::Index};

#[repr(transparent)]
pub struct Dictionary {
    k: K,
}

impl Dictionary {
    fn raw_key_value_lists(&self) -> &[KBox<List<Any>>; 2] {
        unsafe {
            ((&self.k.union.list.g0) as *const *mut u8 as *const [KBox<List<Any>>; 2])
                .as_ref()
                .unwrap()
        }
    }

    fn raw_key_value_lists_mut(&mut self) -> &mut [KBox<List<Any>>; 2] {
        unsafe {
            ((&self.k.union.list.g0) as *const *mut u8 as *mut [KBox<List<Any>>; 2])
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
    pub fn keys(&self) -> &[KBox<Any>] {
        &self.key_list()[..]
    }

    /// Gets a slice containing all the values in this dictionary.
    #[inline]
    pub fn values(&self) -> &[KBox<Any>] {
        &self.value_list()[..]
    }

    /// Insert a specified key and value at the end of the dictionary.
    /// No checks are done on uniqueness so duplicates are possible.
    #[inline]
    pub fn insert(&mut self, key: impl Into<KBox<Any>>, value: impl Into<KBox<Any>>) {
        let key = key.into();
        let value = value.into();
        self.key_list_mut().push(key);
        self.value_list_mut().push(value);
    }

    /// Gets a value by key. Note that KDB dictionaries are unordered and hence is an O(n) operation.
    #[inline]
    pub fn get<T: Into<KBox<Any>>>(&self, key: T) -> Option<&KBox<Any>> {
        let key = key.into();
        let index = self
            .keys()
            .iter()
            .enumerate()
            .find(|(_, k2)| k2.k == key.k)
            .map(|(i, _)| i)?;
        self.values().get(index)
    }

    /// An iterator through every value in the KDB object
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&KBox<Any>, &KBox<Any>)> {
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
    fn k_ptr(&self) -> *const K {
        &self.k
    }

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
