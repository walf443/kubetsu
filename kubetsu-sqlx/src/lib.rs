#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod __private {
    pub use kubetsu;
    pub use sqlx;
}

/// Implement sqlx `Type`, `Encode`, and `Decode` for a kubetsu ID type.
///
/// Which database backends are supported depends on the enabled features:
/// `any`, `mysql`, `postgres`, `sqlite`.
///
/// # Concrete form
///
/// ```rust
/// kubetsu::define_id!(pub struct UserId(i64););
/// kubetsu_sqlx::impl_sqlx!(UserId(i64));
/// ```
///
/// # Generic form
///
/// ```rust
/// kubetsu::define_id!(pub struct MyId<T, U>;);
/// kubetsu_sqlx::impl_sqlx!(MyId<T, U>);
/// ```
#[macro_export]
macro_rules! impl_sqlx {
    // Concrete form: impl_sqlx!(UserId(i64));
    ($name:ident($inner:ty)) => {
        const _: () = {
            fn _assert_kubetsu_id()
            where
                $name: $crate::__private::kubetsu::KubetsuId<Inner = $inner>,
            {
            }
        };

        $crate::__impl_sqlx_any!([] $name, $inner);
        $crate::__impl_sqlx_mysql!([] $name, $inner);
        $crate::__impl_sqlx_postgres!([] $name, $inner);
        $crate::__impl_sqlx_sqlite!([] $name, $inner);
    };
    // Generic form: impl_sqlx!(MyId<T, U>);
    ($name:ident<$phantom:ident, $inner:ident>) => {
        const _: () = {
            fn _assert_kubetsu_id<$phantom, $inner>()
            where
                $name<$phantom, $inner>: $crate::__private::kubetsu::KubetsuId<Inner = $inner>,
            {
            }
        };

        $crate::__impl_sqlx_any!([$phantom, $inner] $name<$phantom, $inner>, $inner);
        $crate::__impl_sqlx_mysql!([$phantom, $inner] $name<$phantom, $inner>, $inner);
        $crate::__impl_sqlx_postgres!([$phantom, $inner] $name<$phantom, $inner>, $inner);
        $crate::__impl_sqlx_sqlite!([$phantom, $inner] $name<$phantom, $inner>, $inner);
    };
}

// =============================================================================
// sqlx-any
// =============================================================================

#[cfg(feature = "any")]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_sqlx_any {
    ([] $name:ty, $inner:ty) => {
        impl $crate::__private::sqlx::Type<$crate::__private::sqlx::Any> for $name {
            fn type_info() -> $crate::__private::sqlx::any::AnyTypeInfo {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Any>>::type_info()
            }

            fn compatible(ty: &$crate::__private::sqlx::any::AnyTypeInfo) -> bool {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Any>>::compatible(ty)
            }
        }

        impl $crate::__private::sqlx::Encode<'_, $crate::__private::sqlx::Any> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut <$crate::__private::sqlx::Any as $crate::__private::sqlx::Database>::ArgumentBuffer<'_>,
            ) -> Result<$crate::__private::sqlx::encode::IsNull, $crate::__private::sqlx::error::BoxDynError>
            {
                <$inner as $crate::__private::sqlx::Encode<$crate::__private::sqlx::Any>>::encode_by_ref(
                    self.inner(),
                    buf,
                )
            }
        }

        impl $crate::__private::sqlx::Decode<'_, $crate::__private::sqlx::Any> for $name {
            fn decode(
                value: <$crate::__private::sqlx::Any as $crate::__private::sqlx::Database>::ValueRef<'_>,
            ) -> Result<Self, $crate::__private::sqlx::error::BoxDynError> {
                let val =
                    <$inner as $crate::__private::sqlx::Decode<$crate::__private::sqlx::Any>>::decode(value)?;
                Ok(Self::new(val))
            }
        }
    };
    ([$($gen:tt)+] $name:ty, $inner:ty) => {
        impl<$($gen)+> $crate::__private::sqlx::Type<$crate::__private::sqlx::Any> for $name
        where
            $inner: $crate::__private::sqlx::Type<$crate::__private::sqlx::Any>,
        {
            fn type_info() -> $crate::__private::sqlx::any::AnyTypeInfo {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Any>>::type_info()
            }

            fn compatible(ty: &$crate::__private::sqlx::any::AnyTypeInfo) -> bool {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Any>>::compatible(ty)
            }
        }

        impl<$($gen)+> $crate::__private::sqlx::Encode<'_, $crate::__private::sqlx::Any> for $name
        where
            $inner: for<'a> $crate::__private::sqlx::Encode<'a, $crate::__private::sqlx::Any>,
        {
            fn encode_by_ref(
                &self,
                buf: &mut <$crate::__private::sqlx::Any as $crate::__private::sqlx::Database>::ArgumentBuffer<'_>,
            ) -> Result<$crate::__private::sqlx::encode::IsNull, $crate::__private::sqlx::error::BoxDynError>
            {
                <$inner as $crate::__private::sqlx::Encode<$crate::__private::sqlx::Any>>::encode_by_ref(
                    self.inner(),
                    buf,
                )
            }
        }

        impl<$($gen)+> $crate::__private::sqlx::Decode<'_, $crate::__private::sqlx::Any> for $name
        where
            $inner: for<'a> $crate::__private::sqlx::Decode<'a, $crate::__private::sqlx::Any>,
        {
            fn decode(
                value: <$crate::__private::sqlx::Any as $crate::__private::sqlx::Database>::ValueRef<'_>,
            ) -> Result<Self, $crate::__private::sqlx::error::BoxDynError> {
                let val =
                    <$inner as $crate::__private::sqlx::Decode<$crate::__private::sqlx::Any>>::decode(value)?;
                Ok(Self::new(val))
            }
        }
    };
}

