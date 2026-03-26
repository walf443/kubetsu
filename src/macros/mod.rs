#[cfg(test)]
mod test;

/// Define a custom ID type with the same capabilities as `kubetsu::Id`.
///
/// This macro generates a standalone struct that wraps an inner value type,
/// along with trait implementations for common operations.
///
/// # Example
///
/// ```rust
/// kubetsu::define_id!(pub struct UserId(i64););
/// kubetsu::define_id!(pub struct ItemId(String););
///
/// let user_id = UserId::new(42);
/// assert_eq!(*user_id.inner(), 42);
///
/// let item_id: ItemId = "abc".to_string().into();
/// assert_eq!(item_id.inner(), "abc");
/// ```
///
/// The generated type always implements:
/// - `new()` and `inner()` methods
/// - `Debug`, `PartialEq`, `Eq`, `Hash`, `Clone`
/// - `From<InnerType>`
///
/// When kubetsu is compiled with optional features, the type also implements:
/// - `serde::Serialize` / `serde::Deserialize` (feature `serde`)
/// - `sqlx::Type` / `sqlx::Encode` / `sqlx::Decode` (features `sqlx-*`)
/// - `fake::Dummy` (feature `fake`)
#[macro_export]
macro_rules! define_id {
    ($(#[$meta:meta])* $vis:vis struct $name:ident($inner:ty);) => {
        $(#[$meta])*
        $vis struct $name {
            inner: $inner,
        }

        impl $name {
            /// Create a new instance. You should use this method carefully because the value is not checked as valid.
            pub fn new(inner: $inner) -> Self {
                Self { inner }
            }

            /// Access the internal value reference. You should use this method carefully.
            pub fn inner(&self) -> &$inner {
                &self.inner
            }
        }

        $crate::__impl_id_core_traits!([] $name, $inner);
        $crate::__impl_id_serde!([] $name, $inner);
        $crate::__impl_id_fake!([] $name, $inner);
        $crate::__impl_id_sqlx_any!([] $name, $inner);
        $crate::__impl_id_sqlx_mysql!([] $name, $inner);
        $crate::__impl_id_sqlx_postgres!([] $name, $inner);
        $crate::__impl_id_sqlx_sqlite!([] $name, $inner);
    };
}

// =============================================================================
// Core traits: Debug, PartialEq, Eq, Hash, Clone, From
// =============================================================================

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_id_core_traits {
    // Concrete type (no generics)
    ([] $name:ty, $inner:ty) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.inner().fmt(f)
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.inner().eq(other.inner())
            }
        }

        impl ::std::cmp::Eq for $name {}

        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.inner().hash(state)
            }
        }

        impl ::std::clone::Clone for $name {
            fn clone(&self) -> Self {
                Self::new(self.inner().clone())
            }
        }

        impl ::std::convert::From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self::new(value)
            }
        }
    };
    // Generic type (e.g. Id<T, U>)
    ([$($gen:tt)+] $name:ty, $inner:ty) => {
        impl<$($gen)+> ::std::fmt::Debug for $name where $inner: ::std::fmt::Debug {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.inner().fmt(f)
            }
        }

        impl<$($gen)+> ::std::cmp::PartialEq for $name where $inner: ::std::cmp::PartialEq {
            fn eq(&self, other: &Self) -> bool {
                self.inner().eq(other.inner())
            }
        }

        impl<$($gen)+> ::std::cmp::Eq for $name where $inner: ::std::cmp::Eq {}

        /// you can use as hash key if value implement [Hash].
        impl<$($gen)+> ::std::hash::Hash for $name where $inner: ::std::cmp::PartialEq + ::std::hash::Hash {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.inner().hash(state)
            }
        }

        /// you can clone if value implement [Clone].
        impl<$($gen)+> ::std::clone::Clone for $name where $inner: ::std::clone::Clone {
            fn clone(&self) -> Self {
                Self::new(self.inner().clone())
            }
        }

        impl<$($gen)+> ::std::convert::From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self::new(value)
            }
        }
    };
}

// =============================================================================
// serde
// =============================================================================

