use sdl2::render::TextureQuery;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;
use sdl2_extras::common::FontDetails;
use sdl2_extras::managers::FontManager;

pub struct TextTexture<'a> {
    pub texture: Texture<'a>,
    pub query: TextureQuery
}

impl<'a> TextTexture<'a> {
    pub fn new<'b>(texture: Texture<'a>, query: TextureQuery) -> Self {
        TextTexture {
            texture: texture,
            query: query
        }
    }
}

unsafe impl<'a> Send for TextTexture<'a> {}
unsafe impl<'a> Sync for TextTexture<'a> {}

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

    pub fn build_text<'a>(&'a self, text: &str, font_details: &FontDetails, color: &Color) -> TextTexture<'a> {
        let font = self.font_manager.load(font_details).unwrap();
        let text_render = font.render(text);
        let text_surface = text_render.solid(*color).unwrap();
        let text_texture = self.texture_creator.create_texture_from_surface(text_surface).unwrap();
        let text_query = text_texture.query();

        return TextTexture::new(text_texture, text_query);
    }
}