#[cfg(not(feature = "any"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_sqlx_any {
    ([] $name:ty, $inner:ty) => {};
    ([$($gen:tt)+] $name:ty, $inner:ty) => {};
}

// =============================================================================
// sqlx-mysql
// =============================================================================

#[cfg(feature = "mysql")]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_sqlx_mysql {
    ([] $name:ty, $inner:ty) => {
        impl $crate::__private::sqlx::Type<$crate::__private::sqlx::MySql> for $name {
            fn type_info() -> $crate::__private::sqlx::mysql::MySqlTypeInfo {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::MySql>>::type_info()
            }

            fn compatible(ty: &$crate::__private::sqlx::mysql::MySqlTypeInfo) -> bool {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::MySql>>::compatible(ty)
            }
        }

        impl $crate::__private::sqlx::Encode<'_, $crate::__private::sqlx::MySql> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut <$crate::__private::sqlx::MySql as $crate::__private::sqlx::Database>::ArgumentBuffer<'_>,
            ) -> Result<$crate::__private::sqlx::encode::IsNull, $crate::__private::sqlx::error::BoxDynError>
            {
                <$inner as $crate::__private::sqlx::Encode<$crate::__private::sqlx::MySql>>::encode_by_ref(
                    self.inner(),
                    buf,
                )
            }
        }

        impl $crate::__private::sqlx::Decode<'_, $crate::__private::sqlx::MySql> for $name {
            fn decode(
                value: <$crate::__private::sqlx::MySql as $crate::__private::sqlx::Database>::ValueRef<'_>,
            ) -> Result<Self, $crate::__private::sqlx::error::BoxDynError> {
                let val =
                    <$inner as $crate::__private::sqlx::Decode<$crate::__private::sqlx::MySql>>::decode(value)?;
                Ok(Self::new(val))
            }
        }
    };
    ([$($gen:tt)+] $name:ty, $inner:ty) => {
        impl<$($gen)+> $crate::__private::sqlx::Type<$crate::__private::sqlx::MySql> for $name
        where
            $inner: $crate::__private::sqlx::Type<$crate::__private::sqlx::MySql>,
        {
            fn type_info() -> $crate::__private::sqlx::mysql::MySqlTypeInfo {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::MySql>>::type_info()
            }

            fn compatible(ty: &$crate::__private::sqlx::mysql::MySqlTypeInfo) -> bool {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::MySql>>::compatible(ty)
            }
        }

        impl<$($gen)+> $crate::__private::sqlx::Encode<'_, $crate::__private::sqlx::MySql> for $name
        where
            $inner: for<'a> $crate::__private::sqlx::Encode<'a, $crate::__private::sqlx::MySql>,
        {
            fn encode_by_ref(
                &self,
                buf: &mut <$crate::__private::sqlx::MySql as $crate::__private::sqlx::Database>::ArgumentBuffer<'_>,
            ) -> Result<$crate::__private::sqlx::encode::IsNull, $crate::__private::sqlx::error::BoxDynError>
            {
                <$inner as $crate::__private::sqlx::Encode<$crate::__private::sqlx::MySql>>::encode_by_ref(
                    self.inner(),
                    buf,
                )
            }
        }

        impl<$($gen)+> $crate::__private::sqlx::Decode<'_, $crate::__private::sqlx::MySql> for $name
        where
            $inner: for<'a> $crate::__private::sqlx::Decode<'a, $crate::__private::sqlx::MySql>,
        {
            fn decode(
                value: <$crate::__private::sqlx::MySql as $crate::__private::sqlx::Database>::ValueRef<'_>,
            ) -> Result<Self, $crate::__private::sqlx::error::BoxDynError> {
                let val =
                    <$inner as $crate::__private::sqlx::Decode<$crate::__private::sqlx::MySql>>::decode(value)?;
                Ok(Self::new(val))
            }
        }
    };
}

