#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin UUID wrappers.

use core::fmt;
use uuid::Uuid;

pub mod prelude;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UuidIdError {
    InvalidFormat(String),
}

impl fmt::Display for UuidIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for UuidIdError {}

#[must_use]
pub fn is_uuid(input: &str) -> bool {
    Uuid::parse_str(input).is_ok()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UuidId(Uuid);

impl UuidId {
    /// Parses a UUID from a canonical string representation.
    ///
    /// # Errors
    ///
    /// Returns [`UuidIdError::InvalidFormat`] when the input is not a valid UUID string.
    pub fn parse(input: &str) -> Result<Self, UuidIdError> {
        Uuid::parse_str(input)
            .map(Self)
            .map_err(|error| UuidIdError::InvalidFormat(error.to_string()))
    }

    #[must_use]
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    #[must_use]
    pub fn to_hyphenated(&self) -> String {
        self.0.hyphenated().to_string()
    }

    #[must_use]
    pub fn to_simple(&self) -> String {
        self.0.simple().to_string()
    }
}

impl fmt::Display for UuidId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_hyphenated())
    }
}

#[cfg(test)]
mod tests {
    use super::{UuidId, is_uuid};

    #[test]
    fn uuid_validation_and_formatting_work() -> Result<(), Box<dyn std::error::Error>> {
        let value = UuidId::parse("123e4567-e89b-12d3-a456-426614174000")?;
        assert!(is_uuid(value.to_hyphenated().as_str()));
        assert_eq!(value.to_simple(), "123e4567e89b12d3a456426614174000");
        Ok(())
    }
}
