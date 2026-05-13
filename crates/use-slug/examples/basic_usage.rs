use use_slug::Slug;

fn main() -> Result<(), use_slug::SlugError> {
    let slug = Slug::new("RustUse Docs")?;

    assert_eq!(slug.as_str(), "rustuse-docs");

    Ok(())
}
