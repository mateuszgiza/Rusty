mod bootstrapper;

use colored::*;
use log::{error, trace};
use sdl2::pixels::Color;
use sdl2_extras::fspecs::WorldExt;
use specs::Write;
use std::error::Error;
use {
    configuration::Configurator,
    events::EventState,
    extensions::ResultExt,
    managers::{EventManager, EventProcessStatus},
};

pub fn start() -> Result<(), Box<Error>> {
    let context = bootstrapper::initialize()
        .on_success(|_| trace!("{}", "Engine initialization succeeded!".green()))
        .on_error(|e| error!("Engine initialization error: {}", e))?;
    let mut world = bootstrapper::create_world(context)?;

    Configurator::register_components(&mut world);
    Configurator::setup_entities(&mut world);
    Configurator::setup_event_handlers(&world);
    let mut dispatcher = Configurator::setup_systems();
    let (mut timer, mut fps_manager) = Configurator::setup_timers();

    world
        .proceed_on_canvas(|canvas| {
            canvas.set_draw_color(Color::RGB(0, 255, 255));
            canvas.clear();
            canvas.present();
        })
        .discard_result();

    'running: loop {
        world.update_delta_time(timer.elapsed_time());

        let event_process_result = world.exec(
            |(mut event_state, mut event_manager): (Write<EventState>, Write<EventManager>)| {
                event_manager.process_events(&mut event_state)
            },
        );

        if let EventProcessStatus::Exit = event_process_result {
            break 'running;
        }

        world
            .proceed_on_canvas(|canvas| {
                canvas.set_draw_color(Color::RGB(39, 58, 93));
                canvas.clear();
            })
            .discard_result();

        dispatcher.dispatch(&mut world.res);
        world.maintain();

        world
            .proceed_on_canvas(|canvas| {
                canvas.present();
            })
            .discard_result();

        timer.update();
        fps_manager.delay();

        world.write_resource::<EventState>().clear_events();
    }

    Ok(())
}
