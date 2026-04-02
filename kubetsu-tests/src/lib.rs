//! Integration tests: all adapter crates combined on the same types.

#[cfg(test)]
mod tests {
    // --- Concrete form ---

    kubetsu::define_id!(pub struct UserId(i64););
    kubetsu_serde::impl_serde!(UserId(i64));
    kubetsu_fake::impl_fake!(UserId(i64));
    kubetsu_sqlx::impl_sqlx!(UserId(i64));

    kubetsu::define_id!(pub struct ItemId(String););
    kubetsu_serde::impl_serde!(ItemId(String));
    kubetsu_fake::impl_fake!(ItemId(String));
    kubetsu_sqlx::impl_sqlx!(ItemId(String));

    // --- Generic form ---

    kubetsu::define_id!(pub struct MyId<T, U>;);
    kubetsu_serde::impl_serde!(MyId<T, U>);
    kubetsu_fake::impl_fake!(MyId<T, U>);
    kubetsu_sqlx::impl_sqlx!(MyId<T, U>);

    struct User;
    type MyUserId = MyId<User, i64>;

    #[test]
    fn test_combined_concrete() {
        use fake::{Fake, Faker};

        let id = UserId::new(42);

        // serde
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "42");
        let deserialized: UserId = serde_json::from_str(&json).unwrap();
        assert_eq!(*deserialized.inner(), 42);

        // fake
        let _fake_id: UserId = Faker.fake();

        // core traits
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_combined_generic() {
        use fake::{Fake, Faker};

        let id = MyUserId::new(42);

        // serde
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "42");
        let deserialized: MyUserId = serde_json::from_str(&json).unwrap();
        assert_eq!(*deserialized.inner(), 42);

        // fake
        let _fake_id: MyUserId = Faker.fake();

        // core traits
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    mod sqlx_tests {
        use super::*;
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use sqlx::{FromRow, SqlitePool};

        async fn get_db_conn() -> Result<SqlitePool, sqlx::Error> {
            let connect_info = SqliteConnectOptions::new();
            let pool = SqlitePoolOptions::new()
                .connect_with(connect_info)
                .await
                .unwrap();
            Ok(pool)
        }

        #[derive(FromRow)]
        struct Row {
            id: UserId,
        }

        #[tokio::test]
        async fn test_combined_sqlx_concrete() {
            let conn = get_db_conn().await.unwrap();
            let mut tx = conn.begin().await.unwrap();
            let row: Row = sqlx::query_as("SELECT 1 as id")
                .fetch_one(&mut *tx)
                .await
                .unwrap();
            assert_eq!(*row.id.inner(), 1);
        }

        #[derive(FromRow)]
        struct GenericRow {
            id: MyUserId,
        }

        #[tokio::test]
        async fn test_combined_sqlx_generic() {
            let conn = get_db_conn().await.unwrap();
            let mut tx = conn.begin().await.unwrap();
            let row: GenericRow = sqlx::query_as("SELECT 1 as id")
                .fetch_one(&mut *tx)
                .await
                .unwrap();
            assert_eq!(*row.id.inner(), 1);
        }
    }
}
