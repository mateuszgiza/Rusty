#![allow(dead_code)]

use sdl2::mouse::MouseUtil;

#[derive(Default)]
pub struct CursorData {
    mouse: Option<MouseUtil>
}

unsafe impl Send for CursorData {}
unsafe impl Sync for CursorData {}

impl CursorData {
    pub fn new(mouse: MouseUtil) -> CursorData {
        CursorData {
            mouse: Some(mouse)
        }
    }

    pub fn show_system(&mut self) {
        self.mouse.as_mut().unwrap().show_cursor(true);
    }

    pub fn hide_system(&mut self) {
        self.mouse.as_mut().unwrap().show_cursor(false);
    }
}
