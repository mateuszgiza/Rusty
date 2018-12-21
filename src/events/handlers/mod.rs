use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use managers::EventProcessStatus;
use events::{EventState, GameEvent, GameEventType};

pub fn on_quit(_state: &mut EventState, event: &Event) -> EventProcessStatus {
    if let Event::Quit {..} = event {
        return EventProcessStatus::Exit;
    }
    else if let Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
        return EventProcessStatus::Exit;
    }

    EventProcessStatus::Ok
}

pub fn event_handler_cursor_move(state: &mut EventState, event: &Event) -> EventProcessStatus {
    if let Event::MouseMotion {x, y, ..} = event {
        state.set_event(GameEventType::CursorMove, GameEvent::CursorMove { x: *x, y: *y });
    }

    EventProcessStatus::Ok
}