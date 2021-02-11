use crate::k::K;
use crate::kbox::KBox;
use crate::symbol::Symbol;
use crate::type_traits::*;
use crate::{date_time_types::*, k_type::KTypeCode};
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
/// let a = KBox::new_atom(42u8); // Creates a KBox<Atom<u8>>
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
    #[inline]
    pub fn value(&self) -> T {
        unsafe { T::from_k(&self.k) }
    }

    /// Changes the value stored in th atom.
    #[inline]
    pub fn set_value(&mut self, val: T) {
        unsafe { *T::as_mutable(&mut self.k) = val }
    }
}

impl<T> KObject for Atom<T> {
    #[inline]
    fn k_ptr(&self) -> *const K {
        &self.k
    }

    #[inline]
    fn k_ptr_mut(&mut self) -> *mut K {
        &mut self.k
    }
}

impl<T: KValue> KTyped for Atom<T> {
    const K_TYPE: KTypeCode = T::TYPE_CODE.as_atom();
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
    #[inline]
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
            #[inline]
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
impl_atom_from!(Date);
impl_atom_from!(Month);
impl_atom_from!(Time);
impl_atom_from!(DateTime);
impl_atom_from!(Timestamp);
impl_atom_from!(Timespan);

impl_atom_from!(Symbol);

#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cfg(feature = "uuid")]
impl_atom_from!(Uuid);

impl<T: KValue> KBox<Atom<T>> {
    /// Creates a new atom with the specified value.
    #[inline]
    pub fn new_atom(value: T) -> KBox<Atom<T>> {
        unsafe { mem::transmute(value.into_k()) }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::clippy::float_cmp)]

    use super::*;
    use crate::any::Any;
    use crate::symbol::symbol;
    use std::convert::TryFrom;

