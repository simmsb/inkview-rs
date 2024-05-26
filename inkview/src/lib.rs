#[cfg(not(feature = "_sdk_selected"))]
compile_error!("No SDK selected, enable one of the 'sdk-*' features.");

pub mod bindings;
pub mod dialogs;
pub mod error;
pub mod event;
pub mod screen;

use std::sync::Mutex;

pub use event::Event;

pub fn load() -> bindings::Inkview {
    unsafe {
        let lib = libloading::Library::new("libinkview.so").unwrap();
        bindings::Inkview::from_library(lib).unwrap()
    }
}

const RES_EVENT_HANDLED: i32 = 0;
const RES_EVENT_NOT_HANDLED: i32 = -1;
const RES_EVENT_ERROR: i32 = -2;

type IvEventHandlerType = Mutex<Option<Box<dyn FnMut(Event) -> Option<()> + Send>>>;

static IV_EVENT_HANDLER: IvEventHandlerType = Mutex::new(None);

/// Kick off inkview main.
///
/// Blocks until app exit.
pub fn iv_main<F: FnMut(Event) -> Option<()> + Send + 'static>(iv: &bindings::Inkview, handler: F) {
    unsafe {
        *IV_EVENT_HANDLER.lock().unwrap() = Some(Box::new(handler));
        iv.InkViewMain(Some(forward_iv_events))
    }
}

extern "C" fn forward_iv_events(event: i32, par1: i32, par2: i32) -> i32 {
    let mut handler = IV_EVENT_HANDLER.lock().unwrap();
    let Some(handler) = handler.as_deref_mut() else {
        return RES_EVENT_ERROR;
    };
    let Some(evt) = Event::from_raw(event, par1, par2) else {
        return RES_EVENT_NOT_HANDLED;
    };

    match handler(evt) {
        Some(()) => RES_EVENT_HANDLED,
        None => RES_EVENT_NOT_HANDLED,
    }
}
