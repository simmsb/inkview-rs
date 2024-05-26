use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::{OriginDimensions, Size};
use embedded_graphics_core::pixelcolor::{Gray8, GrayColor};
use inkview::bindings;
use inkview::screen::{Screen, ScreenOrientation};
use std::convert::Infallible;

pub struct InkviewDisplay {
    iv_screen: Screen<'static>,
    fast_updates_max: usize,
    fast_updates_n: usize,
}

impl InkviewDisplay {
    pub fn new(iv: &'static bindings::inkview) -> Self {
        let iv_screen = inkview::screen::Screen::new(iv);

        Self {
            iv_screen,
            fast_updates_max: 5,
            fast_updates_n: 0,
        }
    }

    pub fn flush(&mut self) {
        if self.fast_updates_n < self.fast_updates_max {
            self.iv_screen.fast_update();
            self.fast_updates_n += 1;
        } else {
            self.fast_updates_n = 0;
            self.iv_screen.full_update();
        }
    }

    pub fn iv_screen_ref(&self) -> &Screen<'static> {
        &self.iv_screen
    }

    pub fn iv_screen_mut(&mut self) -> &mut Screen<'static> {
        &mut self.iv_screen
    }
}

impl OriginDimensions for InkviewDisplay {
    fn size(&self) -> Size {
        match self.iv_screen.orientation() {
            ScreenOrientation::Portrait0Deg | ScreenOrientation::Portrait180Deg => Size::new(
                self.iv_screen.width() as u32,
                self.iv_screen.height() as u32,
            ),
            ScreenOrientation::Landscape90Deg | ScreenOrientation::Landscape270Deg => Size::new(
                self.iv_screen.height() as u32,
                self.iv_screen.width() as u32,
            ),
        }
    }
}

impl DrawTarget for InkviewDisplay {
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
            self.iv_screen.draw(x, y, color);
        }
        Ok(())
    }
}
