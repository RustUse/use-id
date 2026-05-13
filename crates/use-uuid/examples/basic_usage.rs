use use_uuid::UuidId;

fn main() -> Result<(), use_uuid::UuidIdError> {
    let value = UuidId::parse("123e4567-e89b-12d3-a456-426614174000")?;

    assert_eq!(value.to_simple(), "123e4567e89b12d3a456426614174000");

    Ok(())
}
