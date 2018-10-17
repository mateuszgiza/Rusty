use std::time::Duration;

pub fn duration_to_f32(duration: &Duration) -> f32 {
    duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1_000_000_000.0
}