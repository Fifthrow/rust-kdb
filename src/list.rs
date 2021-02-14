use crate::kbox::KBox;
use crate::type_traits::*;
use crate::{k::K, ConversionError};
use crate::{k_type::KTypeCode, kapi};
use std::slice;
use std::{marker::PhantomData, slice::SliceIndex};
use std::{mem, str};
use std::{ops, ptr::NonNull};

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

/// Lists are the KDB equivalent of Rust's `Vec`. They contain collections of values
/// and their contents be looked up by index.
///
/// # Examples
/// ```
/// use kdb::{KBox, List, list};
///
/// let mut l = list![i32; 1, 2, 3, 4, 5];
/// l.push(6);
/// let sl = &l[..]; // we can take slices and use them like other rust slices.
/// assert_eq!(21, sl.iter().copied().sum());
/// ```
///
/// # Appending to lists
///
/// When you append to lists in KDB, it will potentially reallocate the raw list
/// which gives you a new and different pointer. So you can only extend lists safely if you own them,
/// which means methods like `push` and `join` are only available on types of `KBox<List<_>>`.
///
/// You
/// Notes for best performance: using `list!` or `.collect()` to create a populated list will typically result in better performance
/// than using `new` and `push`. This is because they will, where possible, allocate a list large enough for
/// all items up front. `push` will reallocate whenever needed.
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

    /// Returns an iterator over the list.
    #[inline]
    pub fn iter(&self) -> Iter<T::ListItem> {
        self.as_slice().iter()
    }

    /// Returns the number of elements in the list.
    #[inline]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// Returns true if the list has a length of 0.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator that allows modifying each value.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T::ListItem> {
        self.as_slice_mut().iter_mut()
    }

    /// Returns a reference to an element or a subslice, depending on the type of index.
    #[inline]
    pub fn get<I: SliceIndex<[T::ListItem]>>(&self, index: I) -> Option<&I::Output> {
        self.as_slice().get(index)
    }

    /// Returns a mutable reference to an element or a subslice, depending on the type of index.
    #[inline]
    pub fn get_mut<I: SliceIndex<[T::ListItem]>>(&mut self, index: I) -> Option<&mut I::Output> {
        self.as_slice_mut().get_mut(index)
    }
}

impl<T: KListable> KBox<List<T>> {
    /// Creates a new empty list.
    ///
    /// Note that if you are converting a rust collection to a list, `collect` is a more efficient.
    /// If you are creating a list with a set of known elements, use the `list!` macro.
    #[inline]
    pub fn new_list() -> Self {
        unsafe { mem::transmute(kapi::ktn(T::LIST_TYPE_CODE.into(), 0)) }
    }

    /// Appends a list to this one, consuming it and adding it's elements to the new one.
    #[inline]
    pub fn join(&mut self, list: KBox<List<T>>) {
        unsafe {
            self.k = NonNull::new_unchecked(
                kapi::jv(&mut (self.k.as_ptr() as *mut K), list.into_raw() as *const K) as *mut K as *mut List<T>
            )
        }
    }

    /// Appends an element to the end of the list.
    #[inline]
    pub fn push(&mut self, item: T::ListItem) {
        unsafe {
            self.k = NonNull::new_unchecked(T::join_to(item, self.k.as_ptr() as *mut K) as *mut List<T>);
        }
    }
}

impl<T: KListable> KTyped for List<T> {
    const K_TYPE: KTypeCode = T::LIST_TYPE_CODE;
}

impl List<i8> {
    /// Attempts to convert to a valid utf-8 string.
    /// This will return an error if the string contains invalid utf-8 characters.
    /// This function does not allocate.
    pub fn try_as_str(&self) -> Result<&str, ConversionError> {
        #[allow(clippy::transmute_ptr_to_ptr)]
        let s: &[u8] = unsafe { mem::transmute(self.as_slice()) };
        Ok(str::from_utf8(s)?)
    }

