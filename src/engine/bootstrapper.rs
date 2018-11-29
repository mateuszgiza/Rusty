use std::error::Error;

use sdl2::{
    Sdl,
    video::Window
};

use {
    engine::Context,
    resources::Cursor
};

pub struct Bootstrapper {

}

impl Bootstrapper {
    pub fn initialize<'a>() -> Result<Context<'a>, Box<Error>> {
        let sdl_context = sdl2::init()?;
        let window = create_window(&sdl_context)?;
        let cursor = Cursor::new(sdl_context.mouse());

        let context = Context {
            sdl_context,
            window,
            cursor,
            _marker: std::marker::PhantomData
        };

        Ok(context)
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