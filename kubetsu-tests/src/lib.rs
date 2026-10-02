//! Integration tests: all adapter crates combined on the same types.
//!
//! kubetsu-libsql is deliberately not among them. Its own tests enable
//! libsql's `core` feature, and under `cargo test --workspace` that feature
//! unifies into every member that depends on libsql at all. `core` links
//! libsql-ffi's bundled SQLite, which defines the same `sqlite3_*` symbols
//! as the libsqlite3-sys that sqlx links here, so the test binary of this
//! crate would carry two SQLites. On macOS the linker warns and picks one;
//! GNU ld has not been tried here and is expected to refuse. Coexistence
//! with serde and fake is covered in kubetsu-libsql's own tests instead.

#[cfg(test)]
mod tests {
    // --- Concrete form ---

    kubetsu::define_id!(
        pub struct UserId(i64);
    );
    kubetsu_serde::impl_serde!(UserId(i64));
    kubetsu_fake::impl_fake!(UserId(i64));
    kubetsu_sqlx::impl_sqlx!(UserId(i64));

    kubetsu::define_id!(
        pub struct ItemId(String);
    );
    kubetsu_serde::impl_serde!(ItemId(String));
    kubetsu_fake::impl_fake!(ItemId(String));
    kubetsu_sqlx::impl_sqlx!(ItemId(String));

    // --- Generic form ---

    kubetsu::define_id!(
        pub struct MyId<T, U>;
    );
    kubetsu_serde::impl_serde!(MyId<T, U>);
    kubetsu_fake::impl_fake!(MyId<T, U>);
    kubetsu_sqlx::impl_sqlx!(MyId<T, U>);

    struct User;
    type MyUserId = MyId<User, i64>;

    /// Every adapter on one ID. See `tests/combined_tests.rs`.
    mod combined_tests;

    /// The sqlx adapter over SQLite. See `tests/sqlx_tests.rs`.
    mod sqlx_tests;

    /// UUID v7 as an ID inner type. See `tests/uuid_tests.rs`.
    mod uuid_tests;

    /// ULID as an ID inner type. See `tests/ulid_tests.rs`.
    mod ulid_tests;
}
