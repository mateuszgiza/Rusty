use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;

pub struct FontManager<'ttf> {
    font_context: &'ttf Sdl2TtfContext,
    font_names: Vec<String>,
    fonts: Vec<Font<'ttf, 'static>>
}

impl<'ttf> FontManager<'ttf> {
    pub fn new<'a>(font_context: &'a Sdl2TtfContext) -> FontManager<'a> {
        FontManager::<'a> {
            font_context: &font_context,
            font_names: vec![],
            fonts: vec![]
        }
    }

    pub fn load_fonts(&mut self, font_paths: Vec<String>, font_size: u16) {
        for font_path in &font_paths {
            let font = FontManager::load_font(self.font_context, &font_path, font_size);
            
            let font_name = font_path.replace(".ttf", "");
            self.font_names.push(font_name);
            self.fonts.push(font);
        }
    }

    pub fn get_font(&self, font_name: &str) -> &Font {
        let font_index = self.font_names.iter().position(|ref f| f == &font_name).unwrap();
        return &self.fonts[font_index];
    }

    fn load_font(font_context: &'ttf Sdl2TtfContext, font_path: &String, font_size: u16) -> Font<'ttf, 'static> {
        let font = font_context.load_font(&font_path, font_size)
            .expect(&format!("could not load font: {}", &font_path));

        return font;
    }
}