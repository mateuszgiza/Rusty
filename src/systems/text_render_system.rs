use sdl2::pixels::Color;
use sdl2_extras::common::FontDetails;
use specs::{ System, Write, ReadStorage };
use builders::{ TextBuilder };
use components::{ Position, Text };
use sdl2_extras::adapters::{CanvasAdapter, ResourceFacade};
use extensions::ResultExt;
use sdl2_extras::common::TextTexture;

pub struct TextRenderSystem;

impl<'a, 'b: 'a> System<'a> for TextRenderSystem {
    type SystemData = (
        Write<'a, CanvasAdapter>,
        Write<'a, ResourceFacade<'b>>,
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
            }).discard_result();
        }
    }
}

impl TextRenderSystem {
    pub fn build_text<'a>(&'a mut self, text: &str, font_details: &FontDetails, color: &Color) -> TextTexture<'a> {
        let font = self.font_manager.load(font_details).unwrap();
        let text_render = font.render(text);
        let text_surface = text_render.solid(*color).unwrap();
        let text_texture = self.texture_creator.create_texture_from_surface(text_surface).unwrap();
        let text_query = text_texture.query();

        return TextTexture::new(text_texture, text_query);
    }
}