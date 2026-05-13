use use_uuid::{UuidId, is_uuid};

#[test]
fn uuid_wrapper_preserves_canonical_format() -> Result<(), Box<dyn std::error::Error>> {
    let value = UuidId::parse("123e4567-e89b-12d3-a456-426614174000")?;

    assert!(is_uuid(value.to_hyphenated().as_str()));
    assert_eq!(value.to_simple(), "123e4567e89b12d3a456426614174000");

    Ok(())
}
