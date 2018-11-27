use std::time::Duration;
use specs::{ Component, VecStorage };

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