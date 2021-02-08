//! rust-kdb is an idiomatic Rust wrappr around the kdb+ c API,
//! It supports manipulating K objects efficiently using zero cost
//! abstractions, conversion to and from rust native types,
//! and creation and editing of K lists/dictionaries using iterators.
//!
//! The design goals of the library:
//! 1. Minimal copying of data
//! 2. Provide an idiomatic rust API
//! 3. Safety
#![cfg_attr(experimental, feature(try_trait))]

mod any;
mod atoms;
mod connection;
mod dict;
mod error;
mod lists;
mod mixed_list;
pub mod raw;
mod table;
mod unowned;

pub use any::KAny;
pub use atoms::*;
pub use connection::Connection;
pub use dict::KDict;
pub use error::{ConnectionError, ConversionError, Error};
pub use lists::*;
pub use mixed_list::{mixed_list_from_raw, KMixedList};
pub use raw::types::*;
pub use table::KTable;
pub use unowned::Unowned;
pub use unowned::{b9_serialize, d9_deserialize};

pub mod c_api {
    pub use crate::raw::kapi::*;
}

#[macro_export(local_inner_macros)]
macro_rules! count_items {
    ($name:expr) => { 1 };
    ($first:expr, $($rest:expr),*) => {
        1 + count_items!($($rest),*)
    }
}

#[macro_export]
macro_rules! mixed_list {
    ($($x:expr),+ $(,)?) => {
        {
            unsafe { $crate::mixed_list_from_raw($crate::raw::kapi::knk($crate::count_items!($($x),*), $($crate::KAny::from($x).into_ptr()),*)) }
        }
    };
}

pub fn symbol(sym: &'static str) -> Symbol {
    std::convert::TryFrom::try_from(sym).unwrap()
}
