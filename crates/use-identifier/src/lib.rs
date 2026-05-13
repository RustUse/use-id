#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! String-backed identifier primitives.

use core::{fmt, marker::PhantomData};

pub mod prelude;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifierError {
    Empty,
    InvalidCharacter { character: char, index: usize },
}

impl fmt::Display for IdentifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("identifier cannot be empty"),
            Self::InvalidCharacter { character, index } => {
                write!(
                    formatter,
                    "invalid identifier character `{character}` at byte {index}"
                )
            },
        }
    }
}

impl std::error::Error for IdentifierError {}

#[must_use]
pub fn normalize_identifier(input: &str) -> String {
    input.trim().to_owned()
}

/// Validates that an identifier is non-empty and contains only supported ASCII characters.
///
/// # Errors
///
/// Returns [`IdentifierError::Empty`] when the input is empty and
/// [`IdentifierError::InvalidCharacter`] when it contains unsupported characters.
pub fn validate_identifier(input: &str) -> Result<(), IdentifierError> {
    if input.is_empty() {
        return Err(IdentifierError::Empty);
    }

    for (index, character) in input.char_indices() {
        if !(character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')) {
            return Err(IdentifierError::InvalidCharacter { character, index });
        }
    }

    Ok(())
}

#[must_use]
pub fn is_valid_identifier(input: &str) -> bool {
    validate_identifier(input).is_ok()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Identifier(String);

impl Identifier {
    /// Creates a validated identifier from user-provided text.
    ///
    /// # Errors
    ///
    /// Returns [`IdentifierError::Empty`] when the trimmed input is empty and
    /// [`IdentifierError::InvalidCharacter`] when it contains unsupported characters.
    pub fn new(input: &str) -> Result<Self, IdentifierError> {
        let normalized = normalize_identifier(input);
        validate_identifier(&normalized)?;
        Ok(Self(normalized))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

pub trait IdentifierKind {
    const NAME: &'static str;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedIdentifier<K> {
    identifier: Identifier,
    marker: PhantomData<fn() -> K>,
}

impl<K> TypedIdentifier<K> {
    #[must_use]
    pub const fn from_identifier(identifier: Identifier) -> Self {
        Self {
            identifier,
            marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn as_identifier(&self) -> &Identifier {
        &self.identifier
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.identifier.as_str()
    }

    #[must_use]
    pub fn into_inner(self) -> Identifier {
        self.identifier
    }
}

impl<K: IdentifierKind> TypedIdentifier<K> {
    /// Creates a typed identifier from user-provided text.
    ///
    /// # Errors
    ///
    /// Returns [`IdentifierError::Empty`] when the trimmed input is empty and
    /// [`IdentifierError::InvalidCharacter`] when it contains unsupported characters.
    pub fn new(input: &str) -> Result<Self, IdentifierError> {
        Identifier::new(input).map(Self::from_identifier)
    }

    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        K::NAME
    }
}

impl<K> fmt::Display for TypedIdentifier<K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.identifier.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::{Identifier, IdentifierError, IdentifierKind, TypedIdentifier};

    struct User;

    impl IdentifierKind for User {
        const NAME: &'static str = "user";
    }

    #[test]
    fn trims_and_preserves_valid_identifiers() -> Result<(), IdentifierError> {
        let identifier = Identifier::new("  acct_42  ")?;
        assert_eq!(identifier.as_str(), "acct_42");
        Ok(())
    }

    #[test]
    fn rejects_invalid_characters() {
        assert_eq!(
            Identifier::new("bad value"),
            Err(IdentifierError::InvalidCharacter {
                character: ' ',
                index: 3,
            })
        );
    }

    #[test]
    fn typed_identifier_carries_kind_name() -> Result<(), IdentifierError> {
        let typed = TypedIdentifier::<User>::new("usr_7")?;
        assert_eq!(typed.kind_name(), "user");
        Ok(())
    }
}
