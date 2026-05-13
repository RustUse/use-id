use use_slug::Slug;

#[test]
fn slug_normalization_is_predictable() -> Result<(), use_slug::SlugError> {
    let slug = Slug::new("RustUse Docs")?;

    assert_eq!(slug.as_str(), "rustuse-docs");
    assert_eq!(slug.segments().collect::<Vec<_>>(), vec!["rustuse", "docs"]);

    Ok(())
}
