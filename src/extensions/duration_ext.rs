use std::time::Duration;

pub trait As<T> {
    fn as_type<TDest>(&self) -> TDest where TDest: From<T>;
}

impl As<f32> for Duration {
    fn as_type<TDest>(&self) -> TDest where TDest: From<f32> {
        TDest::from(self.as_secs() as f32 + self.subsec_nanos() as f32 / 1_000_000_000.0)
    }
}