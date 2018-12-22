use components::{Cursor, Draw, Position, Size, Sprite, Text, Velocity, FPS};
use events::handlers;
use managers::EventManager;
use sdl2::event::EventType;
use sdl2::gfx::framerate::FPSManager;
use specs::World;

use log::{info, warn};
use sdl2::{pixels::Color, rect::Point};
use specs::{Builder, Dispatcher, DispatcherBuilder};
use std::time::Duration;
use {
    common::{FontType, FrameTimer},
    extensions::ResultExt,
    systems::{CursorDrawSystem, CursorUpdateSystem, DrawSystem, FpsCounter, TextRenderSystem, UpdatePos},
};

pub struct Configurator;

impl Configurator {
    pub fn setup_timers() -> (FrameTimer, FPSManager) {
        let mut timer = FrameTimer::new();
        timer.is_sleep_enabled = false;
        let mut fps_manager = FPSManager::new();
        fps_manager
            .set_framerate(60)
            .on_success(|_| info!("Current framerate: {}", fps_manager.get_framerate()))
            .on_error(|_| warn!("Could not set framerate!"))
            .discard_result();

        return (timer, fps_manager);
    }

    pub fn register_components(world: &mut World) {
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Draw>();
        world.register::<Size>();
        world.register::<Text>();
        world.register::<FPS>();
        world.register::<Cursor>();
        world.register::<Sprite>();
    }

    pub fn setup_systems<'a, 'b>() -> Dispatcher<'a, 'b> {
        DispatcherBuilder::new()
            .with(FpsCounter::new(), "fps_counter", &[])
            .with(CursorUpdateSystem, "cursor_update", &[])
            .with(UpdatePos, "update_pos", &[])
            .with(DrawSystem, "draw_system", &["update_pos"])
            .with(TextRenderSystem, "text_render", &[])
            .with(CursorDrawSystem, "cursor_draw", &[])
            .build()
    }

    pub fn setup_event_handlers(world: &World) {
        let mut event_manager = world.write_resource::<EventManager>();

        event_manager.register(EventType::Quit, Box::new(handlers::on_quit));
        event_manager.register(EventType::KeyDown, Box::new(handlers::on_quit));
        event_manager.register(EventType::MouseMotion, Box::new(handlers::event_handler_cursor_move));
    }

    pub fn setup_entities(world: &mut World) {
        world
            .create_entity()
            .with(Cursor)
            .with(Position { x: 0.0, y: 0.0 })
            .with(Size { width: 32, height: 32 })
            .with(Sprite {
                texture_name: "cursor.png".into(),
                color: Color::RGB(0, 0, 0),
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 0.0, y: 0.0 })
            .with(FPS::new(Duration::from_secs(1)))
            .with(Text {
                text: "FPS: 0".to_string(),
                offset: Point::new(0, 0),
                color: Color::RGB(255, 255, 255),
                font: FontType::SpaceMonoRegular24,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 4.0, y: 7.0 })
            .with(Size { width: 50, height: 50 })
            .with(Draw {
                color: Color::RGB(0, 255, 0),
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 2.0, y: 5.0 })
            .with(Velocity { x: 50.0, y: 30.0 })
            .with(Size { width: 100, height: 50 })
            .with(Draw {
                color: Color::RGB(255, 0, 0),
            })
            .with(Text {
                text: "Elo xD".to_string(),
                offset: Point::new(0, -50),
                color: Color::RGB(255, 255, 255),
                font: FontType::SpaceMonoRegular24,
            })
            .build();
    }
}
