use specs::{ System, Read, WriteStorage, ReadStorage };
use resources::{ DeltaTime, WindowSize };
use components::{ Position, Velocity, Size };

use helpers::convert::duration_to_f32;

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, DeltaTime>,
        Read<'a, WindowSize>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Size>
    );

    fn run (&mut self, data: Self::SystemData) {
        use specs::Join;

        let (delta, window_size, mut vel, mut pos, size) = data;
        let delta = duration_to_f32(&delta.elapsed);
        let window_size = window_size.0;

        for (vel, pos, size) in (&mut vel, &mut pos, &size).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;

            if pos.x <= 0 as f32 && vel.x < 0 as f32 {
                vel.x = -vel.x;
            }
            if pos.y <= 0 as f32 && vel.y < 0 as f32 {
                vel.y = -vel.y;
            }
            if pos.x + size.width as f32 > window_size.0 as f32 {
                vel.x = -vel.x;
            }
            if pos.y + size.height as f32 > window_size.1 as f32 {
                vel.y = -vel.y;
            }
        }
    }
}