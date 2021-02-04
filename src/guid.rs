use std::fmt;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
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

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let b = &self.0;
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
        )
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(feature = "uuid")]
pub mod uuid {
    use super::*;
    use crate::atom::Atom;
    use crate::k::GuidWithLen;
    use crate::kbox::KBox;
    use ::uuid::Uuid;

    impl From<GuidWithLen> for Uuid {
        fn from(x: GuidWithLen) -> Self {
            x.u.into()
        }
    }

    // Support for Uuid lib
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

    impl From<Uuid> for KBox<Atom<Guid>> {
        fn from(uuid: Uuid) -> Self {
            Guid::from(uuid).into()
        }
    }

    impl From<KBox<Atom<Guid>>> for Uuid {
        fn from(atom: KBox<Atom<Guid>>) -> Self {
            atom.value().into()
        }
    }

    //impl TryFrom<KBox<Any>> for Uuid {}
}
