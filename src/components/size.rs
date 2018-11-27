use specs::{ Component, VecStorage };

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Size {
    pub width: i32,
    pub height: i32
}