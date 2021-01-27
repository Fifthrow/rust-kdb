use crate::date_time_types::*;
use crate::guid::Guid;
use crate::k::K;
use crate::kbox::KBox;
use crate::symbol::Symbol;
use crate::type_traits::*;
use std::fmt;
use std::marker::PhantomData;
use std::mem;

/// Atoms are the base primitive values in rust-kdb. You can create a new atom by calling
/// `KBox::new_atom`, or using the `From`/`Into` traits on a value.
/// 
/// # Examples
/// ```
/// use kdb::{KBox, Atom};
///
/// let a = KBox::new(42u8); // Creates a KBox<Atom<u8>>
/// let b: KBox<Atom<u8>> = 27u8.into();
/// println!("{} dudes!", a.value() + b.value());
/// ```
#[repr(transparent)]
pub struct Atom<T> {
    k: K,
    _p: PhantomData<T>,
}

impl<T: KValue> Atom<T> {
    /// Returns a copy of the value stored in the atom.
    pub fn value(&self) -> T {
        unsafe { T::from_k(&self.k) }
    }
}

impl<T> KObject for Atom<T> {
    fn k_ptr(&self) -> *const K {
        &self.k
    }

    fn k_ptr_mut(&mut self) -> *mut K {
        &mut self.k
    }
}

impl<T> private::Sealed for Atom<T> {}

impl<T: KValue + fmt::Debug> fmt::Debug for Atom<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Atom({:?})", self.value())
    }
}

impl<T: KValue + fmt::Display> fmt::Display for Atom<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl<T: KValue> From<T> for KBox<Atom<T>> {
    fn from(val: T) -> KBox<Atom<T>> {
        KBox {
            k: val.into_k() as *mut K as *mut Atom<T>,
        }
    }
}

// Orphan rules mean I have to implement from type into atom
// use a macro. Sad.
macro_rules! impl_atom_from {
    ($ty:ident) => {
        impl From<Atom<$ty>> for $ty {
            fn from(val: Atom<$ty>) -> $ty {
                val.value()
            }
        }
    };
}

impl_atom_from!(u8);
impl_atom_from!(i8);
impl_atom_from!(i16);
impl_atom_from!(i32);
impl_atom_from!(i64);

impl_atom_from!(f32);
impl_atom_from!(f64);
impl_atom_from!(bool);

impl_atom_from!(Second);
impl_atom_from!(Minute);
impl_atom_from!(Month);
impl_atom_from!(Time);
impl_atom_from!(Date);
impl_atom_from!(DateTime);
impl_atom_from!(Symbol);
impl_atom_from!(Guid);
impl_atom_from!(Timestamp);
impl_atom_from!(Timespan);

impl<T: KValue> KBox<Atom<T>> {
    /// Creates a new atom with the specified value.
    #[inline]
    pub fn new_atom(value: T) -> KBox<Atom<T>> {
        unsafe { mem::transmute(value.into_k()) }
    }
}