    /// Converts the symbol to a rust str without checking if it is valid.
    ///
    /// # Safety
    ///
    /// The string must be valid UTF-8.
    /// It's length must be less than or equal to isize::MAX.
    pub unsafe fn as_str_unchecked(&self) -> &str {
        #[allow(clippy::transmute_ptr_to_ptr)]
        let s: &[u8] = mem::transmute(self.as_slice());
        str::from_utf8_unchecked(s)
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

/// Create a list from a set of supplied values.
///
/// #Example
/// ```
/// # use std::time::SystemTime;
/// use kdb::{Any, KBox, List, Timestamp, list, symbol};
/// // A list of ints, using type inference
/// let a = list![i32; 1, 2, 3, 4, 5];
/// assert_eq!(15, a.iter().copied().sum());
///
/// // Same list, without an explicit type in the macro
/// let b: KBox<List<i32>> = list![1, 2, 3, 4, 5];
///
/// // Creating a mixed list requires the type
/// let c = list![Any; 1, symbol("Hello"), Timestamp::from(SystemTime::now())];
/// assert_eq!(c.len(), 3)
#[macro_export]
macro_rules! list{
    ($($expr:expr),+$(,)?) => {
        list![_;$($expr,)+]
    };
    ($t:ty; $($expr:expr),+$(,)?) => {
        vec![$($expr.into(),)+].into_iter().collect::<$crate::KBox<$crate::List<$t>>>()
    };
}

#[cfg(test)]
mod test {
    #![allow(clippy::float_cmp)]

    use crate::{symbol, Date, DateTime, Minute, Month, Second, Symbol, Time, Timespan, Timestamp};

    #[cfg(feature = "uuid")]
    use uuid::Uuid;

    #[test]
    pub fn list_macro_creates_lists() {
        assert_eq!(6u8, list![u8; 1, 2, 3 ].iter().copied().sum());
        assert_eq!(6i8, list![i8; 1, 2, 3 ].iter().copied().sum());
        assert_eq!(6i16, list![i16; 1i16, 2i16, 3i16].iter().copied().sum());
        assert_eq!(6i32, list![i32; 1, 2, 3 ].iter().copied().sum());
        assert_eq!(6i64, list![i64; 1, 2, 3 ].iter().copied().sum());

        assert_eq!(6f32, list![f32; 1., 2., 3. ].iter().copied().sum());
        assert_eq!(6f64, list![f64; 1., 2., 3. ].iter().copied().sum());
        assert_eq!(
            vec![true, false, true],
            list![bool; true, false, true].iter().copied().collect::<Vec<_>>()
        );

        assert_eq!(
            vec![Second::from(1), Second::from(2), Second::from(3)],
            list![Second; 1, 2, 3].iter().copied().collect::<Vec<_>>()
        );

        assert_eq!(
            vec![Minute::from(1), Minute::from(2), Minute::from(3)],
            list![Minute; 1, 2, 3].iter().copied().collect::<Vec<_>>()
        );

        assert_eq!(
            vec![Month::from(1), Month::from(2), Month::from(3)],
            list![Month; 1, 2, 3].iter().copied().collect::<Vec<_>>()
        );

        assert_eq!(
            vec![Time::from(1), Time::from(2), Time::from(3)],
            list![Time; 1, 2, 3].iter().copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![Date::new(2020, 1, 1), Date::new(2020, 1, 2), Date::new(2020, 1, 3)],
            list![Date; Date::new(2020, 1, 1), Date::new(2020, 1, 2), Date::new(2020, 1, 3)]
                .iter()
                .copied()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![DateTime::from(1.), DateTime::from(2.), DateTime::from(3.)],
            list![DateTime; 1., 2., 3.].iter().copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![Timestamp::from(1), Timestamp::from(2), Timestamp::from(3)],
            list![Timestamp; 1, 2, 3].iter().copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![Timespan::from(1), Timespan::from(2), Timespan::from(3)],
            list![Timespan; 1, 2, 3].iter().copied().collect::<Vec<_>>()
        );

        assert_eq!(
            vec![symbol("Hello"), symbol("World")],
            list![Symbol; symbol("Hello"), symbol("World")]
                .iter()
                .copied()
                .collect::<Vec<_>>()
        );

        #[cfg(feature = "uuid")]
        assert_eq!(
            vec![Uuid::from_u128(1), Uuid::from_u128(2), Uuid::from_u128(3)],
            list![Uuid;Uuid::from_u128(1), Uuid::from_u128(2), Uuid::from_u128(3)]
                .iter()
                .copied()
                .collect::<Vec<_>>()
        );
    }
}
