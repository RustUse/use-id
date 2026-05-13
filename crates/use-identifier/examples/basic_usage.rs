use use_identifier::{Identifier, IdentifierKind, TypedIdentifier};

struct Account;

impl IdentifierKind for Account {
    const NAME: &'static str = "account";
}

fn main() -> Result<(), use_identifier::IdentifierError> {
    let plain = Identifier::new("acct_42")?;
    let typed = TypedIdentifier::<Account>::new("acct_42")?;

    assert_eq!(plain.as_str(), typed.as_str());
    assert_eq!(typed.kind_name(), "account");

    Ok(())
}
