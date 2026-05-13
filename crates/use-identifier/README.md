# use-identifier

String-backed and typed identifier primitives for `RustUse`.

## Install

```toml
[dependencies]
use-identifier = "0.0.1"
```

## Foundation

`use-identifier` provides a validated `Identifier` type plus a lightweight `TypedIdentifier<K>` wrapper for cases where an identifier should carry domain meaning without changing its storage shape.

## Example

```rust
use use_identifier::{Identifier, IdentifierKind, TypedIdentifier};

struct Account;

impl IdentifierKind for Account {
    const NAME: &'static str = "account";
}

let plain = Identifier::new("acct_42")?;
let typed = TypedIdentifier::<Account>::new("acct_42")?;

assert_eq!(plain.as_str(), typed.as_str());
assert_eq!(typed.kind_name(), "account");
# Ok::<(), use_identifier::IdentifierError>(())
```

## When to use directly

Choose `use-identifier` when plain validated identifiers or a typed wrapper are the only ID surface you need.

## Scope

- Identifiers stay string-backed.
- Validation stays predictable and ASCII-oriented.
- Registry lookup, persistence, and generation policies are out of scope.

## Status

`use-identifier` is a pre-1.0 crate with a deliberately small API.
