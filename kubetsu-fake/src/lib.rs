#[doc(hidden)]
pub mod __private {
    pub use fake;
    pub use kubetsu;
}

/// Implement `fake::Dummy<fake::Faker>` for a kubetsu ID type.
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

        impl $crate::__private::fake::Dummy<$crate::__private::fake::Faker> for $name {
            fn dummy_with_rng<R: $crate::__private::fake::RngExt + ?Sized>(
                config: &$crate::__private::fake::Faker,
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

        impl<$phantom, $inner> $crate::__private::fake::Dummy<$crate::__private::fake::Faker>
            for $name<$phantom, $inner>
        where
            $inner: $crate::__private::fake::Dummy<$crate::__private::fake::Faker>,
        {
            fn dummy_with_rng<R: $crate::__private::fake::RngExt + ?Sized>(
                config: &$crate::__private::fake::Faker,
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
}
