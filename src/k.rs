use crate::date_time_types::*;
use crate::k_type::KTypeCode;
use crate::symbol::Symbol;
use std::fmt;

pub type S = *const i8;
pub type C = i8;
pub type G = u8;
pub type H = i16;
pub type I = i32;
pub type J = i64;
pub type E = f32;
pub type F = f64;
pub type V = std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct List {
    pub n: J,
    pub g0: *mut G,
}

#[cfg(feature = "uuid")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GuidWithLen {
    pub n: J,
    pub u: uuid::Uuid,
}

#[cfg(feature = "uuid")]
impl From<GuidWithLen> for uuid::Uuid {
    fn from(g: GuidWithLen) -> Self {
        g.u
    }
}

#[repr(C)]
pub union KUnion {
    pub c: C,
    pub g: G,
    pub h: H,
    pub i: I,
    pub j: J,
    pub e: E,
    pub f: F,
    pub s: S,
    #[cfg(feature = "uuid")]
    pub u: GuidWithLen,
    pub k0: *mut K,
    pub list: List,
    // custom accessors for the wrapping types - these make the implementation macros easier
    // so we don't have to do special cases or lots of typecasting - we can just get the union
    // to do that for us! Note that all the wrapping types *must* be repr(transparent), otherwise
    // it's undefined behaviour as the the underlying representation in the compiler is not defined.
    // It's worth noting that KDB uses a bit as a boolean type, but stores it in a byte. Coincidentally
    // that maps exactly to a rust bool (which *must* be either 1 or 0 else behaviour is undefined).
    pub bl: bool,
    pub sym: Symbol,
    pub ts: Timespan,
    pub t: Time,
    pub m: Month,
    pub tst: Timestamp,
    pub min: Minute,
    pub sec: Second,
    pub d: Date,
    pub dt: DateTime,
}

/// The core raw K type. This is exposed, but there should be no need to use it in practice as Atom<T> and List<T>
/// are both ABI compatible with this K.
#[repr(C)]
pub struct K {
    pub m: i8,
    pub a: i8,
    pub t: KTypeCode,
    pub u: C,
    pub r: I,
    pub union: KUnion,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Attr(u8);

impl Attr {
    pub fn sorted(self) -> bool {
        self.0 == 1
    }
    pub fn unique(self) -> bool {
        self.0 == 2
    }
    pub fn partioned(self) -> bool {
        self.0 == 3
    }
    pub fn grouped(self) -> bool {
        self.0 == 5
    }
}

impl std::fmt::Debug for Attr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attributes(")?;
        if self.sorted() {
            write!(f, " Sorted")?;
        }
        if self.unique() {
            write!(f, " Unique")?;
        }
        if self.partioned() {
            write!(f, " Partioned")?;
        }
        if self.grouped() {
            write!(f, " Grouped")?;
        }
        writeln!(f, " )")?;
        Ok(())
    }
}

impl fmt::Debug for K {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "K{{ {k_type:?}, {attrs:?}, ref_count = {r}, value = ...}}",
            k_type = self.t,
            attrs = self.u,
            r = self.r,
        )?;
        Ok(())
    }
}
