#![allow(dead_code)]

pub const SPACE_MONO_REGULAR: &str = "SpaceMono-Regular";

pub fn ttf(font_name: &str) -> String {
    return font_name.to_owned() + ".ttf";
}