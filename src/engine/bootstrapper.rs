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
    resources::{Cursor, WindowSize}
};

pub struct Bootstrapper;

impl Bootstrapper {
    pub fn initialize<'a>() -> Result<World, Box<Error>> {
        let sdl_context = sdl2::init()?;

        let window = create_window(&sdl_context)?;
        let window_size = WindowSize(window.size());
        
        let canvas = window.into_canvas().build()?;
        let canvas = CanvasAdapter::new(Some(canvas));

        let cursor = Cursor::new(sdl_context.mouse());
        cursor.hide_system();

        let mut world = World::new();

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