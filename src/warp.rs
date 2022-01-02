use num_complex::Complex32;
use crate::prelude::{get_color_clamp, get_color_tile, pt};
use image::DynamicImage;
use std::sync::Arc;
use tiny_skia::Color;

type DomWarp = Arc<dyn Fn(Complex32) -> Complex32 + Send + Sync>;

#[derive(Debug, Clone, Copy)]
pub enum Coord {
    Polar,
    Cartesian,
    Absolute,
}
pub enum Final<'a> {
    More(Arc<Warp<'a>>),
    Func(Arc<dyn Fn(f32, f32) -> Color + Sync + Send>),
    Img(&'a DynamicImage, f32, f32),
}

pub struct Warp<'a> {
    dw: DomWarp,
    warp: Final<'a>,
    coord: Coord,
}

impl<'a> Warp<'a> {
    pub fn new(dw: DomWarp, warp: Final<'a>, coord: Coord) -> Self {
        Self { dw, warp, coord }
    }

    pub fn with_image(
        dw: DomWarp,
        img: &'a DynamicImage,
        width: f32,
        height: f32,
        coord: Coord,
    ) -> Self {
        let warp = Final::Img(img, width, height);
        Self { dw, warp, coord }
    }

    pub fn get(&self, x: f32, y: f32) -> Color {
        let c = (self.dw)(Complex32::new(x, y));
        let r = c.im.abs();
        let (x1, y1) = match self.coord {
            Coord::Polar => (x + c.re.cos() * r, y + c.re.sin() * r),
            Coord::Cartesian => (x + c.re, y + c.im),
            Coord::Absolute => (c.re, c.im),
        };
        match &self.warp {
            Final::More(w) => w.get(x1, y1),
            Final::Func(f) => f(x1, y1),
            Final::Img(img, w, h) => get_color_clamp(img, *w, *h, pt(x1, y1)),
        }
    }

    pub fn get_tiled(&self, x: f32, y: f32) -> Color {
        let c = (self.dw)(Complex32::new(x, y));
        let r = c.im.abs();
        let (x1, y1) = match self.coord {
            Coord::Polar => (x + c.re.cos() * r, y + c.re.sin() * r),
            Coord::Cartesian => (x + c.re, y + c.im),
            Coord::Absolute => (c.re, c.im),
        };
        match &self.warp {
            Final::More(w) => w.get(x1, y1),
            Final::Func(f) => f(x1, y1),
            Final::Img(img, _, _) => get_color_tile::<f32>(img, pt(x1, y1)),
        }
    }
}
