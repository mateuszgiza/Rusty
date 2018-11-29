mod bootstrapper;
use self::bootstrapper::Bootstrapper;

mod context;
pub use self::context::Context;

use colored::*;
use log::{trace, info, warn, error};
use std::{
    error::Error,
    time::Duration
};
use specs::{ Builder, DispatcherBuilder, World };
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Point
};
use sdl2_extras::{
    adapters::CanvasAdapter,
    common::GameTime,
    managers::{FontManager, TextureManager},
    fspecs::WorldExt
};
use {
    builders::TextBuilder,
    common::{ FontType, FrameTimer },
    components::{ Draw, Position, Size, Text, Velocity, FPS },
    extensions::ResultExt,
    resources::WindowSize,
    systems::{ DrawSystem, TextRenderSystem, UpdatePos, FpsCounter }
};

pub fn start() -> Result<(), Box<Error>> {
    let mut context = Bootstrapper::initialize()
        .on_success(|_| trace!("{}", "Engine initialization succeeded!".green()))
        .on_error(|e| error!("Engine initialization error: {}", e))?;
    
    context.cursor.hide_system();

    let window_size = context.window.size();
    let mut canvas = context.window.into_canvas().build()?;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let font_context = sdl2::ttf::init()?;
    let mut font_manager = FontManager::new(&font_context);
    font_manager.load(&FontType::SpaceMonoRegular24.get_details())?;

    let text_builder = TextBuilder::new(&canvas, &mut font_manager);
    let font_color = Color::RGB(255, 255, 255);

    // ECS

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Draw>();
    world.register::<Size>();
    world.register::<Text>();
    world.register::<FPS>();

    world.add_resource(GameTime::default());
    world.add_resource(WindowSize(window_size));
    world.add_resource(CanvasAdapter::new(Some(canvas)));

    let texture_creator = world.get_texture_creator()?;
    let mut texture_manager = TextureManager::new(&texture_creator);

    world
        .create_entity()
        .with(Position { x: 0.0, y: 0.0 })
        .with(FPS::new(Duration::from_secs(1)))
        .with(Text {
            text: "FPS: 0".to_string(),
            offset: Point::new(0, 0),
            color: font_color,
            font: FontType::SpaceMonoRegular24,
        }).build();
    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .with(Size {
            width: 50,
            height: 50,
        }).with(Draw {
            color: Color::RGB(0, 255, 0),
        }).build();
    world
        .create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 50.0, y: 30.0 })
        .with(Size {
            width: 100,
            height: 50,
        }).with(Draw {
            color: Color::RGB(255, 0, 0),
        }).with(Text {
            text: "Elo xD".to_string(),
            offset: Point::new(0, -50),
            color: font_color,
            font: FontType::SpaceMonoRegular24,
        }).build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(FpsCounter::new(), "fps_counter", &[])
        .with(UpdatePos, "update_pos", &[])
        .with(DrawSystem, "draw_system", &["update_pos"])
        .with_thread_local(TextRenderSystem::new(text_builder))
        .build();

    // end ECS

    let mut event_pump = context.sdl_context.event_pump()?;
    let mut i = 0;
    let mut timer = FrameTimer::new();
    timer.is_sleep_enabled = false;
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager
        .set_framerate(60)
        .on_success(|_| info!("Current framerate: {}", fps_manager.get_framerate()))
        .on_error(|_| warn!("Could not set framerate!"))
        .discard_result();

    let image_texture = texture_manager.load("cursor.png").on_error(|_| error!("Could not load cursor file!"))?;
    let mut cursor_rect = sdl2::rect::Rect::new(0, 0, 32, 32);

    'running: loop {
        world.update_delta_time(timer.elapsed_time());

        i = (i + 1) % 255;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    cursor_rect.set_x(x);
                    cursor_rect.set_y(y);
                }
                _ => {}
            }
        }

        world.proceed_on_canvas(|canvas| {
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
        }).discard_result();

        dispatcher.dispatch(&mut world.res);
        world.maintain();

        world.proceed_on_canvas(|canvas| {
            canvas.copy(&image_texture, None, Some(cursor_rect))
                .on_error(|_| warn!("Could not draw cursor on canvas!"))
                .discard_result();
            canvas.present();
        }).discard_result();

        timer.update();
        fps_manager.delay();
    }

    Ok(())
}
