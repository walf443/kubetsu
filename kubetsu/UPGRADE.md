# Upgrade Guide

## 0.8.x → 0.9.0

One breaking change, in the concrete form of `define_id!`. Start with the dependency update, which every upgrade needs.

### Update your Cargo.toml dependencies

**Before:**
```toml
[dependencies]
kubetsu = "0.8"
kubetsu-serde = "0.2"
kubetsu-fake = "0.2"
kubetsu-sqlx = { version = "0.3", features = ["sqlite"] }
```

**After:**
```toml
[dependencies]
kubetsu = "0.9"
kubetsu-serde = "0.3"
kubetsu-fake = "0.3"
kubetsu-sqlx = { version = "0.4", features = ["sqlite"] }
```

The adapter crates are bumped together because they expose `kubetsu` through the macros they generate. Mixing an older adapter with kubetsu 0.9 resolves two different `KubetsuId` traits.

### Breaking Change: the concrete form is generated as a tuple struct

The concrete form used to expand to a struct with one named field:

```rust
pub struct UserId {
    inner: i64,
}
```

It now expands to a tuple struct with one unnamed field:

```rust
pub struct UserId(i64);
```

The change lets derive macros that only accept a newtype shape -- a single unnamed field -- be attached through the attribute position of `define_id!`. The generic form is unchanged.

The field was private before and remains private, so code outside the defining module is unaffected: `UserId::new(..)`, `.inner()` and `KubetsuId` behave exactly as they did. Two things do change.

#### Migration: if you named the `inner` field inside the defining module

Within the module that invokes `define_id!` the private field was reachable as `inner`. Such code no longer compiles:

```text
error[E0560]: struct `UserId` has no field named `inner`
error[E0769]: tuple variant `UserId` written as struct variant
error[E0609]: no field `inner` on type `UserId`
```

Replace the field name with `0`, or prefer the public API.

**Before:**
```rust
let id = UserId { inner: 42 };
let raw = id.inner;
let UserId { inner } = id;
```

**After:**
```rust
let id = UserId::new(42);   // or `UserId(42)`
let raw = *id.inner();      // or `id.0`
let UserId(inner) = id;
```

#### Migration: if you derived a shape-sensitive trait on a concrete ID

A derive macro attached through `define_id!` sees the generated struct, and some derives produce different output for a named field than for a newtype. `serde::Serialize` is the common case: on the 0.8 shape it emitted `{"inner":42}`; on the 0.9 shape it emits `42`. This compiles cleanly, so check any concrete ID that derives `Serialize`, `Deserialize`, `schemars::JsonSchema` or a similar shape-dependent trait directly:

```rust
kubetsu::define_id!(
    #[derive(serde::Serialize)]   // wire format changes from {"inner":42} to 42
    pub struct UserId(i64);
);
```

If the newtype output is what you wanted, nothing needs to change -- and `kubetsu_serde::impl_serde!`, which has always serialized as the bare inner value, remains the recommended route. If you need to keep emitting `{"inner":42}` for compatibility, wrap the ID in a struct of your own that has an `inner` field and derive on that instead.

## 0.7.x → 0.8.0

Three breaking changes, all in `define_id!`. Start with the dependency update, which every upgrade needs.

### Update your Cargo.toml dependencies

**Before:**
```toml
[dependencies]
kubetsu = "0.7"
kubetsu-serde = "0.1"
kubetsu-fake = "0.1"
kubetsu-sqlx = { version = "0.1", features = ["sqlite"] }
```

**After:**
```toml
[dependencies]
kubetsu = "0.8"
kubetsu-serde = "0.2"
kubetsu-fake = "0.2"
kubetsu-sqlx = { version = "0.3", features = ["sqlite"] }
```

The adapter crates are bumped together because they expose `kubetsu` through the macros they generate. Mixing an older adapter with kubetsu 0.8 resolves two different `KubetsuId` traits.

### Breaking Change: `PartialOrd` and `Ord` are generated for the generic form

The generic form now implements `PartialOrd` and `Ord`, each conditional on the inner value type, so an ID can be sorted or used as a `BTreeMap` key.

These implementations are generated into your crate, so any `PartialOrd` or `Ord` you supply yourself for a `define_id!` generic type now collides:

```text
error[E0119]: conflicting implementations of trait `PartialOrd` for type `MyId<_, _>`
```

The concrete form is unchanged: derive them there when you need them.

#### Migration: if you derived them on a generic ID

Remove the derive.

**Before:**
```rust
kubetsu::define_id!(
    #[derive(PartialOrd, Ord)]
    pub struct MyId<T, U>;
);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct User; // the derive forced `Ord` onto the phantom tag
```

**After:**
```rust
kubetsu::define_id!(pub struct MyId<T, U>;);

struct User; // no longer needs to be `Ord`
```

The generated implementations bound only the inner value type, so the phantom tag no longer needs to be `Ord`.

