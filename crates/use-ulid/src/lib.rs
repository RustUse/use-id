#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin ULID wrappers.

use core::fmt;
use ulid::Ulid;

pub mod prelude;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UlidIdError {
    InvalidFormat(String),
}

impl fmt::Display for UlidIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for UlidIdError {}

#[must_use]
pub fn is_ulid(input: &str) -> bool {
    input.parse::<Ulid>().is_ok()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UlidId(Ulid);

impl UlidId {
    /// Parses a ULID from its canonical string representation.
    ///
    /// # Errors
    ///
    /// Returns [`UlidIdError::InvalidFormat`] when the input is not a valid ULID string.
    pub fn parse(input: &str) -> Result<Self, UlidIdError> {
        input
            .parse::<Ulid>()
            .map(Self)
            .map_err(|error| UlidIdError::InvalidFormat(error.to_string()))
    }

    #[must_use]
    pub const fn from_ulid(ulid: Ulid) -> Self {
        Self(ulid)
    }

    #[must_use]
    pub const fn as_ulid(&self) -> &Ulid {
        &self.0
    }

    #[must_use]
    pub fn to_canonical(&self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for UlidId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_canonical())
    }
}

#[cfg(test)]
mod tests {
    use super::{UlidId, is_ulid};

    #[test]
    fn ulid_validation_and_formatting_work() -> Result<(), Box<dyn std::error::Error>> {
        let value = UlidId::parse("01ARZ3NDEKTSV4RRFFQ69G5FAV")?;
        assert!(is_ulid(value.to_canonical().as_str()));
        assert_eq!(value.to_canonical(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
        Ok(())
    }
}
