# kubetsu

`kubetsu` (区別) means "distinguish" in Japanese.

A Rust library that distinguishes struct value types as different types, preventing accidental misuse at compile time.

## Crates

| Crate | Description |
|---|---|
| [kubetsu](kubetsu/) | Core crate — `define_id!` macro and core traits |
| [kubetsu-serde](kubetsu-serde/) | serde Serialize / Deserialize support |
| [kubetsu-fake](kubetsu-fake/) | fake dummy data generation support |
| [kubetsu-sqlx](kubetsu-sqlx/) | sqlx Type / Encode / Decode support |

## Quick Start

```rust
kubetsu::define_id!(pub struct UserId(i64););
kubetsu_serde::impl_serde!(UserId(i64));
kubetsu_fake::impl_fake!(UserId(i64));
kubetsu_sqlx::impl_sqlx!(UserId(i64));
```

See each crate's README for detailed usage.

## License

MIT license — see [LICENSE](LICENSE).