#### Migration: if you hand-wrote them for a generic ID

A hand-written implementation collides too, including one written for a single instantiation such as `MyId<User, i64>`. Ordering for the type itself now comes from the macro, so a bespoke order has to move elsewhere:

```rust
// Before: a reversed `Ord` implementation on the ID type itself.
// After: order at the point of use.
ids.sort_by(|a, b| b.cmp(a));
```

Alternatively, wrap the ID in a type of your own and implement the ordering there.

### Breaking Change: the concrete form requires its inner type to implement `Eq`

The concrete form implements `Eq` unconditionally. `Eq` has no methods, so it was claimed even for an inner type that implements only `PartialEq`. The resulting ID advertised `Eq` while reporting `a == a` as `false`, which puts duplicate entries in a `HashSet`. The requirement is now asserted rather than assumed:

```text
error[E0277]: the trait bound `Weight: Eq` is not satisfied
help: the trait `Eq` is not implemented for `Weight`
```

#### Migration

Implement `Eq` for the inner type if it can satisfy the contract:

```rust
impl Eq for Weight {}
```

If it cannot -- an inner type holding a float, say -- use the generic form instead. Its implementations are each conditional on the inner type, so it yields `PartialEq` without `Eq`:

```rust
kubetsu::define_id!(pub struct MyId<T, U>;);

struct WeightTag;
type WeightId = MyId<WeightTag, Weight>;
```

### Breaking Change: the concrete form resolves its core traits unambiguously

The concrete form used to call the inner type's trait methods without qualifying them (`self.inner().fmt(f)`, `.eq(..)`, `.hash(..)`, `.clone()`). Method calls in a macro resolve against the traits in scope where the macro is invoked, not against the trait being implemented, which had two consequences:

- The concrete form failed to compile for any caller with `use std::fmt::Display;` in the same module, including a plain `String` or `i64` inner type, with `error[E0034]: multiple applicable items in scope`.
- An inner type implementing `Display` but not `Debug` received a `Debug` implementation that printed its `Display` output -- the same kind of false claim as the `Eq` case above.

The generated bodies now use fully qualified paths, so the inner type must genuinely implement `Debug`, `PartialEq`, `Hash` and `Clone`.

#### Migration

If an inner type relied on the accidental delegation -- it implements `Display` but not `Debug`, and the ID's `{:?}` output was its `Display` text -- implement `Debug` for it. Nothing else changes; callers previously blocked by `E0034` now compile.

## 0.6.x → 0.7.0

### Breaking Change: serde / fake / sqlx support moved to separate crates

In v0.7.0, serde / fake / sqlx support previously provided via feature flags on the `kubetsu` crate has been split into individual crates.

This allows each adapter crate to release independently, so you can update library versions (e.g., `fake`, `sqlx`) without waiting for a new kubetsu release.

#### Migration: update your Cargo.toml dependencies

**Before:**
```toml
[dependencies]
kubetsu = { version = "0.6", features = ["serde", "fake", "sqlx-sqlite"] }
```

**After:**
```toml
[dependencies]
kubetsu = "0.7"
kubetsu-serde = "0.1"       # if you use serde
kubetsu-fake = "0.1"        # if you use fake
kubetsu-sqlx = { version = "0.1", features = ["sqlite"] }  # if you use sqlx
```

Note that `kubetsu-sqlx` feature names no longer have the `sqlx-` prefix:

| Before (kubetsu 0.6) | After (kubetsu-sqlx 0.1) |
|---|---|
| `sqlx-any` | `any` |
| `sqlx-mysql` | `mysql` |
| `sqlx-postgres` | `postgres` |
| `sqlx-sqlite` | `sqlite` |

#### Migration: add adapter macros after `define_id!`

`define_id!` still generates core traits (Debug, PartialEq, Eq, Hash, Clone, From), but serde / fake / sqlx impls are no longer generated automatically.

**Before:**
```rust
kubetsu::define_id!(pub struct UserId(i64););
// serde / fake / sqlx impls were auto-generated via feature flags
```

**After:**
```rust
kubetsu::define_id!(pub struct UserId(i64););
kubetsu_serde::impl_serde!(UserId(i64));
kubetsu_fake::impl_fake!(UserId(i64));
kubetsu_sqlx::impl_sqlx!(UserId(i64));
```

The same applies to the generic form:

**Before:**
```rust
kubetsu::define_id!(pub struct MyId<T, U>;);
```

**After:**
```rust
kubetsu::define_id!(pub struct MyId<T, U>;);
kubetsu_serde::impl_serde!(MyId<T, U>);
kubetsu_fake::impl_fake!(MyId<T, U>);
kubetsu_sqlx::impl_sqlx!(MyId<T, U>);
```

#### Migration: if you are using the deprecated `Id` type

`kubetsu::Id` was deprecated in 0.6.0. It is still available in v0.7.0 but no longer has serde / fake / sqlx support. Migrating to `define_id!` is recommended.
