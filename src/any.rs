use crate::atom::Atom;
use crate::kapi;
use crate::kbox::KBox;
use crate::type_traits::*;
use crate::{error::ConversionError, Dictionary};
use crate::{k::K, Table};
use crate::{k_type::KTypeCode, k_type::MIXED_LIST};
use crate::{list::List, KError};
use std::fmt;
use std::mem;

/// Any represents any KDB value regardless of type.
/// Unlike atoms or lists you can't do anything with it, except for convert it into an atom or a list.
/// It is ABI compatible with a K object, so it can be safely used as a parameter or return type for a function.
/// See the chapter on embedded functions for more information.
#[repr(transparent)]
#[derive(PartialEq)]
pub struct Any {
    k: K,
}

impl fmt::Debug for Any {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Any(Type={})", self.k.t)
    }
}

impl<T: KValue> AsRef<Any> for Atom<T> {
    fn as_ref(&self) -> &Any {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl<T: KListable> AsRef<Any> for List<T> {
    fn as_ref(&self) -> &Any {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl AsRef<Any> for Dictionary {
    fn as_ref(&self) -> &Any {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl AsRef<Any> for Table {
    fn as_ref(&self) -> &Any {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl AsRef<Any> for KError {
    fn as_ref(&self) -> &Any {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl From<KBox<Dictionary>> for KBox<Any> {
    fn from(value: KBox<Dictionary>) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<KBox<Table>> for KBox<Any> {
    fn from(value: KBox<Table>) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<KBox<KError>> for KBox<Any> {
    fn from(value: KBox<KError>) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl<T: KListable> From<KBox<List<T>>> for KBox<Any> {
    fn from(value: KBox<List<T>>) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl<T: KValue> From<KBox<Atom<T>>> for KBox<Any> {
    fn from(value: KBox<Atom<T>>) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl<T: KValue> From<T> for KBox<Any> {
    fn from(value: T) -> Self {
        unsafe { mem::transmute(KBox::new_atom(value)) }
    }
}

impl KObject for Any {
    #[inline]
    fn k_ptr(&self) -> *const K {
        &self.k
    }

    #[inline]
    fn k_ptr_mut(&mut self) -> *mut K {
        &mut self.k
    }
}

impl KListable for Any {
    const LIST_TYPE_CODE: KTypeCode = MIXED_LIST;
    type ListItem = KBox<Any>;

    unsafe fn join_to(item: Self::ListItem, mut k: *mut K) -> *mut K {
        // don't r0 this - it's owned by the list now.
        kapi::jk(&mut k, mem::ManuallyDrop::new(item).k_ptr())
    }
}

#[doc(hidden)]
pub trait KdbCast<T: Sized> {
    type Output;
    fn try_cast(self) -> Result<Self::Output, ConversionError>;
}

impl<T: KObject + KTyped> KdbCast<T> for KBox<Any> {
    type Output = KBox<T>;
    fn try_cast(self) -> Result<Self::Output, ConversionError> {
        unsafe {
            if (*self.k_ptr()).t != T::K_TYPE {
                Err(ConversionError::InvalidKCast {
                    from: (*self.k_ptr()).t,
                    to: T::K_TYPE,
                })
            } else {
                #[allow(clippy::clippy::transmute_ptr_to_ptr)]
                Ok(mem::transmute(self))
            }
        }
    }
}

impl<'a, T: 'a + KObject + KTyped> KdbCast<T> for &'a KBox<Any> {
    type Output = &'a KBox<T>;
    fn try_cast(self) -> Result<Self::Output, ConversionError> {
        unsafe {
            if (*self.k_ptr()).t != T::K_TYPE {
                Err(ConversionError::InvalidKCast {
                    from: (*self.k_ptr()).t,
                    to: T::K_TYPE,
                })
            } else {
                #[allow(clippy::clippy::transmute_ptr_to_ptr)]
                Ok(mem::transmute(self))
            }
        }
    }
}

impl<'a, T: 'a + KObject + KTyped> KdbCast<T> for &'a Any {
    type Output = &'a T;
    fn try_cast(self) -> Result<Self::Output, ConversionError> {
        unsafe {
            if (*self.k_ptr()).t != T::K_TYPE {
                Err(ConversionError::InvalidKCast {
                    from: (*self.k_ptr()).t,
                    to: T::K_TYPE,
                })
            } else {
                #[allow(clippy::clippy::transmute_ptr_to_ptr)]
                Ok(mem::transmute(self))
            }
        }
    }
}

/// Tries to cast a KBox<Any>, &KBox<Any> or &Any to another KDB type.
///
/// returns a result containing the KDB type, or a `ConversionError`.
///
/// # Example
///
/// ```
/// # use kdb::{try_cast, symbol, Any, Atom, KBox, Symbol};
/// let a: KBox<Any> = symbol("Hello").into();
/// assert_eq!(symbol("Hello"), try_cast!(a; Atom<Symbol>).unwrap().value());
/// ```
#[macro_export]
macro_rules! try_cast {
    ($ex:expr; $t:ty) => {
        $crate::KdbCast::<$t>::try_cast($ex)
    };
}

/// Cast a KBox<Any>, &KBox<Any> or &Any to another KDB type.
///
/// If the KDB type is anything other than the expected one, the cast will panic.
///
/// # Example
///
/// ```
/// # use kdb::{cast, symbol, Any, Atom, KBox, Symbol};
/// let a: KBox<Any> = symbol("Hello").into();
/// assert_eq!(symbol("Hello"), cast!(a; Atom<Symbol>).value());
/// ```
#[macro_export]
macro_rules! cast {
    ($ex:expr; $t:ty) => {
        $crate::KdbCast::<$t>::try_cast($ex).unwrap()
    };
}
