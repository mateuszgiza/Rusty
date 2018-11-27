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

const START_MESSAGE: &'static str = "#: Starting Rusty...";
const CLOSE_MESSAGE: &'static str = "#: Closing Rusty...";

fn main() {
    println!("{}", START_MESSAGE);

    let result = engine::start();

    let close_info = match result {
        Ok(_) => CLOSE_MESSAGE.into(),
        Err(e) => format!("FATAL ERROR: {}", e)
    };

    println!("{}", close_info);
}