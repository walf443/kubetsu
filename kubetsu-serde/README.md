# kubetsu-serde

serde `Serialize` / `Deserialize` support for [kubetsu](https://crates.io/crates/kubetsu) ID types.

## Usage

```rust
kubetsu::define_id!(pub struct UserId(i64););
kubetsu_serde::impl_serde!(UserId(i64));

let id = UserId::new(42);
let json = serde_json::to_string(&id).unwrap();
assert_eq!(json, "42");

let deserialized: UserId = serde_json::from_str(&json).unwrap();
assert_eq!(*deserialized.inner(), 42);
```

Generic form is also supported:

```rust
kubetsu::define_id!(pub struct MyId<T, U>;);
kubetsu_serde::impl_serde!(MyId<T, U>);

struct User;
type UserId = MyId<User, i64>;

let id = UserId::new(42);
let json = serde_json::to_string(&id).unwrap();
assert_eq!(json, "42");
```

## `no_std` support

This crate works in `#![no_std]` environments. The `impl_serde!` macro
expands to `::core::*` paths only and the `serde` dependency is pulled
with `default-features = false`.

## Install

```bash
$ cargo add kubetsu kubetsu-serde
```
