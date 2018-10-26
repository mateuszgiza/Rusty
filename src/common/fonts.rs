#![allow(dead_code)]

use sdl2_extras::common::FontDetails;

#[derive(Debug)]
pub enum FontType {
    SpaceMonoRegular24
}

impl FontType {
    pub fn get_details(&self) -> FontDetails {
        let data = match self {
            &FontType::SpaceMonoRegular24 => ("SpaceMono-Regular.ttf", 24)
        };

        return FontDetails { path: data.0.to_string(), size: data.1 };
    }
}