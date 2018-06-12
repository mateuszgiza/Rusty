use entities::entity_base::*;
use handlers::*;
use common::logger;

pub struct PlayerInputComponent;

impl PlayerInputComponent {
    pub fn new() -> Self {
        PlayerInputComponent {}
    }

    pub fn update(&self, entity: &mut EntityBase) {
        if Input::is_pressed(Key::D) {
            logger::info("Key D was pressed in PlayerInputComponent!");
            entity.pos.x += 5f32;
        }
        if Input::is_pressed(Key::A) {
            logger::info("Key A was pressed in PlayerInputComponent!");
            entity.pos.x -= 5f32;
        }

        if Input::is_pressed(Key::W) {
            logger::info("Key D was pressed in PlayerInputComponent!");
            entity.pos.y -= 5f32;
        }
        if Input::is_pressed(Key::S) {
            logger::info("Key A was pressed in PlayerInputComponent!");
            entity.pos.y += 5f32;
        }
    }
}