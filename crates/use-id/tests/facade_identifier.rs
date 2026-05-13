#[cfg(all(feature = "identifier", feature = "slug", feature = "id-prefix"))]
#[test]
fn facade_reexports_core_identifier_workflow() -> Result<(), Box<dyn std::error::Error>> {
    use use_id::prelude::{Identifier, PrefixedIdentifierKind, Slug, TypedPrefixedId};

    struct UserPrefix;

    impl PrefixedIdentifierKind for UserPrefix {
        const PREFIX: &'static str = "usr";
    }

    let identifier = Identifier::new("acct_42")?;
    let slug = Slug::new("RustUse Docs")?;
    let prefixed = TypedPrefixedId::<UserPrefix>::new(identifier.as_str())?;

    assert_eq!(slug.as_str(), "rustuse-docs");
    assert_eq!(prefixed.to_string(), "usr_acct_42");

    Ok(())
}

#[cfg(feature = "stable-id")]
#[test]
fn facade_reexports_stable_id_helpers() {
    let left = use_id::stable_name_id("rustuse");
    let right = use_id::stable_reference("docs", "intro");

    assert_ne!(left, right);
    assert_eq!(left, use_id::StableId::from_name("rustuse"));
}
