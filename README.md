# kubetsu: distinguish value type of other struct

This is a library that distinguish struct value type as other type value.

`kubetsu` (区別) means distinguish in Japanese.

# Usage

```rust,compile_fail
use kubetsu::Id;

struct User {
  id: Id<Self, i32>
}

type UserId = Id<User, i32>;

struct Item {
  id: Id<Self, i32>
}

type ItemId = Id<Item, i32>;

fn main() {
  let user_id = UserId::new(1);
  let item_id = ItemId::new(1);
  assert_ne!(user_id, item_id); 
  // compile error
  // ---- src/lib.rs - Id (line 10) stdout ----
  //     error[E0308]: mismatched types
  //     --> src/lib.rs:29:3
  //     |
  //     21 |   assert_ne!(user_id, item_id); // compile error
  // |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Id<User, i32>`, found `Id<Item, i32>`
  //     |
  //     = note: expected struct `Id<User, _>`
  //     found struct `Id<Item, _>`
  //     = note: this error originates in the macro `assert_ne` (in Nightly builds, run with -Z macro-backtrace for more info)
  //     error: aborting due to previous error
}
```

```rust
struct User {}
type UserId = kubetsu::Id<User, i32>;

let user_id = UserId::new(1);
// you can access original value reference with `inner()`.
assert_eq!(&1, user_id.inner());
// you can use `==` that have same value`.
assert_eq!(UserId::new(1), user_id);
```

## serde support

You can serialize and deserialize as original value if you use [serde] crate and feature = "serde" enabled.

```rust,ignore
use kubetsu::Id;

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
  id: Id<Self, i32>
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

You can encode and decode `Id` value if you use [sqlx](https://crates.io/crates/sqlx) crate and feature = "sqlx" enabled.
You can select "sqlx-xxxx" feature for each driver. ("sqlx-any", "sqlx-mysql", "sqlx-postgres", "sqlx-sqlite")

```rust,no_run,ignore
#[derive(sqlx::FromRow)]
struct User {
  id: kubetsu::Id<Self, i32>
}

async fn do_something_with_sqlx(conn: sqlx::AnyPool) -> Result<(), sqlx::Error> {
    let mut tx = conn.begin().await?;
    let user: User = sqlx::query_as("SELECT 1 as `id`").fetch_one(&mut *tx).await?;
    // do something with user

    // you can also pass to bind
    let user2: User = sqlx::query_as("SELECT 1 as `id` WHERE 1 = ?").bind(&user.id).fetch_one(&mut *tx).await?;

    Ok(())
}
```

## fake support

You can use `Faker.fake()` if you use [fake](https://crates.io/crates/fake) crate and feature = "fake" enabled.

```rust,ignore
use kubetsu::Id;
use fake::{Faker, Fake, Dummy};

#[derive(Dummy)]
struct User {
  id: Id<Self, i32>
}

fn main() {
    let user: User = Faker.fake();
    // do something with user
}
```

# Install

```bash
$ cargo add kubetsu
```

# License

MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
see [LICENSE](LICENSE)

```ignore
Copyright (c) 2024 Keiji Yoshimi
```


