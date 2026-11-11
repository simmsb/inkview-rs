#[cfg(not(feature = "_sdk_selected"))]
compile_error!("No SDK selected, enable one of the 'sdk-*' features.");

pub mod bindings;
pub mod dialogs;
pub mod error;
pub mod event;
pub mod screen;

use std::sync::atomic::{AtomicI32, Ordering};
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

type PanelFbOffsetFn = unsafe extern "C" fn() -> core::ffi::c_int;

static PANEL_FB_OFFSET_FN: Mutex<Option<PanelFbOffsetFn>> = Mutex::new(None);

/// Panel offset of the framebuffer, read while handling `EVT_INIT` (-1 before)
static PANEL_FB_OFFSET: AtomicI32 = AtomicI32::new(-1);

/// Kick off inkview main.
///
/// Blocks until app exit.
pub fn iv_main<F: FnMut(Event) -> Option<()> + Send + 'static>(iv: &bindings::Inkview, handler: F) {
    #[cfg(not(feature = "sdk-5-19"))]
    {
        *PANEL_FB_OFFSET_FN.lock().unwrap() = iv.PanelHeightFBOffset.as_ref().ok().copied();
    }
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

    let result = match handler(evt) {
        Some(()) => RES_EVENT_HANDLED,
        None => RES_EVENT_NOT_HANDLED,
    };
    // On a device, `PanelHeightFBOffset()` gives the offset only while the application handles
    // `EVT_INIT`, and 0 later. Read it after the handler, which can change the panel type.
    if evt == Event::Init {
        if let Some(offset) = *PANEL_FB_OFFSET_FN.lock().unwrap() {
            PANEL_FB_OFFSET.store(unsafe { offset() }, Ordering::Relaxed);
        }
    }
    result
}

/// Number of rows that the panel takes above the framebuffer
///
/// It is the value read while `iv_main` handled `EVT_INIT`. Without `iv_main`, it is read now,
/// which gives the right value on a device only while the application handles `EVT_INIT`.
pub(crate) fn panel_fb_offset(iv: &bindings::Inkview) -> i32 {
    match PANEL_FB_OFFSET.load(Ordering::Relaxed) {
        -1 => {
            #[cfg(not(feature = "sdk-5-19"))]
            let offset = unsafe { iv.PanelHeightFBOffset() };
            // SDK 5.19 has no `PanelHeightFBOffset`.
            #[cfg(feature = "sdk-5-19")]
            let offset = {
                let _ = iv;
                0
            };
            offset
        }
        offset => offset,
    }
}
