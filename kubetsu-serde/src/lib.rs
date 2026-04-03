#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod __private {
    pub use kubetsu;
    pub use serde;
}

/// Implement `serde::Serialize` and `serde::Deserialize` for a kubetsu ID type.
///
/// # Concrete form
///
/// ```rust
/// kubetsu::define_id!(pub struct UserId(i64););
/// kubetsu_serde::impl_serde!(UserId(i64));
///
/// let id = UserId::new(42);
/// let json = serde_json::to_string(&id).unwrap();
/// assert_eq!(json, "42");
/// ```
///
/// # Generic form
///
/// ```rust
/// kubetsu::define_id!(pub struct MyId<T, U>;);
/// kubetsu_serde::impl_serde!(MyId<T, U>);
///
/// struct User;
/// type UserId = MyId<User, i64>;
///
/// let id = UserId::new(42);
/// let json = serde_json::to_string(&id).unwrap();
/// assert_eq!(json, "42");
/// ```
#[macro_export]
macro_rules! impl_serde {
    // Concrete form: impl_serde!(UserId(i64));
    ($name:ident($inner:ty)) => {
        const _: () = {
            fn _assert_kubetsu_id()
            where
                $name: $crate::__private::kubetsu::KubetsuId<Inner = $inner>,
            {
            }
        };

        impl $crate::__private::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__private::serde::Serializer,
            {
                <$inner as $crate::__private::serde::Serialize>::serialize(self.inner(), serializer)
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
    // Generic form: impl_serde!(MyId<T, U>);
    ($name:ident<$phantom:ident, $inner:ident>) => {
        const _: () = {
            fn _assert_kubetsu_id<$phantom, $inner>()
            where
                $name<$phantom, $inner>: $crate::__private::kubetsu::KubetsuId<Inner = $inner>,
            {
            }
        };

        impl<$phantom, $inner> $crate::__private::serde::Serialize for $name<$phantom, $inner>
        where
            $inner: $crate::__private::serde::Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__private::serde::Serializer,
            {
                <$inner as $crate::__private::serde::Serialize>::serialize(self.inner(), serializer)
            }
        }

        impl<'de, $phantom, $inner> $crate::__private::serde::Deserialize<'de>
            for $name<$phantom, $inner>
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

#[cfg(test)]
mod tests {
    kubetsu::define_id!(
        pub struct UserId(i64);
    );
    kubetsu::define_id!(
        pub struct ItemId(String);
    );
    crate::impl_serde!(UserId(i64));
    crate::impl_serde!(ItemId(String));

    kubetsu::define_id!(
        pub struct MyId<T, U>;
    );
    crate::impl_serde!(MyId<T, U>);

    struct User;
    type MyUserId = MyId<User, i64>;

    #[test]
    fn test_serialize_concrete() {
        let id = UserId::new(42);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "42");
    }

    #[test]
    fn test_deserialize_concrete() {
        let id: UserId = serde_json::from_str("42").unwrap();
        assert_eq!(*id.inner(), 42);
    }

    #[test]
    fn test_serialize_string() {
        let id = ItemId::new("abc".to_string());
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"abc\"");
    }

    #[test]
    fn test_deserialize_string() {
        let id: ItemId = serde_json::from_str("\"abc\"").unwrap();
        assert_eq!(id.inner(), "abc");
    }

    #[test]
    fn test_serialize_generic() {
        let id = MyUserId::new(42);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "42");
    }

    #[test]
    fn test_deserialize_generic() {
        let id: MyUserId = serde_json::from_str("42").unwrap();
        assert_eq!(*id.inner(), 42);
    }
}
