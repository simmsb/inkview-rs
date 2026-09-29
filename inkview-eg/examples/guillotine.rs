use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::prelude::*;
use embedded_graphics_core::pixelcolor::Gray8;
use guillotine::style::{AlignItems, JustifyContent};
use guillotine::{
    Context, DirectTarget, ElementBuilder, Font, FrameStorage, ParentElement, Render,
    StyledElement, StyledFlexContainer, TextStyledElement, Ui,
};
use inkview::Event;
use inkview::event::Key;
use inkview_eg::InkviewDisplay;
use std::sync::LazyLock;

const ACCENT: Gray8 = Gray8::new(0x88);

struct BasicView {
    size: Size,
    dark: bool,
}

impl Render<Gray8> for BasicView {
    // Render lets you declaratively build your UI tree.
    fn render(&self, cx: &Context<'_, Gray8>) -> impl ElementBuilder {
        let bg = if self.dark {
            Gray8::BLACK
        } else {
            Gray8::WHITE
        };
        let fg = if self.dark {
            Gray8::WHITE
        } else {
            Gray8::BLACK
        };
        let greeting = if self.dark {
            "Dark theme"
        } else {
            "Light theme"
        };

        cx.column()
            .size(self.size)
            .padding(12)
            .gap(8)
            .background(bg)
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .child(
                cx.row()
                    .gap(8)
                    .child(
                        cx.text("GUILLOTINE")
                            .flex_grow(1)
                            .font(Font::mono(&FONT_10X20))
                            .text_color(ACCENT),
                    )
                    .child(
                        cx.text("READY")
                            .padding((4, 9))
                            .background(ACCENT)
                            .text_color(fg)
                            .font(Font::mono(&FONT_10X20)),
                    ),
            )
            .child(
                cx.row()
                    .gap(8)
                    .child(
                        cx.text(greeting)
                            .font(Font::mono(&FONT_10X20))
                            .flex(3)
                            .padding(10)
                            .background(bg)
                            .text_color(fg),
                    )
                    .child(
                        cx.text("7 nodes, 54 bytes")
                            .font(Font::mono(&FONT_10X20))
                            .flex(2)
                            .padding(6)
                            .background(bg)
                            .text_color(fg),
                    ),
            )
    }
}

fn main() {
    let (event_tx, event_rx) = std::sync::mpsc::channel::<inkview::Event>();
    let iv = Box::leak(Box::new(inkview::load())) as &_;

    std::thread::spawn(move || {
        let mut view = BasicView {
            dark: false,
            size: Size::new(320, 240),
        };
        let mut ui = LazyLock::new(|| {
            let mut display = InkviewDisplay::new(iv);
            display.clear(Gray8::WHITE);
            let storage = FrameStorage::<Gray8>::default();
            let theme = guillotine::Theme {
                background: Gray8::WHITE,
                foreground: Gray8::BLACK,
            };

            Ui::with_theme(DirectTarget::new(display), storage, theme)
        });

        loop {
            let mut render = false;

            let event = match event_rx.recv() {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Receiving inkview event failed, Err: {e:?}");
                    break;
                }
            };
            match event {
                Event::Init => {
                    let ui = LazyLock::force(&ui);
                    view.size = ui.display().size();
                }
                Event::Show | Event::Repaint => {
                    render = true;
                }
                Event::KeyDown { key: Key::Menu } => {
                    view.dark = !view.dark;
                    render = true;
                }
                Event::KeyDown { .. } | Event::Exit => break,
                _ => {}
            }

            if render && let Some(ui) = LazyLock::get_mut(&mut ui) {
                if let Err(err) = ui.render(&view) {
                    eprintln!("Unable to render app: {err}");
                }
                ui.display_mut().flush();
            }
        }

        unsafe { iv.CloseApp() }
    });

    inkview::iv_main(&iv, move |event| {
        if let Err(e) = event_tx.send(event) {
            eprintln!("Sending inkview event failed, Err: {e:?}");
        }
        Some(())
    });
}
