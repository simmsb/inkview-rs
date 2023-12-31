#[allow(nonstandard_style, unused)]
#[allow(rustdoc::invalid_html_tags, rustdoc::broken_intra_doc_links)]
pub mod bindings;
pub mod event;
pub mod screen;

pub use bindings::inkview as Inkview;
pub use event::*;

pub fn load() -> Inkview {
    unsafe {
        let lib = libloading::Library::new("libinkview.so").unwrap();

        let iv = Inkview::from_library(lib).unwrap();

        iv
    }
}
