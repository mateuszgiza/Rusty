use entities::entity_base::EntityBase;

use sfml::graphics::*;
use std::cell::RefCell;

pub struct EntityShapePositionUpdateSystem;

impl EntityShapePositionUpdateSystem {
    pub fn update(entities: &Vec<RefCell<EntityBase>>) {
        for entity in entities {
            let mut e = entity.borrow_mut();
            let pos = e.pos;
            let mut shape = &mut e.shape;
            shape.set_position(pos);
        }
    }
}