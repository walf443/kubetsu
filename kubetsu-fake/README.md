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

The implementation is generic over fake's config type and forwards to the inner type, so an
ID accepts every config the inner type does, not just `Faker`:

```rust
kubetsu::define_id!(pub struct UserId(i64););
kubetsu_fake::impl_fake!(UserId(i64));

use fake::{Dummy, Fake, Faker};

#[derive(Dummy)]
struct User {
    #[dummy(faker = "1000..2000")]
    id: UserId,
}

let user: User = Faker.fake();
assert!((1000..2000).contains(user.id.inner()));
```

Because the implementation covers every config, a hand-written `Dummy<SomeConfig>` for the
same ID type collides with `error[E0119]`.

## Install

```bash
$ cargo add kubetsu kubetsu-fake
```
