use inkview::bindings::APPLICATION_ATTRIBUTE_APPLICATION_READER;
use inkview::{bindings, Event};
use std::ffi::{c_int, CString};

fn main() {
    let iv = Box::leak(Box::new(inkview::load())) as &_;
    const FONT_SIZE: c_int = 24;
    let mut orientation = 0;
    let draw = |iv: &inkview::bindings::Inkview| {
        unsafe {
            let font_name = CString::new("LiberationSans").unwrap();
            let text = CString::new("Press Menu to toggle the orientation!").unwrap();

            let font = iv.OpenFont(font_name.as_ptr(), FONT_SIZE, 0);
            iv.ClearScreen();
            iv.SetFont(font, bindings::BLACK as c_int);
            iv.FillArea(
                50,
                250,
                iv.ScreenWidth() - 50 * 2,
                iv.ScreenHeight() - 250 * 2,
                0x00E0E0E0,
            );
            iv.FillArea(
                100,
                300,
                iv.ScreenWidth() - 100 * 2,
                iv.ScreenHeight() - 300 * 2,
                0x00A0A0A0,
            );
            iv.DrawTextRect(
                0,
                iv.ScreenHeight() / 2 - FONT_SIZE / 2,
                iv.ScreenWidth(),
                FONT_SIZE,
                text.as_ptr(),
                bindings::ALIGN_CENTER as c_int,
            );
            iv.FullUpdate();
            iv.CloseFont(font);
        }
    };

    inkview::iv_main(&iv, move |event| {
        match event {
            Event::Init => {
                unsafe {
                    iv.SetCurrentApplicationAttribute(APPLICATION_ATTRIBUTE_APPLICATION_READER, 1);
                    iv.SetOrientation(orientation);
                }
                draw(iv);
            }
            Event::KeyDown { key } => match key {
                inkview::event::Key::Menu => {
                    orientation = match orientation {
                        0 => 2,
                        2 => 3,
                        3 => 1,
                        1 => 0,
                        _ => unreachable!(),
                    };
                    unsafe {
                        iv.SetOrientation(orientation);
                    }
                    draw(iv);
                }
                _ => unsafe {
                    iv.CloseApp();
                },
            },
            _ => {}
        }
        Some(())
    });
}