#[cfg(not(feature = "mysql"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_sqlx_mysql {
    ([] $name:ty, $inner:ty) => {};
    ([$($gen:tt)+] $name:ty, $inner:ty) => {};
}

// =============================================================================
// sqlx-postgres
// =============================================================================

#[cfg(feature = "postgres")]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_sqlx_postgres {
    ([] $name:ty, $inner:ty) => {
        impl $crate::__private::sqlx::Type<$crate::__private::sqlx::Postgres> for $name {
            fn type_info() -> <$crate::__private::sqlx::Postgres as $crate::__private::sqlx::Database>::TypeInfo {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Postgres>>::type_info()
            }

            fn compatible(
                ty: &<$crate::__private::sqlx::Postgres as $crate::__private::sqlx::Database>::TypeInfo,
            ) -> bool {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Postgres>>::compatible(ty)
            }
        }

        impl $crate::__private::sqlx::Encode<'_, $crate::__private::sqlx::Postgres> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut <$crate::__private::sqlx::Postgres as $crate::__private::sqlx::Database>::ArgumentBuffer<'_>,
            ) -> Result<$crate::__private::sqlx::encode::IsNull, $crate::__private::sqlx::error::BoxDynError>
            {
                <$inner as $crate::__private::sqlx::Encode<$crate::__private::sqlx::Postgres>>::encode_by_ref(
                    self.inner(),
                    buf,
                )
            }
        }

        impl $crate::__private::sqlx::Decode<'_, $crate::__private::sqlx::Postgres> for $name {
            fn decode(
                value: <$crate::__private::sqlx::Postgres as $crate::__private::sqlx::Database>::ValueRef<'_>,
            ) -> Result<Self, $crate::__private::sqlx::error::BoxDynError> {
                let val =
                    <$inner as $crate::__private::sqlx::Decode<$crate::__private::sqlx::Postgres>>::decode(value)?;
                Ok(Self::new(val))
            }
        }
    };
    ([$($gen:tt)+] $name:ty, $inner:ty) => {
        impl<$($gen)+> $crate::__private::sqlx::Type<$crate::__private::sqlx::Postgres> for $name
        where
            $inner: $crate::__private::sqlx::Type<$crate::__private::sqlx::Postgres>,
        {
            fn type_info() -> <$crate::__private::sqlx::Postgres as $crate::__private::sqlx::Database>::TypeInfo {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Postgres>>::type_info()
            }

            fn compatible(
                ty: &<$crate::__private::sqlx::Postgres as $crate::__private::sqlx::Database>::TypeInfo,
            ) -> bool {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Postgres>>::compatible(ty)
            }
        }

        impl<$($gen)+> $crate::__private::sqlx::Encode<'_, $crate::__private::sqlx::Postgres> for $name
        where
            $inner: for<'a> $crate::__private::sqlx::Encode<'a, $crate::__private::sqlx::Postgres>,
        {
            fn encode_by_ref(
                &self,
                buf: &mut <$crate::__private::sqlx::Postgres as $crate::__private::sqlx::Database>::ArgumentBuffer<'_>,
            ) -> Result<$crate::__private::sqlx::encode::IsNull, $crate::__private::sqlx::error::BoxDynError>
            {
                <$inner as $crate::__private::sqlx::Encode<$crate::__private::sqlx::Postgres>>::encode_by_ref(
                    self.inner(),
                    buf,
                )
            }
        }

        impl<$($gen)+> $crate::__private::sqlx::Decode<'_, $crate::__private::sqlx::Postgres> for $name
        where
            $inner: for<'a> $crate::__private::sqlx::Decode<'a, $crate::__private::sqlx::Postgres>,
        {
            fn decode(
                value: <$crate::__private::sqlx::Postgres as $crate::__private::sqlx::Database>::ValueRef<'_>,
            ) -> Result<Self, $crate::__private::sqlx::error::BoxDynError> {
                let val =
                    <$inner as $crate::__private::sqlx::Decode<$crate::__private::sqlx::Postgres>>::decode(value)?;
                Ok(Self::new(val))
            }
        }
    };
}

