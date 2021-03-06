use specs::VecStorage;
use sdl2::rect::Point;
use sdl2::pixels::Color;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Text {
    pub text: String,
    pub offset: Point,
    pub color: Color,
    pub font: String
}