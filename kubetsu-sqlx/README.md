# kubetsu-sqlx

[sqlx](https://crates.io/crates/sqlx) `Type` / `Encode` / `Decode` support for [kubetsu](https://crates.io/crates/kubetsu) ID types.

## Usage

```rust
kubetsu::define_id!(pub struct UserId(i64););
kubetsu_sqlx::impl_sqlx!(UserId(i64));
```

Which database backends are supported depends on the enabled features:
`any`, `mysql`, `postgres`, `sqlite`.

Generic form is also supported:

```rust
kubetsu::define_id!(pub struct MyId<T, U>;);
kubetsu_sqlx::impl_sqlx!(MyId<T, U>);
```

## Install

```bash
$ cargo add kubetsu
$ cargo add kubetsu-sqlx --features sqlite  # choose your backend
```
