use sdl2::{
    Sdl,
    EventPump,
    event::Event,
    keyboard::Keycode,
};
use std::error::Error;

pub enum EventProcessStatus {
    Ok,
    Exit
}

pub struct EventManager {
    event_pump: EventPump
}

unsafe impl Send for EventManager {}
unsafe impl Sync for EventManager {}

impl EventManager {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<Error>> {
        let event_manager = EventManager {
            event_pump: sdl_context.event_pump()?
        };
        Ok(event_manager)
    }

    pub fn process_events(&mut self) -> EventProcessStatus {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return EventProcessStatus::Exit,
                Event::MouseMotion { x, y, .. } => {
                    // cursor_rect.set_x(x);
                    // cursor_rect.set_y(y);
                },
                _ => {}
            }
        }

        EventProcessStatus::Ok
    }
}