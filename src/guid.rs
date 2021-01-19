use std::fmt;
use uuid::Uuid;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Guid([u8; 16]);

impl From<[u8; 16]> for Guid {
    fn from(x: [u8; 16]) -> Guid {
        Guid(x)
    }
}

impl From<Guid> for [u8; 16] {
    fn from(x: Guid) -> [u8; 16] {
        x.0
    }
}

impl From<Uuid> for Guid {
    fn from(x: Uuid) -> Guid {
        Guid(*x.as_bytes())
    }
}

impl From<Guid> for Uuid {
    fn from(x: Guid) -> Uuid {
        Uuid::from_bytes(x.0)
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&Uuid::from_bytes(self.0), f)
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
