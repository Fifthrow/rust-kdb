use crate::any::KAny;
use crate::atoms::KItem;
use crate::dict::KDict;
use crate::error::ConversionError;
use crate::raw::kapi;
use crate::raw::types::{K, TABLE};
use std::convert::TryFrom;
use std::mem;

#[repr(transparent)]
pub struct KTable(*const K);

impl KTable {
    pub fn column_names(&self) -> &[KAny] {
        let dict = unsafe { &*(&self as *const _ as *const KDict) };
        dict.keys()
    }

    //TODO: This might actually return a KMixedList - check docs
    pub fn columns(&self) -> &[KAny] {
        let dict = unsafe { &*(&self as *const _ as *const KDict) };
        dict.values()
    }
}

impl KItem for KTable {
    fn as_k_ptr(&self) -> *const K {
        self.0
    }
}

impl From<KTable> for KAny {
    fn from(table: KTable) -> KAny {
        unsafe { mem::transmute(table) }
    }
}

impl TryFrom<KAny> for KTable {
    type Error = ConversionError;
    fn try_from(any: KAny) -> Result<Self, Self::Error> {
        if any.k_type() == TABLE {
            Ok(unsafe { mem::transmute(any) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.k_type(),
                to: TABLE,
            })
        }
    }
}

impl Drop for KTable {
    fn drop(&mut self) {
        unsafe {
            kapi::r0(self.0);
        }
    }
}

/*
    table_flip!(table, type: Something, column_transforms => {
        a: KIntList::Item => {}
        b: KBoolList::Item => {}
        c: KMixedList::Item => {}
    })
*/
