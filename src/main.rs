extern crate sdl2;
extern crate chrono;
extern crate specs;

extern crate lazy_static;

#[macro_use]
extern crate specs_derive;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use sdl2::ttf::*;
use sdl2::render::{ Canvas, TextureCreator, TextureQuery };
use sdl2::video::{ Window, WindowContext };

use specs::{World, Builder, DispatcherBuilder};

mod components;
use components::{ Position, Velocity, Draw, Size };

mod systems;
use systems::{ UpdatePos, DrawSystem };

mod resources;
use resources::{ DeltaTime, DrawContainer, WindowSize };

mod objects;
use objects::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let window_size = window.size();
    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    // ECS

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Draw>();
    world.register::<Size>();

    world.add_resource(DeltaTime(0.05));
    world.add_resource(DrawContainer::default());
    world.add_resource(WindowSize(window_size));

    world.create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .with(Size { width: 50, height: 50 })
        .with(Draw { color: Color::RGB(0, 255, 0) })
        .build();
    world.create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 2.0, y: 1.0 })
        .with(Size { width: 100, height: 50 })
        .with(Draw { color: Color::RGB(255, 0, 0) })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(UpdatePos, "update_pos", &[])
        .with(DrawSystem, "draw_system", &["update_pos"])
        .build();

    update_delta_time(&mut world, 1.0);

    // end ECS

    let font_context = sdl2::ttf::init().expect("could not initialize TtfContext");
    let mut font_manager = FontManager::new(&font_context);
    font_manager.load_fonts(vec![String::from("SpaceMono-Regular.ttf")], 24);

    let font = font_manager.get_font(String::from("SpaceMono-Regular"));
    let font_color = Color::RGB(255, 255, 255);

    // TEXT
    let message_render = font.render("Font test");
    let message_surface = message_render.solid(font_color).expect("error rendering message");

    // TEXT::CREATE
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
    let message_texture = texture_creator.create_texture_from_surface(&message_surface).expect("could not create texture from surface");
    let texture_query = message_texture.query();
    let message_target = sdl2::rect::Rect::new(50, 50, texture_query.width, texture_query.height);
    
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

        let draw_container = world.read_resource::<DrawContainer>();

        for draw in &draw_container.instructions {
            (*draw)(&mut canvas);
        }

        canvas.copy(&message_texture, None, Some(message_target)).expect("could not copy texture to canvas");

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("#: Closing Rusty...");
}

fn update_delta_time(world: &mut World, new_delta: f32) {
    let mut delta = world.write_resource::<DeltaTime>();
    *delta = DeltaTime(new_delta);
}