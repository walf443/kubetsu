//! The sqlx adapter against the shared IDs, over SQLite.

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

#[derive(FromRow)]
struct ItemRow {
    id: ItemId,
}

/// Binds an owned `String` ID by value, which takes SQLite's by-value `encode`
/// path, and reads it back through `FromRow`.
#[tokio::test]
async fn test_sqlx_binds_a_string_id_by_value_and_by_reference() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();

    let id = ItemId::new("abc".to_string());
    let by_ref: ItemRow = sqlx::query_as("SELECT ? as id")
        .bind(&id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(by_ref.id, id);

    let by_value: ItemRow = sqlx::query_as("SELECT ? as id")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(by_value.id.inner(), "abc");
}

/// Same for the generic form, with a `Vec<u8>` inner stored as a BLOB.
#[tokio::test]
async fn test_sqlx_binds_a_blob_id_by_value() {
    kubetsu::define_id!(
        pub struct BlobId<T, U>;
    );
    kubetsu_sqlx::impl_sqlx!(BlobId<T, U>);

    #[derive(FromRow)]
    struct BlobRow {
        id: BlobId<User, Vec<u8>>,
    }

    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();

    let row: BlobRow = sqlx::query_as("SELECT ? as id")
        .bind(BlobId::<User, Vec<u8>>::new(vec![1, 2, 3]))
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(row.id.inner(), &[1, 2, 3]);
}
