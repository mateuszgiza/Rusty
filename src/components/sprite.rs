use specs::{ Component, VecStorage };
use sdl2::pixels::Color;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub texture_name: String,
    pub color: Color
}