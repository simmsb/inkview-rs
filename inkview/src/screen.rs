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
        let y = (y)
        unsafe {
            // PartialUpdate:      0,    0 -- normal high quality non-flashing update
            // PartialUpdateBlack: 0xee, 0 -- seems to do a full flash
            // PartialUpdateBW:    1,    0 -- not really any noticeable difference from 0,0
            // PartialUpdateHQ:    0xe9, 0 -- also high quality? maybe forces it?
            // PartialUpdateDU4:   0xe5, 0 -- dithered, seems to corrupt often
            // DynamicUpdate:      0,    0
            // DynamicUpdateBW:    1,    2
            // DynamicUpdateA2:    0xe6, 0  and 0xef, 1
            // ExitUpdateA2:       0xe7, 0  -- cancels an update?
            //
            //
            // tested for flag values:
            //   0xe7 -- cancels
            //   0x8000 -- dithered, flashes aswell
            //   0xeb -- normal?
            //   0xef -- dithered, no flash
            //   0xe6 -- dithered, does a quick flash?
            //   0xed -- normal?
            //   0xe5 -- dithered, corrupt
            //   0xe6 -> 0xf0
            //   0xe8 -- normal?
            //   0xea -- normal?
            //   0xec -- normal?
            //
            //
            self.iv.do_partial_update(x, y, w as i32, h as i32, 0, 2);
            // self.iv.PartialUpdate(x, y, w as i32, h as i32);
        }
    }

    /// Fast but ugly
    pub fn dynamic_update(&mut self, x: i32, y: i32, w: u32, h: u32) {
        unsafe {
            self.iv.do_partial_update(x, y, w as i32, h as i32, 1, 2);
            // self.iv.DynamicUpdateBW(x, y, w as i32, h as i32);
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
