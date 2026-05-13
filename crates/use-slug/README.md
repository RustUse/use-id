# use-slug

Human-readable slug normalization and validation helpers for `RustUse`.

## Install

```toml
[dependencies]
use-slug = "0.0.1"
```

## Foundation

`use-slug` keeps slug behavior explicit and predictable: normalize free-form text into lowercase ASCII segments joined by hyphens, then validate the canonical form.

## Example

```rust
use use_slug::Slug;

let slug = Slug::new("RustUse Docs")?;

assert_eq!(slug.as_str(), "rustuse-docs");
assert_eq!(slug.segments().collect::<Vec<_>>(), vec!["rustuse", "docs"]);
# Ok::<(), use_slug::SlugError>(())
```

## When to use directly

Choose `use-slug` when human-readable identifiers are the only ID surface you need.

## Scope

- Slugs stay lowercase, ASCII, and hyphen-delimited.
- Normalization is deterministic and documented.
- Locale-aware transliteration and routing concerns are out of scope.

## Status

`use-slug` is a pre-1.0 crate with a deliberately narrow API.
