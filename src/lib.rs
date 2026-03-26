mod macros;

mod id;
#[doc = include_str!("../README.md")]
#[deprecated(
    since = "0.6.0",
    note = "Use `kubetsu::define_id!` to define your own ID type instead."
)]
pub use id::Id;

#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "serde")]
    pub use serde;

    #[cfg(feature = "sqlx")]
    pub use sqlx;

    #[cfg(feature = "fake")]
    pub use fake;
}
