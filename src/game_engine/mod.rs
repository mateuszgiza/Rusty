mod gameEngine;

pub trait IGameEngine {
    fn start(&self);
}

pub fn start() {
    let engine = gameEngine::create_game_engine();
    engine.start();
}
