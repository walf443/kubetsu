# Upgrade Guide

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
