use num_traits::AsPrimitive;

pub use crate::base::*;
pub use crate::color_names::*;
pub use crate::kolor::*;
pub use crate::lines::*;
pub use crate::noise::*;
pub use crate::rectangles::*;
pub use crate::shape::*;
pub use crate::subdivision::*;
pub use crate::util::*;
pub use euclid::vec2;

pub type IntRect = euclid::default::Box2D<i32>;
pub type IntPoint = euclid::default::Point2D<i32>;
pub type Point = euclid::default::Point2D<f32>;
pub type Transform = euclid::default::Transform2D<f32>;
pub type Vector = euclid::default::Vector2D<f32>;

pub fn point2<S, T>(x: S, y: T) -> Point
where
    S: AsPrimitive<f32>,
    T: AsPrimitive<f32>,
{
    euclid::point2(x.as_(), y.as_())
}

pub struct Dims {
    width: u32,
    height: u32,
}

impl Dims {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn with_aspect(width: u32, numerator: u32, denominator: u32) -> Self {
        Self {width, height: width * numerator / denominator}
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width_f32(&self) -> f32 {
        self.width as f32
    }

    pub fn height_f32(&self) -> f32 {
        self.height as f32
    }
}
