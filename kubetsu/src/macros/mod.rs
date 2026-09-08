#[cfg(test)]
mod test;

/// Define a custom ID type with the same capabilities as `kubetsu::Id`.
///
/// This macro generates a struct that wraps an inner value type,
/// along with trait implementations for common operations.
///
/// # Generic form
///
/// Generates a generic type with `PhantomData`, equivalent to `kubetsu::Id<T, U>`.
/// The first type parameter is the phantom type tag, the second is the inner value type.
///
/// ```rust
/// kubetsu::define_id!(pub struct MyId<T, U>;);
///
/// struct User;
/// struct Item;
/// type UserId = MyId<User, i64>;
/// type ItemId = MyId<Item, i64>;
///
/// let user_id = UserId::new(42);
/// assert_eq!(*user_id.inner(), 42);
/// ```
///
/// # Concrete form
///
/// Generates a standalone type with a fixed inner type.
///
/// ```rust
/// kubetsu::define_id!(pub struct UserId(i64););
///
/// let user_id = UserId::new(42);
/// assert_eq!(*user_id.inner(), 42);
/// ```
///
/// # Trait implementations
///
/// The generated type always implements:
/// - `new()` and `inner()` methods
/// - `Debug`, `PartialEq`, `Eq`, `Hash`, `Clone`
/// - `From<InnerType>`
///
/// ## Inner type requirements
///
/// The concrete form implements `Debug`, `PartialEq`, `Eq`, `Hash` and `Clone`
/// unconditionally, so the inner type must implement all of them. `Eq` is
/// checked explicitly: alone among those it has no methods, so it would
/// otherwise be claimed for an inner type that implements only `PartialEq`,
/// producing an ID whose `Eq` is a lie and which yields duplicate entries in a
/// `HashSet`.
///
/// ```rust,compile_fail
/// use std::hash::{Hash, Hasher};
///
/// #[derive(Clone, Debug)]
/// pub struct Weight(f64);
/// impl PartialEq for Weight {
///     fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
/// }
/// impl Hash for Weight {
///     fn hash<H: Hasher>(&self, state: &mut H) { self.0.to_bits().hash(state) }
/// }
///
/// // `Weight` is `PartialEq` but not `Eq`, so this does not compile.
/// kubetsu::define_id!(pub struct WeightId(Weight););
/// ```
///
/// Those requirements are real bounds, not just conventions: the generated
/// bodies call each trait through a fully qualified path, so an inner type
/// missing one is rejected rather than silently resolved to some other trait
/// that happens to be in scope where the macro was invoked.
///
/// ```rust
/// use std::fmt::Display; // in scope, and `String` implements it
///
/// kubetsu::define_id!(pub struct ItemId(String););
///
/// assert_eq!(format!("{:?}", ItemId::new("a".to_string())), "\"a\"");
/// ```
///
/// ```rust,compile_fail
/// use std::fmt::{Display, Formatter, Result as FmtResult};
/// use std::hash::{Hash, Hasher};
///
/// #[derive(Clone, PartialEq, Eq)]
/// pub struct Code(u32);
/// impl Display for Code {
///     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "code-{}", self.0) }
/// }
/// impl Hash for Code {
///     fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state) }
/// }
///
/// // `Code` implements `Display` but not `Debug`, so this does not compile.
/// kubetsu::define_id!(pub struct CodeId(Code););
/// ```
///
/// The generic form carries no such requirement, because each of its
/// implementations is conditional on the inner type: `MyId<T, f64>` simply gets
/// `PartialEq` without `Eq`.
///
/// ```rust,compile_fail
/// kubetsu::define_id!(pub struct MyId<T, U>;);
///
/// struct WeightTag;
///
/// fn requires_eq<T: Eq>() {}
///
/// // `f64` is `PartialEq` but not `Eq`, so neither is the ID.
/// requires_eq::<MyId<WeightTag, f64>>();
/// ```
///
/// ## Ordering
///
/// The generic form additionally implements `PartialOrd` and `Ord`, each
/// conditionally on the inner value type, so an ID can be used as a
/// `BTreeMap` key or sorted:
///
/// ```rust
/// use std::collections::BTreeMap;
///
/// kubetsu::define_id!(pub struct MyId<T, U>;);
///
/// struct User;
/// type UserId = MyId<User, i64>;
///
/// let mut map = BTreeMap::new();
/// map.insert(UserId::new(2), "b");
/// map.insert(UserId::new(1), "a");
/// assert_eq!(map.into_values().collect::<Vec<_>>(), vec!["a", "b"]);
/// ```
///
/// Ordering does not cross type tags, the same way equality does not. `Ord` and
/// `PartialOrd` compare `Self` against `Self`, so two IDs that differ only in
/// their phantom tag are still different types:
///
/// ```rust,compile_fail
/// kubetsu::define_id!(pub struct MyId<T, U>;);
///
/// struct User;
/// struct Item;
///
/// let user_id = MyId::<User, i64>::new(1);
/// let item_id = MyId::<Item, i64>::new(1);
/// let _ = user_id < item_id;
/// // error[E0308]: mismatched types
/// //     expected `MyId<User, i64>`, found `MyId<Item, i64>`
/// ```
///
/// The concrete form does not, because a fixed inner type leaves nothing to
/// make the implementation conditional on: an unconditional `Ord` would force
/// every inner type to be `Ord`. Derive it instead when you need it, which
/// works because the macro forwards attributes to the generated struct:
///
/// ```rust
/// use std::collections::BTreeMap;
///
/// kubetsu::define_id!(
///     #[derive(PartialOrd, Ord)]
///     pub struct UserId(i64);
/// );
///
/// let mut map = BTreeMap::new();
/// map.insert(UserId::new(2), "b");
/// map.insert(UserId::new(1), "a");
/// assert_eq!(map.into_values().collect::<Vec<_>>(), vec!["a", "b"]);
/// ```
///
/// Do not supply your own `PartialOrd`/`Ord` for the generic form: the macro
/// already implements them, so any derive or hand-written implementation
/// collides with `error[E0119]`, including one written for a single
/// instantiation such as `MyId<User, i64>`. Upgrading from 0.7 means deleting
/// a derive; a bespoke ordering has to move to the call site (`sort_by`) or
/// onto a wrapper type of your own. See `UPGRADE.md`.
#[macro_export]
macro_rules! define_id {
    // Generic form: define_id!(pub struct MyId<T, U>;);
    ($(#[$meta:meta])* $vis:vis struct $name:ident<$phantom:ident, $inner:ident>;) => {
        $(#[$meta])*
        $vis struct $name<$phantom, $inner> {
            inner: $inner,
            _phantom: ::core::marker::PhantomData<$phantom>,
        }

        impl<$phantom, $inner> $name<$phantom, $inner> {
            /// Create a new instance. You should use this method carefully because the value is not checked as valid.
            pub fn new(inner: $inner) -> Self {
                Self {
                    inner,
                    _phantom: ::core::marker::PhantomData,
                }
            }

            /// Access the internal value reference. You should use this method carefully.
            pub fn inner(&self) -> &$inner {
                &self.inner
            }
        }

        impl<$phantom, $inner> $crate::KubetsuId for $name<$phantom, $inner> {
            type Inner = $inner;

            fn new(inner: $inner) -> Self {
                Self {
                    inner,
                    _phantom: ::core::marker::PhantomData,
                }
            }

            fn inner(&self) -> &$inner {
                &self.inner
            }
        }

        $crate::__impl_id_core_traits!([$phantom, $inner] $name<$phantom, $inner>, $inner);
    };
    // Concrete form: define_id!(pub struct UserId(i64););
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

        impl $crate::KubetsuId for $name {
            type Inner = $inner;

            fn new(inner: $inner) -> Self {
                Self { inner }
            }

            fn inner(&self) -> &$inner {
                &self.inner
            }
        }

        $crate::__impl_id_core_traits!([] $name, $inner);
    };
}

// =============================================================================
// Internal macros for trait implementations.
// These are exported because macro_rules! requires #[macro_export] for
// cross-crate usage, but they are not part of the public API and may change
// without notice.
// =============================================================================

// Core traits: Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, From

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_id_core_traits {
    // Concrete type (no generics)
    ([] $name:ty, $inner:ty) => {
        // `Eq` below is unconditional, and being a marker trait it would be
        // claimed even for an inner type that only implements `PartialEq`.
        // Every other core trait below calls the inner type through a fully
        // qualified path, so its own body checks the bound; `Eq` has no method
        // to check, which is why it needs an explicit assertion.
        const _: () = {
            fn _assert_inner_implements_eq()
            where
                $inner: ::core::cmp::Eq,
            {
            }
        };

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(self.inner(), f)
            }
        }

        impl ::core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                ::core::cmp::PartialEq::eq(self.inner(), other.inner())
            }
        }

        impl ::core::cmp::Eq for $name {}

        impl ::core::hash::Hash for $name {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(self.inner(), state)
            }
        }

        impl ::core::clone::Clone for $name {
            fn clone(&self) -> Self {
                Self::new(::core::clone::Clone::clone(self.inner()))
            }
        }

        impl ::core::convert::From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self::new(value)
            }
        }
    };
    // Generic type (e.g. Id<T, U>)
    ([$($gen:tt)+] $name:ty, $inner:ty) => {
        impl<$($gen)+> ::core::fmt::Debug for $name where $inner: ::core::fmt::Debug {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(self.inner(), f)
            }
        }

        impl<$($gen)+> ::core::cmp::PartialEq for $name where $inner: ::core::cmp::PartialEq {
            fn eq(&self, other: &Self) -> bool {
                ::core::cmp::PartialEq::eq(self.inner(), other.inner())
            }
        }

        impl<$($gen)+> ::core::cmp::Eq for $name where $inner: ::core::cmp::Eq {}

        /// you can compare if value implement [PartialOrd].
        impl<$($gen)+> ::core::cmp::PartialOrd for $name where $inner: ::core::cmp::PartialOrd {
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(self.inner(), other.inner())
            }
        }

        /// you can use as ordered key (e.g. `BTreeMap`) if value implement [Ord].
        impl<$($gen)+> ::core::cmp::Ord for $name where $inner: ::core::cmp::Ord {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(self.inner(), other.inner())
            }
        }

        /// you can use as hash key if value implement [Hash].
        impl<$($gen)+> ::core::hash::Hash for $name where $inner: ::core::cmp::PartialEq + ::core::hash::Hash {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(self.inner(), state)
            }
        }

        /// you can clone if value implement [Clone].
        impl<$($gen)+> ::core::clone::Clone for $name where $inner: ::core::clone::Clone {
            fn clone(&self) -> Self {
                Self::new(::core::clone::Clone::clone(self.inner()))
            }
        }

        impl<$($gen)+> ::core::convert::From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self::new(value)
            }
        }
    };
}
