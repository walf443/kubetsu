# kubetsu-fake

[fake](https://crates.io/crates/fake) dummy data generation support for [kubetsu](https://crates.io/crates/kubetsu) ID types.

## Usage

```rust
kubetsu::define_id!(pub struct UserId(i64););
kubetsu_fake::impl_fake!(UserId(i64));

use fake::{Fake, Faker};
let _id: UserId = Faker.fake();
```

Generic form is also supported:

```rust
kubetsu::define_id!(pub struct MyId<T, U>;);
kubetsu_fake::impl_fake!(MyId<T, U>);

struct User;
type UserId = MyId<User, i64>;

use fake::{Fake, Faker};
let _id: UserId = Faker.fake();
```

## Install

```bash
$ cargo add kubetsu kubetsu-fake
```
