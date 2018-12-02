use sdl2::{
    Sdl,
    EventPump,
    event::EventPollIterator
};
use std::error::Error;

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

    pub fn poll_iter(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }
}