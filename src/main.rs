mod game_engine;
mod common;
mod handlers;
mod entities;
mod components;
mod systems;

extern crate sfml;
extern crate chrono;

#[macro_use]
extern crate lazy_static;

fn main() {
    game_engine::start();

    println!("#: Closing Rusty...");
}
