extern crate sdl2;
extern crate chrono;
extern crate specs;

#[macro_use]
extern crate lazy_static;

use specs::Write;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

use specs::{World, Builder, System, ReadStorage, WriteStorage, DispatcherBuilder, Read};

mod components;
use components::{ Position, Velocity };

mod systems;
use systems::{ UpdatePos, HelloWorld };

mod resources;
use resources::{ DeltaTime, Draw, DrawContainer };

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    // ECS

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    world.add_resource(DeltaTime(0.05));
    world.add_resource(DrawContainer::default());

    world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
    world.create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 2.0, y: 1.0 })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        // .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &[])
        // .with(HelloWorld, "hello_updated", &["update_pos"])
        .build();

    update_delta_time(&mut world, 1.0);

    // end ECS

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        world.write_resource::<DrawContainer>().clear();

        dispatcher.dispatch(&mut world.res);
        world.maintain();

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        let drawContainer = world.read_resource::<DrawContainer>();

        for draw in &drawContainer.instructions {
            canvas.set_draw_color(draw.color);
            canvas.fill_rect(draw.rect);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("#: Closing Rusty...");
}

fn update_delta_time(world: &mut World, new_delta: f32) {
    let mut delta = world.write_resource::<DeltaTime>();
    *delta = DeltaTime(new_delta);
}