use image::{GenericImageView, Pixel};
use noise::NoiseFn;

#[derive(Clone, Debug)]
pub struct ImageNoise {
    img: image::DynamicImage,
}

impl ImageNoise {
    pub fn new(img: image::DynamicImage) -> Self {
        Self { img }
    }
}

impl NoiseFn<f64, 2> for ImageNoise {
    fn get(&self, point: [f64; 2]) -> f64 {
        let (w, h) = self.img.dimensions();
        let (x, y) = (point[0] * w as f64, point[1] * h as f64);
        let pixel = self.img.get_pixel(x as u32 % w, y as u32 % h).to_luma()[0];
        2.0 * (pixel as f64 / 255.0 - 0.5)
    }
}
