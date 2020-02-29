use libc;
use std::convert::TryFrom;
use std::fmt;
use std::slice;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

//TODO: Timespan,
pub const MIXED_LIST: KType = KType(0);
pub const BOOLEAN_ATOM: KType = KType(-1);
pub const GUID_ATOM: KType = KType(-2);
pub const BYTE_ATOM: KType = KType(-4);
pub const SHORT_ATOM: KType = KType(-5);
pub const INT_ATOM: KType = KType(-6);
pub const LONG_ATOM: KType = KType(-7);
pub const REAL_ATOM: KType = KType(-8);
pub const FLOAT_ATOM: KType = KType(-9);
pub const CHAR_ATOM: KType = KType(-10);
pub const SYMBOL_ATOM: KType = KType(-11);
pub const TIMESTAMP_ATOM: KType = KType(-12);
pub const MONTH_ATOM: KType = KType(-13);
pub const DATE_ATOM: KType = KType(-14);
pub const DATE_TIME_ATOM: KType = KType(-15);
pub const TIMESPAN_ATOM: KType = KType(-16);
pub const MINUTE_ATOM: KType = KType(-17);
pub const SECOND_ATOM: KType = KType(-18);
pub const TIME_ATOM: KType = KType(-19);
pub const BOOLEAN_LIST: KType = KType(1);
pub const GUID_LIST: KType = KType(2);
pub const BYTE_LIST: KType = KType(4);
pub const SHORT_LIST: KType = KType(5);
pub const INT_LIST: KType = KType(6);
pub const LONG_LIST: KType = KType(7);
pub const REAL_LIST: KType = KType(8);
pub const FLOAT_LIST: KType = KType(9);
pub const CHAR_LIST: KType = KType(10);
pub const SYMBOL_LIST: KType = KType(11);
pub const TIMESTAMP_LIST: KType = KType(12);
pub const MONTH_LIST: KType = KType(13);
pub const DATE_LIST: KType = KType(14);
pub const DATE_TIME_LIST: KType = KType(15);
pub const TIMESPAN_LIST: KType = KType(16);
pub const MINUTE_LIST: KType = KType(17);
pub const SECOND_LIST: KType = KType(18);
pub const TIME_LIST: KType = KType(19);
pub const TABLE: KType = KType(98);
pub const DICT: KType = KType(99);
pub const ERROR: KType = KType(-128);

pub type S = *const libc::c_char;
pub type C = libc::c_char;
pub type G = libc::c_uchar;
pub type H = libc::c_short;
pub type I = libc::c_int;
pub type J = libc::c_longlong;
pub type E = libc::c_float;
pub type F = libc::c_double;
pub type V = libc::c_void;

#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
pub struct KType(libc::c_char);

impl From<KType> for i32 {
    fn from(kt: KType) -> i32 {
        kt.0 as i32
    }
}

impl fmt::Display for KType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MIXED_LIST => write!(f, "mixed list"),
            BOOLEAN_ATOM => write!(f, "boolean atom"),
            GUID_ATOM => write!(f, "guid atom"),
            BYTE_ATOM => write!(f, "byte atom"),
            SHORT_ATOM => write!(f, "short atom"),
            INT_ATOM => write!(f, "int atom"),
            LONG_ATOM => write!(f, "long atom"),
            REAL_ATOM => write!(f, "real atom"),
            FLOAT_ATOM => write!(f, "float atom"),
            CHAR_ATOM => write!(f, "char atom"),
            SYMBOL_ATOM => write!(f, "symbol atom"),
            TIMESTAMP_ATOM => write!(f, "timestamp atom"),
            MONTH_ATOM => write!(f, "month atom"),
            DATE_ATOM => write!(f, "date atom"),
            DATE_TIME_ATOM => write!(f, "dateTime atom"),
            TIMESPAN_ATOM => write!(f, "timespan atom"),
            MINUTE_ATOM => write!(f, "minute atom"),
            SECOND_ATOM => write!(f, "second atom"),
            TIME_ATOM => write!(f, "time atom"),
            BOOLEAN_LIST => write!(f, "boolean list"),
            GUID_LIST => write!(f, "guid list"),
            BYTE_LIST => write!(f, "byte list"),
            SHORT_LIST => write!(f, "short list"),
            INT_LIST => write!(f, "int list"),
            LONG_LIST => write!(f, "long list"),
            REAL_LIST => write!(f, "real list"),
            FLOAT_LIST => write!(f, "float list"),
            CHAR_LIST => write!(f, "char list"),
            SYMBOL_LIST => write!(f, "symbol list"),
            TIMESTAMP_LIST => write!(f, "timestamp list"),
            MONTH_LIST => write!(f, "month list"),
            DATE_LIST => write!(f, "date list"),
            DATE_TIME_LIST => write!(f, "datetime list"),
            TIMESPAN_LIST => write!(f, "timespan list"),
            MINUTE_LIST => write!(f, "minute list"),
            SECOND_LIST => write!(f, "second list"),
            TIME_LIST => write!(f, "time list"),
            TABLE => write!(f, "table"),
            DICT => write!(f, "dict"),
            ERROR => write!(f, "error"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl fmt::Debug for KType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self, self.0)
    }
}

