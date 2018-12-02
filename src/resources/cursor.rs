#![allow(dead_code)]

use sdl2::mouse::MouseUtil;

#[derive(Default)]
pub struct Cursor {
    mouse: Option<MouseUtil>
}

unsafe impl Send for Cursor {}
unsafe impl Sync for Cursor {}

impl Cursor {
    pub fn new(mouse: MouseUtil) -> Cursor {
        Cursor {
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
