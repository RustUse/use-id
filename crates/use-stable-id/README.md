# use-stable-id

Deterministic stable ID helpers for `RustUse`.

## Install

```toml
[dependencies]
use-stable-id = "0.0.1"
```

## Foundation

`use-stable-id` provides a documented, deterministic 64-bit FNV-1a based ID surface for simple name, content, path, and namespace-derived identifiers.

## Example

```rust
use use_stable_id::{stable_name_id, stable_reference};

let left = stable_name_id("rustuse");
let right = stable_reference("docs", "intro");

assert_ne!(left, right);
assert_eq!(left, stable_name_id("rustuse"));
```

## When to use directly

Choose `use-stable-id` when you need a small deterministic ID surface without bringing in UUID- or ULID-specific formatting.

## Scope

- The algorithm is explicit and stable.
- Helpers stay string-first and deterministic.
- Broader hashing frameworks and cryptographic claims are out of scope.

## Status

`use-stable-id` is a pre-1.0 crate with a narrow deterministic API.