#[cfg(feature = "serde")]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_serde {
    // Concrete type
    ([] $name:ty, $inner:ty) => {
        impl $crate::__private::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__private::serde::Serializer,
            {
                <$inner as $crate::__private::serde::Serialize>::serialize(
                    self.inner(),
                    serializer,
                )
            }
        }

        impl<'de> $crate::__private::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::__private::serde::Deserializer<'de>,
            {
                let inner =
                    <$inner as $crate::__private::serde::Deserialize>::deserialize(deserializer)?;
                Ok(Self::new(inner))
            }
        }
    };
    // Generic type
    ([$($gen:tt)+] $name:ty, $inner:ty) => {
        /// you can serialize if feature serde enabled and inner type implement [Serialize].
        impl<$($gen)+> $crate::__private::serde::Serialize for $name
        where
            $inner: $crate::__private::serde::Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__private::serde::Serializer,
            {
                <$inner as $crate::__private::serde::Serialize>::serialize(
                    self.inner(),
                    serializer,
                )
            }
        }

        /// you can deserialize if feature serde enabled and inner type implement [Deserialize].
        impl<'de, $($gen)+> $crate::__private::serde::Deserialize<'de> for $name
        where
            $inner: $crate::__private::serde::Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::__private::serde::Deserializer<'de>,
            {
                let inner =
                    <$inner as $crate::__private::serde::Deserialize>::deserialize(deserializer)?;
                Ok(Self::new(inner))
            }
        }
    };
}

#[cfg(not(feature = "serde"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_serde {
    ([$($gen:tt)*] $name:ty, $inner:ty) => {};
}

// =============================================================================
// fake
// =============================================================================

#[cfg(feature = "fake")]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_fake {
    // Concrete type
    ([] $name:ty, $inner:ty) => {
        impl $crate::__private::fake::Dummy<$crate::__private::fake::Faker> for $name {
            fn dummy_with_rng<R: $crate::__private::fake::RngExt + ?Sized>(
                config: &$crate::__private::fake::Faker,
                rng: &mut R,
            ) -> Self {
                let inner =
                    $crate::__private::fake::Fake::fake_with_rng::<$inner, R>(config, rng);
                Self::new(inner)
            }
        }
    };
    // Generic type
    ([$($gen:tt)+] $name:ty, $inner:ty) => {
        impl<$($gen)+> $crate::__private::fake::Dummy<$crate::__private::fake::Faker> for $name
        where
            $inner: $crate::__private::fake::Dummy<$crate::__private::fake::Faker>,
        {
            fn dummy_with_rng<R: $crate::__private::fake::RngExt + ?Sized>(
                config: &$crate::__private::fake::Faker,
                rng: &mut R,
            ) -> Self {
                let inner =
                    $crate::__private::fake::Fake::fake_with_rng::<$inner, R>(config, rng);
                Self::new(inner)
            }
        }
    };
}

#[cfg(not(feature = "fake"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_fake {
    ([$($gen:tt)*] $name:ty, $inner:ty) => {};
}

// =============================================================================
// sqlx-any
// =============================================================================

#[cfg(feature = "sqlx-any")]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_sqlx_any {
    // Concrete type
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
    // Generic type
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

#[cfg(not(feature = "sqlx-any"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_sqlx_any {
    ([$($gen:tt)*] $name:ty, $inner:ty) => {};
}

// =============================================================================
// sqlx-mysql
// =============================================================================

#[cfg(feature = "sqlx-mysql")]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_sqlx_mysql {
    // Concrete type
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
    // Generic type
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

#[cfg(not(feature = "sqlx-mysql"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_sqlx_mysql {
    ([$($gen:tt)*] $name:ty, $inner:ty) => {};
}

// =============================================================================
// sqlx-postgres
// =============================================================================

#[cfg(feature = "sqlx-postgres")]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_sqlx_postgres {
    // Concrete type
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
    // Generic type
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

#[cfg(not(feature = "sqlx-postgres"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_sqlx_postgres {
    ([$($gen:tt)*] $name:ty, $inner:ty) => {};
}

// =============================================================================
// sqlx-sqlite
// =============================================================================

#[cfg(feature = "sqlx-sqlite")]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_sqlx_sqlite {
    // Concrete type
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
    // Generic type
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

#[cfg(not(feature = "sqlx-sqlite"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_id_sqlx_sqlite {
    ([$($gen:tt)*] $name:ty, $inner:ty) => {};
}
