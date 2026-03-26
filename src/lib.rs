mod macros;

mod id;
#[doc = include_str!("../README.md")]
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
