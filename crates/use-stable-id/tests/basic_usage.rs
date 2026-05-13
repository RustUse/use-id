use use_stable_id::{StableId, stable_path_id, stable_reference};

#[test]
fn stable_ids_cover_namespace_and_path_inputs() {
    assert_eq!(
        stable_path_id("docs\\intro.md"),
        stable_path_id("docs/intro.md")
    );
    assert_ne!(
        stable_reference("docs", "intro"),
        StableId::from_name("intro")
    );
}
