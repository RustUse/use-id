use use_stable_id::{stable_name_id, stable_reference};

fn main() {
    let left = stable_name_id("rustuse");
    let right = stable_reference("docs", "intro");

    assert_ne!(left, right);
    assert_eq!(left, stable_name_id("rustuse"));
}
