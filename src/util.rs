use tiny_skia::*;

pub const TAU: f32 = std::f32::consts::TAU;
pub const PI: f32 = std::f32::consts::PI;

pub fn pt2(x: f32, y: f32) -> Point {
    Point::from_xy(x, y)
}

pub fn map_range(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}