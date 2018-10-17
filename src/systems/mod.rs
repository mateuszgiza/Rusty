use std::time::Duration;
use resources::CanvasHolder;
use builders::{ TextBuilder, TextTexture };
use specs::{ System, Read, Write, ReadStorage, WriteStorage };
use components::{ Position, Velocity, Draw, Size, Text, FPS };
use resources::{ DeltaTime, WindowSize };

use helpers::convert::*;

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

pub struct DrawSystem;

impl<'a> System<'a> for DrawSystem {
    type SystemData = (
        Write<'a, CanvasHolder>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Draw>
    );

    fn run (&mut self, data: Self::SystemData) {
        use specs::Join;
        use sdl2::rect::Rect;

        let (mut canvas_holder, pos, size, draw) = data;

        for (pos, size, draw) in (&pos, &size, &draw).join() {
            let rect = Rect::new(pos.x as i32, pos.y as i32, size.width as u32, size.height as u32);
            let color = draw.color;

            let canvas = canvas_holder.borrow().unwrap();
            canvas.set_draw_color(color);
            let res = canvas.fill_rect(rect);
        }
    }
}

pub struct TextRenderSystem<'b> {
    text_builder: TextBuilder<'b>
}

impl<'b> TextRenderSystem<'b> {
    pub fn new(text_builder: TextBuilder<'b>) -> Self {
        TextRenderSystem {
            text_builder: text_builder
        }
    }
}

impl<'a, 'b> System<'a> for TextRenderSystem<'b> {
    type SystemData = (
        Write<'a, CanvasHolder>,
        ReadStorage<'a, Text>,
        ReadStorage<'a, Position>
    );

    fn run<'c>(&'c mut self, data: Self::SystemData) {
        use specs::Join;
        use sdl2::rect::Rect;

        let (mut canvas_holder, text, pos) = data;

        for (text, pos) in (&text, &pos).join() {
            let Text { text, offset, color, font } = text;
            let text_texture: TextTexture<'c> = self.text_builder.build_text(text, font, color);
            let message_target = Rect::new(pos.x as i32 + offset.x, pos.y as i32 + offset.y, text_texture.query.width, text_texture.query.height);

            let texture = text_texture.texture;
            canvas_holder.borrow().unwrap().copy(&texture, None, Some(message_target)).expect("could not copy texture to canvas");
        }
    }
}

pub struct FpsCounter {
    counter: u16,
    elapsed_time: Duration
}

impl FpsCounter {
    pub fn new() -> Self {
        FpsCounter {
            counter: 0,
            elapsed_time: Duration::from_nanos(0)
        }
    }
}

impl<'a> System<'a> for FpsCounter {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, FPS>
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (delta, mut text, mut fps) = data;

        self.counter += 1;
        self.elapsed_time += delta.elapsed;

        for (text, fps) in (&mut text, &mut fps).join() {
            if self.elapsed_time >= fps.probe_time {
                fps.fps_count = self.counter;
                self.counter = 0;
                self.elapsed_time -= fps.probe_time;

                text.text = format!("FPS: {} | frame_time: {:?}ms", fps.fps_count, (delta.elapsed.subsec_nanos() as f32 / 1_000_000.0 * 100.0).round() / 100.0);
            }
        }
    }
}