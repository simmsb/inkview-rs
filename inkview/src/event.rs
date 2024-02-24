use std::sync::Mutex;

use crate::bindings::Inkview;

type IvEventHandlerType = Mutex<Option<Box<dyn FnMut(Event) -> Option<()> + Send>>>;

static IV_EVENT_HANDLER: IvEventHandlerType = Mutex::new(None);

pub fn iv_main<F: FnMut(Event) -> Option<()> + Send + 'static>(iv: &Inkview, handler: F) {
    unsafe {
        *IV_EVENT_HANDLER.lock().unwrap() = Some(Box::new(handler));
        iv.InkViewMain(Some(forward_iv_events))
    }
}

extern "C" fn forward_iv_events(event: i32, par1: i32, par2: i32) -> i32 {
    let mut handler = IV_EVENT_HANDLER.lock().unwrap();
    let Some(handler) = handler.as_deref_mut() else {
        return -2;
    };
    let Some(evt) = Event::from_raw(event, par1, par2) else {
        return -1;
    };

    match handler(evt) {
        Some(()) => 0,
        None => -1,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Event {
    Init,
    Show,
    Repaint,
    Hide,
    Exit,
    Foreground { pid: i32 },
    Background { pid: i32 },
    KeyDown { key: i32 },
    KeyRepeat { key: i32 },
    KeyUp { key: i32 },
    PointerDown { x: i32, y: i32 },
    PointerMove { x: i32, y: i32 },
    PointerUp { x: i32, y: i32 },
}

impl Event {
    fn from_raw(event: i32, par1: i32, par2: i32) -> Option<Self> {
        use crate::bindings::*;

        let evt = match event as u32 {
            EVT_INIT => Event::Init,
            EVT_SHOW => Event::Show,
            EVT_REPAINT => Event::Repaint,
            EVT_HIDE => Event::Hide,
            EVT_EXIT => Event::Exit,
            EVT_FOREGROUND => Event::Foreground { pid: par1 },
            EVT_BACKGROUND => Event::Background { pid: par1 },
            EVT_KEYDOWN => Event::KeyDown { key: par1 },
            // KEYPRESS == KEYDOWN
            EVT_KEYREPEAT => Event::KeyRepeat { key: par1 },
            EVT_KEYUP => Event::KeyUp { key: par1 },
            EVT_POINTERDOWN => Event::PointerDown { x: par1, y: par2 },
            EVT_POINTERMOVE => Event::PointerMove { x: par1, y: par2 },
            EVT_POINTERUP => Event::PointerUp { x: par1, y: par2 },
            _ => return None,
        };

        Some(evt)
    }
}
