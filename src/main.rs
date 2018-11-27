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

    let result = engine::start();

    let close_info = match result {
        Ok(_) => "#: Closing Rusty...".into(),
        Err(e) => format!("FATAL ERROR: {}", e)
    };

    println!("{}", close_info);
}