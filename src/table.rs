use crate::{k::K, k_type::TABLE, type_traits::KObject};

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
