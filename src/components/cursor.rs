use specs::{ Component, VecStorage };

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Cursor;