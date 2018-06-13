use game_engine::IGameEngine;

use common::logger;
use handlers::*;
use entities::Ball;
use components::PlayerInputComponent;
use systems::*;
use std::cell::RefCell;

use sfml::graphics::*;
use sfml::window::*;

struct GameEngine;

impl IGameEngine for GameEngine {
    fn start(&self) {
        logger::info("Rusty Game Engine starting...");
        self.create_window();
    }
}

impl GameEngine {
    fn new() -> Box<IGameEngine> {
        Box::new(GameEngine {})
    }

    fn create_window(&self) {
        let mut window = RenderWindow::new((800, 600), "Rusty", Style::CLOSE, &Default::default());

        window.set_vertical_sync_enabled(true);

        let mut handler = SfmlWindowEventHandler::new();
        let mut ball = RefCell::new(Ball::new().base);
        let playerInputComponent = PlayerInputComponent::new();

        let mut entities = vec![ball];

        while window.is_open() {
            handler.handle_events(&mut window);
            playerInputComponent.update(&mut entities[0].borrow_mut());
            EntityShapePositionUpdateSystem::update(&entities);

            window.clear(&Color::BLACK);

            EntityRenderSystem::render(&mut window, &entities);

            window.display();
        }
    }
}

pub fn create_game_engine() -> Box<IGameEngine> {
    GameEngine::new()
}