#[cfg(not(feature = "postgres"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_sqlx_postgres {
    ([] $name:ty, $inner:ty) => {};
    ([$($gen:tt)+] $name:ty, $inner:ty) => {};
}

// =============================================================================
// sqlx-sqlite
// =============================================================================

#[cfg(feature = "sqlite")]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_sqlx_sqlite {
    ([] $name:ty, $inner:ty) => {
        impl $crate::__private::sqlx::Type<$crate::__private::sqlx::Sqlite> for $name {
            fn type_info() -> $crate::__private::sqlx::sqlite::SqliteTypeInfo {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Sqlite>>::type_info()
            }

            fn compatible(ty: &$crate::__private::sqlx::sqlite::SqliteTypeInfo) -> bool {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Sqlite>>::compatible(ty)
            }
        }

        impl $crate::__private::sqlx::Encode<'_, $crate::__private::sqlx::Sqlite> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut <$crate::__private::sqlx::Sqlite as $crate::__private::sqlx::Database>::ArgumentBuffer<'_>,
            ) -> Result<$crate::__private::sqlx::encode::IsNull, $crate::__private::sqlx::error::BoxDynError>
            {
                <$inner as $crate::__private::sqlx::Encode<$crate::__private::sqlx::Sqlite>>::encode_by_ref(
                    self.inner(),
                    buf,
                )
            }
        }

        impl $crate::__private::sqlx::Decode<'_, $crate::__private::sqlx::Sqlite> for $name {
            fn decode(
                value: <$crate::__private::sqlx::Sqlite as $crate::__private::sqlx::Database>::ValueRef<'_>,
            ) -> Result<Self, $crate::__private::sqlx::error::BoxDynError> {
                let val =
                    <$inner as $crate::__private::sqlx::Decode<$crate::__private::sqlx::Sqlite>>::decode(value)?;
                Ok(Self::new(val))
            }
        }
    };
    ([$($gen:tt)+] $name:ty, $inner:ty) => {
        impl<$($gen)+> $crate::__private::sqlx::Type<$crate::__private::sqlx::Sqlite> for $name
        where
            $inner: $crate::__private::sqlx::Type<$crate::__private::sqlx::Sqlite>,
        {
            fn type_info() -> $crate::__private::sqlx::sqlite::SqliteTypeInfo {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Sqlite>>::type_info()
            }

            fn compatible(ty: &$crate::__private::sqlx::sqlite::SqliteTypeInfo) -> bool {
                <$inner as $crate::__private::sqlx::Type<$crate::__private::sqlx::Sqlite>>::compatible(ty)
            }
        }

        impl<$($gen)+> $crate::__private::sqlx::Encode<'_, $crate::__private::sqlx::Sqlite> for $name
        where
            $inner: for<'a> $crate::__private::sqlx::Encode<'a, $crate::__private::sqlx::Sqlite>,
        {
            fn encode_by_ref(
                &self,
                buf: &mut <$crate::__private::sqlx::Sqlite as $crate::__private::sqlx::Database>::ArgumentBuffer<'_>,
            ) -> Result<$crate::__private::sqlx::encode::IsNull, $crate::__private::sqlx::error::BoxDynError>
            {
                <$inner as $crate::__private::sqlx::Encode<$crate::__private::sqlx::Sqlite>>::encode_by_ref(
                    self.inner(),
                    buf,
                )
            }
        }

        impl<$($gen)+> $crate::__private::sqlx::Decode<'_, $crate::__private::sqlx::Sqlite> for $name
        where
            $inner: for<'a> $crate::__private::sqlx::Decode<'a, $crate::__private::sqlx::Sqlite>,
        {
            fn decode(
                value: <$crate::__private::sqlx::Sqlite as $crate::__private::sqlx::Database>::ValueRef<'_>,
            ) -> Result<Self, $crate::__private::sqlx::error::BoxDynError> {
                let val =
                    <$inner as $crate::__private::sqlx::Decode<$crate::__private::sqlx::Sqlite>>::decode(value)?;
                Ok(Self::new(val))
            }
        }
    };
}

