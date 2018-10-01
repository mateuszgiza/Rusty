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

pub mod components;
use components::{ Position, Velocity };

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

struct Draw {
    color: Color,
    rect: Rect
}

impl Default for Draw {
    fn default() -> Self { Draw { color: Color::RGB(0, 0, 0), rect: Rect::new(0, 0, 0, 0) } }
}

#[derive(Default)]
struct DrawContainer {
    instructions: Vec<Draw>
}

impl DrawContainer {
    fn insert(&mut self, draw: Draw) {
        self.instructions.push(draw);
    }

    fn clear(&mut self) {
        self.instructions.clear();
    }
}

#[derive(Default)]
struct DeltaTime(f32); // Change to std::time::Duration

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, DeltaTime>,
        Write<'a, DrawContainer>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>
    );

    fn run (&mut self, data: Self::SystemData) {
        use specs::Join;

        let (delta, mut draw_container, vel, mut pos) = data;
        let delta = delta.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;

            draw_container.insert(Draw { color: Color::RGB(255, 0, 0), rect: Rect::new(pos.x as i32, pos.y as i32, 100, 50) });
        }
    }
}