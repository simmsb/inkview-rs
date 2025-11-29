#![allow(
    clippy::all,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    rustdoc::all
)]

#[cfg(feature = "sdk-5-19")]
mod bindings_5_19;
#[cfg(feature = "sdk-5-19")]
pub use bindings_5_19::*;

#[cfg(feature = "sdk-6-5")]
mod bindings_6_5;
#[cfg(feature = "sdk-6-5")]
pub use bindings_6_5::*;

#[cfg(feature = "sdk-6-8")]
mod bindings_6_8;
#[cfg(feature = "sdk-6-8")]
pub use bindings_6_8::*;

pub use inkview as Inkview;

unsafe impl Send for inkview {}
unsafe impl Sync for inkview {}
