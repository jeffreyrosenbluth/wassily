use crate::base::RGBA;
use crate::prelude::{get_color_clamp, point2};
use image::DynamicImage;
use std::rc::Rc;

type DomWarp = Rc<dyn Fn(f32, f32) -> f32>;

pub enum Coord {
    Polar,
    Cartesian,
}
pub enum Final<'a> {
    More(Box<Warp<'a>>),
    Func(Box<dyn Fn(f32, f32) -> RGBA>),
    Img(&'a DynamicImage, f32, f32),
}

pub struct Warp<'a> {
    f: DomWarp,
    g: DomWarp,
    warp: Final<'a>,
    coord: Coord,
}

impl<'a> Warp<'a> {
    pub fn new(f: DomWarp, g: DomWarp, warp: Final<'a>, coord: Coord) -> Self {
        Self { f, g, warp, coord }
    }

    pub fn with_image(
        f: DomWarp,
        g: DomWarp,
        img: &'a DynamicImage,
        width: f32,
        height: f32,
        coord: Coord,
    ) -> Self {
        let warp = Final::Img(img, width, height);
        Self { f, g, warp, coord }
    }

    pub fn get(&self, x: f32, y: f32) -> RGBA {
        let a = (self.f)(x, y);
        let b = (self.g)(x, y).abs();
        let (x1, y1) = match self.coord {
            Coord::Polar => (x + a.cos() * b, y + a.sin() * b),
            Coord::Cartesian => (x + a, y + b),
        };
        match &self.warp {
            Final::More(w) => w.get(x1, y1),
            Final::Func(f) => f(x1, y1),
            Final::Img(img, w, h) => get_color_clamp(img, *w, *h, point2(x1, y1)),
        }
    }
}
