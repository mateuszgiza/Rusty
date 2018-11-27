extern crate chrono;
extern crate sdl2;
extern crate specs;
extern crate sdl2_extras;
extern crate lazy_static;
#[macro_use] extern crate specs_derive;

mod builders;
mod common;
mod components;
mod engine;
mod extensions;
mod resources;
mod systems;

fn main() {
    println!("#: Starting Rusty...");

    engine::start();
    
    println!("#: Closing Rusty...");
}