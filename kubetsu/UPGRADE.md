# Upgrade Guide

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

### Breaking Change: `kubetsu-fake` forwards every fake config

`impl_fake!` used to implement `fake::Dummy<Faker>` only, so an ID accepted no config other than `Faker`: `#[dummy(faker = "1000..2000")]` on an ID field did not compile, and neither did `UUIDv7.fake::<EventId>()`. It is now generic over the config type and forwards to the inner type, so an ID accepts everything its inner type does.

```rust
#[derive(Dummy)]
struct User {
    #[dummy(faker = "1000..2000")]
    id: UserId,
}
```

#### Migration

Nothing to do unless you worked around the old limitation with your own implementation. Because the macro now covers every config, a hand-written one collides:

```text
error[E0119]: conflicting implementations of trait `Dummy<UUIDv7>` for type `EventId`
```

Delete it; the macro forwards that config on its own. If instead you hand-wrote `Dummy<Faker>` and deliberately did not call `impl_fake!` for that type, nothing changes.

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
