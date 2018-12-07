use std::collections::HashMap;
use sdl2::{
    Sdl,
    EventPump,
    event::{Event, EventType},
    keyboard::Keycode,
};
use std::error::Error;
use std::borrow::Borrow;

pub enum EventProcessStatus {
    Ok,
    Exit,
    UnknownEventType
}

pub struct EventManager {
    event_pump: EventPump,
    registered_handlers: HashMap<EventType, Vec<Box<Fn(&Event)>>>
}

unsafe impl Send for EventManager {}
unsafe impl Sync for EventManager {}

impl EventManager {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<Error>> {
        let event_manager = EventManager {
            event_pump: sdl_context.event_pump()?,
            registered_handlers: HashMap::new()
        };
        Ok(event_manager)
    }

    pub fn register<T>(&mut self, event_type: EventType, handler: Box<Fn(&Event)>) {
        let handlers = self.get_event_handlers(event_type);
        handlers.push(handler);
    }

    pub fn process_events(&mut self) -> EventProcessStatus {
        let event_iterator = self.event_pump.poll_iter();

        for event in event_iterator {
            let event_type = Self::get_event_type(&event);
            if event_type.is_none() {
                return EventProcessStatus::UnknownEventType;
            }

            let event_type = event_type.unwrap();
            if self.registered_handlers.contains_key(&event_type) {
                let handlers = self.get_event_handlers(event_type);
                for handler in handlers {
                    handler(&event);
                }
            }
        }

        EventProcessStatus::Ok
    }

    fn get_event_handlers(&mut self, event_type: EventType) -> &mut Vec<Box<Fn(&Event)>> {
        self.registered_handlers.entry(event_type).or_insert(vec![])
    }

    fn get_event_type(event: &Event) -> Option<EventType> {
        match event {
            Event::Quit {..} => Some(EventType::Quit),
            Event::KeyDown {..} => Some(EventType::KeyDown),
            Event::MouseMotion {..} => Some(EventType::MouseMotion),
            _ => None
        }
    }
}