extern crate chrono;
extern crate sdl2;
extern crate specs;

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
use resources::{ CanvasHolder, DeltaTime, WindowSize };

mod objects;
use objects::*;

mod common;
use common::FrameTimer;
use common::fonts;
use common::fonts::ttf;

mod builders;
use builders::*;

mod helpers;
use helpers::*;

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
    font_manager.load_fonts(vec![ttf(fonts::SPACE_MONO_REGULAR)], 24);

    let text_builder = TextBuilder::new(&canvas, &font_manager);
    let font_color = Color::RGB(255, 255, 255);

    // ECS

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Draw>();
    world.register::<Size>();
    world.register::<Text>();
    world.register::<FPS>();

    world.add_resource(DeltaTime::new(None));
    world.add_resource(WindowSize(window_size));
    world.add_resource(CanvasHolder::new(Some(canvas)));

    world.create_entity()
        .with(Position { x: 0.0, y: 0.0 })
        .with(FPS::new(Duration::from_secs(1)))
        .with(Text { text: "FPS: 0".to_string(), offset: Point::new(0, 0), color: font_color, font: fonts::SPACE_MONO_REGULAR.to_string()})
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
        .with(Text { text: "Elo xD".to_string(), offset: Point::new(0, -50), color: font_color, font: fonts::SPACE_MONO_REGULAR.to_string()})
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

    use sdl2::image::*;

    let texture_creator = world.write_resource::<CanvasHolder>().borrow().unwrap().texture_creator();
    let image_texture = texture_creator.load_texture("cursor.png").unwrap();
    let mut cursor_rect = sdl2::rect::Rect::new(0, 0, 32, 32);

    'running: loop {
        update_delta_time(&mut world, timer.elapsed_time());

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

        canvas::proceed_on_canvas(&world, |canvas| {
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
        });

        dispatcher.dispatch(&mut world.res);
        world.maintain();

        canvas::proceed_on_canvas(&world, |canvas| {
            canvas.copy(&image_texture, None, Some(cursor_rect));
            canvas.present();
        });

        timer.update();
    }

    println!("#: Closing Rusty...");
}

fn update_delta_time(world: &mut World, new_delta: Duration) {
    let mut delta = world.write_resource::<DeltaTime>();
    *delta = DeltaTime::new(Some(new_delta));
}