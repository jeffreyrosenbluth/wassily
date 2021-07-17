use std::path;

pub use crate::base::*;
pub use crate::kolor::*;
pub use crate::noise::*;
pub use crate::shape::*;
pub use crate::util::*;
pub use crate::lines::*;
pub use crate::rectangles::*;
pub use crate::color_names::*;
pub use euclid::vec2;

pub type IntRect = euclid::default::Box2D<i32>;
pub type IntPoint = euclid::default::Point2D<i32>;
pub type Point = euclid::default::Point2D<f32>;
pub type Transform = euclid::default::Transform2D<f32>;
pub type Vector = euclid::default::Vector2D<f32>;

pub fn file_path(path: &str) -> &path::Path {
    path::Path::new(path)
}

pub fn point2(x: f32, y: f32) -> Point {
    euclid::point2(x, y)
}