# kubetsu-libsql

[libsql](https://crates.io/crates/libsql) parameter binding support for [kubetsu](https://crates.io/crates/kubetsu) ID types.

## Usage

```rust
kubetsu::define_id!(pub struct UserId(i64););
kubetsu_libsql::impl_libsql!(UserId(i64));

let id = UserId::new(42);
assert_eq!(libsql::Value::from(id), libsql::Value::Integer(42));
```

The macro implements `From<UserId>` and `From<&UserId>` for `libsql::Value`, which is all
libsql needs to accept an ID anywhere it accepts a parameter: `params!`, `named_params!`,
tuples, arrays, `Vec`, and `Option<UserId>` for a nullable column. An ID is not `Copy`, so
pass `&id` when you still need it afterwards:

```rust,no_run
# kubetsu::define_id!(pub struct UserId(i64););
# kubetsu_libsql::impl_libsql!(UserId(i64));
# async fn run(conn: &libsql::Connection, id: UserId, parent: Option<UserId>) -> libsql::Result<()> {
conn.execute("INSERT INTO users (id, parent_id) VALUES (?1, ?2)", libsql::params![&id, parent]).await?;
conn.execute("DELETE FROM users WHERE id = ?1", [id]).await?;
# Ok(())
# }
```

Generic form is also supported:

```rust
kubetsu::define_id!(pub struct MyId<T, U>;);
kubetsu_libsql::impl_libsql!(MyId<T, U>);

struct User;
type UserId = MyId<User, i64>;

let id = UserId::new(42);
assert_eq!(libsql::Value::from(id), libsql::Value::Integer(42));
```

## Write side only

libsql's `FromValue` trait, which backs `Row::get::<T>`, is sealed, so this crate cannot
make `row.get::<UserId>(0)` work. Read a column as the inner type and wrap it:

```rust,no_run
# kubetsu::define_id!(pub struct UserId(i64););
# fn run(row: &libsql::Row) -> libsql::Result<()> {
let id = UserId::new(row.get::<i64>(0)?);
# Ok(())
# }
```

Or, when a whole row maps onto a struct, enable libsql's `serde` feature and use
`libsql::de::from_row` together with
[kubetsu-serde](https://crates.io/crates/kubetsu-serde).

## Install

```bash
$ cargo add kubetsu kubetsu-libsql
```
