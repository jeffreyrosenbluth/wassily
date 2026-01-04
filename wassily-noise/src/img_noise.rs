use image::{GenericImageView, Rgba};
use noise::NoiseFn;
use palette::{GetHue, IntoColor, Okhsl, Okhsv, Srgb};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ColorMap {
    Luma,
    Hue,
    Saturation,
    WrappedHue,
    HueSat,
    LumaSat,
    Chroma,
    Value,
}

impl std::fmt::Display for ColorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorMap::Luma => write!(f, "Luma"),
            ColorMap::Hue => write!(f, "Hue"),
            ColorMap::Saturation => write!(f, "Saturation"),
            ColorMap::WrappedHue => write!(f, "Wrapped Hue"),
            ColorMap::HueSat => write!(f, "Hue Sat"),
            ColorMap::LumaSat => write!(f, "Luma Sat"),
            ColorMap::Chroma => write!(f, "Chroma"),
            ColorMap::Value => write!(f, "Value"),
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
            color_map: ColorMap::Luma,
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
        let pixel = self
            .img
            .get_pixel(reflect(x, w as f64) as u32, reflect(y, h as f64) as u32);
        match self.color_map {
            ColorMap::Luma => luma(pixel) as f64,
            ColorMap::Hue => hue(pixel) as f64,
            ColorMap::Saturation => saturation(pixel) as f64,
            ColorMap::Value => value(pixel) as f64,
            ColorMap::WrappedHue => wrapped_hue(pixel) as f64,
            ColorMap::HueSat => hue_sat(pixel) as f64,
            ColorMap::LumaSat => luma_sat(pixel) as f64,
            ColorMap::Chroma => chroma(pixel) as f64,
        }
    }
}

pub fn reflect(p: f64, period: f64) -> f64 {
    let mut p = p;
    while p < 0.0 {
        p += period * 2.0
    }
    p = p % (2.0 * period);
    let r = if p >= period { 2.0 * period - p } else { p };
    r.clamp(0.0, period - 1.0)
}

fn to_okhsl(c: Rgba<u8>) -> Okhsl {
    let r = c.0[0] as f32 / 255.0;
    let g = c.0[1] as f32 / 255.0;
    let b = c.0[2] as f32 / 255.0;
    let srgb = Srgb::new(r, g, b);
    srgb.into_color()
}

fn to_okhsv(c: Rgba<u8>) -> Okhsv {
    let r = c.0[0] as f32 / 255.0;
    let g = c.0[1] as f32 / 255.0;
    let b = c.0[2] as f32 / 255.0;
    let srgb = Srgb::new(r, g, b);
    srgb.into_color()
}

pub(crate) fn luma(c: Rgba<u8>) -> f64 {
    2.0 * (to_okhsl(c).lightness as f64 - 0.5)
}

pub(crate) fn hue(c: Rgba<u8>) -> f64 {
    let degrees = to_okhsl(c).get_hue().into_positive_degrees();
    2.0 * (degrees as f64 / 360.0 - 0.5)
}

pub(crate) fn saturation(c: Rgba<u8>) -> f64 {
    2.0 * (to_okhsl(c).saturation as f64 - 0.5)
}

pub(crate) fn wrapped_hue(c: Rgba<u8>) -> f64 {
    let okhsl = to_okhsl(c);
    let h_degrees = okhsl.get_hue().into_positive_degrees();
    let h = f64::min(h_degrees as f64, 360.0 - h_degrees as f64);
    2.0 * (h / 360.0 - 0.5)
}

pub(crate) fn hue_sat(c: Rgba<u8>) -> f64 {
    let raw_hue = {
        let hsl = to_okhsl(c);
        hsl.get_hue().into_positive_degrees() as f64 / 360.0
    };
    let raw_sat = {
        let okhsl = to_okhsl(c);
        okhsl.saturation as f64
    };
    2.0 * (raw_hue * raw_sat - 0.5)
}

pub(crate) fn luma_sat(c: Rgba<u8>) -> f64 {
    let raw_luma = {
        let okhsl = to_okhsl(c);
        okhsl.lightness as f64
    };
    let raw_sat = {
        let okhsl = to_okhsl(c);
        okhsl.saturation as f64
    };
    2.0 * (raw_luma * raw_sat - 0.5)
}

pub(crate) fn chroma(c: Rgba<u8>) -> f64 {
    let mx = c.0[0].max(c.0[1]).max(c.0[2]);
    let mn = c.0[0].min(c.0[1]).min(c.0[2]);
    2.0 * ((mx - mn) as f64 / 255.0 - 0.5)
}

pub(crate) fn value(c: Rgba<u8>) -> f64 {
    2.0 * (to_okhsv(c).value as f64 - 0.5)
}
