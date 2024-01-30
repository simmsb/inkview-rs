use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::{OriginDimensions, Size};
use embedded_graphics_core::pixelcolor::{Gray8, GrayColor};
use inkview::bindings;
use inkview::screen::Screen;
use std::cell::RefCell;
use std::convert::Infallible;

pub struct InkViewDisplay {
    screen: RefCell<Screen<'static>>,
    width: usize,
    height: usize,
}

impl InkViewDisplay {
    pub fn new(iv: &'static bindings::inkview) -> Self {
        let screen = inkview::screen::Screen::new(iv);
        let width = screen.width();
        let height = screen.height();

        Self {
            screen: screen.into(),
            width,
            height,
        }
    }

    pub fn flush(&mut self) {
        self.screen.borrow_mut().full_update()
    }
}

impl OriginDimensions for InkViewDisplay {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl DrawTarget for InkViewDisplay {
    type Color = Gray8;

    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics_core::prelude::Pixel<Self::Color>>,
    {
        let mut screen = self.screen.borrow_mut();
        for pixel in pixels {
            let x = pixel.0.x as usize;
            let y = pixel.0.y as usize;
            let color = pixel.1.luma();
            screen.draw(x, y, color);
        }
        Ok(())
    }
}
