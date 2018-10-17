use sdl2::pixels::Color;
use sdl2::render::Texture;
use objects::FontManager;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;

pub struct TextBuilder<'f> {
    texture_creator: TextureCreator<WindowContext>,
    font_manager: &'f FontManager<'f>
}

impl<'f> TextBuilder<'f> {
    pub fn new(canvas: &Canvas<Window>, font_manager: &'f FontManager) -> Self {
        let texture_creator = canvas.texture_creator();

        TextBuilder {
            texture_creator: texture_creator,
            font_manager: font_manager
        }
    }

    pub fn build_text(&self, text: &str, font_name: &str, color: &Color) -> Texture {
        let font = self.font_manager.get_font(font_name);
        let text_render = font.render(text);
        let text_surface = text_render.solid(*color).unwrap();
        let text_texture = self.texture_creator.create_texture_from_surface(text_surface).unwrap();

        return text_texture;
    }
}