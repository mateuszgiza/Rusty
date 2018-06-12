use sfml::graphics::*;
use entities::entity_base::EntityBase;
use std::cell::{RefCell, Ref};

pub struct EntityRenderSystem;

impl EntityRenderSystem {
    pub fn render(window: &mut RenderWindow, entities: &Vec<RefCell<EntityBase>>) {
        for ref entity in entities {
            let borrowed = entity.borrow();
            let drawable = Ref::map(borrowed, |e| e);
            window.draw(&*drawable);
        }
    }
}