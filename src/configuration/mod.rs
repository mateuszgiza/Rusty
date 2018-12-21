use events::handlers;
use managers::EventManager;
use sdl2::event::EventType;
use specs::World;

pub struct Configurator;

impl Configurator {
    pub fn register_components(world: &World) {
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Draw>();
        world.register::<Size>();
        world.register::<Text>();
        world.register::<FPS>();
        world.register::<Cursor>();
        world.register::<Sprite>();
    }

    pub fn setup_event_handlers(world: &World) {
        let mut event_manager = world.write_resource::<EventManager>();

        event_manager.register(EventType::Quit, Box::new(handlers::on_quit));
        event_manager.register(EventType::KeyDown, Box::new(handlers::on_quit));
        event_manager.register(EventType::MouseMotion, Box::new(handlers::event_handler_cursor_move));
    }
}
