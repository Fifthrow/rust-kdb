use crate::{k::K, type_traits::KObject};

/// Represents a table (a dictionary of columns) in KDB
#[repr(transparent)]
pub struct Table {
    k: K,
}

impl KObject for Table {
    #[inline]
    fn k_ptr(&self) -> *const K {
        &self.k
    }

    #[inline]
    fn k_ptr_mut(&mut self) -> *mut K {
        &mut self.k
    }
}
