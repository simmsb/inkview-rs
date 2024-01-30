use std::convert::Infallible;

use embedded_graphics::image::Image;
use embedded_graphics::prelude::*;
use embedded_graphics_core::pixelcolor::Gray8;
use embedded_iconoir::prelude::*;
use embedded_iconoir::size144px;
use inkview_eg::InkViewDisplay;

enum Event {
    Redraw,
    Exit,
}

impl Event {
    fn from_inkview(event: inkview::Event) -> Option<Event> {
        match event {
            inkview::Event::Show | inkview::Event::Repaint => Some(Event::Redraw),
            inkview::Event::Exit => Some(Event::Exit),
            _ => None,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let (event_tx, event_rx) = std::sync::mpsc::channel::<Event>();
    let iv = Box::leak(Box::new(inkview::load())) as &_;

    std::thread::spawn(move || -> anyhow::Result<()> {
        // Create a new inkview display
        let mut display = InkViewDisplay::new(iv);

        draw_content(&mut display)?;
        display.flush();

        loop {
            match event_rx.recv() {
                Ok(event) => match event {
                    Event::Redraw => {
                        draw_content(&mut display)?;
                        display.flush();
                    }
                    Event::Exit => break,
                },
                Err(e) => {
                    eprintln!("Receiving event failed, Err: {e:?}");
                    break;
                }
            }
        }

        Ok(())
    });

    inkview::iv_main(&iv, {
        move |event| {
            if let Some(event) = Event::from_inkview(event) {
                if let Err(e) = event_tx.send(event) {
                    eprintln!("Sending Event failed, Err: {e:?}");
                    unsafe {
                        iv.CloseApp();
                    }
                }
            }
            Some(())
        }
    });

    Ok(())
}

fn draw_content(
    display: &mut impl DrawTarget<Color = Gray8, Error = Infallible>,
) -> anyhow::Result<()> {
    Image::new(
        &size144px::development::Code::new(Gray8::new(0x00)),
        Point::new(0, 600),
    )
    .draw(display)?;
    Image::new(
        &size144px::communication::ChatBubble::new(Gray8::new(0x00)),
        Point::new(200, 600),
    )
    .draw(display)?;
    Image::new(
        &size144px::editor::TaskList::new(Gray8::new(0x00)),
        Point::new(400, 600),
    )
    .draw(display)?;
    Image::new(
        &size144px::actions::SaveActionFloppy::new(Gray8::new(0x00)),
        Point::new(600, 600),
    )
    .draw(display)?;

    Ok(())
}
