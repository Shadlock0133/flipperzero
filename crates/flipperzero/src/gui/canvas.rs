//! Canvases.

use core::cell::UnsafeCell;
use core::ffi::CStr;
use core::marker::PhantomPinned;

use flipperzero_sys as sys;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Font {
    Primary,
    Secondary,
    Keyboard,
    BigNumbers,
}

impl Font {
    pub fn as_sys(&self) -> sys::Font {
        match self {
            Font::Primary => sys::FontPrimary,
            Font::Secondary => sys::FontSecondary,
            Font::Keyboard => sys::FontKeyboard,
            Font::BigNumbers => sys::FontBigNumbers,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Align {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Align {
    pub fn horizontal(&self) -> sys::Align {
        match self {
            Align::TopLeft => sys::AlignLeft,
            Align::Top => sys::AlignCenter,
            Align::TopRight => sys::AlignRight,
            Align::Left => sys::AlignLeft,
            Align::Center => sys::AlignCenter,
            Align::Right => sys::AlignRight,
            Align::BottomLeft => sys::AlignLeft,
            Align::Bottom => sys::AlignCenter,
            Align::BottomRight => sys::AlignRight,
        }
    }
    pub fn vertical(&self) -> sys::Align {
        match self {
            Align::TopLeft => sys::AlignTop,
            Align::Top => sys::AlignTop,
            Align::TopRight => sys::AlignTop,
            Align::Left => sys::AlignCenter,
            Align::Center => sys::AlignCenter,
            Align::Right => sys::AlignCenter,
            Align::BottomLeft => sys::AlignBottom,
            Align::Bottom => sys::AlignBottom,
            Align::BottomRight => sys::AlignBottom,
        }
    }
}

/// Graphics Canvas.
#[repr(transparent)]
pub struct Canvas {
    raw: UnsafeCell<sys::Canvas>,
    _marker: PhantomPinned,
}

impl Canvas {
    /// Get Canvas reference from raw pointer.
    ///
    /// # Safety
    /// Pointer must be non-null and point to a valid `sys::Canvas`.
    /// This pointer must outlive this reference.
    pub unsafe fn from_raw<'a>(raw: *mut sys::Canvas) -> &'a Self {
        unsafe { &*(raw.cast()) }
    }

    /// Get Canvas reference from raw pointer.
    ///
    /// # Safety
    /// Pointer must be non-null and point to a valid `sys::Canvas`.
    /// This pointer must outlive this reference.
    pub unsafe fn from_raw_mut<'a>(raw: *mut sys::Canvas) -> &'a mut Self {
        unsafe { &mut *(raw.cast()) }
    }

    /// Get pointer to raw [`sys::Canvas`].
    pub fn as_ptr(&self) -> *mut sys::Canvas {
        self.raw.get()
    }

    /// Get Canvas width and height.
    pub fn get_size(&self) -> (usize, usize) {
        unsafe {
            (
                sys::canvas_width(self.as_ptr()),
                sys::canvas_height(self.as_ptr()),
            )
        }
    }

    /// Clear Canvas.
    pub fn clear(&self) {
        unsafe { sys::canvas_clear(self.as_ptr()) }
    }

    /// Commit Canvas and send buffer to display.
    pub fn commit(&self) {
        unsafe { sys::canvas_commit(self.as_ptr()) }
    }

    pub fn width(&self) -> usize {
        unsafe { sys::canvas_width(self.as_ptr()) }
    }

    pub fn height(&self) -> usize {
        unsafe { sys::canvas_height(self.as_ptr()) }
    }

    pub fn current_font_height(&self) -> usize {
        unsafe { sys::canvas_current_font_height(self.as_ptr()) }
    }

    pub fn set_font(&self, font: Font) {
        unsafe { sys::canvas_set_font(self.as_ptr(), font.as_sys()) };
    }

    pub fn draw_box(&self, x: i32, y: i32, width: usize, height: usize) {
        unsafe { sys::canvas_draw_box(self.as_ptr(), x, y, width, height) };
    }

    pub fn draw_circle(&self, x: i32, y: i32, r: usize) {
        unsafe { sys::canvas_draw_circle(self.as_ptr(), x, y, r) };
    }

    pub fn draw_disc(&self, x: i32, y: i32, r: usize) {
        unsafe { sys::canvas_draw_disc(self.as_ptr(), x, y, r) };
    }

    pub fn draw_dot(&self, x: i32, y: i32) {
        unsafe { sys::canvas_draw_dot(self.as_ptr(), x, y) };
    }

    pub fn draw_frame(&self, x: i32, y: i32, width: usize, height: usize) {
        unsafe { sys::canvas_draw_frame(self.as_ptr(), x, y, width, height) };
    }

    pub fn draw_glyph(&self, x: i32, y: i32, ch: u16) {
        unsafe { sys::canvas_draw_glyph(self.as_ptr(), x, y, ch) };
    }

    pub fn draw_icon(&self, x: i32, y: i32, icon: &sys::Icon) {
        unsafe {
            sys::canvas_draw_icon(self.as_ptr(), x, y, icon);
        }
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe { sys::canvas_draw_line(self.as_ptr(), x1, y1, x2, y2) };
    }

    #[doc(alias = "draw_rbox")]
    pub fn draw_rounded_box(&self, x: i32, y: i32, width: usize, height: usize, r: usize) {
        unsafe { sys::canvas_draw_rbox(self.as_ptr(), x, y, width, height, r) };
    }

    #[doc(alias = "draw_rframe")]
    pub fn draw_rounded_frame(&self, x: i32, y: i32, width: usize, height: usize, r: usize) {
        unsafe { sys::canvas_draw_rframe(self.as_ptr(), x, y, width, height, r) };
    }

    pub fn draw_str(&self, x: i32, y: i32, str: &CStr) {
        unsafe { sys::canvas_draw_str(self.as_ptr(), x, y, str.as_ptr()) };
    }

    pub fn draw_str_aligned(&self, x: i32, y: i32, align: Align, str: &CStr) {
        unsafe {
            sys::canvas_draw_str_aligned(
                self.as_ptr(),
                x,
                y,
                align.horizontal(),
                align.vertical(),
                str.as_ptr(),
            )
        };
    }

    pub fn string_width(&self, str: &CStr) -> u16 {
        unsafe { sys::canvas_string_width(self.as_ptr(), str.as_ptr()) }
    }

    /// Set transparency mode
    pub fn set_bitmap_mode(&self, alpha: bool) {
        unsafe {
            sys::canvas_set_bitmap_mode(self.as_ptr(), alpha);
        }
    }
}

/// Support for [`embedded-graphics``](https://crates.io/crates/embedded-graphics) crate.
#[cfg(feature = "embedded-graphics")]
mod embedded_graphics {
    use super::*;
    use embedded_graphics_core::pixelcolor::BinaryColor;
    use embedded_graphics_core::prelude::*;
    use embedded_graphics_core::primitives::Rectangle;

    impl Dimensions for Canvas {
        fn bounding_box(&self) -> Rectangle {
            let (width, height) = self.get_size();

            Rectangle {
                top_left: (0, 0).into(),
                size: (width as u32, height as u32).into(),
            }
        }
    }

    impl DrawTarget for Canvas {
        type Color = BinaryColor;
        type Error = core::convert::Infallible;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            let (width, height) = self.get_size();
            let (width, height) = (width as i32, height as i32);

            unsafe {
                for Pixel(Point { x, y }, color) in pixels.into_iter() {
                    if (0..=width).contains(&x) && (0..=height).contains(&y) {
                        sys::canvas_set_color(self.as_ptr(), map_color(color));
                        sys::canvas_draw_dot(self.as_ptr(), x, y);
                    }
                }
            }

            Ok(())
        }

        fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
            // Clamp rectangle coordinates to visible display area
            let area = area.intersection(&self.bounding_box());

            // Do not draw if the intersection size is zero.
            if area.bottom_right().is_none() {
                return Ok(());
            }

            unsafe {
                sys::canvas_set_color(self.as_ptr(), map_color(color));
                sys::canvas_draw_box(
                    self.as_ptr(),
                    area.top_left.x,
                    area.top_left.y,
                    area.size.width as usize,
                    area.size.height as usize,
                );
            }

            Ok(())
        }
    }

    /// Map embedded-graphics color to Furi color.
    #[inline]
    const fn map_color(color: BinaryColor) -> sys::Color {
        if color.is_on() {
            sys::ColorBlack
        } else {
            sys::ColorWhite
        }
    }
}
