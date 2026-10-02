#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod __private {
    pub use kubetsu;
    pub use libsql;
}

/// Implement `From<Id>` and `From<&Id>` for `libsql::Value` for a kubetsu ID
/// type, so the ID can be bound as a query parameter.
///
/// libsql binds anything that converts into `Value`: its `IntoValue` trait has
/// a blanket implementation over `TryInto<Value>`, and `Option<T>` converts
/// whenever `T` does. Two `From` implementations are therefore enough to make
/// an ID usable in `params!`, `named_params!`, tuples, arrays, `Vec`, and as
/// `Option<Id>` for a nullable column.
///
/// # Concrete form
///
/// ```rust
/// kubetsu::define_id!(pub struct UserId(i64););
/// kubetsu_libsql::impl_libsql!(UserId(i64));
///
/// let id = UserId::new(42);
/// assert_eq!(libsql::Value::from(&id), libsql::Value::Integer(42));
/// assert_eq!(libsql::Value::from(id), libsql::Value::Integer(42));
/// ```
///
/// # Generic form
///
/// ```rust
/// kubetsu::define_id!(pub struct MyId<T, U>;);
/// kubetsu_libsql::impl_libsql!(MyId<T, U>);
///
/// struct User;
/// type UserId = MyId<User, i64>;
///
/// let id = UserId::new(42);
/// assert_eq!(libsql::Value::from(id), libsql::Value::Integer(42));
/// ```
///
/// # The inner type must convert into `libsql::Value`
///
/// The generic form bounds its implementations on `U: Into<Value>`, so an
/// inner type libsql does not know simply gets no implementation. The concrete
/// form has nothing to make the implementation conditional on, so it fails to
/// compile instead. libsql converts the sized integers (`u64` only through
/// `TryFrom`, so it does not qualify), floats, `bool`, `String`, `&str`, byte
/// slices and `Vec<u8>`. A `Uuid` inner, for example, is not among them:
///
/// ```rust,compile_fail
/// kubetsu::define_id!(pub struct EventId(uuid::Uuid););
/// kubetsu_libsql::impl_libsql!(EventId(uuid::Uuid));
/// ```
///
/// # Reading back
///
/// This macro covers the write side only. libsql's `FromValue` is sealed, so
/// `row.get::<UserId>(0)` cannot be made to work from outside libsql. Read the
/// inner type and wrap it with `UserId::new`, or deserialize whole rows through
/// libsql's `serde` feature together with `kubetsu-serde`.
#[macro_export]
macro_rules! impl_libsql {
    // Concrete form: impl_libsql!(UserId(i64));
    ($name:ident($inner:ty)) => {
        const _: () = {
            fn _assert_kubetsu_id()
            where
                $name: $crate::__private::kubetsu::KubetsuId<Inner = $inner>,
            {
            }
        };

        impl ::core::convert::From<$name> for $crate::__private::libsql::Value {
            fn from(value: $name) -> Self {
                <$inner as ::core::convert::Into<$crate::__private::libsql::Value>>::into(
                    ::core::clone::Clone::clone(value.inner()),
                )
            }
        }

        impl ::core::convert::From<&$name> for $crate::__private::libsql::Value {
            fn from(value: &$name) -> Self {
                <$inner as ::core::convert::Into<$crate::__private::libsql::Value>>::into(
                    ::core::clone::Clone::clone(value.inner()),
                )
            }
        }
    };
    // Generic form: impl_libsql!(MyId<T, U>);
    ($name:ident<$phantom:ident, $inner:ident>) => {
        const _: () = {
            fn _assert_kubetsu_id<$phantom, $inner>()
            where
                $name<$phantom, $inner>: $crate::__private::kubetsu::KubetsuId<Inner = $inner>,
            {
            }
        };

        impl<$phantom, $inner> ::core::convert::From<$name<$phantom, $inner>>
            for $crate::__private::libsql::Value
        where
            $inner: ::core::convert::Into<$crate::__private::libsql::Value> + ::core::clone::Clone,
        {
            fn from(value: $name<$phantom, $inner>) -> Self {
                <$inner as ::core::convert::Into<$crate::__private::libsql::Value>>::into(
                    ::core::clone::Clone::clone(value.inner()),
                )
            }
        }

        impl<$phantom, $inner> ::core::convert::From<&$name<$phantom, $inner>>
            for $crate::__private::libsql::Value
        where
            $inner: ::core::convert::Into<$crate::__private::libsql::Value> + ::core::clone::Clone,
        {
            fn from(value: &$name<$phantom, $inner>) -> Self {
                <$inner as ::core::convert::Into<$crate::__private::libsql::Value>>::into(
                    ::core::clone::Clone::clone(value.inner()),
                )
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use libsql::Value;
    use libsql::params::{IntoParams, IntoValue};

    kubetsu::define_id!(
        pub struct UserId(i64);
    );
    kubetsu::define_id!(
        pub struct ItemId(String);
    );
    crate::impl_libsql!(UserId(i64));
    crate::impl_libsql!(ItemId(String));

    kubetsu::define_id!(
        pub struct MyId<T, U>;
    );
    crate::impl_libsql!(MyId<T, U>);

    struct User;
    type MyUserId = MyId<User, i64>;
    struct Item;
    type MyItemId = MyId<Item, String>;

    #[test]
    fn test_value_from_concrete() {
        assert_eq!(Value::from(UserId::new(42)), Value::Integer(42));
        assert_eq!(
            Value::from(ItemId::new("abc".to_string())),
            Value::Text("abc".to_string())
        );
    }

    #[test]
    fn test_value_from_generic() {
        assert_eq!(Value::from(MyUserId::new(42)), Value::Integer(42));
        assert_eq!(
            Value::from(MyItemId::new("abc".to_string())),
            Value::Text("abc".to_string())
        );
    }

    #[test]
    fn test_value_from_reference() {
        let id = ItemId::new("abc".to_string());
        assert_eq!(Value::from(&id), Value::Text("abc".to_string()));
        // The reference form leaves the ID usable afterwards.
        assert_eq!(id.inner(), "abc");

        let id = MyItemId::new("abc".to_string());
        assert_eq!(Value::from(&id), Value::Text("abc".to_string()));
        assert_eq!(id.inner(), "abc");
    }

    #[test]
    fn test_into_value() {
        // `IntoValue` is what `params!` and the tuple/array `IntoParams`
        // implementations go through, so this is the bound that matters.
        assert_eq!(UserId::new(1).into_value().unwrap(), Value::Integer(1));
        assert_eq!(MyUserId::new(1).into_value().unwrap(), Value::Integer(1));
    }

    #[test]
    fn test_option_for_nullable_column() {
        assert_eq!(Value::from(Some(UserId::new(1))), Value::Integer(1));
        assert_eq!(Value::from(None::<UserId>), Value::Null);
        assert_eq!(Value::from(Some(MyUserId::new(1))), Value::Integer(1));
        assert_eq!(Value::from(None::<MyUserId>), Value::Null);
    }

    #[test]
    fn test_into_params_shapes() {
        // Positional: tuple, array, Vec, and the `params!` macro.
        let _ = (UserId::new(1), MyUserId::new(2)).into_params().unwrap();
        let _ = [UserId::new(1), UserId::new(2)].into_params().unwrap();
        let _ = vec![UserId::new(1), UserId::new(2)].into_params().unwrap();
        let _ = libsql::params![UserId::new(1), Some(MyUserId::new(2)), None::<UserId>]
            .into_params()
            .unwrap();
        // Named.
        let _ = libsql::named_params![":id": UserId::new(1)]
            .into_params()
            .unwrap();
    }

    #[tokio::test]
    async fn test_round_trip_through_a_connection() {
        let db = libsql::Builder::new_local(":memory:")
            .build()
            .await
            .unwrap();
        let conn = db.connect().unwrap();
        conn.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, parent_id INTEGER, item_id TEXT)",
            (),
        )
        .await
        .unwrap();

        let id = UserId::new(42);
        let item_id = MyItemId::new("abc".to_string());
        conn.execute(
            "INSERT INTO users (id, parent_id, item_id) VALUES (?1, ?2, ?3)",
            libsql::params![&id, None::<UserId>, &item_id],
        )
        .await
        .unwrap();

        let mut rows = conn
            .query(
                "SELECT id, parent_id, item_id FROM users WHERE id = ?1",
                [&id],
            )
            .await
            .unwrap();
        let row = rows.next().await.unwrap().unwrap();

        // Read side: libsql's `FromValue` is sealed, so go through the inner
        // type and wrap it.
        assert_eq!(UserId::new(row.get::<i64>(0).unwrap()), id);
        assert_eq!(row.get::<Option<i64>>(1).unwrap(), None);
        assert_eq!(MyItemId::new(row.get::<String>(2).unwrap()), item_id);
    }
}
