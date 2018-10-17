use std::time::Duration;

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