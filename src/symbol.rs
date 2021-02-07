use crate::error::ConversionError;
use crate::kapi;
use std::ffi::{c_void, CStr};
use std::fmt;
use thiserror::Error;

/// Error type for string to symbol conversions
#[derive(Debug, Error)]
pub enum SymbolError {
    /// An embedded NUL character was found in the string. The string would be truncated at this point if it were passed to KDB
    #[error("Embedded NUL character found at index {0}")]
    InternalNul(usize),
    /// The string is too long to convert to a symbol. It can be a maximum of 2GB.
    #[error("String too long ({0} chars)")]
    StringTooLong(usize),
}

/// Represents a KDB Symbol (interned string)
/// Implements basic symbol operations for efficiency
/// Can be converted to/from strings
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Symbol(*const i8);

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl Eq for Symbol {}

impl std::hash::Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state)
    }
}

extern "C" {
    fn memchr(cx: *const c_void, c: i32, n: usize) -> *mut c_void;
}

impl Symbol {
    /// Create a new symbol from the specified string. If the string is
    /// too long, or contains an embedded nul character, then it returns an error.
    pub fn new<T: AsRef<str>>(st: T) -> Result<Symbol, SymbolError> {
        let s = st.as_ref();

        if s.len() > isize::MAX as usize {
            return Err(SymbolError::StringTooLong(s.len()));
        }

        let first_nul = unsafe { memchr(s.as_ptr() as *const c_void, 0, s.len()) };
        if !first_nul.is_null() {
            return Err(SymbolError::InternalNul(first_nul as usize - s.as_ptr() as usize));
        }

        Ok(Symbol(unsafe { kapi::sn(s.as_ptr() as *const i8, s.len() as i32) }))
    }

    /// Creates a new symbol, skipping the safety checks for length
    ///
    /// # Safety
    ///
    /// Any string passed in must not contain embedded nul characters (`\0`).
    /// It's length must be less than or equal to isize::MAX.
    pub unsafe fn new_unchecked<T: AsRef<str>>(st: T) -> Symbol {
        let s = st.as_ref();
        Symbol(kapi::sn(s.as_ptr() as *const i8, s.len() as i32))
    }

    /// Attempts to convert to a valid utf-8 string.
    /// This will return an error if the string contains invalid utf-8 characters.
    /// This function does not allocate.
    pub fn try_as_str(&self) -> Result<&'static str, ConversionError> {
        Ok(unsafe { CStr::from_ptr(self.0).to_str()? })
    }

    /// Converts the symbol to a rust str without checking if it is valid.
    ///
    /// # Safety
    ///
    /// The string must be valid UTF-8.
    /// It's length must be less than or equal to isize::MAX.
    pub unsafe fn as_str_unchecked(&self) -> &'static str {
        std::str::from_utf8_unchecked(CStr::from_ptr(self.0).to_bytes())
    }
}

impl From<Symbol> for *const i8 {
    fn from(sym: Symbol) -> Self {
        sym.0
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Symbol {
    /// Display for symbol will always render a string representation of the symbol. If the
    /// string contains invalid characters it will strip them from the string.
    /// This function will allocate only if the string conatins invalid utf-8 characters.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cs = unsafe { CStr::from_ptr(self.0) };
        write!(f, "{}", String::from_utf8_lossy(cs.to_bytes()))
    }
}

/// Helper for succinctly creating a symbol. If the string passed in is not a valid symbol,
/// i.e. it contains embedded nuls, then this function will panic.
pub fn symbol(s: &str) -> Symbol {
    Symbol::new(s).unwrap()
}
