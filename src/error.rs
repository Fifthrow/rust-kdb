use crate::k_type::KTypeCode;
use std::str::Utf8Error;
use thiserror::Error;

/// Error type for converting from/to kdb types.
#[derive(Debug, Error)]
pub enum ConversionError {
    /// Attempted to cast between objects with two different KDB types
    #[error("Invalid k object cast from {from} to {to}")]
    InvalidKCast {
        /// The type of the object being cast
        from: KTypeCode,
        /// The type wanted
        to: KTypeCode,
    },
    /// Duration too long - the timespan would overflow.
    #[error("Duration too long for K timespan type")]
    DurationTooLong,
    /// Symbol is not a valid rust string (not UTF-8)
    #[error("Symbol not a valid Rust string")]
    InvalidString,
}

impl From<Utf8Error> for ConversionError {
    fn from(_: Utf8Error) -> ConversionError {
        ConversionError::InvalidString
    }
}

/// The error type for connecting to KDB.
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// Credentials were incorrect.
    #[error("Bad credentials")]
    BadCredentials,
    /// Unable to connect.
    #[error("Could not connect")]
    CouldNotConnect,
    /// Timed out.
    #[error("Timeout")]
    Timeout,
}

/// The error type for Q query execution.
#[derive(Debug, Error)]
pub enum Error {
    /// Network error accessing remote KDB instance.
    #[error("Network error")]
    NetworkError,
    /// Custom error from query.
    #[error("Query failed: {0}")]
    QError(String),
    /// Unknown Error.
    #[error("Query failed: [unknown q error]")]
    UnknownQError,
}
