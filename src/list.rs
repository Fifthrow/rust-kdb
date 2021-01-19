use crate::k::K;
use crate::kapi;
use crate::kbox::KBox;
use crate::type_traits::*;
use std::marker::PhantomData;
use std::mem;
use std::ops;
use std::slice;

use std::{
    iter::FromIterator,
    slice::{Iter, IterMut},
};

unsafe fn as_slice_uninit<'a, T>(k: *mut K) -> &'a mut [mem::MaybeUninit<T>] {
    let list = &(*k).union.list;
    slice::from_raw_parts_mut(&list.g0 as *const _ as *mut _, list.n as usize)
}

unsafe fn as_slice_mut<'a, T>(k: *mut K) -> &'a mut [T] {
    let list = &(*k).union.list;
    slice::from_raw_parts_mut(&list.g0 as *const _ as *mut _, list.n as usize)
}

unsafe fn as_slice<'a, T>(k: *const K) -> &'a [T] {
    let list = &(*k).union.list;
    slice::from_raw_parts(&list.g0 as *const _ as *const _, list.n as usize)
}

#[repr(transparent)]
pub struct List<T> {
    k: K,
    _p: PhantomData<T>,
}

impl<T: KListable> List<T> {
    /// Returns the contents of the list as a slice
    #[inline]
    pub fn as_slice(&self) -> &[T::ListItem] {
        unsafe { as_slice(&self.k) }
    }

    /// Returns the contents of the list as a mutable slice
    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T::ListItem] {
        unsafe { as_slice_mut(&mut self.k) }
    }

    #[inline]
    pub fn iter(&self) -> Iter<T::ListItem> {
        self.as_slice().iter()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T::ListItem> {
        self.as_slice_mut().iter_mut()
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T::ListItem> {
        self.as_slice().get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T::ListItem> {
        self.as_slice_mut().get_mut(index)
    }
}

// * Actions on owned lists
// Because KDB may return a different pointer when it reallocates
// the list, we can only extend the list if we own it, therefore
// actions that extend lists are only available for KBox<List<T>>

impl<T: KListable> KBox<List<T>> {
    #[inline]
    pub fn new_list() -> Self {
        unsafe { mem::transmute(kapi::ktn(T::LIST_TYPE_CODE.into(), 0)) }
    }

    pub fn join(&mut self, list: KBox<List<T>>) {
        unsafe { self.k = kapi::jv(&mut (self.k as *mut K), list.into_raw() as *const K) as *mut K as *mut List<T> }
    }

    #[inline]
    pub fn push(&mut self, item: T::ListItem) {
        unsafe {
            self.k = T::join_to(item, self.k as *mut K) as * mut List<T>;
        }
    }
}

impl<T> KObject for List<T> {
    fn k_ptr(&self) -> *const K {
        &self.k
    }

    fn k_ptr_mut(&mut self) -> *mut K {
        &mut self.k
    }
}

impl<T> private::Sealed for List<T> {}

impl<T: KListable> ops::Index<ops::RangeFrom<usize>> for List<T> {
    type Output = [T::ListItem];
    fn index(&self, i: ops::RangeFrom<usize>) -> &Self::Output {
        self.as_slice().index(i)
    }
}

impl<T: KListable> ops::Index<ops::RangeTo<usize>> for List<T> {
    type Output = [T::ListItem];
    fn index(&self, i: ops::RangeTo<usize>) -> &Self::Output {
        self.as_slice().index(i)
    }
}

impl<T: KListable> ops::Index<ops::Range<usize>> for List<T> {
    type Output = [T::ListItem];
    fn index(&self, i: ops::Range<usize>) -> &Self::Output {
        self.as_slice().index(i)
    }
}

impl<T: KListable> ops::Index<usize> for List<T> {
    type Output = T::ListItem;
    fn index(&self, i: usize) -> &Self::Output {
        self.as_slice().index(i)
    }
}

impl<T: KListable> ops::Index<ops::RangeFull> for List<T> {
    type Output = [T::ListItem];
    fn index(&self, _: ops::RangeFull) -> &Self::Output {
        self.as_slice()
    }
}

impl<T: KListable> ops::IndexMut<ops::RangeFrom<usize>> for List<T> {
    fn index_mut(&mut self, i: ops::RangeFrom<usize>) -> &mut Self::Output {
        self.as_slice_mut().index_mut(i)
    }
}

impl<T: KListable> ops::IndexMut<ops::RangeTo<usize>> for List<T> {
    fn index_mut(&mut self, i: ops::RangeTo<usize>) -> &mut Self::Output {
        self.as_slice_mut().index_mut(i)
    }
}

impl<T: KListable> ops::IndexMut<ops::Range<usize>> for List<T> {
    fn index_mut(&mut self, i: ops::Range<usize>) -> &mut Self::Output {
        self.as_slice_mut().index_mut(i)
    }
}

impl<T: KListable> ops::IndexMut<usize> for List<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.as_slice_mut().index_mut(i)
    }
}

impl<T: KListable> ops::IndexMut<ops::RangeFull> for List<T> {
    fn index_mut(&mut self, _: ops::RangeFull) -> &mut Self::Output {
        self.as_slice_mut()
    }
}

impl<T: KListable> FromIterator<T::ListItem> for KBox<List<T>> {
    fn from_iter<I: IntoIterator<Item = T::ListItem>>(iter: I) -> Self {
        let iter = iter.into_iter();
        //TODO: Better here would be to specialize on ExactSizeIterator. However I can't do that
        // until specialization is stabilized. This is reliable modulo buggy Iterator impls.
        match iter.size_hint() {
            (x, Some(y)) if x == y => {
                let k = unsafe { kapi::ktn(T::LIST_TYPE_CODE.into(), y as i64) };
                let slice = unsafe { as_slice_uninit(k) };
                slice
                    .iter_mut()
                    .zip(iter)
                    .for_each(|(dest, src)| *dest = mem::MaybeUninit::new(src));
                unsafe { mem::transmute(k) }
            }
            _ => {
                let mut list = Self::new_list();
                list.extend(iter);
                list
            }
        }
    }
}

impl<T: KListable> Extend<T::ListItem> for KBox<List<T>> {
    fn extend<I: IntoIterator<Item = T::ListItem>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}
