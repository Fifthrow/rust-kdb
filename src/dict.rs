use crate::any::KAny;
use crate::atoms::KItem;
use crate::error::ConversionError;
use crate::raw::kapi;
use crate::raw::types::{as_slice, DICT, K, MIXED_LIST};
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::mem;
use std::ops::Index;

pub struct KDict(*const K);

impl KItem for KDict {
    const K_TYPE: KType = DICT;
    fn as_k_ptr(&self) -> *const K {
        self.0
    }
}

impl KDict {
    pub fn len(&self) -> usize {
        unsafe {
            let key_list = (*self.0).union.list.g0 as *const _ as *const K;
            (*key_list).union.list.n as usize
        }
    }

    /// Gets a slice containing all the keys in this dictionary
    pub fn keys(&self) -> &[KAny] {
        unsafe {
            let key_list = (*self.0).union.list.g0 as *const _ as *const K;
            as_slice(key_list)
        }
    }

    /// Gets a slice containing all the values in this dictionary
    pub fn values(&self) -> &[KAny] {
        unsafe {
            let value_list = ((*self.0).union.list.g0 as *const _ as *const K).offset(1);
            as_slice(value_list)
        }
    }

    /// Gets a value by key. Note that K dictionaries are unordered and hence is an O(n) operation.
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
                kapi::jk(&mut keys, mem::ManuallyDrop::new(key.into()).as_k_ptr());
                kapi::jk(&mut values, mem::ManuallyDrop::new(value.into()).as_k_ptr());
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
