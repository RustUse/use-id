use use_id::{Identifier, Slug};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let identifier = Identifier::new("acct_42")?;
    let slug = Slug::new("RustUse Docs")?;

    assert_eq!(identifier.as_str(), "acct_42");
    assert_eq!(slug.as_str(), "rustuse-docs");

    Ok(())
}
