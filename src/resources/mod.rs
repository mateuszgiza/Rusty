use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Default)]
pub struct DeltaTime(pub f32); // Change to std::time::Duration

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