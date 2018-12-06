extern crate chrono;
extern crate colored;
extern crate floating_duration;
extern crate lazy_static;
extern crate log;
extern crate pretty_env_logger;
extern crate sdl2;
extern crate sdl2_extras;
extern crate specs;
#[macro_use] extern crate specs_derive;

mod common;
mod components;
mod engine;
mod extensions;
mod managers;
mod resources;
mod systems;

use colored::*;
use log::{error, trace, LevelFilter};

const START_MESSAGE: &'static str = "Starting Rusty...";
const CLOSE_MESSAGE: &'static str = "Closing Rusty...";
const FATAL_ERROR_MSG: &'static str = "[FATAL]";

fn main() {
    match init_logger(LevelFilter::Trace) {
        Ok(_) => trace!("{}", "Logger Initialization complete!".green()),
        Err(e) => error!("{}", e)
    };

    trace!("{}", START_MESSAGE.green());

    let result = engine::start();
    match result {
        Ok(_) => trace!("{}", CLOSE_MESSAGE.green()),
        Err(e) => error!("{} {}", FATAL_ERROR_MSG.red(), e.to_string().bright_red()),
    };
}

fn init_logger(log_level: LevelFilter) -> Result<(), Box<std::error::Error>> {
    pretty_env_logger::formatted_timed_builder()
        .filter(None, log_level)
        .init();

    Ok(())
}
