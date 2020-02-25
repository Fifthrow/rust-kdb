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
mod atoms;
mod connection;
mod dict;
mod error;
mod lists;
pub mod raw;
mod table;

pub use any::KAny;
pub use atoms::*;
pub use connection::Connection;
pub use dict::KDict;
pub use error::{ConnectionError, ConversionError, Error};
pub use lists::*;
pub use raw::types::*;
pub use table::KTable;
