# use-id

Feature-gated facade for the focused `RustUse` identifier crates.

## Install

```toml
[dependencies]
use-id = { version = "0.0.1", default-features = false, features = ["identifier", "slug"] }
```

## Foundation

`use-id` re-exports the focused crates in this workspace behind opt-in features. It keeps the facade thin: direct root re-exports for common types and functions, plus nested modules that mirror the concrete crate boundaries.

## Example

```rust
# #[cfg(all(feature = "identifier", feature = "slug"))]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use use_id::{Identifier, Slug};

let identifier = Identifier::new("acct_42")?;
let slug = Slug::new("RustUse Docs")?;

assert_eq!(identifier.as_str(), "acct_42");
assert_eq!(slug.as_str(), "rustuse-docs");
# Ok(())
# }
# #[cfg(not(all(feature = "identifier", feature = "slug")))]
# fn main() {}
```

## When to use directly

Choose `use-id` when you want one dependency and one import surface. Prefer the focused crates directly when you only need one domain such as slugs, prefixed IDs, UUIDs, or ULIDs.

## Scope

- The facade stays close to the focused crate APIs.
- Feature flags map directly to the focused crates in this workspace.
- Broad hashing, registry lookup, and persistence concerns are out of scope.

## Status

`use-id` is a pre-1.0 crate with a deliberately small facade over focused identifier primitives.
