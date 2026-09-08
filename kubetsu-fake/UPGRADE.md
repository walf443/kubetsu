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

Nothing to do unless you worked around the old limitation with your own implementation.

The macro's implementation applies to a config whenever the inner type implements it, so a hand-written one collides in two cases:

- the inner type implements that config; or
- rustc cannot prove it never will, which is every config type from another crate -- `std`'s `Range` included. It has to assume a future release could add the impl, so it reports a conflict even though none exists today:

```text
error[E0119]: conflicting implementations of trait `Dummy<std::ops::Range<i32>>`
              for type `UserId`
note: upstream crates may add a new impl of trait
      `fake::Dummy<std::ops::Range<i32>>` for type `i64` in future versions
```

That leaves one case that compiles: a config type declared in your own crate, for which you have not also implemented `Dummy` on the inner type. The orphan rule stops other crates adding that impl, so rustc can rule it out -- but it does not stop you, and implementing `Dummy<YourConfig>` for the inner type as well brings the conflict back.

Where it does collide, delete yours: the macro forwards that config on its own. Check what the deleted implementation did first, though. Deleting it swaps your logic for the inner type's, and that is not a compile error -- if yours returned a fixed value so fixtures stayed reproducible, you will now get whatever the inner type generates for that config. To keep such logic, put it on a config type declared in your own crate and leave the inner type without a `Dummy` for it.

If instead you hand-wrote `Dummy<Faker>` and deliberately did not call `impl_fake!` for that type, nothing changes.
