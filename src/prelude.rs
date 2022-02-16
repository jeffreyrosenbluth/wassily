pub use tiny_skia::*;

pub use crate::canvas::*;
pub use crate::color_names::*;
pub use crate::flow::*;
pub use crate::grain::*;
pub use crate::kolor::*;
pub use crate::lines::*;
pub use crate::math::*;
pub use crate::matrix::*;
pub use crate::noises::curl::*;
pub use crate::noises::gabor::*;
pub use crate::noises::trig::*;
pub use crate::noises::white::*;
pub use crate::noises::*;
pub use crate::rectangles::*;
pub use crate::shape::*;
pub use crate::sphere::*;
pub use crate::stipple::*;
pub use crate::subdivision::*;
pub use crate::util::*;
pub use crate::warp::*;
pub use image::*;
pub use noise::*;
pub use palette;

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
