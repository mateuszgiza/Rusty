extern crate chrono;
extern crate sdl2;
extern crate specs;

extern crate lazy_static;

#[macro_use]
extern crate specs_derive;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use specs::Dispatcher;
use std::time::{ Duration, Instant };

use sdl2::rect::Point;
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::video::{Window, WindowContext};

use specs::{Builder, DispatcherBuilder, World};

mod components;
use components::{ Draw, Position, Size, Text, Velocity, FPS };

mod systems;
use systems::{ DrawSystem, TextRenderSystem, UpdatePos, FpsCounter };

mod resources;
use resources::{CanvasHolder, DeltaTime, DrawContainer, WindowSize};

mod objects;
use objects::*;

mod common;
use common::fonts;
use common::fonts::ttf;

mod builders;
use builders::*;

mod helpers;
use helpers::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

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
    world.add_resource(DrawContainer::default());
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
                _ => {}
            }
        }

        canvas::proceed_on_canvas(&world, |canvas| {
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
        });

        dispatcher.dispatch(&mut world.res);
        world.maintain();

        canvas::proceed_on_canvas(&world, |canvas| canvas.present());

        timer.update();
    }

    println!("#: Closing Rusty...");
}

fn update_delta_time(world: &mut World, new_delta: Duration) {
    let mut delta = world.write_resource::<DeltaTime>();
    *delta = DeltaTime::new(Some(new_delta));
}

pub struct FrameTimer {
    timer: Instant,
    elapsed_time: Duration,
    calc_time: Duration,
    time_to_sleep: Duration
}

impl FrameTimer {
    const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000u64 / 60);

    pub fn new() -> Self {
        FrameTimer {
            timer: Instant::now(),
            elapsed_time: Duration::from_nanos(0),
            calc_time: Duration::from_nanos(0),
            time_to_sleep: Duration::from_nanos(0)
        }
    }

    pub fn elapsed_time(&self) -> Duration { self.elapsed_time }

    pub fn update(&mut self) {
        self.elapsed_time = self.timer.elapsed() + self.calc_time;
        let s = Instant::now();

        self.time_to_sleep = Duration::from_nanos(0);
        if Self::FRAME_TIME > self.elapsed_time {
            self.time_to_sleep = Self::FRAME_TIME - self.elapsed_time;
        }

        self.print();

        ::std::thread::sleep(self.time_to_sleep);

        self.calc_time = s.elapsed() - self.time_to_sleep;
        self.elapsed_time = self.timer.elapsed();
        self.timer = Instant::now();
    }

    fn print(&mut self) {
        println!("elapsed: {:?} | sleep: {:?} | calc: {:?} | sum: {:?}", self.elapsed_time, self.time_to_sleep, self.calc_time, self.elapsed_time + self.time_to_sleep);
    }
}