impl KType {
    pub fn atom_size(&self) -> usize {
        match KType(self.0.abs()) {
            BOOLEAN_LIST | BYTE_LIST | CHAR_LIST => 1,
            SHORT_LIST => 2,
            INT_LIST | REAL_LIST | DATE_LIST | MINUTE_LIST | SECOND_LIST | MONTH_LIST | TIME_LIST => 4,
            LONG_LIST | FLOAT_LIST | DATE_TIME_LIST | TIMESTAMP_LIST | TIMESPAN_LIST => 8,
            GUID_LIST => 16,
            SYMBOL_LIST | MIXED_LIST | TABLE | DICT | ERROR => std::mem::size_of::<*const u8>(),
            _ => panic!("Unknown K type: {}", self.0),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct List {
    pub n: J,
    pub g0: *mut G,
}

pub(crate) unsafe fn as_slice<'a, T>(k: *const K) -> &'a [T] {
    let list = &(*k).union.list;
    slice::from_raw_parts(&list.g0 as *const _ as *const _, list.n as usize)
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
    pub u: KGuid,
    pub k0: *mut K,
    pub list: List,
    // custom accessors for the wrapping types  - these make the implementation macros easier
    // so we don't have to do special cases or lots of typecasting - we can just get the union
    // to do that for us! Note that all the wrapping types *must* be repr(transparent), otherwise
    // it's undefined behaviour as the the underlying representation in the compiler is not defined.
    // It's worth noting that KDB uses a bit as a boolean type, but stores it in a byte. Coincidentally
    // That maps exactly to a rust bool (which *must* be either 1 or 0 else behavious is undefined).
    pub bl: bool,
    pub sym: KSymbol,
    pub ts: KTimespan,
    pub t: KTime,
    pub m: KMonth,
    pub tst: KTimestamp,
    pub min: KMinute,
    pub sec: KSecond,
    pub d: KDate,
    pub dt: KDateTime,
}

#[repr(C)]
pub struct K {
    pub m: libc::c_char,
    pub a: libc::c_char,
    pub t: KType,
    pub u: C,
    pub r: I,
    pub union: KUnion,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Attr(u8);

impl Attr {
    pub fn sorted(&self) -> bool {
        self.0 == 1
    }
    pub fn unique(&self) -> bool {
        self.0 == 2
    }
    pub fn partioned(&self) -> bool {
        self.0 == 3
    }
    pub fn grouped(&self) -> bool {
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

impl PartialEq for K {
    fn eq(&self, other: &K) -> bool {
        if self.t != other.t {
            return false;
        }

        match self.t {
            //Atoms
            t if t > KType(0) && t < KType(20) => unsafe {
                libc::memcmp(
                    &self.union as *const _ as _,
                    &other.union as *const _ as _,
                    t.atom_size(),
                ) == 0
            },
            _ => unimplemented!("Comparison for non-atom K objects not implemented"),
        }
    }
}

impl fmt::Debug for K {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "K{{ {k_type:?}, {attrs:?}, ref_count = {r}, value = {value}}}",
            k_type = self.t,
            attrs = self.u,
            r = self.r,
            value = "..." //self.debug_value_str()
        )?;
        Ok(())
    }
}

/// Represents a KDB Symbol (interned string)
/// Implements basic symbol operations for efficiency
/// Can be converted to/from strings using comparisons
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct KSymbol(*const i8);

impl PartialEq for KSymbol {
    fn eq(&self, other: &KSymbol) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl Eq for KSymbol {}

impl std::hash::Hash for KSymbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state)
    }
}

// TODO: safety warning - need to do this in some private way. Might need to reimplement KSymbolAtom manually instead of via macro
impl From<KSymbol> for *const i8 {
    fn from(sym: KSymbol) -> Self {
        sym.0
    }
}

impl std::convert::TryFrom<String> for KSymbol {
    type Error = std::ffi::NulError;
    fn try_from(s: String) -> Result<KSymbol, Self::Error> {
        Ok(KSymbol(unsafe { super::kapi::ss(std::ffi::CString::new(s)?.as_ptr()) }))
    }
}

impl std::convert::TryFrom<&str> for KSymbol {
    type Error = std::ffi::NulError;
    fn try_from(s: &str) -> Result<KSymbol, Self::Error> {
        Ok(KSymbol(unsafe { super::kapi::ss(std::ffi::CString::new(s)?.as_ptr()) }))
    }
}

impl std::convert::TryFrom<KSymbol> for String {
    type Error = std::str::Utf8Error;
    fn try_from(sym: KSymbol) -> Result<String, Self::Error> {
        let sym = unsafe { std::ffi::CStr::from_ptr(sym.0 as *const _) };
        sym.to_str().map(str::to_owned)
    }
}

impl std::convert::TryFrom<&KSymbol> for String {
    type Error = std::str::Utf8Error;
    fn try_from(sym: &KSymbol) -> Result<String, Self::Error> {
        let sym = unsafe { std::ffi::CStr::from_ptr(sym.0 as *const _) };
        sym.to_str().map(str::to_owned)
    }
}

impl fmt::Display for KSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            String::try_from(self)
                .as_ref()
                .map(|x| &x[..])
                .unwrap_or("<invalid rust string>")
        )
    }
}

