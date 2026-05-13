#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Prefixed identifier helpers.

use core::{fmt, marker::PhantomData};

pub mod prelude;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdPrefixError {
    EmptyPrefix,
    InvalidPrefixCharacter {
        character: char,
        index: usize,
    },
    EmptyValue,
    InvalidValueCharacter {
        character: char,
        index: usize,
    },
    MissingSeparator,
    MismatchedPrefix {
        expected: &'static str,
        actual: String,
    },
}

impl fmt::Display for IdPrefixError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPrefix => formatter.write_str("prefix cannot be empty"),
            Self::InvalidPrefixCharacter { character, index } => {
                write!(
                    formatter,
                    "invalid prefix character `{character}` at byte {index}"
                )
            },
            Self::EmptyValue => formatter.write_str("identifier value cannot be empty"),
            Self::InvalidValueCharacter { character, index } => {
                write!(
                    formatter,
                    "invalid value character `{character}` at byte {index}"
                )
            },
            Self::MissingSeparator => formatter.write_str("expected a `_` separator"),
            Self::MismatchedPrefix { expected, actual } => {
                write!(
                    formatter,
                    "expected prefix `{expected}` but found `{actual}`"
                )
            },
        }
    }
}

impl std::error::Error for IdPrefixError {}

#[must_use]
pub fn normalize_prefix(input: &str) -> String {
    input.trim().to_ascii_lowercase()
}

/// Validates that a prefix starts with a lowercase ASCII letter and only contains lowercase
/// ASCII letters or digits.
///
/// # Errors
///
/// Returns a [`IdPrefixError`] variant describing the first invalid prefix condition.
pub fn validate_prefix(input: &str) -> Result<(), IdPrefixError> {
    if input.is_empty() {
        return Err(IdPrefixError::EmptyPrefix);
    }

    for (index, character) in input.char_indices() {
        let is_valid = if index == 0 {
            character.is_ascii_lowercase()
        } else {
            character.is_ascii_lowercase() || character.is_ascii_digit()
        };

        if !is_valid {
            return Err(IdPrefixError::InvalidPrefixCharacter { character, index });
        }
    }

    Ok(())
}

#[must_use]
pub fn is_valid_prefix(input: &str) -> bool {
    validate_prefix(&normalize_prefix(input)).is_ok()
}

