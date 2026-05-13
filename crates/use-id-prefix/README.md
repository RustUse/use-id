# use-id-prefix

Prefixed identifier parsing and formatting helpers for `RustUse`.

## Install

```toml
[dependencies]
use-id-prefix = "0.0.1"
```

## Foundation

`use-id-prefix` models prefixes such as `usr`, `org`, `repo`, and `evt`, then composes them with validated identifier bodies using a fixed `prefix_value` shape.

## Example

```rust
use use_id_prefix::{PrefixedId, PrefixedIdentifierKind, TypedPrefixedId};

struct User;

impl PrefixedIdentifierKind for User {
    const PREFIX: &'static str = "usr";
}

let parsed = PrefixedId::parse("usr_123")?;
let typed = TypedPrefixedId::<User>::new("123")?;

assert_eq!(parsed.to_string(), "usr_123");
assert_eq!(typed.to_string(), "usr_123");
# Ok::<(), use_id_prefix::IdPrefixError>(())
```

## When to use directly

Choose `use-id-prefix` when prefix parsing and formatting are the only identifier concern you need.

## Scope

- Prefixes stay lowercase ASCII with optional digits after the first character.
- Values stay lightweight and string-backed.
- Registry lookup and storage policies are out of scope.

## Status

`use-id-prefix` is a pre-1.0 crate with a narrow prefixed-ID API.
