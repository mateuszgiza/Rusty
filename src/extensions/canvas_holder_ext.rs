use sdl2::video::Window;
use sdl2::render::Canvas;
use resources::CanvasHolder;

pub trait CanvasHolderExt {
    fn proceed<F>(&mut self, canvas_action: F) where F: Fn(&mut Canvas<Window>);
}

impl CanvasHolderExt for CanvasHolder {
    fn proceed<F>(&mut self, canvas_action: F) where F: Fn(&mut Canvas<Window>) {
        let mut canvas = self.borrow().unwrap();
        canvas_action(&mut canvas);
    }
}