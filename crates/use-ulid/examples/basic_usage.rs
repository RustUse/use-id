use use_ulid::UlidId;

fn main() -> Result<(), use_ulid::UlidIdError> {
    let value = UlidId::parse("01ARZ3NDEKTSV4RRFFQ69G5FAV")?;

    assert_eq!(value.to_canonical(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");

    Ok(())
}
