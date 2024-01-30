use std::convert::Infallible;

use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{
    Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
};
use embedded_graphics::text::{Alignment, Text};
use embedded_graphics_core::pixelcolor::Gray8;
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
    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(Gray8::new(0x00), 1);
    let thick_stroke = PrimitiveStyle::with_stroke(Gray8::new(0x00), 3);
    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(Gray8::new(0x00))
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let fill = PrimitiveStyle::with_fill(Gray8::new(0x00));
    let character_style = MonoTextStyle::new(&FONT_6X10, Gray8::new(0x00));

    let yoffset = 10;

    // Draw a 3px wide outline around the display.
    display
        .bounding_box()
        .into_styled(border_stroke)
        .draw(display)?;

    // Draw a triangle.
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(thin_stroke)
    .draw(display)?;

    // Draw a filled square
    Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
        .into_styled(fill)
        .draw(display)?;

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new(88, yoffset), 17)
        .into_styled(thick_stroke)
        .draw(display)?;

    // Draw centered text.
    let text = "embedded-graphics";
    Text::with_alignment(
        text,
        display.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(display)?;

    Ok(())
}
