use crate::{bindings, error};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
)]
pub enum Key {
    Power = bindings::IV_KEY_POWER as isize,
    Delete = bindings::IV_KEY_DELETE as isize,
    Ok = bindings::IV_KEY_OK as isize,
    Up = bindings::IV_KEY_UP as isize,
    Down = bindings::IV_KEY_DOWN as isize,
    Left = bindings::IV_KEY_LEFT as isize,
    Right = bindings::IV_KEY_RIGHT as isize,
    Minus = bindings::IV_KEY_MINUS as isize,
    Plus = bindings::IV_KEY_PLUS as isize,
    Menu = bindings::IV_KEY_MENU as isize,
    Prev = bindings::IV_KEY_PREV as isize,
    Next = bindings::IV_KEY_NEXT as isize,
    Home = bindings::IV_KEY_HOME as isize,
    Back = bindings::IV_KEY_BACK as isize,
    Prev2 = bindings::IV_KEY_PREV2 as isize,
    Next2 = bindings::IV_KEY_NEXT2 as isize,
    Music = bindings::IV_KEY_MUSIC as isize,
    CoverOpen = bindings::IV_KEY_COVEROPEN as isize,
    CoverClose = bindings::IV_KEY_COVERCLOSE as isize,
    ZoomOut = bindings::IV_KEY_ZOOMOUT as isize,
    ZoomIn = bindings::IV_KEY_ZOOMIN as isize,
    MenuPower = bindings::IV_KEY_MENU_POWER as isize,
    Shift = bindings::IV_KEY_SHIFT as isize,
    LanguageChange = bindings::IV_KEY_LANGUAGECHANGE as isize,
    KeyboardClose = bindings::IV_KEY_KEYBOARDCLOSE as isize,
    Key0 = bindings::IV_KEY_0 as isize,
    Key1 = bindings::IV_KEY_1 as isize,
    Key2 = bindings::IV_KEY_2 as isize,
    Key3 = bindings::IV_KEY_3 as isize,
    Key4 = bindings::IV_KEY_4 as isize,
    Key5 = bindings::IV_KEY_5 as isize,
    Key6 = bindings::IV_KEY_6 as isize,
    Key7 = bindings::IV_KEY_7 as isize,
    Key8 = bindings::IV_KEY_8 as isize,
    Key9 = bindings::IV_KEY_9 as isize,
}

impl TryFrom<isize> for Key {
    type Error = error::EnumPrimitiveConversionError<isize>;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        Self::from_isize(value).ok_or(error::EnumPrimitiveConversionError {
            src_t: "isize".to_string(),
            dest_t: "Key".to_string(),
            value,
        })
    }
}

impl TryFrom<Key> for isize {
    type Error = error::EnumPrimitiveConversionError<Key>;

    fn try_from(value: Key) -> Result<Self, Self::Error> {
        value.to_isize().ok_or(error::EnumPrimitiveConversionError {
            src_t: "Key".to_string(),
            dest_t: "isize".to_string(),
            value,
        })
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
    KeyDown { key: Key },
    KeyRepeat { key: Key },
    KeyUp { key: Key },
    PointerDown { x: i32, y: i32 },
    PointerMove { x: i32, y: i32 },
    PointerUp { x: i32, y: i32 },
}

impl Event {
    pub(crate) fn from_raw(event: i32, par1: i32, par2: i32) -> Option<Self> {
        let event = match event as u32 {
            bindings::EVT_INIT => Event::Init,
            bindings::EVT_SHOW => Event::Show,
            bindings::EVT_REPAINT => Event::Repaint,
            bindings::EVT_HIDE => Event::Hide,
            bindings::EVT_EXIT => Event::Exit,
            bindings::EVT_FOREGROUND => Event::Foreground { pid: par1 },
            bindings::EVT_BACKGROUND => Event::Background { pid: par1 },
            bindings::EVT_KEYDOWN => Event::KeyDown {
                key: Key::try_from(par1 as isize).ok()?,
            },
            // KEYPRESS == KEYDOWN
            bindings::EVT_KEYREPEAT => Event::KeyRepeat {
                key: Key::try_from(par1 as isize).ok()?,
            },
            bindings::EVT_KEYUP => Event::KeyUp {
                key: Key::try_from(par1 as isize).ok()?,
            },
            bindings::EVT_POINTERDOWN => Event::PointerDown { x: par1, y: par2 },
            bindings::EVT_POINTERMOVE => Event::PointerMove { x: par1, y: par2 },
            bindings::EVT_POINTERUP => Event::PointerUp { x: par1, y: par2 },
            _ => return None,
        };

        Some(event)
    }
}
