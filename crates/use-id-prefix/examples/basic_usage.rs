use use_id_prefix::{PrefixedIdentifierKind, TypedPrefixedId};

struct User;

impl PrefixedIdentifierKind for User {
    const PREFIX: &'static str = "usr";
}

fn main() -> Result<(), use_id_prefix::IdPrefixError> {
    let typed = TypedPrefixedId::<User>::new("123")?;

    assert_eq!(typed.to_string(), "usr_123");

    Ok(())
}
