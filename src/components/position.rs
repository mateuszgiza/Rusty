use specs::VecStorage;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32
}