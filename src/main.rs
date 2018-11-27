extern crate chrono;
extern crate sdl2;
extern crate specs;
extern crate sdl2_extras;
extern crate lazy_static;

#[macro_use]
extern crate specs_derive;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::Duration;

use specs::{ Dispatcher, Builder, DispatcherBuilder, World };

mod components;
use components::{ Draw, Position, Size, Text, Velocity, FPS };

mod systems;
use systems::{ DrawSystem, TextRenderSystem, UpdatePos, FpsCounter };

mod resources;
use resources::{ WindowSize };

mod common;
use common::{ FontType, FrameTimer };

mod builders;
use builders::*;

mod extensions;
use extensions::*;

use sdl2_extras::adapters::CanvasAdapter;
use sdl2_extras::common::GameTime;
use sdl2_extras::managers::FontManager;
use sdl2_extras::fspecs::WorldExt;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let system_cursor = sdl_context.mouse();
    let video_subsystem = sdl_context.video().unwrap();

    system_cursor.show_cursor(false);

    let window = video_subsystem
        .window("rust demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let window_size = window.size();
    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let font_context = sdl2::ttf::init().expect("could not initialize TtfContext");
    let mut font_manager = FontManager::new(&font_context);
    font_manager.load(&FontType::SpaceMonoRegular24.get_details()).expect("Could not load font!");

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

    world.create_entity()
        .with(Position { x: 0.0, y: 0.0 })
        .with(FPS::new(Duration::from_secs(1)))
        .with(Text { text: "FPS: 0".to_string(), offset: Point::new(0, 0), color: font_color, font: FontType::SpaceMonoRegular24 })
        .build();
    world.create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .with(Size { width: 50, height: 50 })
        .with(Draw { color: Color::RGB(0, 255, 0)})
        .build();
    world.create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 50.0, y: 30.0 })
        .with(Size { width: 100, height: 50 })
        .with(Draw { color: Color::RGB(255, 0, 0) })
        .with(Text { text: "Elo xD".to_string(), offset: Point::new(0, -50), color: font_color, font: FontType::SpaceMonoRegular24 })
        .build();

    let mut dispatcher: Dispatcher<'_, '_> = DispatcherBuilder::new()
        .with(FpsCounter::new(), "fps_counter", &[])
        .with(UpdatePos, "update_pos", &[])
        .with(DrawSystem, "draw_system", &["update_pos"])
        .with_thread_local(TextRenderSystem::new(text_builder))
        .build();

    // end ECS

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut timer = FrameTimer::new();
    timer.is_sleep_enabled = false;
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(60).log_on_error("Could not set framerate!");
    println!("Current framerate: {}", fps_manager.get_framerate());

    use sdl2::image::*;

    let texture_creator = world.write_resource::<CanvasAdapter>().borrow().unwrap().texture_creator();
    let image_texture = texture_creator.load_texture("cursor.png").expect("Cursor could not loaded");
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
                },
                _ => {}
            }
        }

        world.proceed_on_canvas(|canvas| {
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
        });

        dispatcher.dispatch(&mut world.res);
        world.maintain();

        world.proceed_on_canvas(|canvas| {
            canvas.copy(&image_texture, None, Some(cursor_rect)).log_on_error("Could not draw cursor on canvas!");
            canvas.present();
        });

        timer.update();
        fps_manager.delay();
    }

    println!("#: Closing Rusty...");
}