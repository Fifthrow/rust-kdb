use crate::atom::Atom;
use crate::error::ConversionError;
use crate::k::K;
use crate::kapi;
use crate::kbox::KBox;
use crate::list::List;
use crate::type_traits::*;
use crate::{k_type::KTypeCode, k_type::MIXED_LIST};
use std::convert::TryFrom;
use std::fmt;
use std::mem;

/// Any represents any KDB value regardless of type.
/// Unlike atoms or lists you can't do anything with it, except for convert it into an atom or a list.
/// It is ABI compatible with a K object, so it can be safely used as a parameter or return type for a function.
/// See the chapter on embedded functions for more information.
#[repr(transparent)]
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
    T: KValue,
{
    type Error = ConversionError;

    fn try_from(any: &Any) -> Result<Self, Self::Error> {
        if any.k.t == T::TYPE_CODE.as_list() {
            Ok(unsafe { &*(any as *const _ as *const _) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.k.t,
                to: T::TYPE_CODE.as_list(),
            })
        }
    }
}

impl<T> TryFrom<KBox<Any>> for KBox<List<T>>
where
    T: KValue,
{
    type Error = ConversionError;

    fn try_from(any: KBox<Any>) -> Result<Self, Self::Error> {
        if any.as_ref().k.t == T::TYPE_CODE.as_list() {
            Ok(unsafe { mem::transmute(any) })
        } else {
            Err(ConversionError::InvalidKCast {
                from: any.as_ref().k.t,
                to: T::TYPE_CODE.as_list(),
            })
        }
    }
}

impl<T: KValue> From<KBox<List<T>>> for KBox<Any> {
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
    fn k_ptr(&self) -> *const K {
        &self.k
    }

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
