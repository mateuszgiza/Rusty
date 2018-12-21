use std::collections::HashMap;
use events::{GameEvent, GameEventType};

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