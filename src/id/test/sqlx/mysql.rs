use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{FromRow, MySqlPool};
use crate::Id;

async fn get_db_conn() -> Result<MySqlPool, sqlx::Error> {
    let connect_info = MySqlConnectOptions::new();
    let pool = MySqlPoolOptions::new().connect_with(connect_info).await.unwrap();
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
    let foo: FooI64 = sqlx::query_as("SELECT 1 as id").fetch_one(&mut *tx).await.unwrap();

    assert_eq!(foo.id.inner, 1);
}

#[derive(FromRow)]
struct FooI32 {
    id: Id<Self, i32>,
}

#[tokio::test]
async fn test_query_as_i32() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let foo: FooI32 = sqlx::query_as("SELECT 1 as id").fetch_one(&mut *tx).await.unwrap();

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
    let foo: crate::id::test::sqlx::mysql::FooI16 = sqlx::query_as("SELECT 1 as id").fetch_one(&mut *tx).await.unwrap();

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
    let foo: crate::id::test::sqlx::mysql::FooI8 = sqlx::query_as("SELECT 1 as id").fetch_one(&mut *tx).await.unwrap();

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
    let foo: crate::id::test::sqlx::mysql::FooStr = sqlx::query_as("SELECT \"1\" as id").fetch_one(&mut *tx).await.unwrap();

    assert_eq!(foo.id.inner, "1");
}