pub mod bindings;
pub mod event;
pub mod screen;

pub use event::*;

pub fn load() -> bindings::Inkview {
    unsafe {
        let lib = libloading::Library::new("libinkview.so").unwrap();

        let iv = bindings::Inkview::from_library(lib).unwrap();

        iv
    }
}
