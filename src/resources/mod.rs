use std::any::Any;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Default)]
pub struct DeltaTime(pub f32); // Change to std::time::Duration

#[derive(Default)]
pub struct WindowSize(pub (u32, u32));

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

struct DrawInstruction {
    action: Box<Fn(&mut Canvas<Window>) + Send + Sync>,
    data: Any
}

#[derive(Default)]
pub struct DrawContainer {
    pub instructions: Vec<Box<Fn(&mut Canvas<Window>) + Send + Sync>>
}

impl DrawContainer {
    pub fn insert<F>(&mut self, draw_fn: F)
    where F: Fn(&mut Canvas<Window>) + 'static + Send + Sync {
        self.instructions.push(Box::new(draw_fn));
    }

    pub fn clear(&mut self) {
        self.instructions.clear();
    }
}