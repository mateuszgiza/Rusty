use specs::{ System, Read, Write, ReadStorage, WriteStorage };
use components::{ Position, Velocity, Draw, Size };
use resources::{ DeltaTime, DrawContainer };

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
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>
    );

    fn run (&mut self, data: Self::SystemData) {
        use specs::Join;

        let (delta, vel, mut pos) = data;
        let delta = delta.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
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
            draw_container.insert(move |mut canvas| {
                canvas.set_draw_color(color);
                canvas.fill_rect(rect);
            });
        }
    }
}