#[cfg(not(feature = "sqlite"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_sqlx_sqlite {
    ([] $name:ty, $inner:ty) => {};
    ([$($gen:tt)+] $name:ty, $inner:ty) => {};
}

#[cfg(test)]
mod tests {
    kubetsu::define_id!(
        pub struct UserId(i64);
    );
    crate::impl_sqlx!(UserId(i64));

    kubetsu::define_id!(
        pub struct MyId<T, U>;
    );
    crate::impl_sqlx!(MyId<T, U>);

    #[cfg(feature = "sqlite")]
    mod sqlite_tests {
        use super::*;

        struct User;
        type MyUserId = MyId<User, i64>;
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

    #[cfg(feature = "mysql")]
    mod mysql_tests {
        use super::*;
        use ctor::dtor;
        use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
        use sqlx::{FromRow, MySqlPool};
        use std::sync::Mutex;
        use testcontainers::ContainerAsync;
        use testcontainers::runners::AsyncRunner;
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
            if let Some(container) = MYSQL_CONTAINER.lock().ok().and_then(|mut g| g.take()) {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let _ = rt.block_on(container.rm());
            }
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
    }

    #[cfg(feature = "postgres")]
    mod postgres_tests {
        use super::*;
        use ctor::dtor;
        use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
        use sqlx::{FromRow, PgPool};
        use std::sync::Mutex;
        use testcontainers::ContainerAsync;
        use testcontainers::runners::AsyncRunner;
        use testcontainers_modules::postgres::Postgres;
        use tokio::sync::OnceCell;

        static POSTGRES_CONTAINER: Mutex<Option<ContainerAsync<Postgres>>> = Mutex::new(None);
        static POSTGRES_POOL: OnceCell<PgPool> = OnceCell::const_new();

        async fn get_db_conn() -> Result<PgPool, sqlx::Error> {
            let pool = POSTGRES_POOL
                .get_or_init(|| async {
                    let container = Postgres::default().start().await.unwrap();
                    let host_port = container.get_host_port_ipv4(5432).await.unwrap();
                    let connect_info = PgConnectOptions::new()
                        .host("127.0.0.1")
                        .port(host_port)
                        .username("postgres")
                        .password("postgres");
                    let pool = PgPoolOptions::new()
                        .connect_with(connect_info)
                        .await
                        .unwrap();
                    *POSTGRES_CONTAINER.lock().unwrap() = Some(container);
                    pool
                })
                .await;
            Ok(pool.clone())
        }

        #[dtor]
        fn cleanup_postgres() {
            if let Some(container) = POSTGRES_CONTAINER.lock().ok().and_then(|mut g| g.take()) {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let _ = rt.block_on(container.rm());
            }
        }

        #[derive(FromRow)]
        struct Row {
            id: UserId,
        }

        #[tokio::test]
        async fn test_query_as() {
            let conn = get_db_conn().await.unwrap();
            let mut tx = conn.begin().await.unwrap();
            let row: Row = sqlx::query_as("SELECT 1::bigint as id")
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
            let got: i64 = sqlx::query_scalar("SELECT $1::bigint")
                .bind(&id)
                .fetch_one(&mut *tx)
                .await
                .unwrap();

            assert_eq!(got, 1);
        }
    }

    #[cfg(feature = "any")]
    mod any_tests {
        use super::*;
        use sqlx::any::{AnyConnectOptions, AnyPoolOptions, install_default_drivers};
        use sqlx::{AnyPool, FromRow};
        use std::str::FromStr;

        async fn get_db_conn() -> Result<AnyPool, sqlx::Error> {
            install_default_drivers();
            let connect_info = AnyConnectOptions::from_str("sqlite:").unwrap();
            let pool = AnyPoolOptions::new()
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
    }
}
