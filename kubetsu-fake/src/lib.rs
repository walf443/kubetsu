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
/// Because the implementation applies to whatever config the inner type
/// accepts, a hand-written `Dummy<SomeConfig>` for the same ID type collides
/// with `error[E0119]`. A config from another crate conflicts even when the
/// inner type does not implement it, since rustc must assume a future release
/// could add the impl:
///
/// ```rust,compile_fail
/// use fake::{Dummy, RngExt};
///
/// kubetsu::define_id!(pub struct UserId(i64););
/// kubetsu_fake::impl_fake!(UserId(i64));
///
/// // `i64: Dummy<Range<i32>>` does not exist, but `Range` is not ours to
/// // reason about, so this still conflicts.
/// impl Dummy<std::ops::Range<i32>> for UserId {
///     fn dummy_with_rng<R: RngExt + ?Sized>(_: &std::ops::Range<i32>, _: &mut R) -> Self {
///         Self::new(1)
///     }
/// }
/// ```
///
/// The one case that compiles is a config declared in your own crate for which
/// the inner type has no `Dummy` -- the orphan rule stops other crates adding
/// one, though it does not stop you. That is where logic of your own belongs:
///
/// ```rust
/// use fake::{Dummy, Fake, RngExt};
///
/// kubetsu::define_id!(pub struct UserId(i64););
/// kubetsu_fake::impl_fake!(UserId(i64));
///
/// pub struct FirstUser;
/// impl Dummy<FirstUser> for UserId {
///     fn dummy_with_rng<R: RngExt + ?Sized>(_: &FirstUser, _: &mut R) -> Self {
///         Self::new(1)
///     }
/// }
///
/// let id: UserId = FirstUser.fake();
/// assert_eq!(*id.inner(), 1);
/// ```
///
/// Implementing `Dummy<YourConfig>` for the inner type as well brings the
/// conflict back, since the macro's implementation then applies too:
///
/// ```rust,compile_fail
/// use fake::{Dummy, RngExt};
///
/// kubetsu::define_id!(pub struct UserId(i64););
/// kubetsu_fake::impl_fake!(UserId(i64));
///
/// pub struct FirstUser;
/// impl Dummy<FirstUser> for i64 {
///     fn dummy_with_rng<R: RngExt + ?Sized>(_: &FirstUser, _: &mut R) -> Self { 1 }
/// }
/// impl Dummy<FirstUser> for UserId {
///     fn dummy_with_rng<R: RngExt + ?Sized>(_: &FirstUser, _: &mut R) -> Self {
///         Self::new(1)
///     }
/// }
/// ```
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
    struct Item;
    type MyItemId = MyId<Item, String>;

    /// Characters an item ID is made of, so a generated value looks like an
    /// identifier rather than arbitrary text.
    const ITEM_CHARSET: &[u8] = b"ABCDEF0123456789";

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
    fn test_forwards_config_and_rng_faithfully() {
        use fake::rand::SeedableRng;
        use fake::rand::rngs::StdRng;

        // The sharpest check that forwarding is faithful rather than merely
        // compiling: with the same seed and config, the ID must hold exactly
        // what the inner type would have produced on its own.
        let mut rng = StdRng::seed_from_u64(42);
        let id: UserId = (1000..2000).fake_with_rng(&mut rng);

        let mut same_rng = StdRng::seed_from_u64(42);
        let inner: i64 = (1000..2000).fake_with_rng(&mut same_rng);

        assert_eq!(*id.inner(), inner);

        let mut rng = StdRng::seed_from_u64(7);
        let generic: MyUserId = (1000..2000).fake_with_rng(&mut rng);

        let mut same_rng = StdRng::seed_from_u64(7);
        let inner: i64 = (1000..2000).fake_with_rng(&mut same_rng);

        assert_eq!(*generic.inner(), inner);
    }

    #[test]
    fn test_forwards_a_named_faker() {
        use fake::faker::internet::en::Username;

        let id: MyItemId = Username().fake();
        assert!(!id.inner().is_empty());
    }

    #[test]
    fn test_forwards_a_configured_faker() {
        use fake::StringFaker;

        // A charset-and-length config, so the assertion can check that the
        // config was honoured rather than just that something was produced.
        let faker = StringFaker::with(ITEM_CHARSET.to_vec(), 8..9);

        let id: MyItemId = faker.fake();
        assert_eq!(id.inner().len(), 8);
        assert!(id.inner().bytes().all(|b| ITEM_CHARSET.contains(&b)));
    }

    #[test]
    fn test_collection_of_ids() {
        let ids: Vec<UserId> = fake::vec![UserId; 3];
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn test_forwards_config_through_derive() {
        use fake::Dummy;
        use fake::StringFaker;

        // Three config shapes on one struct -- a std range, a configured
        // faker, and a field left on `Faker` -- across both macro forms.
        #[derive(Dummy)]
        struct Order {
            #[dummy(faker = "1000..2000")]
            user_id: UserId,
            #[dummy(faker = "1000..2000")]
            generic_user_id: MyUserId,
            #[dummy(faker = "StringFaker::with(ITEM_CHARSET.to_vec(), 8..9)")]
            item_id: MyItemId,
            default_id: UserId,
        }

        let order: Order = Faker.fake();
        assert!((1000..2000).contains(order.user_id.inner()));
        assert!((1000..2000).contains(order.generic_user_id.inner()));
        assert_eq!(order.item_id.inner().len(), 8);
        let _ = order.default_id;
    }
}
