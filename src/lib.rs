pub mod base;
pub mod kolor;
pub mod noise;
pub mod raqote;
pub mod shape;
pub mod skia;
pub mod util;
pub mod svg;

pub use euclid::point2;

pub type IntRect = euclid::default::Box2D<i32>;
pub type IntPoint = euclid::default::Point2D<i32>;
pub type Point = euclid::default::Point2D<f32>;
pub type Transform = euclid::default::Transform2D<f32>;
pub type Vector = euclid::default::Vector2D<f32>;
