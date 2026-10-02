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
/// inner type libsql does not know simply gets no implementation and the ID
/// stays usable for everything else:
///
/// ```rust
/// kubetsu::define_id!(pub struct MyId<T, U>;);
/// kubetsu_libsql::impl_libsql!(MyId<T, U>);
///
/// struct Event;
/// type EventId = MyId<Event, uuid::Uuid>;
///
/// // Compiles; there is just no `From<EventId> for libsql::Value`.
/// let _ = EventId::new(uuid::Uuid::nil());
/// ```
///
/// The concrete form has nothing to make the implementation conditional on,
/// so it fails to compile instead. (This is a smoke test: `compile_fail`
/// accepts any error, and the intended one is `Value: From<Uuid>` not
/// satisfied.)
///
/// ```rust,compile_fail
/// kubetsu::define_id!(pub struct EventId(uuid::Uuid););
/// kubetsu_libsql::impl_libsql!(EventId(uuid::Uuid));
/// ```
///
/// libsql converts `i8` through `i64`, `u8` through `u32`, `f32`, `f64`,
/// `bool`, `String`, `&str`, `&[u8]` and `Vec<u8>` through `From`.
///
/// # `u64` inner types
///
/// `u64` is the one common ID shape that does not qualify: libsql converts it
/// only through `TryFrom`, because a value above `i64::MAX` does not fit an
/// SQLite integer. This macro builds on `From` rather than `TryFrom` so that
/// `Option<Id>` keeps converting (libsql's `From<Option<T>>` needs
/// `T: Into<Value>`), which leaves `u64` out of both forms. Prefer `i64`,
/// which is what SQLite stores anyway. If the inner type has to stay `u64`,
/// bind the inner value at the call site and let libsql's own range check
/// run:
///
/// ```rust
/// kubetsu::define_id!(pub struct BigId(u64););
///
/// let id = BigId::new(42);
/// let value = libsql::Value::try_from(*id.inner()).unwrap();
/// assert_eq!(value, libsql::Value::Integer(42));
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
                    value.into_inner(),
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
            $inner: ::core::convert::Into<$crate::__private::libsql::Value>,
        {
            fn from(value: $name<$phantom, $inner>) -> Self {
                <$inner as ::core::convert::Into<$crate::__private::libsql::Value>>::into(
                    value.into_inner(),
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
    fn test_u64_inner_binds_through_try_from_at_the_call_site() {
        // `u64` gets no `From` impl from the macro (see the docs), so the
        // documented workaround is to convert the inner value directly and
        // let libsql's own range check run. Pin that it works for both forms
        // and that the range check is actually reached.
        kubetsu::define_id!(
            pub struct BigId(u64);
        );
        struct Big;
        type MyBigId = MyId<Big, u64>;

        let id = BigId::new(42);
        assert_eq!(Value::try_from(*id.inner()).unwrap(), Value::Integer(42));
        let id = MyBigId::new(42);
        assert_eq!(Value::try_from(*id.inner()).unwrap(), Value::Integer(42));

        let id = BigId::new(u64::MAX);
        assert!(matches!(
            Value::try_from(*id.inner()),
            Err(libsql::Error::ToSqlConversionFailure(_))
        ));
    }

    #[test]
    fn test_owned_conversion_needs_no_clone() {
        // The by-value conversion moves the inner value out, so it has no
        // `Clone` bound; only the by-reference one clones. This inner type is
        // neither `Clone` nor `Debug`, so the test fails to compile if a bound
        // returns to the owned generic impl.
        struct Opaque(i64);
        impl From<Opaque> for Value {
            fn from(v: Opaque) -> Value {
                Value::Integer(v.0)
            }
        }
        struct Tag;

        let id: MyId<Tag, Opaque> = MyId::new(Opaque(7));
        assert_eq!(Value::from(id), Value::Integer(7));
    }

    #[test]
    fn test_owned_conversion_moves_the_allocation() {
        // A blob inner converts without being copied.
        struct Tag;
        let bytes = vec![1u8, 2, 3];
        let ptr = bytes.as_ptr();
        let Value::Blob(out) = Value::from(MyId::<Tag, Vec<u8>>::new(bytes)) else {
            panic!("expected a blob");
        };
        assert_eq!(out.as_ptr(), ptr);
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

    #[test]
    fn test_coexists_with_the_other_adapters_on_one_type() {
        // Not in kubetsu-tests, which links sqlx's SQLite and cannot also
        // link libsql's (see the note there). Stack the adapters that can
        // share a binary and check they agree on the inner value.
        use fake::{Fake, Faker};

        kubetsu::define_id!(
            pub struct OrderId(i64);
        );
        kubetsu_serde::impl_serde!(OrderId(i64));
        kubetsu_fake::impl_fake!(OrderId(i64));
        crate::impl_libsql!(OrderId(i64));

        kubetsu::define_id!(
            pub struct AnyId<T, U>;
        );
        kubetsu_serde::impl_serde!(AnyId<T, U>);
        kubetsu_fake::impl_fake!(AnyId<T, U>);
        crate::impl_libsql!(AnyId<T, U>);
        struct Order;
        type MyOrderId = AnyId<Order, i64>;

        let id: OrderId = serde_json::from_str("42").unwrap();
        assert_eq!(Value::from(&id), Value::Integer(42));
        let faked: OrderId = Faker.fake();
        assert_eq!(Value::from(&faked), Value::Integer(*faked.inner()));

        let id: MyOrderId = serde_json::from_str("42").unwrap();
        assert_eq!(Value::from(&id), Value::Integer(42));
        let faked: MyOrderId = Faker.fake();
        assert_eq!(Value::from(&faked), Value::Integer(*faked.inner()));
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
