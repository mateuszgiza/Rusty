use sfml::system::*;
use sfml::graphics::*;

pub struct EntityBase<'c> {
    pub pos: Vector2f,
    pub size: Vector2u,
    pub shape: CircleShape<'c>
}

impl<'c> EntityBase<'c> {
    pub fn new(posX: f32, posY: f32, width: u16, height: u16) -> Self {
        let shape = CircleShape::new(width.into(), 16);

        EntityBase {
            pos: Vector2::new(posX, posY),
            size: Vector2::new(width.into(), height.into()),
            shape: shape
        }
    }
}

impl<'l> Drawable for EntityBase<'l> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, renderTarget: &mut RenderTarget, renderStates: RenderStates<'texture, 'shader, 'shader_texture>) {
        renderTarget.draw(&self.shape);
    }
}