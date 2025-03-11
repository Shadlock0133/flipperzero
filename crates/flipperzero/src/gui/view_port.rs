use core::{ffi::c_void, mem::MaybeUninit, ops::Deref, ptr::NonNull};

use alloc::boxed::Box;

use super::canvas::Canvas;
use flipperzero_sys as sys;

use super::Gui;

#[derive(Clone, Copy)]
pub struct InputEvent {
    pub type_: InputType,
    pub key: InputKey,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Press,
    Release,
    Short,
    Long,
    Repeat,
    Unknown(u8),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputKey {
    Up,
    Down,
    Right,
    Left,
    Ok,
    Back,
    Unknown(u8),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    HorizontalFlip,
    Vertical,
    VerticalFlip,
}

type DrawCallback<'a> = dyn Fn(&Canvas) + 'a;
type InputCallback<'a> = dyn Fn(InputEvent) + 'a;
type ThinBox<T> = Box<Box<T>>;

pub struct ViewPort<'a> {
    hnd: NonNull<sys::ViewPort>,
    draw_cb: Option<ThinBox<DrawCallback<'a>>>,
    input_cb: Option<ThinBox<InputCallback<'a>>>,
}

impl Drop for ViewPort<'_> {
    fn drop(&mut self) {
        unsafe { self.free() }
    }
}

impl<'a> ViewPort<'a> {
    #[doc(alias = "view_port_alloc")]
    pub fn new() -> Self {
        let hnd = unsafe { NonNull::new_unchecked(sys::view_port_alloc()) };
        Self {
            hnd,
            draw_cb: None,
            input_cb: None,
        }
    }

    pub fn as_ptr(&self) -> *mut sys::ViewPort {
        self.hnd.as_ptr()
    }

    /// # Safety
    /// Only call once if manually dropping
    pub unsafe fn free(&mut self) {
        unsafe { sys::view_port_free(self.as_ptr()) }
    }

    pub fn set_draw_callback(&mut self, f: impl Fn(&Canvas) + 'a) {
        type CallbackStorage<'a> = Box<DrawCallback<'a>>;
        unsafe extern "C" fn draw_cb(canvas: *mut sys::Canvas, state: *mut c_void) {
            let canvas = unsafe { Canvas::from_raw(canvas) };
            let f = unsafe { &*state.cast::<CallbackStorage>() };
            f(canvas)
        }
        let state = self.draw_cb.insert(Box::new(Box::new(f)));
        let state_ptr: *mut CallbackStorage = &raw mut **state;
        unsafe { sys::view_port_draw_callback_set(self.as_ptr(), Some(draw_cb), state_ptr.cast()) }
    }

    pub fn set_input_callback(&mut self, f: impl Fn(InputEvent) + 'a) {
        type CallbackStorage<'a> = Box<InputCallback<'a>>;
        unsafe extern "C" fn input_cb(input: *mut sys::InputEvent, state: *mut c_void) {
            let input = unsafe { *input };
            let type_ = match input.type_ {
                sys::InputTypePress => InputType::Press,
                sys::InputTypeRelease => InputType::Release,
                sys::InputTypeShort => InputType::Short,
                sys::InputTypeLong => InputType::Long,
                sys::InputTypeRepeat => InputType::Repeat,
                sys::InputType(x) => InputType::Unknown(x),
            };
            let key = match input.key {
                sys::InputKeyUp => InputKey::Up,
                sys::InputKeyDown => InputKey::Down,
                sys::InputKeyRight => InputKey::Right,
                sys::InputKeyLeft => InputKey::Left,
                sys::InputKeyOk => InputKey::Ok,
                sys::InputKeyBack => InputKey::Back,
                sys::InputKey(x) => InputKey::Unknown(x),
            };
            let input = InputEvent { type_, key };
            let f = unsafe { &*state.cast::<CallbackStorage>() };
            f(input)
        }
        let state = self.input_cb.insert(Box::new(Box::new(f)));
        let state_ptr: *mut CallbackStorage = &raw mut **state;
        unsafe {
            sys::view_port_input_callback_set(self.as_ptr(), Some(input_cb), state_ptr.cast())
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe { sys::view_port_enabled_set(self.as_ptr(), enabled) };
    }

    pub fn set_orientation(&self, orientation: Orientation) {
        let orientation = match orientation {
            Orientation::Horizontal => sys::ViewPortOrientationHorizontal,
            Orientation::HorizontalFlip => sys::ViewPortOrientationHorizontalFlip,
            Orientation::Vertical => sys::ViewPortOrientationVertical,
            Orientation::VerticalFlip => sys::ViewPortOrientationVerticalFlip,
        };
        unsafe { sys::view_port_set_orientation(self.as_ptr(), orientation) };
    }

    pub fn update(&self) {
        unsafe { sys::view_port_update(self.as_ptr()) }
    }
}

pub struct BoundViewPort<'g, 'a> {
    pub(super) view_port: ViewPort<'a>,
    pub(super) gui: &'g Gui,
}

impl<'a> BoundViewPort<'_, 'a> {
    /// Safety:
    /// Call this no more than once if dropping manually
    pub unsafe fn remove_from_gui_raw(&mut self) {
        unsafe { sys::gui_remove_view_port(self.gui.as_ptr(), self.view_port.as_ptr()) }
    }

    pub fn remove_from_gui(mut self) -> ViewPort<'a> {
        unsafe { self.remove_from_gui_raw() };
        let mut this = MaybeUninit::new(self);
        let view_port_ptr = unsafe { &raw const (*this.as_mut_ptr()).view_port };
        unsafe { view_port_ptr.read() }
    }
}

impl<'a> Deref for BoundViewPort<'_, 'a> {
    type Target = ViewPort<'a>;

    fn deref(&self) -> &Self::Target {
        &self.view_port
    }
}

impl Drop for BoundViewPort<'_, '_> {
    fn drop(&mut self) {
        unsafe { self.remove_from_gui_raw() }
    }
}
