mod bootstrapper;

use colored::*;
use log::{trace, info, warn, error};
use std::{
    error::Error,
    time::Duration
};
use specs::{ Builder, DispatcherBuilder };
use sdl2::{
    pixels::Color,
    rect::Point,
    event::{Event, EventType},
    keyboard::Keycode
};
use sdl2_extras::{
    managers::TextureManager,
    fspecs::WorldExt
};
use {
    common::{ FontType, FrameTimer },
    components::{ Draw, Position, Size, Text, Velocity, FPS },
    extensions::ResultExt,
    managers::{EventManager, EventProcessStatus, EventState},
    systems::{ DrawSystem, TextRenderSystem, UpdatePos, FpsCounter }
};

pub fn start() -> Result<(), Box<Error>> {
    let context = bootstrapper::initialize()
        .on_success(|_| trace!("{}", "Engine initialization succeeded!".green()))
        .on_error(|e| error!("Engine initialization error: {}", e))?;
    let mut world = bootstrapper::create_world(context)?;

    // ECS
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Draw>();
    world.register::<Size>();
    world.register::<Text>();
    world.register::<FPS>();

    world.proceed_on_canvas(|canvas| {
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
    }).discard_result();

    let texture_creator = world.get_texture_creator()?;
    let mut texture_manager = TextureManager::new(&texture_creator);

    let font_color = Color::RGB(255, 255, 255);

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
        .with(TextRenderSystem, "text_render", &[])
        .build();

    // end ECS

    let mut timer = FrameTimer::new();
    timer.is_sleep_enabled = false;
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager
        .set_framerate(60)
        .on_success(|_| info!("Current framerate: {}", fps_manager.get_framerate()))
        .on_error(|_| warn!("Could not set framerate!"))
        .discard_result();

    let image_texture = texture_manager.load("cursor.png").on_error(|_| error!("Could not load cursor file!"))?;
    let cursor_rect = sdl2::rect::Rect::new(0, 0, 32, 32);

    'running: loop {
        world.update_delta_time(timer.elapsed_time());

        {
            let mut event_manager = world.write_resource::<EventManager>();
            event_manager.register(EventType::Quit, Box::new(on_quit));
            event_manager.register(EventType::KeyDown, Box::new(on_quit));

            let event_process_result = event_manager.process_events();
            if let EventProcessStatus::Exit = event_process_result {
                break 'running;
            }
        }

        world.proceed_on_canvas(|canvas| {
            canvas.set_draw_color(Color::RGB(39, 58, 93));
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

fn on_quit(state: &EventState, event: &Event) -> EventProcessStatus {
    if let Event::Quit {..} = event {
        return EventProcessStatus::Exit;
    }
    else if let Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
        return EventProcessStatus::Exit;
    }

    EventProcessStatus::Ok
}

fn event_handler_cursor_move(state: &EventState, event: &Event) -> EventProcessStatus {
    if let Event::MouseMotion {x, y, ..} = event {
        //
    }

    EventProcessStatus::Ok
}

pub enum GameEvent {
    CursorMove {x: i32, y: i32},
}
