use specs::{ System, Write, ReadStorage };
use components::{ Position, Draw, Size };
use resources::CanvasHolder;
use extensions::*;

pub struct DrawSystem;

impl<'a> System<'a> for DrawSystem {
    type SystemData = (
        Write<'a, CanvasHolder>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Draw>
    );

    fn run (&mut self, data: Self::SystemData) {
        use specs::Join;
        use sdl2::rect::Rect;

        let (mut canvas_holder, pos, size, draw) = data;

        for (pos, size, draw) in (&pos, &size, &draw).join() {
            let rect = Rect::new(pos.x as i32, pos.y as i32, size.width as u32, size.height as u32);
            let color = draw.color;

            let canvas = canvas_holder.borrow().unwrap();
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).log_on_error("Could not draw on canvas!");
        }
    }
}