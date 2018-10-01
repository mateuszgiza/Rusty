use specs::{ System, Read, Write, ReadStorage, WriteStorage };
use components::{ Position, Velocity, Draw, Size };
use resources::{ DeltaTime, DrawContainer, WindowSize };

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
        let delta = delta.0;
        let window_size = window_size.0;

        for (vel, pos, size) in (&mut vel, &mut pos, &size).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;

            if (pos.x <= 0 as f32 && vel.x < 0 as f32) {
                vel.x = -vel.x;
            }
            if (pos.y <= 0 as f32 && vel.y < 0 as f32) {
                vel.y = -vel.y;
            }
            if (pos.x + size.width as f32 > window_size.0 as f32) {
                vel.x = -vel.x;
            }
            if (pos.y + size.height as f32 > window_size.1 as f32) {
                vel.y = -vel.y;
            }
        }
    }
}

pub struct DrawSystem;

impl<'a> System<'a> for DrawSystem {
    type SystemData = (
        Write<'a, DrawContainer>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Draw>
    );

    fn run (&mut self, data: Self::SystemData) {
        use specs::Join;
        use sdl2::rect::Rect;

        let (mut draw_container, pos, size, draw) = data;

        for (pos, size, draw) in (&pos, &size, &draw).join() {
            let rect = Rect::new(pos.x as i32, pos.y as i32, size.width as u32, size.height as u32);
            let color = draw.color;
            draw_container.insert(move |canvas| {
                canvas.set_draw_color(color);
                let res = canvas.fill_rect(rect);
                match res {
                    Ok(_) => {},
                    Err(e) => {println!("{}", e)}
                }
            });
        }
    }
}