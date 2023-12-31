use crate::{bindings::icanvas_s, Inkview};

pub struct Screen<'a> {
    iv: &'a Inkview,

    #[allow(unused)]
    fb: &'a mut icanvas_s,

    width: usize,
    height: usize,
    stride: usize,
    buf: *mut u8,
}

impl<'a> Screen<'a> {
    pub fn new(iv: &'a Inkview) -> Self {
        let fb = unsafe { iv.GetTaskFramebuffer(iv.GetCurrentTask()).as_mut().unwrap() };

        let width = fb.width as usize;
        let height = fb.height as usize;
        let stride = fb.scanline as usize;
        let buf = fb.addr;

        Self {
            iv,
            fb,

            width,
            height,
            stride,
            buf,
        }
    }

    #[inline(always)]
    pub fn draw(&mut self, x: usize, y: usize, c: u8) {
        let i = self.stride * y + x;

        unsafe {
            self.buf.add(i).write(c);
        }
    }

    /// High res partial update
    pub fn partial_update(&mut self, x: i32, y: i32, w: u32, h: u32) {
        unsafe {
            self.iv.PartialUpdate(x, y, w as i32, h as i32);
        }
    }

    /// Fast but ugly
    pub fn dynamic_update(&mut self, x: i32, y: i32, w: u32, h: u32) {
        unsafe {
            self.iv.DynamicUpdateBW(x, y, w as i32, h as i32);
        }
    }

    pub fn is_updating(&mut self) -> bool {
        unsafe { self.iv.IsUpdateInProcess() != 0 }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
