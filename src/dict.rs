use crate::any::KAny;
use crate::atoms::KItem;
use crate::error::ConversionError;
use crate::mixed_list::KMixedList;
use crate::raw::kapi;
use crate::raw::types::{KType, DICT, K, MIXED_LIST};
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::mem;
use std::ops::Index;

#[repr(transparent)]
pub struct KDict(*const K);

impl KItem for KDict {
    const K_TYPE: KType = DICT;
    fn as_k_ptr(&self) -> *const K {
        self.0
    }
}

//TODO: Check array alignment is defined as per C.
impl KDict {
    fn raw_key_value_lists(&self) -> &[KMixedList; 2] {
        unsafe {
            ((&(*self.0).union.list.g0) as *const *mut u8 as *const [KMixedList; 2])
                .as_ref()
                .unwrap()
        }
    }

    fn raw_key_value_lists_mut(&mut self) -> &mut [KMixedList; 2] {
        unsafe {
            ((&(*self.0).union.list.g0) as *const *mut u8 as *mut [KMixedList; 2])
                .as_mut()
                .unwrap()
        }
    }

    fn key_list_mut(&mut self) -> &mut KMixedList {
        &mut self.raw_key_value_lists_mut()[0]
    }

    fn value_list_mut(&mut self) -> &mut KMixedList {
        &mut self.raw_key_value_lists_mut()[1]
    }

    fn key_list(&self) -> &KMixedList {
        &self.raw_key_value_lists()[0]
    }

    fn value_list(&self) -> &KMixedList {
        &self.raw_key_value_lists()[1]
    }

    pub fn len(&self) -> usize {
        self.key_list().len()
    }

    /// Gets a slice containing all the keys in this dictionary
    pub fn keys(&self) -> &[KAny] {
        &self.key_list()[..]
    }

    /// Gets a slice containing all the values in this dictionary
    pub fn values(&self) -> &[KAny] {
        &self.value_list()[..]
    }

    /// Create a new empty dictionary.
    pub fn new() -> Self {
        unsafe {
            let keys = kapi::ktn(MIXED_LIST.into(), 0) as *mut K;
            let values = kapi::ktn(MIXED_LIST.into(), 0) as *mut K;
            KDict(kapi::xD(keys, values))
        }
    }

    /// Insert a specified key and value at the end of the dictionary.
    /// No checks are done on uniqueness so duplicates are possible.
    pub fn insert(&mut self, key: impl Into<KAny>, value: impl Into<KAny>) {
        let key = key.into();
        let value = value.into();
        self.key_list_mut().push(key);
        self.value_list_mut().push(value);
    }

    /// Gets a value by key. Note that KDB dictionaries are unordered and hence is an O(n) operation.
    pub fn get<T: Into<KAny>>(&self, key: T) -> Option<&KAny> {
        let key = key.into();
        let index = self
            .keys()
            .into_iter()
            .enumerate()
            .find(|(_, k2)| **k2 == key)
            .map(|(i, _)| i)?;
        self.values().get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&KAny, &KAny)> {
        self.keys().into_iter().zip(self.values().iter())
    }
}

impl Default for KDict {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<T> for KDict
where
    T: Into<KAny>,
{
    type Output = KAny;

    fn index(&self, index: T) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl TryFrom<KAny> for KDict {
    type Error = ConversionError;
    fn try_from(any: KAny) -> Result<Self, Self::Error> {
        if any.k_type() == DICT {
            Ok(unsafe { mem::transmute(any) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.k_type(),
                to: DICT,
            })
        }
    }
}

impl From<KDict> for KAny {
    fn from(dict: KDict) -> KAny {
        unsafe { mem::transmute(dict) }
    }
}

impl<Key, Val> FromIterator<(Key, Val)> for KDict
where
    Key: Into<KAny>,
    Val: Into<KAny>,
{
    fn from_iter<I: IntoIterator<Item = (Key, Val)>>(iter: I) -> KDict {
        let iter = iter.into_iter();
        let (bound, _) = iter.size_hint();
        unsafe {
            let mut keys = kapi::ktn(MIXED_LIST.into(), bound as i64) as *mut K;
            let mut values = kapi::ktn(MIXED_LIST.into(), bound as i64) as *mut K;
            for (key, value) in iter {
                keys = kapi::jk(&mut keys, mem::ManuallyDrop::new(key.into()).as_k_ptr()) as *mut _;
                values = kapi::jk(&mut values, mem::ManuallyDrop::new(value.into()).as_k_ptr()) as *mut _;
            }
            KDict(kapi::xD(keys, values))
        }
    }
}

impl Drop for KDict {
    fn drop(&mut self) {
        unsafe {
            kapi::r0(self.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_an_empty_dictionary() {
        let d = KDict::new();
        assert_eq!(0, d.len());
    }

    #[test]
    fn insert_adds_an_item_to_the_dictionary() {
        let mut d = KDict::new();
        d.insert(1i32, 3i32);
        assert_eq!(
            vec![(&KAny::from(1i32), &KAny::from(3i32))],
            d.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn iter_returns_key_value_tuples() {
        let mut d = KDict::new();
        d.insert(1i32, 3i32);
        d.insert(2i32, 6i32);
        d.insert(4i32, 9i32);
        assert_eq!(
            vec![
                (&KAny::from(1i32), &KAny::from(3i32)),
                (&KAny::from(2i32), &KAny::from(6i32)),
                (&KAny::from(4i32), &KAny::from(9i32))
            ],
            d.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn len_returns_length_of_the_dictionary() {
        let mut d = KDict::new();
        d.insert(1i32, 3i32);
        d.insert(2i32, 6i32);
        d.insert(4i32, 9i32);
        assert_eq!(3, d.len());
    }

    #[test]
    fn index_key_returns_matching_item() {
        let mut d = KDict::new();
        d.insert(1i32, 3i32);
        d.insert(2i32, 6i32);
        d.insert(4i32, 9i32);
        assert_eq!(KAny::from(6i32), d[2i32]);
    }
}
