//! rust-kdb is an idiomatic Rust wrapper around the KDB+ C API.
//! It supports manipulating K objects efficiently using zero cost
//! abstractions, conversion to and from rust native types,
//! and creation and editing of K lists/dictionaries using iterators.
//!
//! The design goals of the library:
//! 1. Performance
//! 2. Provide an idiomatic rust API
//! 3. Safety
//!
//! # Why you might like it
//!
//! 1. It's really fast. The abstractions around the KDB types have zero overhead
//! so while it feels like you are using rust types with all the trimmings, it is as fast as calling
//! the raw C API directly.
//! 2. It's safe! Using the C API is full of pitfalls that cause undefined behaviour.
//! The abstractions in rust-kdb manage these for you, so that you don't need to worry about
//! freeing your KDB objects or weird behaviour.
//! 3. It's just like using other rust data types. We've worked hard to make Lists work like
//! Rust vectors, and to add similar convenience functions to dictionaries.
//!
//! # Why you might not like it
//!
//! 1. Type conversions are common. It's not unusual to see rust-kdb code littered with `.try_into().unwrap()`
//!    Especially when working with mixed lists. This is a consequence of retrofitting the kdb type system into
//!    Rust. These type conversions are cheap (from/into are free, try_from/try_into are a single check of the KDB type code)
//!    But they do dirty the code somewhat.
//! 2. It's incomplete. It currently lacks support for tables and not all functions have been included. The plan is to include these features
//!    in time.
//!
//! # Creating KDB types
//!
//! KDB deals with atoms and lists.
//! rust-kdb maps this to the Atom And List generic types.
//! We create a kdb type using a `KBox`. Just like a Rust `Box` wraps
//! Rust heap managed pointers, KBox wraps KDB managed pointers.
//!
//! To create a new 32-bit integer atom:
//! ```
//! use kdb::KBox;
//! let a = KBox::new_atom(42);
//! assert_eq!(a.value(), 42);
//! ```
//! To create a list of bytes:
//! ```
//! # use kdb::*;
//! let mut l = KBox::<List<u8>>::new_list();
//! for i in 1..=10 {
//!     l.push(i);   
//! }
//! assert_eq!(l.len(), 10);
//! ```
//!
//! # Working with unknown types
//!
//! KDB is dynamically typed, in that each type is an instance of a k object
//! This is similar to variant types in C++. In rust-kdb, we manage this with
//! the `Any` type.
//!
//! The `Any` type can be used in place of any valid KDB value (atom or list).
//! You can't do much with it, except try to convert it to a different type, using
//! the `TryFrom`/`TryInto` traits.
//!
//! ```
//! use kdb::*;
//! use std::convert::{TryFrom, TryInto};
//!
//! let a = KBox::new_atom(42);
//! let b: KBox<Any> = a.into();
//! let c: KBox<Atom<i32>> = b.try_into().unwrap();
//! ```
//!
//! # Writing embedded KDB plug-ins
//!
//! Writing embedded plugins is straightforward using rust-kdb.
//! Youll need to use the "embedded" feature for your plugin to use the correct
//! library bindings.
//!
//! Below is an example of a simple KDB plugin
//! ```
//! use kdb::*;
//! use std::convert::TryFrom;
//! use std::f64::consts::PI;
//!
//! /// Calculates the circumference of a circle. Returns a null value if the radius is not a real number.
//! #[no_mangle]
//! pub extern "C" fn circumference(radius: &Any) -> Option<KBox<Atom<f64>>> {
//!     if let Ok(r) = <&Atom<f64>>::try_from(radius) {
//!         return Some(KBox::new_atom(r.value() * r.value() * PI));
//!     }
//!     None
//! }
//! ```
//!
//! Key points:
//! 1. Note that KDB parameters in extern "C" functions are references to KDB types, rather than a KBox. In KDB, the caller owns the parameters.
//!    Using a KBox here will likely cause a segfault.
//! 2. The return type is always either a KBox<T> or Option<KBox<T>>. This is equivalent to returning a K pointer. Always return an owned type.
//! 3. You can use typed atoms for parameters, not just Any. Bear in mind that this is unsafe as it is possible for q code to call the function
//! with a type other than that one. Any is always safest.

#![warn(missing_docs)] // warn if there are missing docs

mod any;
mod atom;
mod connection;
mod date_time_types;
mod dictionary;
mod error;
mod k;
mod k_error;
mod k_type;
pub mod kapi;
mod kbox;
mod list;
mod serialization;
mod symbol;
mod type_traits;

pub use any::Any;
pub use atom::Atom;
pub use connection::Connection;
pub use date_time_types::*;
pub use dictionary::Dictionary;
pub use error::{ConnectionError, ConversionError, Error};
pub use k_error::KError;
pub use kbox::KBox;
pub use list::List;
pub use serialization::*;
pub use symbol::{symbol, Symbol};

pub use array_iterator;
#[cfg(feature = "uuid")]
pub use uuid;
