# Upgrade Guide

## 0.1.x → 0.2.0

Requires `kubetsu` 0.8. An older `kubetsu` alongside `kubetsu-fake` 0.2 resolves two different `KubetsuId` traits, so upgrade them together. See [kubetsu's upgrade guide](https://github.com/walf443/kubetsu/blob/main/kubetsu/UPGRADE.md) for the changes in that crate.

```toml
[dependencies]
kubetsu = "0.8"
kubetsu-fake = "0.2"
```

### Breaking Change: `impl_fake!` forwards every fake config

`impl_fake!` used to implement `fake::Dummy<Faker>` only, so an ID accepted no config other than `Faker`. `#[dummy(faker = "1000..2000")]` on an ID field did not compile, and neither did `UUIDv7.fake::<EventId>()` -- the config never reached the inner type.

The implementation is now generic over the config and forwards it, so an ID accepts everything its inner type does:

```rust
#[derive(Dummy)]
struct User {
    #[dummy(faker = "1000..2000")]
    id: UserId,
}
```

`Dummy<Faker>` is the `C = Faker` case, so existing use is unaffected.

### Migration

Nothing to do unless you worked around the old limitation with your own implementation. Because the macro now covers every config, a hand-written one collides:

```text
error[E0119]: conflicting implementations of trait `Dummy<UUIDv7>` for type `EventId`
```

Delete it; the macro forwards that config on its own.

If instead you hand-wrote `Dummy<Faker>` and deliberately did not call `impl_fake!` for that type, nothing changes.
