use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct CanvasHolder {
    canvas: Option<Canvas<Window>>
}

unsafe impl Send for CanvasHolder {}
unsafe impl Sync for CanvasHolder {}

impl<'a> CanvasHolder {
    pub fn new(canvas: Option<Canvas<Window>>) -> Self {
        CanvasHolder {
            canvas: canvas
        }
    }

    pub fn borrow(&mut self) -> Option<&mut Canvas<Window>> {
        return self.canvas.as_mut();
    }
}

impl Default for CanvasHolder {
    fn default() -> Self { CanvasHolder::new(None) }
}