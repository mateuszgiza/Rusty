#![allow(dead_code)]

use std::time::Duration;
use convert;

pub fn duration_mul_f32(duration: &Duration, rhs: f32) -> f32 {
    convert::duration_to_f32(duration) * rhs
}