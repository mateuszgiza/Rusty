use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Default)]
pub struct DeltaTime(pub f32); // Change to std::time::Duration

pub struct Draw {
    pub color: Color,
    pub rect: Rect
}

impl Default for Draw {
    fn default() -> Self { Draw { color: Color::RGB(0, 0, 0), rect: Rect::new(0, 0, 0, 0) } }
}

#[derive(Default)]
pub struct DrawContainer {
    pub instructions: Vec<Draw>
}

impl DrawContainer {
    pub fn insert(&mut self, draw: Draw) {
        self.instructions.push(draw);
    }

    pub fn clear(&mut self) {
        self.instructions.clear();
    }
}