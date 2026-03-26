use std::collections::HashMap;

// --- Concrete form ---

crate::define_id!(
    pub struct UserId(i64);
);
crate::define_id!(
    pub struct ItemId(String);
);

// --- Generic form ---

crate::define_id!(
    pub struct MyId<T, U>;
);

struct User;
struct Item;
type MyUserId = MyId<User, i64>;
type MyItemId = MyId<Item, i64>;

#[test]
fn test_new_and_inner() {
    let id = UserId::new(42);
    assert_eq!(*id.inner(), 42);
}

#[test]
fn test_from() {
    let id: UserId = 42.into();
    assert_eq!(*id.inner(), 42);
}

#[test]
fn test_eq() {
    let a = UserId::new(1);
    let b = UserId::new(1);
    let c = UserId::new(2);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_clone() {
    let a = UserId::new(1);
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_debug() {
    let id = UserId::new(42);
    assert_eq!(format!("{:?}", id), "42");
}

#[test]
fn test_hash() {
    let mut map = HashMap::new();
    let id = UserId::new(1);
    map.insert(id.clone(), "user");
    assert_eq!(map.get(&id), Some(&"user"));
}

#[test]
fn test_string_id() {
    let id = ItemId::new("abc".to_string());
    assert_eq!(id.inner(), "abc");
}

mod generic_tests {
    use super::*;

    #[test]
    fn test_new_and_inner() {
        let id = MyUserId::new(42);
        assert_eq!(*id.inner(), 42);
    }

    #[test]
    fn test_from() {
        let id: MyUserId = 42.into();
        assert_eq!(*id.inner(), 42);
    }

    #[test]
    fn test_eq() {
        let a = MyUserId::new(1);
        let b = MyUserId::new(1);
        assert_eq!(a, b);
    }

    #[test]
    fn test_type_distinction() {
        // MyUserId and MyItemId are different types even with same inner value
        let _user_id = MyUserId::new(1);
        let _item_id = MyItemId::new(1);
        // assert_eq!(user_id, item_id); // would not compile
    }

    #[test]
    fn test_clone() {
        let a = MyUserId::new(1);
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_debug() {
        let id = MyUserId::new(42);
        assert_eq!(format!("{:?}", id), "42");
    }

    #[test]
    fn test_hash() {
        let mut map = HashMap::new();
        let id = MyUserId::new(1);
        map.insert(id.clone(), "user");
        assert_eq!(map.get(&id), Some(&"user"));
    }
}

#[cfg(feature = "serde")]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let id = UserId::new(42);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "42");
    }

    #[test]
    fn test_deserialize() {
        let id: UserId = serde_json::from_str("42").unwrap();
        assert_eq!(*id.inner(), 42);
    }

    #[test]
    fn test_serialize_string() {
        let id = ItemId::new("abc".to_string());
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"abc\"");
    }

    #[test]
    fn test_deserialize_string() {
        let id: ItemId = serde_json::from_str("\"abc\"").unwrap();
        assert_eq!(id.inner(), "abc");
    }

    #[test]
    fn test_generic_serialize() {
        let id = MyUserId::new(42);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "42");
    }

    #[test]
    fn test_generic_deserialize() {
        let id: MyUserId = serde_json::from_str("42").unwrap();
        assert_eq!(*id.inner(), 42);
    }
}

#[cfg(feature = "fake")]
mod fake_tests {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn test_fake() {
        let _id: UserId = Faker.fake();
    }

    #[test]
    fn test_fake_string() {
        let _id: ItemId = Faker.fake();
    }

    #[test]
    fn test_generic_fake() {
        let _id: MyUserId = Faker.fake();
    }
}

#[cfg(feature = "sqlx-sqlite")]
mod sqlx_sqlite_tests {
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
    async fn test_query_as() {
        let conn = get_db_conn().await.unwrap();
        let mut tx = conn.begin().await.unwrap();
        let row: Row = sqlx::query_as("SELECT 1 as id")
            .fetch_one(&mut *tx)
            .await
            .unwrap();

        assert_eq!(*row.id.inner(), 1);
    }

    #[tokio::test]
    async fn test_encode() {
        let conn = get_db_conn().await.unwrap();
        let id = UserId::new(1);

        let mut tx = conn.begin().await.unwrap();
        let got: i64 = sqlx::query_scalar("SELECT 1 WHERE 1 = ?")
            .bind(&id)
            .fetch_one(&mut *tx)
            .await
            .unwrap();

        assert_eq!(got, 1);
    }

    #[derive(FromRow)]
    struct GenericRow {
        id: MyUserId,
    }

    #[tokio::test]
    async fn test_generic_query_as() {
        let conn = get_db_conn().await.unwrap();
        let mut tx = conn.begin().await.unwrap();
        let row: GenericRow = sqlx::query_as("SELECT 1 as id")
            .fetch_one(&mut *tx)
            .await
            .unwrap();

        assert_eq!(*row.id.inner(), 1);
    }

    #[tokio::test]
    async fn test_generic_encode() {
        let conn = get_db_conn().await.unwrap();
        let id = MyUserId::new(1);

        let mut tx = conn.begin().await.unwrap();
        let got: i64 = sqlx::query_scalar("SELECT 1 WHERE 1 = ?")
            .bind(&id)
            .fetch_one(&mut *tx)
            .await
            .unwrap();

        assert_eq!(got, 1);
    }
}
