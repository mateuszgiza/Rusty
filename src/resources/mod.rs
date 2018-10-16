use std::time::Duration;

mod canvas_holder;
pub use self::canvas_holder::CanvasHolder;

#[derive(Default)]
pub struct DeltaTime {
    pub elapsed: Duration
}

impl DeltaTime {
    pub fn new(elapsed: Option<Duration>) -> Self {
        DeltaTime {
            elapsed: match elapsed {
                Some(duration) => duration,
                None => Duration::from_nanos(0)
            }
        }
    }
}

#[derive(Default)]
pub struct WindowSize(pub (u32, u32));