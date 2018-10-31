use specs::{ System, Write, ReadStorage };
use builders::{ TextBuilder, TextTexture };
use components::{ Position, Text };
use sdl2_extras::adapters::CanvasAdapter;

pub struct TextRenderSystem<'b, 'fm: 'b> {
    text_builder: TextBuilder<'b, 'fm>
}

impl<'b, 'fm> TextRenderSystem<'b, 'fm> {
    pub fn new(text_builder: TextBuilder<'b, 'fm>) -> Self {
        TextRenderSystem {
            text_builder: text_builder
        }
    }
}

impl<'a, 'b, 'fm> System<'a> for TextRenderSystem<'b, 'fm> {
    type SystemData = (
        Write<'a, CanvasAdapter>,
        ReadStorage<'a, Text>,
        ReadStorage<'a, Position>
    );

    fn run<'c>(&'c mut self, data: Self::SystemData) {
        use specs::Join;
        use sdl2::rect::Rect;

        let (mut canvas_adapter, text, pos) = data;

        for (text, pos) in (&text, &pos).join() {
            let Text { text, offset, color, font } = text;
            let text_texture = self.text_builder.build_text(text, &font.get_details(), color);
            let message_target = Rect::new(pos.x as i32 + offset.x, pos.y as i32 + offset.y, text_texture.query.width, text_texture.query.height);

            let texture = text_texture.texture;
            canvas_adapter.proceed(|canvas| {
                canvas.copy(&texture, None, Some(message_target)).expect("could not copy texture to canvas");
            });
        }
    }
}