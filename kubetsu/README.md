# kubetsu: distinguish value type of other struct

This is a library that distinguish struct value type as other type value.

`kubetsu` (区別) means distinguish in Japanese.

# Usage

Define your own ID type using the `define_id!` macro. Different type tags prevent accidental misuse at compile time.

```rust, compile_fail
kubetsu::define_id!(pub struct Id<T, U>;);

struct User;
struct Item;
type UserId = Id<User, i32>;
type ItemId = Id<Item, i32>;

fn main() {
  let user_id = UserId::new(1);
  let item_id = ItemId::new(1);
  assert_ne!(user_id, item_id);
  // compile error
  // error[E0308]: mismatched types
  //     expected `Id<User, i32>`, found `Id<Item, i32>`
}
```

```rust
kubetsu::define_id!(pub struct Id<T, U>;);

struct User;
type UserId = Id<User, i32>;

let user_id = UserId::new(1);
// you can access original value reference with `inner()`.
assert_eq!(&1, user_id.inner());
// you can use `==` that have same value.
assert_eq!(UserId::new(1), user_id);
```

The generated type implements `Debug`, `PartialEq`, `Eq`, `Hash`, `Clone`, and `From<InnerType>`.

You can also generate a concrete type with a fixed inner type:

```rust
kubetsu::define_id!(pub struct UserId(i32););

let user_id = UserId::new(1);
assert_eq!(&1, user_id.inner());
```

## serde support

Use the [kubetsu-serde](https://crates.io/crates/kubetsu-serde) crate to serialize and deserialize as the inner value.

```rust,ignore
kubetsu::define_id!(pub struct Id<T, U>;);
kubetsu_serde::impl_serde!(Id<T, U>);

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
  id: Id<Self, i32>,
}

fn main() {
    let user = User { id: Id::new(1) };
    let str = serde_json::to_string(&user).unwrap();
    assert_eq!("{\"id\":1}", str);

    // you can deserialize
    let _: User = serde_json::from_str(&str).unwrap();
}
```

## sqlx support

Use the [kubetsu-sqlx](https://crates.io/crates/kubetsu-sqlx) crate to encode and decode ID values with [sqlx](https://crates.io/crates/sqlx). Enable the feature for each driver you need (`any`, `mysql`, `postgres`, `sqlite`).

```rust,no_run,ignore
kubetsu::define_id!(pub struct Id<T, U>;);
kubetsu_sqlx::impl_sqlx!(Id<T, U>);

#[derive(sqlx::FromRow)]
struct User {
  id: Id<Self, i32>,
}

async fn do_something_with_sqlx(conn: sqlx::AnyPool) -> Result<(), sqlx::Error> {
    let mut tx = conn.begin().await?;
    let user: User = sqlx::query_as("SELECT 1 as `id`").fetch_one(&mut *tx).await?;

    // you can also pass to bind
    let user2: User = sqlx::query_as("SELECT 1 as `id` WHERE 1 = ?").bind(&user.id).fetch_one(&mut *tx).await?;

    Ok(())
}
```

## fake support

Use the [kubetsu-fake](https://crates.io/crates/kubetsu-fake) crate to generate dummy values with [fake](https://crates.io/crates/fake).

```rust,ignore
kubetsu::define_id!(pub struct Id<T, U>;);
kubetsu_fake::impl_fake!(Id<T, U>);

use fake::{Faker, Fake, Dummy};

#[derive(Dummy)]
struct User {
  id: Id<Self, i32>,
}

fn main() {
    let user: User = Faker.fake();
    // do something with user
}
```

# `no_std` support

`kubetsu` and `kubetsu-serde` work in `#![no_std]` crates. The
`define_id!` macro expands to `::core::*` paths only, so it has no
dependency on `std` or `alloc`. Adapter crates `kubetsu-fake` and
`kubetsu-sqlx` remain `std`-only because their upstream dependencies
require `std`.

# Install

```bash
$ cargo add kubetsu
$ cargo add kubetsu-serde   # if you need serde support
$ cargo add kubetsu-fake    # if you need fake support
$ cargo add kubetsu-sqlx    # if you need sqlx support
```

# License

MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
see [LICENSE](LICENSE)

```ignore
Copyright (c) 2024 Keiji Yoshimi
```
