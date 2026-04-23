//! Verifies that `kubetsu::define_id!` and `kubetsu_serde::impl_serde!`
//! expand to code that compiles in a `#![no_std]` consumer crate.
//!
//! This crate has no runtime tests; the act of building it for a no_std
//! target (e.g. `thumbv7em-none-eabihf`) is the test.

#![no_std]

kubetsu::define_id!(
    pub struct MyId<T, U>;
);
kubetsu_serde::impl_serde!(MyId<T, U>);
