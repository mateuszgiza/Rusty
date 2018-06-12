use entities::entity_base::EntityBase;

pub struct Ball<'a> {
    pub base: EntityBase<'a>,
}

impl<'a> Ball<'a> {
    pub fn new() -> Self {
        Ball {
            base: EntityBase::new(50f32, 50f32, 30u16, 30u16),
        }
    }
}
