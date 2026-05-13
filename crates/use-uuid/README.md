# use-uuid

Thin UUID parsing and formatting helpers for `RustUse`.

## Install

```toml
[dependencies]
use-uuid = "0.0.1"
```

## Foundation

`use-uuid` wraps the established `uuid` crate with a minimal `UuidId` type plus parsing, formatting, and validation helpers that match the rest of this workspace.

## Example

```rust
use use_uuid::UuidId;

let value = UuidId::parse("123e4567-e89b-12d3-a456-426614174000")?;

assert_eq!(value.to_hyphenated(), "123e4567-e89b-12d3-a456-426614174000");
# Ok::<(), use_uuid::UuidIdError>(())
```

## When to use directly

Choose `use-uuid` when UUID parsing or formatting is the only identifier concern you need.

## Scope

- UUID generation policy is intentionally left to the upstream crate.
- This crate stays focused on parsing, formatting, and validation.
- Broader UUID registries and persistence policies are out of scope.

## Status

`use-uuid` is a pre-1.0 crate with a deliberately thin wrapper API.
