use specs::{ Component, VecStorage };

use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Draw {
    pub color: Color,
    pub rect: Rect
}