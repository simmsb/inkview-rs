use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::{OriginDimensions, Size};
use embedded_graphics_core::pixelcolor::{Gray8, GrayColor};
use inkview::bindings;
use inkview::screen::Screen;
use std::convert::Infallible;

pub struct InkViewDisplay {
    screen: Screen<'static>,
}

impl InkViewDisplay {
    pub fn new(iv: &'static bindings::inkview) -> Self {
        let screen = inkview::screen::Screen::new(iv);

        Self { screen }
    }

    pub fn flush(&mut self) {
        self.screen.full_update()
    }

    pub fn screen(&mut self) -> &mut Screen<'static> {
        &mut self.screen
    }
}

impl OriginDimensions for InkViewDisplay {
    fn size(&self) -> Size {
        Size::new(self.screen.width() as u32, self.screen.height() as u32)
    }
}

impl DrawTarget for InkViewDisplay {
    type Color = Gray8;

    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics_core::prelude::Pixel<Self::Color>>,
    {
        for pixel in pixels {
            let x = pixel.0.x as usize;
            let y = pixel.0.y as usize;
            let color = pixel.1.luma();
            self.screen.draw(x, y, color);
        }
        Ok(())
    }
}
