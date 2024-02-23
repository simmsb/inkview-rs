#[cfg(not(feature = "_sdk_selected"))]
compile_error!("No SDK selected, enable one of the 'sdk-*' features.");

pub mod bindings;
pub mod event;
pub mod screen;

pub use event::*;

pub fn load() -> bindings::Inkview {
    unsafe {
        let lib = libloading::Library::new("libinkview.so").unwrap();
        bindings::Inkview::from_library(lib).unwrap()
    }
}