impl fmt::Debug for KSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct KSecond(i32);
impl From<i32> for KSecond {
    fn from(val: i32) -> KSecond {
        KSecond(val)
    }
}

impl From<KSecond> for i32 {
    fn from(val: KSecond) -> i32 {
        val.0
    }
}

impl fmt::Display for KSecond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} seconds", self.0)
    }
}

impl fmt::Debug for KSecond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct KMinute(i32);
impl From<i32> for KMinute {
    fn from(val: i32) -> KMinute {
        KMinute(val)
    }
}

impl From<KMinute> for i32 {
    fn from(val: KMinute) -> i32 {
        val.0
    }
}

impl fmt::Display for KMinute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} seconds", self.0)
    }
}

impl fmt::Debug for KMinute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct KDate(i32);
impl From<i32> for KDate {
    fn from(val: i32) -> KDate {
        KDate(val)
    }
}

impl From<KDate> for i32 {
    fn from(val: KDate) -> i32 {
        val.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct KMonth(i32);
impl From<i32> for KMonth {
    fn from(val: i32) -> KMonth {
        KMonth(val)
    }
}

impl From<KMonth> for i32 {
    fn from(val: KMonth) -> i32 {
        val.0
    }
}

impl fmt::Display for KMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} months", self.0)
    }
}

impl fmt::Debug for KMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct KTime(i32);
impl From<i32> for KTime {
    fn from(val: i32) -> KTime {
        KTime(val)
    }
}

impl From<KTime> for i32 {
    fn from(val: KTime) -> i32 {
        val.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct KDateTime(f64);

impl From<f64> for KDateTime {
    fn from(val: f64) -> KDateTime {
        KDateTime(val)
    }
}

impl From<KDateTime> for f64 {
    fn from(val: KDateTime) -> f64 {
        val.0
    }
}

/// Represents 01/01/2000
const K_DATE_START_F64: f64 = 0f64;

impl From<KDateTime> for SystemTime {
    fn from(val: KDateTime) -> SystemTime {
        let secs = f64::from(val) * 86400.0;
        let offset = Duration::from_secs_f64(secs + K_DATE_START_F64);
        SystemTime::UNIX_EPOCH + offset
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct KTimestamp(i64);

impl From<i64> for KTimestamp {
    fn from(val: i64) -> KTimestamp {
        KTimestamp(val)
    }
}

impl From<KTimestamp> for i64 {
    fn from(val: KTimestamp) -> i64 {
        val.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct KTimespan(i64);

impl From<i64> for KTimespan {
    fn from(val: i64) -> KTimespan {
        KTimespan(val)
    }
}

impl From<KTimespan> for i64 {
    fn from(val: KTimespan) -> i64 {
        val.0
    }
}

impl From<KTimespan> for Duration {
    fn from(span: KTimespan) -> Duration {
        Duration::from_nanos(span.0 as u64)
    }
}

impl TryFrom<Duration> for KTimespan {
    type Error = crate::error::ConversionError;

    fn try_from(d: Duration) -> Result<KTimespan, Self::Error> {
        let d = d.as_nanos();
        if d > std::i64::MAX as u128 {
            Err(crate::error::ConversionError::DurationTooLong)
        } else {
            Ok(KTimespan(d as i64))
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct KGuid([u8; 16]);

impl From<[u8; 16]> for KGuid {
    fn from(x: [u8; 16]) -> KGuid {
        KGuid(x)
    }
}

impl From<KGuid> for [u8; 16] {
    fn from(x: KGuid) -> [u8; 16] {
        x.0
    }
}

impl From<Uuid> for KGuid {
    fn from(x: Uuid) -> KGuid {
        KGuid(*x.as_bytes())
    }
}

impl From<KGuid> for Uuid {
    fn from(x: KGuid) -> Uuid {
        Uuid::from_bytes(x.0)
    }
}

impl fmt::Display for KGuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&Uuid::from_bytes(self.0), f)
    }
}

impl fmt::Debug for KGuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
