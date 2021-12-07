use num_traits::AsPrimitive;

pub use crate::base::*;
pub use crate::color_names::*;
pub use crate::kolor::*;
pub use crate::lines::*;
pub use crate::matrix::*;
pub use crate::quiet::gabor::*;
pub use crate::quiet::*;
pub use crate::rectangles::*;
pub use crate::shape::*;
pub use crate::subdivision::*;
pub use crate::util::*;
pub use crate::warp::*;
pub use euclid::vec2;
pub use image::*;
pub use noise::*;

pub type IntRect = euclid::default::Box2D<i32>;
pub type IntPoint = euclid::default::Point2D<i32>;
pub type Point = euclid::default::Point2D<f32>;
pub type Transform = euclid::default::Transform2D<f32>;
pub type Vector = euclid::default::Vector2D<f32>;

#[deprecated(note = "please use `pt` instead")]
pub fn point2<S, T>(x: S, y: T) -> Point
where
    S: AsPrimitive<f32>,
    T: AsPrimitive<f32>,
{
    euclid::point2(x.as_(), y.as_())
}

pub fn pt<S, T>(x: S, y: T) -> Point
where
    S: AsPrimitive<f32>,
    T: AsPrimitive<f32>,
{
    euclid::point2(x.as_(), y.as_())
}

pub fn polar<S, T>(theta: S, r: T) -> Point
where
    S: AsPrimitive<f32>,
    T: AsPrimitive<f32>,
{
    euclid::point2(r.as_() * theta.as_().cos(), r.as_() * theta.as_().sin())
}

#[derive(Debug, Clone, Copy)]
pub struct Dims {
    pub width: u32,
    pub height: u32,
    pub width_f32: f32,
    pub height_f32: f32,
}

impl Dims {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            width_f32: width as f32,
            height_f32: height as f32,
        }
    }

    pub fn with_aspect(width: u32, numerator: u32, denominator: u32) -> Self {
        Self::new(width, width * denominator / numerator)
    }
}

pub trait BasicModel {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn name(&self) -> &str;
    fn ext(&self) -> &str;
    fn dir(&self) -> &str;
    fn width_f32(&self) -> f32 {
        self.width() as f32
    }
    fn height_f32(&self) -> f32 {
        self.height() as f32
    }
}

#[macro_export]
macro_rules! basic_model {
    ($t:ident < $lt:lifetime >) => {
        impl<$lt> BasicModel for $t<$lt> {
            fn width(&self) -> u32 {
                self.width
            }
            fn height(&self) -> u32 {
                self.height
            }
            fn name(&self) -> &str {
                self.name
            }
            fn ext(&self) -> &str {
                self.ext
            }
            fn dir(&self) -> &str {
                self.dir
            }
        }
    };
}
