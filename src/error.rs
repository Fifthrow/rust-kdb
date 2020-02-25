use crate::raw::types::KType;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Invalid k object cast from {from} to {to}")]
    InvalidKCast { from: KType, to: KType },
    #[error("Duration too long for K timespan type")]
    DurationTooLong,
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
