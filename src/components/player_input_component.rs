use entities::*;
use handlers::*;
use components::*;

pub struct PlayerInputComponent;

impl Component for PlayerInputComponent {
    fn name() -> &'static str { stringify!(PlayerInputComponent) }
}

impl PlayerInputComponent {
    pub fn new() -> Self {
        PlayerInputComponent {}
    }

    pub fn update(&self, entity: &mut EntityBase) {
        if Input::is_pressed(Key::D) {
            Input::log_pressed_key::<PlayerInputComponent>(Key::D);
            entity.pos.x += 5f32;
        }
        if Input::is_pressed(Key::A) {
            Input::log_pressed_key::<PlayerInputComponent>(Key::A);
            entity.pos.x -= 5f32;
        }

        if Input::is_pressed(Key::W) {
            Input::log_pressed_key::<PlayerInputComponent>(Key::W);
            entity.pos.y -= 5f32;
        }
        if Input::is_pressed(Key::S) {
            Input::log_pressed_key::<PlayerInputComponent>(Key::S);
            entity.pos.y += 5f32;
        }
    }
}