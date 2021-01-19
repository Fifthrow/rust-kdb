use crate::k_type::KTypeCode;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Invalid k object cast from {from} to {to}")]
    InvalidKCast { from: KTypeCode, to: KTypeCode },
    #[error("Duration too long for K timespan type")]
    DurationTooLong,
    #[error("Symbol not a valid Rust string")]
    InvalidString,
}

impl From<Utf8Error> for ConversionError {
    fn from(_: Utf8Error) -> ConversionError {
        ConversionError::InvalidString
    }
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Bad credentials")]
    BadCredentials,
    #[error("Could not connect")]
    CouldNotConnect,
    #[error("Timeout")]
    Timeout,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Network error")]
    NetworkError,
    #[error("Query failed: {0}")]
    QError(String),
    #[error("Query failed: [unknown q error]")]
    UnknownQError,
}
