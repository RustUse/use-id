#[cfg(all(
    feature = "identifier",
    feature = "slug",
    feature = "stable-id",
    feature = "id-prefix",
    feature = "uuid",
    feature = "ulid"
))]
#[test]
fn facade_exposes_all_namespace_features() {
    use use_id::{
        id_prefix as _, identifier as _, slug as _, stable_id as _, ulid as _, uuid as _,
    };
}

#[cfg(all(
    feature = "identifier",
    not(feature = "slug"),
    not(feature = "stable-id"),
    not(feature = "id-prefix"),
    not(feature = "uuid"),
    not(feature = "ulid")
))]
#[test]
fn facade_supports_identifier_only() -> Result<(), use_id::IdentifierError> {
    let identifier = use_id::Identifier::new("acct_42")?;
    assert_eq!(identifier.as_str(), "acct_42");
    Ok(())
}
