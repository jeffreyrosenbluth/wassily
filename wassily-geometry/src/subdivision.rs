//! # Recursive Subdivision Algorithms
//!
//! **Advanced subdivision algorithms for quadrilaterals and triangles with noise-based distortion.**
//! This module provides powerful tools for creating complex geometric patterns through
//! recursive subdivision, essential for fractal-like structures, organic tessellations,
//! and algorithmic mesh generation.
//!
//! ## Key Features
//!
//! - **[`Quad`]**: Quadrilateral subdivision with customizable split strategies
//! - **[`Tri`]**: Triangle subdivision for triangular mesh generation
//! - **Noise-based distortion**: Organic deformation using Perlin noise
//! - **Perspective projections**: 3D-style quad generation with vanishing points
//! - **Batch operations**: Efficient subdivision of large shape collections
//!
//! ## Core Concepts
//!
//! ### Subdivision Strategies
//!
//! Shapes can be subdivided using various strategies:
//! - **Adaptive**: Split along the longest dimension for balanced shapes
//! - **Fixed ratios**: Consistent split ratios for regular patterns
//! - **Random**: Probabilistic splitting for organic appearance
//! - **Function-driven**: Custom logic based on shape properties
//!
//! ### Noise-based Distortion
//!
//! Points can be perturbed using Perlin noise to create organic variations:
//! - Maintains overall structure while adding natural irregularity
//! - Configurable noise scale and intensity
//! - Useful for simulating natural phenomena and hand-drawn aesthetics
//!
//! ## Quick Start
//!
//! ```no_run
//! use wassily_geometry::*;
//! use wassily_core::points::pt;
//!
//! // Create initial quadrilateral
//! let quad = Quad::new(
//!     pt(0.0, 0.0),   // Bottom left
//!     pt(0.0, 100.0), // Top left  
//!     pt(100.0, 100.0), // Top right
//!     pt(100.0, 0.0), // Bottom right
//! );
//!
//! // Recursive subdivision
//! let mut quads = vec![quad];
//! for _ in 0..5 {
//!     quads = quad_divide_vec(
//!         &quads,
//!         |q| q.best_dir(),    // Split along longest dimension
//!         |_| (0.3, 0.7),      // Split ratios
//!     );
//! }
//! // Result: 32 subdivided quadrilaterals
//! ```
//!
//! ## Advanced Examples
//!
//! ### Organic Subdivision with Noise
//!
//! ```no_run
//! use wassily_geometry::*;
//! use wassily_core::points::pt;
//!
//! // Create base quadrilateral
//! let quad = Quad::new(pt(0.0, 0.0), pt(0.0, 200.0), pt(200.0, 200.0), pt(200.0, 0.0));
//! let mut quads = vec![quad];
//!
//! // Subdivide with organic distortion
//! for _ in 0..4 {
//!     quads = quad_divide_vec(&quads, |q| q.best_dir(), |_| (0.4, 0.6));
//!     
//!     // Apply noise-based distortion to all points
//!     for quad in &mut quads {
//!         let points = vec![quad.bl, quad.tl, quad.tr, quad.br];
//!         let warped = warp_points(&points, 0.02, 10.0); // scale=0.02, factor=10.0
//!         *quad = Quad::new(warped[0], warped[1], warped[2], warped[3]);
//!     }
//! }
//! ```
//!
//! ### Triangle Mesh Generation
//!
//! ```no_run
//! use wassily_geometry::*;
//! use wassily_core::points::pt;
//!
//! // Create initial triangle
//! let triangle = Tri::new(
//!     pt(0.0, 0.0),   // Bottom left
//!     pt(50.0, 100.0), // Top
//!     pt(100.0, 0.0), // Bottom right
//! );
//!
//! // Recursive triangle subdivision
//! let mut triangles = vec![triangle];
//! for _ in 0..6 {
//!     triangles = tri_divide_vec(
//!         &triangles,
//!         |t| t.best_dir(),    // Split from vertex opposite longest side
//!         || 0.5,              // Split at midpoint
//!     );
//! }
//! ```

use wassily_noise::*;
use wassily_core::points::{pt, Algebra};
use wassily_core::util::Orientation;
use noise::{Perlin, Seedable};
use tiny_skia::Point;

/// A Quadrilateral.
/// Cartesian coordinates
#[derive(Debug, Clone, Copy)]
pub struct Quad {
    /// Bottom Left
    pub bl: Point,
    /// Top Left
    pub tl: Point,
    /// Top Right
    pub tr: Point,
    /// Bottom Right
    pub br: Point,
}

impl Quad {
    pub fn new(bl: Point, tl: Point, tr: Point, br: Point) -> Self {
        Self { bl, tl, tr, br }
    }

