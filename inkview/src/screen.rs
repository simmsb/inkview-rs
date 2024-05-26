use crate::bindings::APPLICATION_ATTRIBUTE_APPLICATION_READER;
use crate::error;
use crate::{bindings::icanvas_s, bindings::Inkview};
use core::ffi::c_int;
use core::fmt::Display;
use num_traits::{FromPrimitive, ToPrimitive};

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
        unsafe {
            iv.SetCurrentApplicationAttribute(APPLICATION_ATTRIBUTE_APPLICATION_READER, 1);
        }
        let fb = unsafe { iv.GetTaskFramebuffer(iv.GetCurrentTask()).as_mut() }
            .expect("Failed to get current task framebuffer while creating new screen.");

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

    pub fn clear(&mut self) {
        unsafe {
            self.iv.ClearScreen();
        }
    }

    #[inline(always)]
    pub fn draw(&mut self, x: usize, y: usize, c: u8) {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            return;
        }
        let i = self.stride * y + x;

        unsafe {
            self.buf.add(i).write(c);
        }
    }

    pub fn full_update(&mut self) {
        unsafe {
            self.iv.FullUpdate();
        }
    }

    pub fn fast_update(&mut self) {
        unsafe {
            self.iv.SoftUpdate();
        }
    }

    /// High res partial update
    pub fn partial_update(&mut self, x: i32, y: i32, w: u32, h: u32) {
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
            //self.iv.do_partial_update(x, y, w as i32, h as i32, 0, 2);
            self.iv.PartialUpdate(x, y, w as i32, h as i32);
        }
    }

    /// Fast but ugly
    pub fn dynamic_update(&mut self, x: i32, y: i32, w: u32, h: u32) {
        unsafe {
            //self.iv.do_partial_update(x, y, w as i32, h as i32, 1, 2);
            self.iv.DynamicUpdate(x, y, w as i32, h as i32);
        }
    }

    pub fn is_updating(&mut self) -> bool {
        unsafe { self.iv.IsUpdateInProcess() != 0 }
    }

    /// Width of the physical screen (independent of the orientation)
    pub fn width(&self) -> usize {
        self.width
    }

    /// Height of the physical screen (independent of the orientation)
    pub fn height(&self) -> usize {
        self.height
    }

    /// Current screen orientation
    pub fn orientation(&self) -> ScreenOrientation {
        ScreenOrientation::from_iv(unsafe { self.iv.GetOrientation() })
    }

    /// Set the current screen orientation
    pub fn set_orientation(&mut self, orientation: ScreenOrientation) {
        unsafe { self.iv.SetOrientation(orientation.to_iv()) }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, num_derive::FromPrimitive, num_derive::ToPrimitive,
)]
pub enum ScreenOrientation {
    Portrait0Deg = 0,
    Landscape90Deg,
    Portrait180Deg,
    Landscape270Deg,
}

impl Default for ScreenOrientation {
    fn default() -> Self {
        Self::Portrait0Deg
    }
}

impl Display for ScreenOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScreenOrientation::Portrait0Deg => write!(f, "Portrait0Deg"),
            ScreenOrientation::Landscape90Deg => write!(f, "Landscape90Deg"),
            ScreenOrientation::Portrait180Deg => write!(f, "Portrait180Deg"),
            ScreenOrientation::Landscape270Deg => write!(f, "Landscape270Deg"),
        }
    }
}

impl TryFrom<i32> for ScreenOrientation {
    type Error = error::EnumPrimitiveConversionError<i32>;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_i32(value).ok_or_else(|| error::EnumPrimitiveConversionError {
            src_t: "i32".to_string(),
            dest_t: "ScreenOrientation".to_string(),
            value,
        })
    }
}

impl TryFrom<ScreenOrientation> for i32 {
    type Error = error::EnumPrimitiveConversionError<ScreenOrientation>;

    fn try_from(value: ScreenOrientation) -> Result<Self, Self::Error> {
        value
            .to_i32()
            .ok_or_else(|| error::EnumPrimitiveConversionError {
                src_t: "ScreenOrientation".to_string(),
                dest_t: "i32".to_string(),
                value,
            })
    }
}

impl ScreenOrientation {
    fn from_iv(raw: c_int) -> Self {
        match raw {
            0 => Self::Portrait0Deg,
            1 => Self::Landscape270Deg,
            2 => Self::Landscape90Deg,
            3 => Self::Portrait180Deg,
            raw => panic!("ScreenOrientation from inkview enum invalid num: {raw}"),
        }
    }

    fn to_iv(self) -> c_int {
        match self {
            ScreenOrientation::Portrait0Deg => 0,
            ScreenOrientation::Landscape90Deg => 2,
            ScreenOrientation::Portrait180Deg => 3,
            ScreenOrientation::Landscape270Deg => 1,
        }
    }
}
