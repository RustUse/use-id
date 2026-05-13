#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Deterministic stable ID helpers.

use core::fmt;
use std::path::Path;

pub mod prelude;

const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[must_use]
pub fn stable_hash64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(FNV_OFFSET_BASIS, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(FNV_PRIME)
    })
}

#[must_use]
pub fn stable_hex(bytes: &[u8]) -> String {
    format!("{:016x}", stable_hash64(bytes))
}

#[must_use]
pub fn stable_name_id(name: &str) -> StableId {
    StableId::from_bytes(name.as_bytes())
}

#[must_use]
pub fn stable_content_id(content: &[u8]) -> StableId {
    StableId::from_bytes(content)
}

#[must_use]
pub fn stable_reference(namespace: &str, name: &str) -> StableId {
    StableId::from_bytes(format!("{namespace}:{name}").as_bytes())
}

#[must_use]
pub fn stable_path_id(path: impl AsRef<Path>) -> StableId {
    StableId::from_path(path)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StableId(String);

impl StableId {
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(stable_hex(bytes))
    }

    #[must_use]
    pub fn from_name(name: &str) -> Self {
        stable_name_id(name)
    }

    #[must_use]
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let normalized = path.as_ref().to_string_lossy().replace('\\', "/");
        Self::from_bytes(normalized.as_bytes())
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

impl fmt::Display for StableId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{StableId, stable_name_id, stable_path_id, stable_reference};

    #[test]
    fn name_ids_are_deterministic() {
        assert_eq!(stable_name_id("rustuse"), stable_name_id("rustuse"));
    }

    #[test]
    fn path_ids_normalize_separators() {
        assert_eq!(
            stable_path_id("docs\\intro.md"),
            stable_path_id("docs/intro.md")
        );
    }

    #[test]
    fn namespace_changes_the_output() {
        assert_ne!(
            stable_reference("docs", "intro"),
            StableId::from_name("intro")
        );
    }
}