    /// Split the quadilateral horizontally returning the two new quadrilaterals.
    pub fn split_h(&self, a: f32, b: f32) -> (Self, Self) {
        let u = self.tl - self.bl;
        let v = self.tr - self.br;
        let p = self.bl + u.scale(a);
        let q = self.br + v.scale(b);
        (
            Self::new(self.bl, p, q, self.br),
            Self::new(p, self.tl, self.tr, q),
        )
    }

    /// Split the quadilateral vertically returning the two new quadrilaterals.
    pub fn split_v(&self, a: f32, b: f32) -> (Self, Self) {
        let u = self.br - self.bl;
        let v = self.tr - self.tl;
        let p = self.bl + u.scale(a);
        let q = self.tl + v.scale(b);
        (
            Self::new(self.bl, self.tl, q, p),
            Self::new(p, q, self.tr, self.br),
        )
    }

    /// Split the quadrilateral into 2 new quadrilaterals providing a function
    /// to determine whether to split horizontally or vertically. And another
    /// function to chosse the two split points.
    pub fn subdivide(
        &self,
        mut dir: impl FnMut(&Quad) -> Orientation,
        mut ab: impl FnMut(Orientation) -> (f32, f32),
    ) -> (Self, Self) {
        let or = dir(self);
        let (a, b) = ab(or);
        match or {
            Orientation::Horizontal => self.split_h(a, b),
            Orientation::Vertical => self.split_v(a, b),
        }
    }

    /// Convert the `Quad` to a list of points in clockwise order
    /// starting from the bottom left.
    pub fn to_vec(&self) -> Vec<Point> {
        vec![self.bl, self.tl, self.tr, self.br]
    }

    pub fn bounds(&self) -> (Point, Point) {
        let v = self.to_vec();
        let xs = v.iter().map(|p| p.x);
        let ys = v.iter().map(|p| p.y);
        let min_x = xs.clone().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max_x = xs.max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let min_y = ys.clone().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max_y = ys.max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        (pt(min_x, min_y), pt(max_x, max_y))
    }

    /// Find the direction of the longest length.
    pub fn best_dir(&self) -> Orientation {
        let x0 = if self.bl.x < self.tl.x {
            self.bl.x
        } else {
            self.tl.x
        };
        let x1 = if self.br.x > self.tr.x {
            self.br.x
        } else {
            self.tr.x
        };
        let y0 = if self.bl.y > self.br.y {
            self.bl.y
        } else {
            self.br.y
        };
        let y1 = if self.tl.y < self.tr.y {
            self.tl.y
        } else {
            self.tr.y
        };
        if (x1 - x0).abs() > (y0 - y1).abs() {
            Orientation::Vertical
        } else {
            Orientation::Horizontal
        }
    }
}

impl PartialEq for Quad {
    fn eq(&self, other: &Self) -> bool {
        self.bl.x == other.bl.x && self.bl.y == other.bl.y
    }
}

impl Eq for Quad {}

impl PartialOrd for Quad {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Quad {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Subdivide each quadrilateral in a `Vec`.
pub fn quad_divide_vec(
    quads: &[Quad],
    mut dir: impl FnMut(&Quad) -> Orientation,
    mut ab: impl FnMut(Orientation) -> (f32, f32),
) -> Vec<Quad> {
    let mut sub = vec![];
    for quad in quads {
        let q = quad.subdivide(&mut dir, &mut ab);
        sub.push(q.0);
        sub.push(q.1);
    }
    sub
}

/// Make a list of quadrilaterals from a list of lines.
pub fn lines_to_quads(lines: Vec<Vec<Point>>) -> Vec<Quad> {
    let mut quads: Vec<Quad> = Vec::new();
    for pair in lines.windows(2) {
        assert_eq!(pair[0].len(), pair[1].len());
        for p in pair[0]
            .iter()
            .zip(pair[1].clone())
            .collect::<Vec<(&Point, Point)>>()
            .windows(2)
        {
            quads.push(Quad::new(p[0].1, *p[0].0, *p[1].0, p[1].1))
        }
    }
    quads
}

pub fn warp_points(points: &[Point], scale: f32, factor: f32) -> Vec<Point> {
    let nfx = Perlin::default().set_seed(0);
    let nfy = Perlin::default().set_seed(1);
    let opts = NoiseOpts::new(1.0, 1.0, scale, scale, scale, factor);
    let qs = points.iter().map(|p| {
        let dx = noise2d(nfx, &opts, p.x, p.y);
        let dy = noise2d(nfy, &opts, p.x, p.y);
        pt(p.x + dx, p.y + dy)
    });
    qs.collect()
}

/// stops should be between 0 and 1.
pub fn ray_points(start: Point, end: Point, stops: &[f32]) -> Vec<Point> {
    let mut ps: Vec<Point> = Vec::new();
    let dir = end - start;
    for s in stops {
        ps.push(start + dir.scale(*s));
    }
    ps
}

pub fn ray_points_perspective(
    start: Point,
    middle: Point,
    end: Point,
    stops1: &[f32],
    stops2: &[f32],
) -> Vec<Point> {
    let mut ps = ray_points(start, middle, stops1);
    let mut qs = ray_points(end, middle, stops2);
    qs.reverse();
    ps.extend(qs[1..].iter());
    ps
}

pub fn perspective_quads(
    start: Point,
    top: Point,
    bottom: Point,
    end: Point,
    stops_left: &[f32],
    stops_right: &[f32],
    stops_vert: &[f32],
    noise_scale: f32,
    noise_factor: f32,
) -> Vec<Quad> {
    let mut lines: Vec<Vec<Point>> = Vec::new();
    let vs = ray_points(top, bottom, stops_vert);
    let vs = warp_points(&vs, noise_scale, noise_factor);
    for v in vs {
        lines.push(ray_points_perspective(
            start,
            v,
            end,
            stops_left,
            stops_right,
        ));
    }
    lines_to_quads(lines)
}

pub fn neighbor_quads(quad: Quad, quads: &[Quad]) -> Vec<Quad> {
    let f = |q: &Quad| {
        if (q.bl == quad.tl && q.br == quad.tr)
            || (q.bl == quad.br && q.tl == quad.tr)
            || (q.tl == quad.bl && q.tr == quad.br)
            || (q.tr == quad.tl && q.br == quad.bl)
        {
            Some(*q)
        } else {
            None
        }
    };
    quads.iter().filter_map(f).collect()
}

#[derive(Debug, Clone, Copy)]
/// A Triangle
pub struct Tri {
    /// Bottom Left.
    pub bl: Point,
    /// Top.
    pub top: Point,
    /// Bottom Right.
    pub br: Point,
}

impl Tri {
    pub fn new(bl: Point, top: Point, br: Point) -> Self {
        Self { bl, top, br }
    }

