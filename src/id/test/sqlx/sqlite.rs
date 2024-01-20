use crate::Id;
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
struct FooI64 {
    id: Id<Self, i64>,
}

#[tokio::test]
async fn test_query_as_i64() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let foo: FooI64 = sqlx::query_as("SELECT 1 as id")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(foo.id.inner, 1);
}

#[tokio::test]
async fn test_encode_i64() {
    let conn = get_db_conn().await.unwrap();

    let id: Id<FooI64, i64> = Id::new(1);

    let mut tx = conn.begin().await.unwrap();
    let got: i64 = sqlx::query_scalar("SELECT 1 WHERE 1 = ?")
        .bind(&id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(got, 1);
}

#[derive(FromRow)]
struct FooI32 {
    id: Id<Self, i32>,
}

#[tokio::test]
async fn test_query_as_i32() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let foo: FooI32 = sqlx::query_as("SELECT 1 as id")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(foo.id.inner, 1);
}

#[derive(FromRow)]
struct FooI16 {
    id: Id<Self, i16>,
}

#[tokio::test]
async fn test_query_as_i16() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let foo: FooI16 = sqlx::query_as("SELECT 1 as id")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(foo.id.inner, 1);
}

#[derive(FromRow)]
struct FooI8 {
    id: Id<Self, i8>,
}

#[tokio::test]
async fn test_query_as_i8() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let foo: FooI8 = sqlx::query_as("SELECT 1 as id")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(foo.id.inner, 1);
}

#[derive(FromRow)]
struct FooStr {
    id: Id<Self, String>,
}

#[tokio::test]
async fn test_query_as_str() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let foo: FooStr = sqlx::query_as("SELECT \"1\" as id")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(foo.id.inner, "1");
}
