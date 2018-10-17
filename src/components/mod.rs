use std::time::Duration;
use sdl2::rect::Point;
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

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Text {
    pub text: String,
    pub offset: Point,
    pub color: Color,
    pub font: String
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct FPS {
    pub fps_count: u16,
    pub probe_time: Duration
}

impl FPS {
    pub fn new(probe_time: Duration) -> Self {
        FPS { fps_count: 0, probe_time }
    }
}