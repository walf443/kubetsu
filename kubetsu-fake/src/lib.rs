#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod __private {
    pub use fake;
    pub use kubetsu;
}

/// Implement `fake::Dummy` for a kubetsu ID type.
///
/// The implementation is generic over fake's config type and forwards to the
/// inner type, so an ID accepts every config the inner type does -- not just
/// `Faker`. That is what lets a field carry `#[dummy(faker = "...")]`:
///
/// ```rust
/// use fake::{Dummy, Fake, Faker};
///
/// kubetsu::define_id!(pub struct UserId(i64););
/// kubetsu_fake::impl_fake!(UserId(i64));
///
/// #[derive(Dummy)]
/// struct User {
///     #[dummy(faker = "1000..2000")]
///     id: UserId,
/// }
///
/// let user: User = Faker.fake();
/// assert!((1000..2000).contains(user.id.inner()));
/// ```
///
/// Because the implementation covers every config, a hand-written
/// `Dummy<SomeConfig>` for the same ID type collides with `error[E0119]`.
///
/// # Concrete form
///
/// ```rust
/// kubetsu::define_id!(pub struct UserId(i64););
/// kubetsu_fake::impl_fake!(UserId(i64));
///
/// use fake::{Fake, Faker};
/// let _id: UserId = Faker.fake();
/// ```
///
/// # Generic form
///
/// ```rust
/// kubetsu::define_id!(pub struct MyId<T, U>;);
/// kubetsu_fake::impl_fake!(MyId<T, U>);
///
/// struct User;
/// type UserId = MyId<User, i64>;
///
/// use fake::{Fake, Faker};
/// let _id: UserId = Faker.fake();
/// ```
#[macro_export]
macro_rules! impl_fake {
    // Concrete form: impl_fake!(UserId(i64));
    ($name:ident($inner:ty)) => {
        const _: () = {
            fn _assert_kubetsu_id()
            where
                $name: $crate::__private::kubetsu::KubetsuId<Inner = $inner>,
            {
            }
        };

        impl<__C> $crate::__private::fake::Dummy<__C> for $name
        where
            $inner: $crate::__private::fake::Dummy<__C>,
        {
            fn dummy_with_rng<R: $crate::__private::fake::RngExt + ?Sized>(
                config: &__C,
                rng: &mut R,
            ) -> Self {
                let inner = $crate::__private::fake::Fake::fake_with_rng::<$inner, R>(config, rng);
                Self::new(inner)
            }
        }
    };
    // Generic form: impl_fake!(MyId<T, U>);
    ($name:ident<$phantom:ident, $inner:ident>) => {
        const _: () = {
            fn _assert_kubetsu_id<$phantom, $inner>()
            where
                $name<$phantom, $inner>: $crate::__private::kubetsu::KubetsuId<Inner = $inner>,
            {
            }
        };

        impl<$phantom, $inner, __C> $crate::__private::fake::Dummy<__C> for $name<$phantom, $inner>
        where
            $inner: $crate::__private::fake::Dummy<__C>,
        {
            fn dummy_with_rng<R: $crate::__private::fake::RngExt + ?Sized>(
                config: &__C,
                rng: &mut R,
            ) -> Self {
                let inner = $crate::__private::fake::Fake::fake_with_rng::<$inner, R>(config, rng);
                Self::new(inner)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    kubetsu::define_id!(
        pub struct UserId(i64);
    );
    kubetsu::define_id!(
        pub struct ItemId(String);
    );
    crate::impl_fake!(UserId(i64));
    crate::impl_fake!(ItemId(String));

    kubetsu::define_id!(
        pub struct MyId<T, U>;
    );
    crate::impl_fake!(MyId<T, U>);

    struct User;
    type MyUserId = MyId<User, i64>;

    #[test]
    fn test_fake_concrete() {
        let _id: UserId = Faker.fake();
    }

    #[test]
    fn test_fake_string() {
        let _id: ItemId = Faker.fake();
    }

    #[test]
    fn test_fake_generic() {
        let _id: MyUserId = Faker.fake();
    }

    #[test]
    fn test_forwards_config_concrete() {
        // Not just `Faker`: any config the inner type accepts reaches the ID.
        let id: UserId = (1000..2000).fake();
        assert!((1000..2000).contains(id.inner()));
    }

    #[test]
    fn test_forwards_config_generic() {
        let id: MyUserId = (1000..2000).fake();
        assert!((1000..2000).contains(id.inner()));
    }

    #[test]
    fn test_forwards_config_through_derive() {
        use fake::Dummy;

        #[derive(Dummy)]
        struct User {
            #[dummy(faker = "1000..2000")]
            id: UserId,
            #[dummy(faker = "1000..2000")]
            generic_id: MyUserId,
        }

        let user: User = Faker.fake();
        assert!((1000..2000).contains(user.id.inner()));
        assert!((1000..2000).contains(user.generic_id.inner()));
    }
}
