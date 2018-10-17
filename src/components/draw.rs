use specs::VecStorage;
use sdl2::pixels::Color;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Draw {
    pub color: Color
}