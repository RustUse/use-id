#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Slug normalization and validation helpers.

use core::fmt;

pub mod prelude;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlugError {
    Empty,
    InvalidCharacter { character: char, index: usize },
    LeadingSeparator,
    TrailingSeparator,
    RepeatedSeparator { index: usize },
}

impl fmt::Display for SlugError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("slug cannot be empty"),
            Self::InvalidCharacter { character, index } => {
                write!(
                    formatter,
                    "invalid slug character `{character}` at byte {index}"
                )
            },
            Self::LeadingSeparator => formatter.write_str("slug cannot start with `-`"),
            Self::TrailingSeparator => formatter.write_str("slug cannot end with `-`"),
            Self::RepeatedSeparator { index } => {
                write!(formatter, "slug cannot repeat `-` at byte {index}")
            },
        }
    }
}

impl std::error::Error for SlugError {}

#[must_use]
pub fn normalize_slug(input: &str) -> String {
    let mut normalized = String::new();
    let mut previous_was_separator = false;

    for character in input.trim().chars() {
        let lowered = character.to_ascii_lowercase();

        if lowered.is_ascii_alphanumeric() {
            normalized.push(lowered);
            previous_was_separator = false;
        } else if !normalized.is_empty() && !previous_was_separator {
            normalized.push('-');
            previous_was_separator = true;
        }
    }

    while normalized.ends_with('-') {
        normalized.pop();
    }

    normalized
}

/// Validates that a slug is non-empty, lowercase ASCII, and hyphen-delimited.
///
/// # Errors
///
/// Returns a [`SlugError`] variant describing the first invalid slug condition.
pub fn validate_slug(input: &str) -> Result<(), SlugError> {
    if input.is_empty() {
        return Err(SlugError::Empty);
    }

    if input.starts_with('-') {
        return Err(SlugError::LeadingSeparator);
    }

    if input.ends_with('-') {
        return Err(SlugError::TrailingSeparator);
    }

    let mut previous_was_separator = false;

    for (index, character) in input.char_indices() {
        if character == '-' {
            if previous_was_separator {
                return Err(SlugError::RepeatedSeparator { index });
            }

            previous_was_separator = true;
            continue;
        }

        if !character.is_ascii_lowercase() && !character.is_ascii_digit() {
            return Err(SlugError::InvalidCharacter { character, index });
        }

        previous_was_separator = false;
    }

    Ok(())
}

#[must_use]
pub fn is_valid_slug(input: &str) -> bool {
    validate_slug(input).is_ok()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Slug(String);

impl Slug {
    /// Creates a normalized slug from user-provided text.
    ///
    /// # Errors
    ///
    /// Returns [`SlugError::Empty`] when normalization produces an empty value or another
    /// [`SlugError`] variant when the canonical representation is invalid.
    pub fn new(input: &str) -> Result<Self, SlugError> {
        let normalized = normalize_slug(input);
        validate_slug(&normalized)?;
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

    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.0.split('-')
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{Slug, SlugError, validate_slug};

    #[test]
    fn normalizes_spaces_and_case() -> Result<(), SlugError> {
        let slug = Slug::new("RustUse Docs")?;
        assert_eq!(slug.as_str(), "rustuse-docs");
        Ok(())
    }

    #[test]
    fn constructor_collapses_repeated_separators() -> Result<(), SlugError> {
        let slug = Slug::new("rustuse--docs")?;
        assert_eq!(slug.as_str(), "rustuse-docs");
        Ok(())
    }

    #[test]
    fn validator_rejects_repeated_separators() {
        assert_eq!(
            validate_slug("rustuse--docs"),
            Err(SlugError::RepeatedSeparator { index: 8 })
        );
    }
}
