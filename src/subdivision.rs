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

use crate::prelude::{Orientation, mag_squared, scale};
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
        let p = self.bl + scale(a, u);
        let q = self.br + scale(b, v);
        (
            Self::new(self.bl, p, q, self.br),
            Self::new(p, self.tl, self.tr, q),
        )
    }

    /// Split the quadilateral vertically returning the two new quadrilaterals.
    pub fn split_v(&self, a: f32, b: f32) -> (Self, Self) {
        let u = self.br - self.bl;
        let v = self.tr - self.tl;
        let p = self.bl + scale(a, u);
        let q = self.tl + scale(b, v);
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
        let p = self.br + scale(a, u);
        (
            Self::new(self.bl, self.top, p),
            Self::new(self.bl, p, self.br),
        )
    }

    /// Split the triangle from the top vertex into tow triangles.
    pub fn split_top(&self, a: f32) -> (Self, Self) {
        let u = self.br - self.bl;
        let p = self.bl + scale(a, u);
        (
            Self::new(self.bl, self.top, p),
            Self::new(p, self.top, self.br),
        )
    }

    /// Split the triangle from the bottom right vertex into tow triangles.
    pub fn split_br(&self, a: f32) -> (Self, Self) {
        let u = self.top - self.bl;
        let p = self.bl + scale(a, u);
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
        let bl = mag_squared(self.top - self.br);
        let top = mag_squared(self.br - self.bl);
        let br = mag_squared(self.top - self.bl);
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