fn validate_value(input: &str) -> Result<(), IdPrefixError> {
    if input.is_empty() {
        return Err(IdPrefixError::EmptyValue);
    }

    for (index, character) in input.char_indices() {
        if !(character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')) {
            return Err(IdPrefixError::InvalidValueCharacter { character, index });
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdPrefix(String);

impl IdPrefix {
    /// Creates a validated normalized prefix.
    ///
    /// # Errors
    ///
    /// Returns a [`IdPrefixError`] variant when the prefix is empty or contains unsupported
    /// characters.
    pub fn new(input: &str) -> Result<Self, IdPrefixError> {
        let normalized = normalize_prefix(input);
        validate_prefix(&normalized)?;
        Ok(Self(normalized))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for IdPrefix {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Formats a prefix and value into the canonical `prefix_value` representation.
///
/// # Errors
///
/// Returns [`IdPrefixError::EmptyValue`] when the trimmed value is empty and
/// [`IdPrefixError::InvalidValueCharacter`] when it contains unsupported characters.
pub fn format_prefixed_id(prefix: &IdPrefix, value: &str) -> Result<String, IdPrefixError> {
    let normalized = value.trim();
    validate_value(normalized)?;
    Ok(format!("{}_{}", prefix.as_str(), normalized))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixedId {
    prefix: IdPrefix,
    value: String,
}

impl PrefixedId {
    /// Creates a prefixed identifier from a validated prefix and value.
    ///
    /// # Errors
    ///
    /// Returns [`IdPrefixError::EmptyValue`] when the trimmed value is empty and
    /// [`IdPrefixError::InvalidValueCharacter`] when it contains unsupported characters.
    pub fn new(prefix: IdPrefix, value: &str) -> Result<Self, IdPrefixError> {
        let normalized = value.trim().to_owned();
        validate_value(&normalized)?;
        Ok(Self {
            prefix,
            value: normalized,
        })
    }

    /// Parses the canonical `prefix_value` representation.
    ///
    /// # Errors
    ///
    /// Returns [`IdPrefixError::MissingSeparator`] when the input does not contain `_` and the
    /// corresponding prefix or value validation error for malformed parts.
    pub fn parse(input: &str) -> Result<Self, IdPrefixError> {
        let Some((prefix, value)) = input.split_once('_') else {
            return Err(IdPrefixError::MissingSeparator);
        };

        Self::new(IdPrefix::new(prefix)?, value)
    }

    #[must_use]
    pub const fn prefix(&self) -> &IdPrefix {
        &self.prefix
    }

    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[must_use]
    pub fn into_parts(self) -> (IdPrefix, String) {
        (self.prefix, self.value)
    }
}

impl fmt::Display for PrefixedId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&format!("{}_{}", self.prefix, self.value))
    }
}

pub trait PrefixedIdentifierKind {
    const PREFIX: &'static str;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedPrefixedId<K> {
    prefixed_id: PrefixedId,
    marker: PhantomData<fn() -> K>,
}

impl<K> TypedPrefixedId<K> {
    #[must_use]
    pub const fn as_prefixed_id(&self) -> &PrefixedId {
        &self.prefixed_id
    }

    #[must_use]
    pub fn value(&self) -> &str {
        self.prefixed_id.value()
    }
}

impl<K: PrefixedIdentifierKind> TypedPrefixedId<K> {
    /// Creates a typed prefixed identifier using the type-level prefix.
    ///
    /// # Errors
    ///
    /// Returns the underlying [`IdPrefixError`] when the type-level prefix or value is invalid.
    pub fn new(value: &str) -> Result<Self, IdPrefixError> {
        let prefixed_id = PrefixedId::new(IdPrefix::new(K::PREFIX)?, value)?;
        Ok(Self {
            prefixed_id,
            marker: PhantomData,
        })
    }

    /// Parses a typed prefixed identifier and verifies that the parsed prefix matches the type.
    ///
    /// # Errors
    ///
    /// Returns the underlying parsing error or [`IdPrefixError::MismatchedPrefix`] when the
    /// parsed prefix does not match the type-level prefix.
    pub fn parse(input: &str) -> Result<Self, IdPrefixError> {
        let prefixed_id = PrefixedId::parse(input)?;

        if prefixed_id.prefix().as_str() != K::PREFIX {
            return Err(IdPrefixError::MismatchedPrefix {
                expected: K::PREFIX,
                actual: prefixed_id.prefix().as_str().to_owned(),
            });
        }

        Ok(Self {
            prefixed_id,
            marker: PhantomData,
        })
    }

    #[must_use]
    pub const fn prefix(&self) -> &'static str {
        K::PREFIX
    }
}

impl<K> fmt::Display for TypedPrefixedId<K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.prefixed_id.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::{IdPrefixError, PrefixedId, PrefixedIdentifierKind, TypedPrefixedId};

    struct User;

    impl PrefixedIdentifierKind for User {
        const PREFIX: &'static str = "usr";
    }

    #[test]
    fn parses_prefixed_ids() -> Result<(), IdPrefixError> {
        let prefixed = PrefixedId::parse("usr_123")?;
        assert_eq!(prefixed.prefix().as_str(), "usr");
        assert_eq!(prefixed.value(), "123");
        Ok(())
    }

    #[test]
    fn typed_ids_enforce_the_expected_prefix() -> Result<(), IdPrefixError> {
        let typed = TypedPrefixedId::<User>::parse("usr_123")?;
        assert_eq!(typed.prefix(), "usr");
        Ok(())
    }
}
