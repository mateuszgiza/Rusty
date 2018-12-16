use log::warn;
use specs::{ System, Write, ReadStorage };
use components::{ Cursor, Position, Size, Sprite };
use extensions::ResultExt;
use sdl2_extras::adapters::{CanvasAdapter, ResourceFacade};

pub struct CursorDrawSystem;

impl<'a> System<'a> for CursorDrawSystem {
    type SystemData = (
        Write<'a, CanvasAdapter>,
        Write<'a, ResourceFacade<'static>>,
        ReadStorage<'a, Cursor>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Sprite>
    );

    fn run (&mut self, data: Self::SystemData) {
        use specs::Join;
        use sdl2::rect::Rect;

        let (mut canvas_adapter, mut resource_facade, cursor, pos, size, sprite) = data;

        for (_cursor, pos, size, sprite) in (&cursor, &pos, &size, &sprite).join() {
            let rect = Rect::new(pos.x as i32, pos.y as i32, size.width as u32, size.height as u32);
            let texture = resource_facade.get_texture(&sprite.texture_name).unwrap();

            canvas_adapter.proceed(|canvas| {
                canvas.copy(&texture, None, Some(rect))
                    .on_error(|_| warn!("Could not draw cursor on canvas!"))
                    .discard_result();
            }).discard_result();
        }
    }
}