use specs::{ System, Read, Write, ReadStorage, WriteStorage };
use components::{ Position, Velocity };
use resources::{ DeltaTime, Draw, DrawContainer };

use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, DeltaTime>,
        Write<'a, DrawContainer>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>
    );

    fn run (&mut self, data: Self::SystemData) {
        use specs::Join;

        let (delta, mut draw_container, vel, mut pos) = data;
        let delta = delta.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;

            draw_container.insert(Draw { color: Color::RGB(255, 0, 0), rect: Rect::new(pos.x as i32, pos.y as i32, 100, 50) });
        }
    }
}