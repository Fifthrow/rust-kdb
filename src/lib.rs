//! rust-kdb is an idiomatic Rust wrappr around the kdb+ c API,
//! It supports manipulating K objects efficiently using zero cost
//! abstractions, conversion to and from rust native types,
//! and creation and editing of K lists/dictionaries using iterators.
//!
//! The design goals of the library:
//! 1. Minimal copying of data
//! 2. Provide an idiomatic rust API
//! 3. Safety

mod any;
mod atom;
mod connection;
mod date_time_types;
mod dictionary;
pub mod error;
mod guid;
mod k;
mod k_error;
mod k_type;
pub mod kapi;
mod kbox;
mod list;
mod symbol;
mod type_traits;

pub use any::Any;
pub use atom::Atom;
pub use connection::Connection;
pub use date_time_types::*;
pub use dictionary::Dictionary;
pub use error::{ConnectionError, ConversionError, Error};
pub use guid::Guid;
pub use k_error::KError;
pub use kbox::KBox;
pub use list::List;
pub use symbol::{symbol, Symbol};
