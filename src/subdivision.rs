//! Quadrilateral and Triangle subdivision.
//! # Example
//! ```rust
//! use wassily::prelude::*;
//!
//! let quad = Quad::new(
//!     point2(0, 0),
//!     point2(0, HEIGHT),
//!     point2(WIDTH, HEIGHT),
//!     point2(WIDTH, 0),
//! );
//! let mut qs = vec![quad];
//! let n = 8;
//! for _ in 0..n {
//!     qs = subdivide_vec(
//!         &qs,
//!         |q| q.best_dir(), // Choose the longer orientaion to subdivide along.
//!         || (.25, .5),
//!     );
//! }
//! ```

use crate::noises::*;
use crate::prelude::{pt, Algebra, NoiseOpts, Orientation};
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
        if x1 - x0 > y1 - y0 {
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
        self.bl.x.partial_cmp(&other.bl.x)
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
        let dx = noise2d(&nfx, &opts, p.x, p.y);
        let dy = noise2d(&nfy, &opts, p.x, p.y);
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
    ps.extend(qs[1..].into_iter());
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
        let r = if (q.bl == quad.tl && q.br == quad.tr)
            || (q.bl == quad.br && q.tl == quad.tr)
            || (q.tl == quad.bl && q.tr == quad.br)
            || (q.tr == quad.tl && q.br == quad.bl)
        {
            Some(q.clone())
        } else {
            None
        };
        r
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
        self.bl.x.partial_cmp(&other.bl.x)
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