    #[test]
    fn value_returns_underlying_value() {
        assert_eq!(KBox::new_atom(12u8).value(), 12u8);
        assert_eq!(KBox::new_atom(13i16).value(), 13i16);
        assert_eq!(KBox::new_atom(14i32).value(), 14i32);
        assert_eq!(KBox::new_atom(15i64).value(), 15i64);
        assert_eq!(KBox::new_atom(5.3f32).value(), 5.3f32);
        assert_eq!(KBox::new_atom(true).value(), true);
        assert_eq!(KBox::new_atom(6.4f64).value(), 6.4f64);

        assert_eq!(KBox::new_atom(Second::new(5)).value(), Second::new(5));
        assert_eq!(KBox::new_atom(Minute::new(6)).value(), Minute::new(6));
        assert_eq!(KBox::new_atom(Date::new(2020, 2, 6)).value(), Date::new(2020, 2, 6));
        assert_eq!(KBox::new_atom(Month::new(8)).value(), Month::new(8));
        assert_eq!(KBox::new_atom(Time::new(9)).value(), Time::new(9));
        assert_eq!(KBox::new_atom(DateTime::new(10.0)).value(), DateTime::new(10.0));
        assert_eq!(KBox::new_atom(Timestamp::from_raw(11)).value(), Timestamp::from_raw(11));
        assert_eq!(KBox::new_atom(Timespan::new(12)).value(), Timespan::new(12));

        assert_eq!(KBox::new_atom(symbol("Foo")).value(), symbol("Foo"));
        #[cfg(feature = "uuid")]
        assert_eq!(
            KBox::new_atom(uuid::Uuid::from_bytes([12u8; 16])).value(),
            Uuid::from_bytes([12u8; 16])
        );
    }
    #[test]
    fn set_value_changes_underlying_value() {
        assert_eq!(
            {
                let mut a = KBox::new_atom(11i8);
                a.set_value(12i8);
                a.value()
            },
            12i8
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(12u8);
                a.set_value(13u8);
                a.value()
            },
            13u8
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(13i16);
                a.set_value(14i16);
                a.value()
            },
            14i16
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(14i32);
                a.set_value(15i32);
                a.value()
            },
            15i32
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(15i64);
                a.set_value(16i64);
                a.value()
            },
            16i64
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(5.3f32);
                a.set_value(4.3);
                a.value()
            },
            4.3f32
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(6.4f64);
                a.set_value(4.6f64);
                a.value()
            },
            4.6f64
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(true);
                a.set_value(false);
                a.value()
            },
            false
        );

        assert_eq!(
            {
                let mut a = KBox::new_atom(Second::new(5));
                a.set_value(Second::new(6));
                a.value()
            },
            Second::new(6)
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(Minute::new(6));
                a.set_value(Minute::new(7));
                a.value()
            },
            Minute::new(7)
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(Date::new(2020, 2, 6));
                a.set_value(Date::new(2020, 2, 7));
                a.value()
            },
            Date::new(2020, 2, 7)
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(Month::new(8));
                a.set_value(Month::new(9));
                a.value()
            },
            Month::new(9)
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(Time::new(9));
                a.set_value(Time::new(10));
                a.value()
            },
            Time::new(10)
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(DateTime::new(10.0));
                a.set_value(DateTime::new(11.0));
                a.value()
            },
            DateTime::new(11.0)
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(Timestamp::from_raw(11));
                a.set_value(Timestamp::from_raw(12));
                a.value()
            },
            Timestamp::from_raw(12)
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(Timespan::new(12));
                a.set_value(Timespan::new(13));
                a.value()
            },
            Timespan::new(13)
        );

        assert_eq!(
            {
                let mut a = KBox::new_atom(symbol("Foo"));
                a.set_value(symbol("Bar"));
                a.value()
            },
            symbol("Bar")
        );
        assert_eq!(
            {
                let mut a = KBox::new_atom(Uuid::from_u128(13));
                a.set_value(Uuid::from_u128(14));
                a.value()
            },
            Uuid::from_u128(14)
        );
    }
    #[test]
    fn atoms_round_trip_to_any() {
        assert_eq!(
            KBox::<Atom<u8>>::try_from(KBox::<Any>::from(KBox::new_atom(12u8)))
                .unwrap()
                .value(),
            12u8
        );
        assert_eq!(
            KBox::<Atom<i16>>::try_from(KBox::<Any>::from(KBox::new_atom(13i16)))
                .unwrap()
                .value(),
            13i16
        );
        assert_eq!(
            KBox::<Atom<i32>>::try_from(KBox::<Any>::from(KBox::new_atom(14i32)))
                .unwrap()
                .value(),
            14i32
        );
        assert_eq!(
            KBox::<Atom<i64>>::try_from(KBox::<Any>::from(KBox::new_atom(15i64)))
                .unwrap()
                .value(),
            15i64
        );
        assert_eq!(
            KBox::<Atom<f32>>::try_from(KBox::<Any>::from(KBox::new_atom(5.3f32)))
                .unwrap()
                .value(),
            5.3f32
        );
        assert_eq!(
            KBox::<Atom<bool>>::try_from(KBox::<Any>::from(KBox::new_atom(true)))
                .unwrap()
                .value(),
            true
        );
        assert_eq!(
            KBox::<Atom<f64>>::try_from(KBox::<Any>::from(KBox::new_atom(6.4f64)))
                .unwrap()
                .value(),
            6.4f64
        );

        assert_eq!(
            KBox::<Atom<Second>>::try_from(KBox::<Any>::from(KBox::new_atom(Second::new(5))))
                .unwrap()
                .value(),
            Second::new(5)
        );
        assert_eq!(
            KBox::<Atom<Minute>>::try_from(KBox::<Any>::from(KBox::new_atom(Minute::new(6))))
                .unwrap()
                .value(),
            Minute::new(6)
        );
        assert_eq!(
            KBox::<Atom<Date>>::try_from(KBox::<Any>::from(KBox::new_atom(Date::new(2020, 2, 6))))
                .unwrap()
                .value(),
            Date::new(2020, 2, 6)
        );
        assert_eq!(
            KBox::<Atom<Month>>::try_from(KBox::<Any>::from(KBox::new_atom(Month::new(8))))
                .unwrap()
                .value(),
            Month::new(8)
        );
        assert_eq!(
            KBox::<Atom<Time>>::try_from(KBox::<Any>::from(KBox::new_atom(Time::new(9))))
                .unwrap()
                .value(),
            Time::new(9)
        );
        assert_eq!(
            KBox::<Atom<DateTime>>::try_from(KBox::<Any>::from(KBox::new_atom(DateTime::new(10.0))))
                .unwrap()
                .value(),
            DateTime::new(10.0)
        );
        assert_eq!(
            KBox::<Atom<Timestamp>>::try_from(KBox::<Any>::from(KBox::new_atom(Timestamp::from_raw(11))))
                .unwrap()
                .value(),
            Timestamp::from_raw(11)
        );
        assert_eq!(
            KBox::<Atom<Timespan>>::try_from(KBox::<Any>::from(KBox::new_atom(Timespan::new(12))))
                .unwrap()
                .value(),
            Timespan::new(12)
        );

        assert_eq!(
            KBox::<Atom<Symbol>>::try_from(KBox::<Any>::from(KBox::new_atom(symbol("Foo"))))
                .unwrap()
                .value(),
            symbol("Foo")
        );
        #[cfg(feature = "uuid")]
        assert_eq!(
            KBox::<Atom<uuid::Uuid>>::try_from(KBox::<Any>::from(KBox::new_atom(uuid::Uuid::from_bytes([12u8; 16]))))
                .unwrap()
                .value(),
            uuid::Uuid::from_bytes([12u8; 16])
        );
    }
}