    /// Split the triangle from the bottom left vertex into tow triangles.
    pub fn split_bl(&self, a: f32) -> (Self, Self) {
        let u = self.top - self.br;
        let p = self.br + u.scale(a);
        (
            Self::new(self.bl, self.top, p),
            Self::new(self.bl, p, self.br),
        )
    }

    /// Split the triangle from the top vertex into tow triangles.
    pub fn split_top(&self, a: f32) -> (Self, Self) {
        let u = self.br - self.bl;
        let p = self.bl + u.scale(a);
        (
            Self::new(self.bl, self.top, p),
            Self::new(p, self.top, self.br),
        )
    }

    /// Split the triangle from the bottom right vertex into tow triangles.
    pub fn split_br(&self, a: f32) -> (Self, Self) {
        let u = self.top - self.bl;
        let p = self.bl + u.scale(a);
        (
            Self::new(self.bl, p, self.br),
            Self::new(p, self.top, self.br),
        )
    }

    /// Split the triangle into 2 new triangles providing a function
    /// to determine which vertex to split from. And another
    /// function to chosse the split point.
    pub fn subdivide(
        &self,
        mut dir: impl FnMut(&Tri) -> Vertex,
        mut a: impl FnMut() -> f32,
    ) -> (Self, Self) {
        let vert = dir(self);
        let p = a();
        match vert {
            Vertex::Bl => self.split_bl(p),
            Vertex::Top => self.split_top(p),
            Vertex::Br => self.split_br(p),
        }
    }

    /// Convert the `Tri` to a list of points in clockwise order
    /// starting from the bottom left.
    pub fn to_vec(&self) -> Vec<Point> {
        vec![self.bl, self.top, self.br]
    }

    /// Find the vertex opposite the longest side of the triangle.
    pub fn best_dir(&self) -> Vertex {
        let bl = (self.top - self.br).mag2();
        let top = (self.br - self.bl).mag2();
        let br = (self.top - self.bl).mag2();
        let v = vec![(bl, Vertex::Bl), (top, Vertex::Top), (br, Vertex::Br)];
        let m = v
            .into_iter()
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap();
        m.1
    }
}

impl PartialEq for Tri {
    fn eq(&self, other: &Self) -> bool {
        self.bl.x == other.bl.x && self.bl.y == other.bl.y
    }
}

impl Eq for Tri {}

impl PartialOrd for Tri {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tri {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn tri_divide_vec(
    tris: &[Tri],
    mut dir: impl FnMut(&Tri) -> Vertex,
    mut a: impl FnMut() -> f32,
) -> Vec<Tri> {
    let mut sub = vec![];
    for tri in tris {
        let t = tri.subdivide(&mut dir, &mut a);
        sub.push(t.0);
        sub.push(t.1);
    }
    sub
}

#[derive(Debug, Clone, Copy)]
pub enum Vertex {
    /// bottom left
    Bl,
    /// Top.
    Top,
    /// Bottom right.
    Br,
}
