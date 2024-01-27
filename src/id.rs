use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[cfg(test)]
mod test;

#[cfg(feature = "fake")]
use fake::{Dummy, Fake, Faker, Rng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "sqlx-any")]
use sqlx::any::AnyTypeInfo;
#[cfg(feature = "sqlx")]
use sqlx::database::{HasArguments, HasValueRef};
#[cfg(feature = "sqlx")]
use sqlx::encode::IsNull;
#[cfg(feature = "sqlx")]
use sqlx::error::BoxDynError;
#[cfg(feature = "sqlx-mysql")]
use sqlx::mysql::MySqlTypeInfo;
#[cfg(feature = "sqlx-sqlite")]
use sqlx::sqlite::SqliteTypeInfo;
#[cfg(feature = "sqlx-any")]
use sqlx::Any;
#[cfg(feature = "sqlx-mysql")]
use sqlx::MySql;
#[cfg(feature = "sqlx-postgres")]
use sqlx::Postgres;
#[cfg(feature = "sqlx-sqlite")]
use sqlx::Sqlite;
#[cfg(feature = "sqlx")]
use sqlx::{Decode, Encode, Type};

pub struct Id<T, U> {
    inner: U,
    _phantom: PhantomData<T>,
}

impl<T, U> Id<T, U> {
    /// create Id object. you should use this method carefully because value was not checked as valid
    pub fn new(inner: U) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    /// access to internal value reference. you should use this method carefully.
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

/// you can use `Id` as hash key if value implement [Hash].
impl<T, U: PartialEq + Hash> Hash for Id<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl<T, U: Eq> Eq for Id<T, U> {}

impl<T, U> From<U> for Id<T, U> {
    fn from(value: U) -> Self {
        Self::new(value)
    }
}

/// you can clone if value implement [Clone].
impl<T, U: Clone> Clone for Id<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.inner().clone())
    }
}

/// you can serialize if feature serde enabled and U implement [Serialize].
#[cfg(feature = "serde")]
impl<T, U: Serialize> Serialize for Id<T, U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        U::serialize(self.inner(), serializer)
    }
}

/// you can deserialize if feature serde enabled and U implement [Deserialize].
#[cfg(feature = "serde")]
impl<'de, T, U: Deserialize<'de>> Deserialize<'de> for Id<T, U> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let result = U::deserialize(deserializer);
        match result {
            Ok(inner) => Ok(Self::new(inner)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(feature = "sqlx-any")]
impl<T, U: sqlx::Type<sqlx::Any>> Type<Any> for Id<T, U> {
    fn type_info() -> AnyTypeInfo {
        <U as Type<Any>>::type_info()
    }

    fn compatible(ty: &AnyTypeInfo) -> bool {
        <U as Type<Any>>::compatible(ty)
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

#[cfg(feature = "sqlx-postgres")]
impl<T, U: sqlx::Type<sqlx::Postgres>> Type<Postgres> for Id<T, U> {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <U as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        <U as Type<Postgres>>::compatible(ty)
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

#[cfg(feature = "sqlx-any")]
impl<T, U: for<'a> sqlx::Encode<'a, sqlx::Any>> Encode<'_, Any> for Id<T, U> {
    fn encode_by_ref(&self, buf: &mut <Any as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        <U as Encode<Any>>::encode_by_ref(self.inner(), buf)
    }
}

#[cfg(feature = "sqlx-mysql")]
impl<T, U: for<'a> sqlx::Encode<'a, sqlx::MySql>> Encode<'_, MySql> for Id<T, U> {
    fn encode_by_ref(&self, buf: &mut <MySql as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        <U as Encode<MySql>>::encode_by_ref(self.inner(), buf)
    }
}

#[cfg(feature = "sqlx-postgres")]
impl<T, U: for<'a> sqlx::Encode<'a, sqlx::Postgres>> Encode<'_, Postgres> for Id<T, U> {
    fn encode_by_ref(&self, buf: &mut <Postgres as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        <U as Encode<Postgres>>::encode_by_ref(self.inner(), buf)
    }
}

#[cfg(feature = "sqlx-sqlite")]
impl<T, U: for<'a> sqlx::Encode<'a, sqlx::Sqlite>> Encode<'_, Sqlite> for Id<T, U> {
    fn encode_by_ref(&self, buf: &mut <Sqlite as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        <U as Encode<Sqlite>>::encode_by_ref(self.inner(), buf)
    }
}

#[cfg(feature = "sqlx-any")]
impl<T, U: for<'a> sqlx::Decode<'a, sqlx::Any>> Decode<'_, Any> for Id<T, U> {
    fn decode(value: <Any as HasValueRef<'_>>::ValueRef) -> Result<Self, BoxDynError> {
        let val = <U as Decode<Any>>::decode(value)?;
        Ok(Self::new(val))
    }
}

#[cfg(feature = "sqlx-mysql")]
impl<T, U: for<'a> sqlx::Decode<'a, sqlx::MySql>> Decode<'_, MySql> for Id<T, U> {
    fn decode(value: <MySql as HasValueRef<'_>>::ValueRef) -> Result<Self, BoxDynError> {
        let val = <U as Decode<MySql>>::decode(value)?;
        Ok(Self::new(val))
    }
}

#[cfg(feature = "sqlx-postgres")]
impl<T, U: for<'a> sqlx::Decode<'a, sqlx::Postgres>> Decode<'_, Postgres> for Id<T, U> {
    fn decode(value: <Postgres as HasValueRef<'_>>::ValueRef) -> Result<Self, BoxDynError> {
        let val = <U as Decode<Postgres>>::decode(value)?;
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
