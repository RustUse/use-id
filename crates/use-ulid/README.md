# use-ulid

Thin ULID parsing and formatting helpers for `RustUse`.

## Install

```toml
[dependencies]
use-ulid = "0.0.1"
```

## Foundation

`use-ulid` wraps the established `ulid` crate with a small `UlidId` type plus parsing, formatting, and validation helpers that match `use-uuid`.

## Example

```rust
use use_ulid::UlidId;

let value = UlidId::parse("01ARZ3NDEKTSV4RRFFQ69G5FAV")?;

assert_eq!(value.to_canonical(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
# Ok::<(), use_ulid::UlidIdError>(())
```

## When to use directly

Choose `use-ulid` when ULID parsing or formatting is the only identifier concern you need.

## Scope

- ULID generation policy is intentionally left to the upstream crate.
- This crate stays focused on parsing, formatting, and validation.
- Broader registries and persistence policies are out of scope.

## Status

`use-ulid` is a pre-1.0 crate with a deliberately thin wrapper API.
