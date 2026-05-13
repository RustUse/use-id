use use_id_prefix::{PrefixedId, PrefixedIdentifierKind, TypedPrefixedId};

struct User;

impl PrefixedIdentifierKind for User {
    const PREFIX: &'static str = "usr";
}

#[test]
fn prefixed_identifier_parsing_is_explicit() -> Result<(), use_id_prefix::IdPrefixError> {
    let parsed = PrefixedId::parse("usr_123")?;
    let typed = TypedPrefixedId::<User>::parse("usr_123")?;

    assert_eq!(parsed.to_string(), "usr_123");
    assert_eq!(typed.to_string(), "usr_123");

    Ok(())
}
