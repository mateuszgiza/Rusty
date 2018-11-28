use sdl2::{
    mouse::MouseUtil,
    render::Texture
};

#[derive(Default)]
pub struct Cursor<'a> {
    mouse: Option<MouseUtil>,
    texture: Option<Texture<'a>>
}