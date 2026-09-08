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
