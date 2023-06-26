use image::{GenericImageView, Pixel};
use noise::NoiseFn;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ColorMap {
    GrayScale,
    RotatedGray,
    Red,
    Green,
    Blue,
}

impl std::fmt::Display for ColorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorMap::GrayScale => write!(f, "GrayScale"),
            ColorMap::RotatedGray => write!(f, "RotatedGray"),
            ColorMap::Red => write!(f, "Red"),
            ColorMap::Green => write!(f, "Green"),
            ColorMap::Blue => write!(f, "Blue"),
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
        let rgb: Vec<f64> = pixel
            .to_rgb()
            .channels()
            .into_iter()
            .map(|v| From::from(*v))
            .collect();
        let x = 0.49 * rgb[0] + 0.31 * rgb[1] + 0.2 * rgb[2];
        let y = 0.17697 * rgb[0] * 0.8124 * rgb[1] + 0.01063 * rgb[2];
        let v = match self.color_map {
            ColorMap::GrayScale => x,   //pixel.to_luma()[0],
            ColorMap::RotatedGray => y, // {
            //     pixel = self
            //         .img
            //         .get_pixel(w - 1 - (x as u32 % w), h - 1 - (y as u32 % h));
            //     pixel.to_luma()[0]
            // }
            ColorMap::Red => pixel.to_rgb()[0] as f64,
            ColorMap::Green => pixel.to_rgb()[1] as f64,
            ColorMap::Blue => pixel.to_rgb()[2] as f64,
        };
        2.0 * (v - 0.5)
        // 2.0 * (v as f64 / 255.0 - 0.5)
    }
}
