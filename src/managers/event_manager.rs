use std::collections::HashMap;
use sdl2::{
    Sdl,
    EventPump,
    event::{Event, EventType}
};
use std::error::Error;

pub type EventHandler = Box<Fn(&mut EventState, &Event) -> EventProcessStatus>;

#[allow(dead_code)]
pub enum EventProcessStatus {
    Ok,
    Exit,
    UnknownEventType
}

#[derive(Default)]
pub struct EventManager {
    event_pump: Option<EventPump>,
    registered_handlers: HashMap<EventType, Vec<EventHandler>>
}

unsafe impl Send for EventManager {}
unsafe impl Sync for EventManager {}

impl EventManager {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<Error>> {
        let event_manager = EventManager {
            event_pump: Some(sdl_context.event_pump()?),
            registered_handlers: HashMap::new()
        };
        Ok(event_manager)
    }

    pub fn register(&mut self, event_type: EventType, handler: EventHandler) {
        let handlers = self.get_event_handlers(event_type);
        handlers.push(handler);
    }

    pub fn process_events(&mut self, event_state: &mut EventState) -> EventProcessStatus {
        let event_iterator = self.event_pump.as_mut().unwrap().poll_iter();

        for event in event_iterator {
            let event_type = Self::get_event_type(&event);
            if event_type.is_none() {
                //return EventProcessStatus::UnknownEventType;
                continue;
            }

            let event_type = event_type.unwrap();
            if self.registered_handlers.contains_key(&event_type) {
                let handlers = self.registered_handlers.get(&event_type).unwrap();
                for handler in handlers {
                    let result = handler(event_state, &event);
                    if let EventProcessStatus::Exit = result {
                        return result;
                    }
                }
            }
        }

        EventProcessStatus::Ok
    }

    fn get_event_handlers(&mut self, event_type: EventType) -> &mut Vec<EventHandler> {
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

#[derive(Default)]
pub struct EventState {
    game_events: HashMap<GameEventType, GameEvent>,
}

unsafe impl Send for EventState {}
unsafe impl Sync for EventState {}

impl EventState {
    pub fn new() -> Self {
        EventState {
            game_events: HashMap::new(),
        }
    }

    pub fn set_event(&mut self, event_type: GameEventType, game_event: GameEvent) {
        self.game_events.insert(event_type, game_event);
    }

    pub fn get_event(&self, event_type: GameEventType) -> Option<&GameEvent> {
        self.game_events.get(&event_type)
    }

    pub fn clear_events(&mut self) {
        self.game_events.clear();
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.game_events.len()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum GameEventType {
    CursorMove,
}

pub enum GameEvent {
    CursorMove { x: i32, y: i32 },
}
