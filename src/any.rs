use crate::kbox::KBox;
use crate::list::List;
use crate::type_traits::*;
use crate::{atom::Atom, k_type::DICT};
use crate::{error::ConversionError, Dictionary};
use crate::{k::K, Table};
use crate::{k_type::KTypeCode, k_type::MIXED_LIST};
use crate::{k_type::TABLE, kapi};
use std::convert::TryFrom;
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

impl<T> TryFrom<&Any> for &Atom<T>
where
    T: KValue,
{
    type Error = ConversionError;

    fn try_from(any: &Any) -> Result<Self, Self::Error> {
        if any.k.t == T::TYPE_CODE.as_atom() {
            Ok(unsafe { &*(any as *const _ as *const _) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.k.t,
                to: T::TYPE_CODE.as_atom(),
            })
        }
    }
}

impl<T> TryFrom<KBox<Any>> for KBox<Atom<T>>
where
    T: KValue,
{
    type Error = ConversionError;

    fn try_from(any: KBox<Any>) -> Result<Self, Self::Error> {
        if any.as_ref().k.t == T::TYPE_CODE.as_atom() {
            Ok(unsafe { mem::transmute(any) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.as_ref().k.t,
                to: T::TYPE_CODE.as_atom(),
            })
        }
    }
}

impl<T> TryFrom<&Any> for &List<T>
where
    T: KListable,
{
    type Error = ConversionError;

    fn try_from(any: &Any) -> Result<Self, Self::Error> {
        if any.k.t == T::LIST_TYPE_CODE {
            Ok(unsafe { &*(any as *const _ as *const _) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.k.t,
                to: T::LIST_TYPE_CODE,
            })
        }
    }
}

impl<T> TryFrom<KBox<Any>> for KBox<List<T>>
where
    T: KListable,
{
    type Error = ConversionError;

    fn try_from(any: KBox<Any>) -> Result<Self, Self::Error> {
        if any.as_ref().k.t == T::LIST_TYPE_CODE {
            Ok(unsafe { mem::transmute(any) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.as_ref().k.t,
                to: T::LIST_TYPE_CODE,
            })
        }
    }
}

impl TryFrom<&Any> for &Dictionary {
    type Error = ConversionError;

    fn try_from(any: &Any) -> Result<Self, Self::Error> {
        if any.k.t == DICT {
            Ok(unsafe { &*(any as *const _ as *const _) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.k.t,
                to: DICT,
            })
        }
    }
}

impl TryFrom<KBox<Any>> for KBox<Dictionary> {
    type Error = ConversionError;

    fn try_from(any: KBox<Any>) -> Result<Self, Self::Error> {
        if any.as_ref().k.t == DICT {
            Ok(unsafe { mem::transmute(any) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.as_ref().k.t,
                to: DICT,
            })
        }
    }
}

impl From<KBox<Dictionary>> for KBox<Any> {
    fn from(value: KBox<Dictionary>) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl TryFrom<&Any> for &Table {
    type Error = ConversionError;

    fn try_from(any: &Any) -> Result<Self, Self::Error> {
        if any.k.t == TABLE {
            Ok(unsafe { &*(any as *const _ as *const _) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.k.t,
                to: TABLE,
            })
        }
    }
}

impl TryFrom<KBox<Any>> for KBox<Table> {
    type Error = ConversionError;

    fn try_from(any: KBox<Any>) -> Result<Self, Self::Error> {
        if any.as_ref().k.t == TABLE {
            Ok(unsafe { mem::transmute(any) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.as_ref().k.t,
                to: TABLE,
            })
        }
    }
}

impl From<KBox<Table>> for KBox<Any> {
    fn from(value: KBox<Table>) -> Self {
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
    fn cast(self) -> Self::Output;
}

impl<T: KObject + KTyped> KdbCast<T> for KBox<Any> {
    type Output = KBox<T>;
    fn cast(self) -> KBox<T> {
        unsafe {
            if (*self.k_ptr()).t != T::K_TYPE {
                panic!("Invalid KDB cast from {} to {}", (*self.k_ptr()).t, T::K_TYPE);
            }
            mem::transmute(self)
        }
    }
}

impl<'a, T: 'a + KObject + KTyped> KdbCast<T> for &'a KBox<Any> {
    type Output = &'a KBox<T>;
    fn cast(self) -> &'a KBox<T> {
        unsafe {
            if (*self.k_ptr()).t != T::K_TYPE {
                panic!("Invalid KDB cast from {} to {}", (*self.k_ptr()).t, T::K_TYPE);
            }
            #[allow(clippy::clippy::transmute_ptr_to_ptr)]
            mem::transmute(self)
        }
    }
}

impl<'a, T: 'a + KObject + KTyped> KdbCast<T> for &'a Any {
    type Output = &'a T;
    fn cast(self) -> &'a T {
        unsafe {
            if (*self.k_ptr()).t != T::K_TYPE {
                panic!("Invalid KDB cast from {} to {}", (*self.k_ptr()).t, T::K_TYPE);
            }
            #[allow(clippy::clippy::transmute_ptr_to_ptr)]
            mem::transmute(self)
        }
    }
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
        kdb::KdbCast::<$t>::cast($ex)
    };
}
