# RustUse/use-id

Composable identifier, slug, sequence, and stable-reference primitives for Rust.

## Workspace crates

| Crate            | Path                     | Purpose                                                       |
| ---------------- | ------------------------ | ------------------------------------------------------------- |
| `use-id`         | `crates/use-id/`         | Feature-gated facade over the focused identifier crates       |
| `use-identifier` | `crates/use-identifier/` | String-backed and typed identifier primitives                 |
| `use-slug`       | `crates/use-slug/`       | Human-readable slug normalization and validation              |
| `use-stable-id`  | `crates/use-stable-id/`  | Deterministic stable ID helpers for names, content, and paths |
| `use-id-prefix`  | `crates/use-id-prefix/`  | Prefixed identifier parsing and formatting                    |
| `use-uuid`       | `crates/use-uuid/`       | Thin UUID parsing and formatting helpers                      |
| `use-ulid`       | `crates/use-ulid/`       | Thin ULID parsing and formatting helpers                      |

## Installation

Use the workspace directly or depend on a Git revision until the first crates.io release is published.

## Basic usage

```rust
use use_identifier::Identifier;
use use_slug::Slug;

let identifier = Identifier::new("acct_42")?;
let slug = Slug::new("RustUse Docs")?;

assert_eq!(identifier.as_str(), "acct_42");
assert_eq!(slug.as_str(), "rustuse-docs");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## License

Licensed under either of the following, at your option:

- MIT license: https://github.com/RustUse/.github/blob/main/LICENSE-MIT
- Apache License, Version 2.0: https://github.com/RustUse/.github/blob/main/LICENSE-APACHE

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the local validation flow and repository policy.
