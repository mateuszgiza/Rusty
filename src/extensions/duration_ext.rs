use std::time::Duration;

pub trait As<T> {
    fn as_type<TDest>(&self) -> T where TDest: Into<T>;
}

impl As<f32> for Duration {
    fn as_type<TDest>(&self) -> f32 where TDest: Into<f32> {
        self.as_secs() as f32 + self.subsec_nanos() as f32 / 1_000_000_000.0
    }
}