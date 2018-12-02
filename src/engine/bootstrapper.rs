use std::error::Error;
use sdl2::{
    Sdl,
    video::Window
};
use sdl2_extras::{
    adapters::CanvasAdapter
};
use specs::World;
use {
    resources::{Cursor, WindowSize, EventManager}
};

pub struct Bootstrapper;

impl Bootstrapper {
    pub fn initialize<'a>() -> Result<World, Box<Error>> {
        let sdl_context = sdl2::init()?;

        let window = create_window(&sdl_context)?;
        let window_size = WindowSize(window.size());
        
        let canvas = window.into_canvas().build()?;
        let canvas_adapter = CanvasAdapter::new(Some(canvas));
        
        let event_manager = EventManager::new(&sdl_context)?;

        let mut cursor = Cursor::new(sdl_context.mouse());
        cursor.hide_system();

        let mut world = World::new();
        world.add_resource(window_size);
        world.add_resource(canvas_adapter);
        world.add_resource(event_manager);
        world.add_resource(cursor);

        Ok(world)
    }
}

fn create_window(sdl_context: &Sdl) -> Result<Window, Box<Error>> {
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust demo", 800, 600)
        .position_centered()
        .build()?;

    Ok(window)
}