use std::fmt;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TypeCode(pub(crate) u8);

impl TypeCode {
    pub const BOOLEAN: Self = TypeCode(1);
    pub const GUID: Self = TypeCode(2);
    pub const BYTE: Self = TypeCode(4);
    pub const SHORT: Self = TypeCode(5);
    pub const INT: Self = TypeCode(6);
    pub const LONG: Self = TypeCode(7);
    pub const REAL: Self = TypeCode(8);
    pub const FLOAT: Self = TypeCode(9);
    pub const CHAR: Self = TypeCode(10);
    pub const SYMBOL: Self = TypeCode(11);
    pub const TIMESTAMP: Self = TypeCode(12);
    pub const MONTH: Self = TypeCode(13);
    pub const DATE: Self = TypeCode(14);
    pub const DATE_TIME: Self = TypeCode(15);
    pub const TIMESPAN: Self = TypeCode(16);
    pub const MINUTE: Self = TypeCode(17);
    pub const SECOND: Self = TypeCode(18);
    pub const TIME: Self = TypeCode(19);

    pub const fn as_list(self) -> KTypeCode {
        KTypeCode(self.0 as i8)
    }
    pub const fn as_atom(self) -> KTypeCode {
        KTypeCode(-(self.0 as i8))
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
pub struct KTypeCode(libc::c_char);

impl From<KTypeCode> for i32 {
    fn from(kt: KTypeCode) -> i32 {
        kt.0 as i32
    }
}

impl fmt::Display for KTypeCode {
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

impl fmt::Debug for KTypeCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self, self.0)
    }
}

impl KTypeCode {
    pub fn atom_size(self) -> usize {
        match KTypeCode(self.0.abs()) {
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

pub const MIXED_LIST: KTypeCode = KTypeCode(0);
pub const BOOLEAN_ATOM: KTypeCode = KTypeCode(-1);
pub const GUID_ATOM: KTypeCode = KTypeCode(-2);
pub const BYTE_ATOM: KTypeCode = KTypeCode(-4);
pub const SHORT_ATOM: KTypeCode = KTypeCode(-5);
pub const INT_ATOM: KTypeCode = KTypeCode(-6);
pub const LONG_ATOM: KTypeCode = KTypeCode(-7);
pub const REAL_ATOM: KTypeCode = KTypeCode(-8);
pub const FLOAT_ATOM: KTypeCode = KTypeCode(-9);
pub const CHAR_ATOM: KTypeCode = KTypeCode(-10);
pub const SYMBOL_ATOM: KTypeCode = KTypeCode(-11);
pub const TIMESTAMP_ATOM: KTypeCode = KTypeCode(-12);
pub const MONTH_ATOM: KTypeCode = KTypeCode(-13);
pub const DATE_ATOM: KTypeCode = KTypeCode(-14);
pub const DATE_TIME_ATOM: KTypeCode = KTypeCode(-15);
pub const TIMESPAN_ATOM: KTypeCode = KTypeCode(-16);
pub const MINUTE_ATOM: KTypeCode = KTypeCode(-17);
pub const SECOND_ATOM: KTypeCode = KTypeCode(-18);
pub const TIME_ATOM: KTypeCode = KTypeCode(-19);
pub const BOOLEAN_LIST: KTypeCode = KTypeCode(1);
pub const GUID_LIST: KTypeCode = KTypeCode(2);
pub const BYTE_LIST: KTypeCode = KTypeCode(4);
pub const SHORT_LIST: KTypeCode = KTypeCode(5);
pub const INT_LIST: KTypeCode = KTypeCode(6);
pub const LONG_LIST: KTypeCode = KTypeCode(7);
pub const REAL_LIST: KTypeCode = KTypeCode(8);
pub const FLOAT_LIST: KTypeCode = KTypeCode(9);
pub const CHAR_LIST: KTypeCode = KTypeCode(10);
pub const SYMBOL_LIST: KTypeCode = KTypeCode(11);
pub const TIMESTAMP_LIST: KTypeCode = KTypeCode(12);
pub const MONTH_LIST: KTypeCode = KTypeCode(13);
pub const DATE_LIST: KTypeCode = KTypeCode(14);
pub const DATE_TIME_LIST: KTypeCode = KTypeCode(15);
pub const TIMESPAN_LIST: KTypeCode = KTypeCode(16);
pub const MINUTE_LIST: KTypeCode = KTypeCode(17);
pub const SECOND_LIST: KTypeCode = KTypeCode(18);
pub const TIME_LIST: KTypeCode = KTypeCode(19);
pub const TABLE: KTypeCode = KTypeCode(98);
pub const DICT: KTypeCode = KTypeCode(99);
pub const ERROR: KTypeCode = KTypeCode(-128);
