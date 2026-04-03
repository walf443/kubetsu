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
#[macro_export]
macro_rules! define_id {
    // Generic form: define_id!(pub struct MyId<T, U>;);
    ($(#[$meta:meta])* $vis:vis struct $name:ident<$phantom:ident, $inner:ident>;) => {
        $(#[$meta])*
        $vis struct $name<$phantom, $inner> {
            inner: $inner,
            _phantom: ::std::marker::PhantomData<$phantom>,
        }

        impl<$phantom, $inner> $name<$phantom, $inner> {
            /// Create a new instance. You should use this method carefully because the value is not checked as valid.
            pub fn new(inner: $inner) -> Self {
                Self {
                    inner,
                    _phantom: ::std::marker::PhantomData,
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
                    _phantom: ::std::marker::PhantomData,
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

// Core traits: Debug, PartialEq, Eq, Hash, Clone, From

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
