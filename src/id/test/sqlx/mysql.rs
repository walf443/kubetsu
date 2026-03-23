use crate::Id;
use ctor::dtor;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{FromRow, MySqlPool};
use std::sync::Mutex;
use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers_modules::mysql::Mysql;
use tokio::sync::OnceCell;

static MYSQL_CONTAINER: Mutex<Option<ContainerAsync<Mysql>>> = Mutex::new(None);
static MYSQL_POOL: OnceCell<MySqlPool> = OnceCell::const_new();

async fn get_db_conn() -> Result<MySqlPool, sqlx::Error> {
    let pool = MYSQL_POOL
        .get_or_init(|| async {
            let container = Mysql::default().start().await.unwrap();
            let host_port = container.get_host_port_ipv4(3306).await.unwrap();
            let connect_info = MySqlConnectOptions::new()
                .host("127.0.0.1")
                .port(host_port)
                .username("root");
            let pool = MySqlPoolOptions::new()
                .connect_with(connect_info)
                .await
                .unwrap();
            *MYSQL_CONTAINER.lock().unwrap() = Some(container);
            pool
        })
        .await;
    Ok(pool.clone())
}

#[dtor]
fn cleanup_mysql() {
    if let Some(container) = MYSQL_CONTAINER.lock().unwrap().take() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(container.rm());
    }
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
async fn test_query_scalar_i32() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let id: Id<FooI32, i32> = sqlx::query_scalar("SELECT 1")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(id.inner, 1);
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

#[tokio::test]
async fn test_encode_i32() {
    let conn = get_db_conn().await.unwrap();

    let id: Id<FooI32, i32> = Id::new(1);

    let mut tx = conn.begin().await.unwrap();
    let got: i64 = sqlx::query_scalar("SELECT 1 WHERE 1 = ?")
        .bind(&id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(got, 1);
}

#[derive(FromRow)]
struct FooI16 {
    id: Id<Self, i16>,
}

#[tokio::test]
async fn test_query_scalar_i16() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let id: Id<FooI16, i16> = sqlx::query_scalar("SELECT 1")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(id.inner, 1);
}
#[tokio::test]
async fn test_query_as_i16() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let foo: crate::id::test::sqlx::mysql::FooI16 = sqlx::query_as("SELECT 1 as id")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(foo.id.inner, 1);
}

#[tokio::test]
async fn test_encode_i16() {
    let conn = get_db_conn().await.unwrap();

    let id: Id<FooI16, i16> = Id::new(1);

    let mut tx = conn.begin().await.unwrap();
    let got: i64 = sqlx::query_scalar("SELECT 1 WHERE 1 = ?")
        .bind(&id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(got, 1);
}

#[derive(FromRow)]
struct FooI8 {
    id: Id<Self, i8>,
}

#[tokio::test]
async fn test_query_scalar_i8() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let id: Id<FooI8, i8> = sqlx::query_scalar("SELECT 1")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(id.inner, 1);
}
#[tokio::test]
async fn test_query_as_i8() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let foo: crate::id::test::sqlx::mysql::FooI8 = sqlx::query_as("SELECT 1 as id")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(foo.id.inner, 1);
}

#[tokio::test]
async fn test_encode_i8() {
    let conn = get_db_conn().await.unwrap();

    let id: Id<FooI8, i8> = Id::new(1);

    let mut tx = conn.begin().await.unwrap();
    let got: i64 = sqlx::query_scalar("SELECT 1 WHERE 1 = ?")
        .bind(&id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(got, 1);
}

#[derive(FromRow)]
struct FooStr {
    id: Id<Self, String>,
}

#[tokio::test]
async fn test_query_scalar_str() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let id: Id<FooStr, String> = sqlx::query_scalar("SELECT \"1\"")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(id.inner, "1");
}
#[tokio::test]
async fn test_query_as_str() {
    let conn = get_db_conn().await.unwrap();
    let mut tx = conn.begin().await.unwrap();
    let foo: crate::id::test::sqlx::mysql::FooStr = sqlx::query_as("SELECT \"1\" as id")
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(foo.id.inner, "1");
}
#[tokio::test]
async fn test_encode_str() {
    let conn = get_db_conn().await.unwrap();

    let id: Id<FooStr, String> = Id::new("1".to_string());

    let mut tx = conn.begin().await.unwrap();
    let got: i64 = sqlx::query_scalar("SELECT 1 WHERE \"1\" = ?")
        .bind(&id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    assert_eq!(got, 1);
}
