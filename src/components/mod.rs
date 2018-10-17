use specs::{ VecStorage };

use sdl2::pixels::Color;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Size {
    pub width: i32,
    pub height: i32
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
    pub color: Color
}