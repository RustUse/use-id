#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Facade for `RustUse` identifier primitives.

#[cfg(feature = "id-prefix")]
pub use use_id_prefix as id_prefix;

#[cfg(feature = "id-prefix")]
pub use use_id_prefix::{
    IdPrefix, IdPrefixError, PrefixedId, PrefixedIdentifierKind, TypedPrefixedId,
    format_prefixed_id, is_valid_prefix, normalize_prefix, validate_prefix,
};

#[cfg(feature = "identifier")]
pub use use_identifier as identifier;

#[cfg(feature = "identifier")]
pub use use_identifier::{
    Identifier, IdentifierError, IdentifierKind, TypedIdentifier, is_valid_identifier,
    normalize_identifier, validate_identifier,
};

#[cfg(feature = "slug")]
pub use use_slug as slug;

#[cfg(feature = "slug")]
pub use use_slug::{Slug, SlugError, is_valid_slug, normalize_slug, validate_slug};

#[cfg(feature = "stable-id")]
pub use use_stable_id as stable_id;

#[cfg(feature = "stable-id")]
pub use use_stable_id::{
    StableId, stable_content_id, stable_hash64, stable_hex, stable_name_id, stable_path_id,
    stable_reference,
};

#[cfg(feature = "ulid")]
pub use use_ulid as ulid;

#[cfg(feature = "ulid")]
pub use use_ulid::{UlidId, UlidIdError, is_ulid};

#[cfg(feature = "uuid")]
pub use use_uuid as uuid;

#[cfg(feature = "uuid")]
pub use use_uuid::{UuidId, UuidIdError, is_uuid};

pub mod prelude;
