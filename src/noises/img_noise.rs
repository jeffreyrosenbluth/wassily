use image::{GenericImageView, Pixel};
use noise::NoiseFn;

#[derive(Clone, Debug)]
pub struct ImgNoise {
    img: image::DynamicImage,
    invert: bool,
}

impl ImgNoise {
    pub fn new(img: image::DynamicImage) -> Self {
        Self { img, invert: false }
    }

    pub fn invert(mut self) -> Self {
        self.invert = true;
        self
    }

    pub fn no_invert(mut self) -> Self {
        self.invert = false;
        self
    }
}

impl NoiseFn<f64, 2> for ImgNoise {
    fn get(&self, point: [f64; 2]) -> f64 {
        let (w, h) = self.img.dimensions();
        let (x, y) = (point[0] * w as f64, point[1] * h as f64);
        let mut pixel = self.img.get_pixel(x as u32 % w, y as u32 % h).to_luma();
        if self.invert {
            pixel.invert()
        };
        2.0 * (pixel[0] as f64 / 255.0 - 0.5)
    }
}
