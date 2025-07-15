//! # Domain Warping
//!
//! Advanced coordinate transformation and domain warping effects.
//! This module provides tools for transforming coordinate spaces to create
//! distortion effects, non-linear transformations, and complex visual warping
//! of images and procedural content.
//!
//! ## Key Concepts
//!
//! Domain warping transforms the coordinate space before sampling colors or textures,
//! creating powerful distortion effects:
//!
//! - **Coordinate Transformation**: Map input coordinates to new positions
//! - **Chain Composition**: Combine multiple warping operations
//! - **Multi-Source Support**: Warp images, functions, or other warp nodes
//! - **Flexible Coordinate Systems**: Polar, Cartesian, or absolute coordinates
//!
//! ## Components
//!
//! - **[`Warp`]**: Main warping transformation container
//! - **[`WarpNode`]**: Source content (image, function, or nested warp)
//! - **[`Coord`]**: Coordinate system specification
//! - **Domain Functions**: Functions that transform coordinate spaces
//!
//! ## Basic Usage
//!
//! ```no_run
//! use wassily_effects::*;
//! use wassily_core::*;
//! use std::sync::Arc;
//!
//! // Create a simple warping function
//! let warp_func = Arc::new(|p: Point| {
//!     // Simple swirl transformation
//!     let r = (p.x * p.x + p.y * p.y).sqrt();
//!     let angle = r * 0.1;
//!     pt(
//!         p.x * angle.cos() - p.y * angle.sin(),
//!         p.x * angle.sin() + p.y * angle.cos()
//!     )
//! });
//!
//! // Create color function
//! let color_func = Arc::new(|x: f32, y: f32| {
//!     rgb8((x * 255.0) as u8, (y * 255.0) as u8, 128)
//! });
//!
//! // Combine into warp
//! let warp_node = WarpNode::Func(color_func);
//! let warp = Warp::new(warp_func, warp_node, Coord::Cartesian);
//! ```
//!
//! ## Applications
//!
//! - **Image Distortion**: Non-linear image transformations
//! - **Procedural Effects**: Warped noise and pattern generation
//! - **Artistic Distortion**: Creative coordinate space manipulation
//! - **Animation**: Time-varying coordinate transformations
use wassily_color::{get_color, get_color_reflect, get_color_tile, get_color_wrap};
use wassily_core::points::pt;
use image::DynamicImage;
use std::sync::Arc;
use tiny_skia::{Color, Point};

type DomWarp = Arc<dyn Fn(Point) -> Point + Send + Sync>;

#[derive(Debug, Clone, Copy)]
pub enum Coord {
    Polar,
    Cartesian,
    Absolute,
}
pub enum WarpNode<'a> {
    More(Arc<Warp<'a>>),
    Func(Arc<dyn Fn(f32, f32) -> Color + Sync + Send>),
    Img(&'a DynamicImage, f32, f32),
}

pub struct Warp<'a> {
    dw: DomWarp,
    warp: WarpNode<'a>,
    coord: Coord,
}

impl<'a> Warp<'a> {
    pub fn new(dw: DomWarp, warp: WarpNode<'a>, coord: Coord) -> Self {
        Self { dw, warp, coord }
    }

    pub fn with_image(
        dw: DomWarp,
        img: &'a DynamicImage,
        width: f32,
        height: f32,
        coord: Coord,
    ) -> Self {
        let warp = WarpNode::Img(img, width, height);
        Self { dw, warp, coord }
    }

    pub fn coords(&self, x: f32, y: f32) -> Point {
        let c = (self.dw)(pt(x, y));
        let (x1, y1) = match self.coord {
            Coord::Polar => {
                let r = c.y.abs();
                (x + c.x.cos() * r, y + c.x.sin() * r)
            }
            Coord::Cartesian => (x + c.x, y + c.y),
            Coord::Absolute => (c.x, c.y),
        };
        match &self.warp {
            WarpNode::More(w) => w.coords(x1, y1),
            _ => pt(x1, y1),
        }
    }

    pub fn get(&self, x: f32, y: f32) -> Color {
        let c = (self.dw)(pt(x, y));
        let (x1, y1) = match self.coord {
            Coord::Polar => {
                let r = c.y.abs();
                (x + c.x.cos() * r, y + c.x.sin() * r)
            }
            Coord::Cartesian => (x + c.x, y + c.y),
            Coord::Absolute => (c.x, c.y),
        };
        match &self.warp {
            WarpNode::More(w) => w.get(x1, y1),
            WarpNode::Func(f) => f(x1, y1),
            WarpNode::Img(img, w, h) => get_color(img, *w, *h, pt(x1, y1)).unwrap(),
        }
    }

    pub fn get_wrapped(&self, x: f32, y: f32) -> Color {
        let c = (self.dw)(pt(x, y));
        let (x1, y1) = match self.coord {
            Coord::Polar => {
                let r = c.y.abs();
                (x + c.x.cos() * r, y + c.x.sin() * r)
            }
            Coord::Cartesian => (x + c.x, y + c.y),
            Coord::Absolute => (c.x, c.y),
        };
        match &self.warp {
            WarpNode::More(w) => w.get(x1, y1),
            WarpNode::Func(f) => f(x1, y1),
            WarpNode::Img(img, w, h) => get_color_wrap(img, *w, *h, pt(x1, y1)),
        }
    }

    pub fn get_reflected(&self, x: f32, y: f32) -> Color {
        let c = (self.dw)(pt(x, y));
        let (x1, y1) = match self.coord {
            Coord::Polar => {
                let r = c.y.abs();
                (x + c.x.cos() * r, y + c.x.sin() * r)
            }
            Coord::Cartesian => (x + c.x, y + c.y),
            Coord::Absolute => (c.x, c.y),
        };
        match &self.warp {
            WarpNode::More(w) => w.get(x1, y1),
            WarpNode::Func(f) => f(x1, y1),
            WarpNode::Img(img, w, h) => get_color_reflect(img, *w, *h, pt(x1, y1)),
        }
    }

    pub fn get_tiled(&self, x: f32, y: f32) -> Color {
        let c = (self.dw)(pt(x, y));
        let (x1, y1) = match self.coord {
            Coord::Polar => {
                let r = c.y.abs();
                (x + c.x.cos() * r, y + c.x.sin() * r)
            }
            Coord::Cartesian => (x + c.x, y + c.y),
            Coord::Absolute => (c.x, c.y),
        };
        match &self.warp {
            WarpNode::More(w) => w.get(x1, y1),
            WarpNode::Func(f) => f(x1, y1),
            WarpNode::Img(img, _, _) => get_color_tile::<f32>(img, pt(x1, y1)),
        }
    }
}
