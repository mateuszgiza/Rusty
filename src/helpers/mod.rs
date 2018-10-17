pub mod canvas {
    use sdl2::video::Window;
    use sdl2::render::Canvas;
    use specs::World;
    use resources::CanvasHolder;

    pub fn proceed_on_canvas<F>(world: &World, canvas_action: F)
    where F: Fn(&mut Canvas<Window>) {
        let mut canvas_holder = world.write_resource::<CanvasHolder>();
        let mut canvas = canvas_holder.borrow().unwrap();

        canvas_action(&mut canvas);
    }
}

pub mod convert {
    use std::time::Duration;
    
    pub fn duration_to_f32(duration: &Duration) -> f32 {
        duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1_000_000_000.0
    }
}

pub mod ops {
    use std::time::Duration;
    use convert;

    pub fn duration_mul_f32(duration: &Duration, rhs: f32) -> f32 {
        convert::duration_to_f32(duration) * rhs
    }
}