mod bootstrapper;

use colored::*;
use log::{error, info, trace, warn};
use sdl2::{pixels::Color, rect::Point};
use sdl2_extras::fspecs::WorldExt;
use specs::{Builder, DispatcherBuilder, Write};
use std::{error::Error, time::Duration};
use {
    common::{FontType, FrameTimer},
    components::{Cursor, Draw, Position, Size, Sprite, Text, Velocity, FPS},
    configuration::Configurator,
    events::EventState,
    extensions::ResultExt,
    managers::{EventManager, EventProcessStatus},
    systems::{CursorDrawSystem, CursorUpdateSystem, DrawSystem, FpsCounter, TextRenderSystem, UpdatePos},
};

pub fn start() -> Result<(), Box<Error>> {
    let context = bootstrapper::initialize()
        .on_success(|_| trace!("{}", "Engine initialization succeeded!".green()))
        .on_error(|e| error!("Engine initialization error: {}", e))?;
    let mut world = bootstrapper::create_world(context)?;

    // ECS
    Configurator::register_components(&mut world);

    world
        .proceed_on_canvas(|canvas| {
            canvas.set_draw_color(Color::RGB(0, 255, 255));
            canvas.clear();
            canvas.present();
        }).discard_result();

    let font_color = Color::RGB(255, 255, 255);

    world
        .create_entity()
        .with(Cursor)
        .with(Position { x: 0.0, y: 0.0 })
        .with(Size { width: 32, height: 32 })
        .with(Sprite {
            texture_name: "cursor.png".into(),
            color: Color::RGB(0, 0, 0),
        }).build();
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
        .with(Size { width: 50, height: 50 })
        .with(Draw {
            color: Color::RGB(0, 255, 0),
        }).build();
    world
        .create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 50.0, y: 30.0 })
        .with(Size { width: 100, height: 50 })
        .with(Draw {
            color: Color::RGB(255, 0, 0),
        }).with(Text {
            text: "Elo xD".to_string(),
            offset: Point::new(0, -50),
            color: font_color,
            font: FontType::SpaceMonoRegular24,
        }).build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(FpsCounter::new(), "fps_counter", &[])
        .with(CursorUpdateSystem, "cursor_update", &[])
        .with(UpdatePos, "update_pos", &[])
        .with(DrawSystem, "draw_system", &["update_pos"])
        .with(TextRenderSystem, "text_render", &[])
        .with(CursorDrawSystem, "cursor_draw", &[])
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

    Configurator::setup_event_handlers(&world);

    'running: loop {
        world.update_delta_time(timer.elapsed_time());

        let event_process_result = world.exec(
            |(mut event_state, mut event_manager): (Write<EventState>, Write<EventManager>)| {
                event_manager.process_events(&mut event_state)
            },
        );

        if let EventProcessStatus::Exit = event_process_result {
            break 'running;
        }

        world
            .proceed_on_canvas(|canvas| {
                canvas.set_draw_color(Color::RGB(39, 58, 93));
                canvas.clear();
            }).discard_result();

        dispatcher.dispatch(&mut world.res);
        world.maintain();

        world
            .proceed_on_canvas(|canvas| {
                canvas.present();
            }).discard_result();

        timer.update();
        fps_manager.delay();

        world.write_resource::<EventState>().clear_events();
    }

    Ok(())
}
