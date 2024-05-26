use std::ffi::{c_int, CString};
use std::time::Duration;

use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Icon {
    Information = bindings::ICON_INFORMATION as isize,
    Question = bindings::ICON_QUESTION as isize,
    Warning = bindings::ICON_WARNING as isize,
    Error = bindings::ICON_ERROR as isize,
    WiFi = bindings::ICON_WIFI as isize,
}

pub fn message(
    iv: &bindings::Inkview,
    icon: Icon,
    title: impl Into<String>,
    text: impl Into<String>,
    timeout: Duration,
) {
    let title = CString::new(title.into()).unwrap();
    let text = CString::new(text.into()).unwrap();
    let timeout = timeout.as_millis();

    unsafe {
        iv.Message(icon as i32, title.as_ptr(), text.as_ptr(), timeout as i32);
    }
}

pub fn dialog(
    iv: &bindings::Inkview,
    icon: Icon,
    title: impl Into<String>,
    text: impl Into<String>,
    button_1: impl Into<String>,
    button_2: impl Into<String>,
    button_3: Option<impl Into<String>>,
) {
    let title = CString::new(title.into()).unwrap();
    let text = CString::new(text.into()).unwrap();
    let button_1 = CString::new(button_1.into()).unwrap();
    let button_2 = CString::new(button_2.into()).unwrap();

    if let Some(button_3) = button_3 {
        let button_3 = CString::new(button_3.into()).unwrap();

        unsafe {
            iv.Dialog3(
                icon as c_int,
                title.as_ptr(),
                text.as_ptr(),
                button_1.as_ptr(),
                button_2.as_ptr(),
                button_3.as_ptr(),
                None,
            )
        }
    } else {
        unsafe {
            iv.Dialog(
                icon as c_int,
                title.as_ptr(),
                text.as_ptr(),
                button_1.as_ptr(),
                button_2.as_ptr(),
                None,
            )
        }
    }
}
