use specs::{ System, Write, ReadStorage, WriteStorage };
use components::{ Cursor, Position };
use managers::{EventState, GameEvent, GameEventType};

pub struct CursorUpdateSystem;

impl<'a> System<'a> for CursorUpdateSystem {
    type SystemData = (
        Write<'a, EventState>,
        ReadStorage<'a, Cursor>,
        WriteStorage<'a, Position>,
    );

    fn run (&mut self, data: Self::SystemData) {
        use specs::Join;
        let (event_state, cursor, mut pos) = data;

        for (_cursor, pos) in (&cursor, &mut pos).join() {
            let event = event_state.get_event(GameEventType::CursorMove);
            if event.is_some() {
                let GameEvent::CursorMove {x,y} = event.unwrap();
                pos.x = *x as f32;
                pos.y = *y as f32;
            }
        }
    }
}