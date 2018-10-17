use sdl2::video::Window;
use sdl2::render::Canvas;
use specs::World;
use resources::CanvasHolder;

pub fn proceed_on_canvas<F>(world: &World, canvas_action: F)
where F: Fn(&mut Canvas<Window>) {
    let mut canvas_holder = world.write_resource::<CanvasHolder>();
    let mut canvas = canvas_holder.borrow().unwrap();
    
    canvas_action(&mut canvas);
}