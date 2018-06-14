pub mod player_input_component;

pub use self::player_input_component::PlayerInputComponent;

pub trait Component {
    fn name() -> &'static str;
}