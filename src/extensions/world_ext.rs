use sdl2::video::Window;
use sdl2::render::Canvas;
use specs::World;
use resources::CanvasHolder;
use extensions::CanvasHolderExt;

pub trait WorldExt {
    fn proceed_on_canvas<F>(&self, canvas_action: F) where F: Fn(&mut Canvas<Window>);
}

impl WorldExt for World {
    fn proceed_on_canvas<F>(&self, canvas_action: F) where F: Fn(&mut Canvas<Window>) {
        self.write_resource::<CanvasHolder>().proceed(canvas_action);
    }
}