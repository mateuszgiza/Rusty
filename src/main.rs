extern crate chrono;
extern crate colored;
extern crate lazy_static;
extern crate sdl2;
extern crate sdl2_extras;
extern crate specs;
#[macro_use] extern crate specs_derive;

mod builders;
mod common;
mod components;
mod engine;
mod extensions;
mod resources;
mod systems;

use colored::*;

const START_MESSAGE: &'static str = "#: Starting Rusty...";
const CLOSE_MESSAGE: &'static str = "#: Closing Rusty...";
const FATAL_ERROR_MSG: &'static str = "[FATAL]";

fn main() {
    println!("{}", START_MESSAGE.green());

    let result = engine::start();

    let close_info = match result {
        Ok(_) => format!("{}", CLOSE_MESSAGE.green()),
        Err(e) => format!("{} {}", FATAL_ERROR_MSG.red(), e.to_string().bright_red())
    };

    println!("{}", close_info);
}