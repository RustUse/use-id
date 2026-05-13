use use_ulid::{UlidId, is_ulid};

#[test]
fn ulid_wrapper_preserves_canonical_format() -> Result<(), Box<dyn std::error::Error>> {
    let value = UlidId::parse("01ARZ3NDEKTSV4RRFFQ69G5FAV")?;

    assert!(is_ulid(value.to_canonical().as_str()));
    assert_eq!(value.to_canonical(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");

    Ok(())
}
