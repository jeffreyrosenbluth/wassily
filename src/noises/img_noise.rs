use image::{GenericImageView, Pixel};
use noise::NoiseFn;
use palette::{IntoColor, Lab, Srgb};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ColorMap {
    GrayScale,
    Lightness,
    RedGreen,
    YellowBlue,
}

impl std::fmt::Display for ColorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorMap::GrayScale => write!(f, "GrayScale"),
            ColorMap::Lightness => write!(f, "Lighness (L*)"),
            ColorMap::RedGreen => write!(f, "Red - Green (a*)"),
            ColorMap::YellowBlue => write!(f, "Yellow - Blue (b*)"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImgNoise {
    img: image::DynamicImage,
    color_map: ColorMap,
}

impl ImgNoise {
    pub fn new(img: image::DynamicImage) -> Self {
        Self {
            img,
            color_map: ColorMap::GrayScale,
        }
    }

    pub fn set_map(mut self, color_map: ColorMap) -> Self {
        self.color_map = color_map;
        self
    }
}

impl NoiseFn<f64, 2> for ImgNoise {
    fn get(&self, point: [f64; 2]) -> f64 {
        let (w, h) = self.img.dimensions();
        let (x, y) = (point[0] * w as f64, point[1] * h as f64);
        let pixel = self.img.get_pixel(x as u32 % w, y as u32 % h);
        let rgb: Vec<f32> = pixel
            .to_rgb()
            .channels()
            .into_iter()
            .map(|v| *v as f32 / 255.0)
            .collect();
        let lab: Lab = Srgb::new(rgb[0], rgb[1], rgb[2]).into_color();
        match self.color_map {
            ColorMap::GrayScale => 2.0 * (pixel.to_luma()[0] as f64 / 255.0 - 0.5),
            ColorMap::Lightness => 2.0 * (0.01 * lab.l as f64 - 0.5),
            ColorMap::RedGreen => lab.a as f64 / 128.0,
            ColorMap::YellowBlue => lab.b as f64 / 128.0,
        }
    }
}
