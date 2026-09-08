# Upgrade Guide

## 0.7.x → 0.8.0

### Breaking Change: `PartialOrd` and `Ord` are generated for the generic form

`define_id!`'s generic form now implements `PartialOrd` and `Ord`, each conditional on the inner value type, so an ID can be sorted or used as a `BTreeMap` key.

These implementations are generated into your crate, so any `PartialOrd` or `Ord` you supply yourself for a `define_id!` generic type now collides:

```text
error[E0119]: conflicting implementations of trait `PartialOrd` for type `MyId<_, _>`
```

The concrete form is unchanged.

### Migration

#### 1. Update your Cargo.toml dependencies

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

#### 2. If you derived `PartialOrd` / `Ord` on a generic ID

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

#### 3. If you hand-wrote `PartialOrd` / `Ord` for a generic ID

A hand-written implementation collides too, including one written for a single instantiation such as `MyId<User, i64>`. Ordering for the type itself now comes from the macro, so a bespoke order has to move elsewhere:

```rust
// Before: a reversed `Ord` implementation on the ID type itself.
// After: order at the point of use.
ids.sort_by(|a, b| b.cmp(a));
```

Alternatively, wrap the ID in a type of your own and implement the ordering there.

## 0.6.x → 0.7.0

### Breaking Change: serde / fake / sqlx support moved to separate crates

In v0.7.0, serde / fake / sqlx support previously provided via feature flags on the `kubetsu` crate has been split into individual crates.

This allows each adapter crate to release independently, so you can update library versions (e.g., `fake`, `sqlx`) without waiting for a new kubetsu release.

### Migration

#### 1. Update your Cargo.toml dependencies

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

#### 2. Add adapter macros after `define_id!`

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

#### 3. If you are using the deprecated `Id` type

`kubetsu::Id` was deprecated in 0.6.0. It is still available in v0.7.0 but no longer has serde / fake / sqlx support. Migrating to `define_id!` is recommended.
