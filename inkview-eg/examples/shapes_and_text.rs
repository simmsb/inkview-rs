use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{
    Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
};
use embedded_graphics::text::{Alignment, Text};
use embedded_graphics_core::pixelcolor::Gray8;
use inkview::Event;
use inkview_eg::InkviewDisplay;
use std::cell::OnceCell;
use std::convert::Infallible;

fn main() {
    let (event_tx, event_rx) = std::sync::mpsc::channel::<inkview::Event>();
    let iv = Box::leak(Box::new(inkview::load())) as &_;

    std::thread::spawn(move || {
        let mut display = OnceCell::new();

        loop {
            let event = match event_rx.recv() {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Receiving inkview event failed, Err: {e:?}");
                    break;
                }
            };
            match event {
                Event::Init => {
                    // Create a new inkview display which implements [embedded_graphics_core::DrawTarget]
                    let _ = display.set(InkviewDisplay::new(&iv));
                    display.get_mut().unwrap().screen().clear();
                }
                Event::Show | Event::Repaint => {
                    draw_content(display.get_mut().unwrap()).unwrap();
                    display.get_mut().unwrap().flush();
                }
                Event::KeyDown { .. } | Event::Exit => break,
                _ => {}
            }
        }

        unsafe { iv.CloseApp() }
    });

    inkview::iv_main(&iv, {
        move |event| {
            if let Err(e) = event_tx.send(event) {
                eprintln!("Sending inkview event failed, Err: {e:?}");
            }
            Some(())
        }
    });
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
