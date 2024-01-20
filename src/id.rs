use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

#[cfg(test)]
mod test;

#[cfg(feature = "fake")]
use fake::{Dummy, Fake, Faker, Rng};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
#[cfg(feature = "sqlx")]
use sqlx::database::{HasArguments, HasValueRef};
#[cfg(feature = "sqlx")]
use sqlx::encode::IsNull;
#[cfg(feature = "sqlx")]
use sqlx::error::BoxDynError;
#[cfg(feature = "sqlx-mysql")]
use sqlx::mysql::MySqlTypeInfo;
#[cfg(feature = "sqlx-mysql")]
use sqlx::MySql;
#[cfg(feature = "sqlx")]
use sqlx::{Decode, Encode, Type};
#[cfg(feature = "sqlx-sqlite")]
use sqlx::Sqlite;
#[cfg(feature = "sqlx-sqlite")]
use sqlx::sqlite::SqliteTypeInfo;

pub struct Id<T, U> {
    inner: U,
    _phantom: PhantomData<T>,
}

impl<T, U> Id<T, U> {
    pub fn new(inner: U) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    pub fn inner(&self) -> &U {
        &self.inner
    }
}

impl<T, U: Debug> Debug for Id<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner().fmt(f)
    }
}

impl<T, U: PartialEq> PartialEq for Id<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<T, U: Eq> Eq for Id<T, U> {}

impl<T, U> From<U> for Id<T, U> {
    fn from(value: U) -> Self {
        Self::new(value)
    }
}

impl<T, U: Clone> Clone for Id<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, i128> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_i128(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, u128> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u128(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, i64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, u64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u64(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, i32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, u32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u32(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, i16> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_i16(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, u16> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u16(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, i8> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_i8(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, u8> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u8(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, f64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_f64(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, f32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_f32(self.inner().clone())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Id<T, String> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.inner())
    }
}

#[cfg(feature = "sqlx-mysql")]
impl<T, U: sqlx::Type<sqlx::MySql>> Type<MySql> for Id<T, U> {
    fn type_info() -> MySqlTypeInfo {
        <U as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <U as Type<MySql>>::compatible(ty)
    }
}

#[cfg(feature = "sqlx-sqlite")]
impl<T, U: sqlx::Type<sqlx::Sqlite>> Type<Sqlite> for Id<T, U> {
    fn type_info() -> SqliteTypeInfo {
        <U as Type<Sqlite>>::type_info()
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        <U as Type<Sqlite>>::compatible(ty)
    }
}

#[cfg(feature = "sqlx-mysql")]
impl<T, U: Clone + for<'a> sqlx::Encode<'a, sqlx::MySql>> Encode<'_, MySql> for Id<T, U> {
    fn encode_by_ref(&self, buf: &mut <MySql as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        <U as Encode<MySql>>::encode(self.inner().clone(), buf)
    }
}

#[cfg(feature = "sqlx-mysql")]
impl<T, U: for<'a> sqlx::Decode<'a, sqlx::MySql>> Decode<'_, MySql> for Id<T, U> {
    fn decode(value: <MySql as HasValueRef<'_>>::ValueRef) -> Result<Self, BoxDynError> {
        let val = <U as Decode<MySql>>::decode(value)?;
        Ok(Self::new(val))
    }
}

#[cfg(feature = "sqlx-sqlite")]
impl<T, U: for<'a> sqlx::Decode<'a, sqlx::Sqlite>> Decode<'_, Sqlite> for Id<T, U> {
    fn decode(value: <Sqlite as HasValueRef<'_>>::ValueRef) -> Result<Self, BoxDynError> {
        let val = <U as Decode<Sqlite>>::decode(value)?;
        Ok(Self::new(val))
    }
}

#[cfg(feature = "fake")]
impl<T, U: fake::Dummy<fake::Faker>> Dummy<Faker> for Id<T, U> {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let inner = Fake::fake_with_rng::<U, R>(config, rng);
        Self::new(inner)
    }
}
