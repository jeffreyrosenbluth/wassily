//! Quadrilateral subdivision.

use crate::prelude::{Orientation, Point};

/// A Quadrilateral.
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
        let p = self.bl + u * a;
        let q = self.br + v * b;
        (
            Self::new(self.bl, p, q, self.br),
            Self::new(p, self.tl, self.tr, q),
        )
    }

    /// Split the quadilateral vertically returning the two new quadrilaterals.
    pub fn split_v(&self, a: f32, b: f32) -> (Self, Self) {
        let u = self.br - self.bl;
        let v = self.tr - self.tl;
        let p = self.bl + u * a;
        let q = self.tl + v * b;
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
        mut ab: impl FnMut() -> (f32, f32),
    ) -> (Self, Self) {
        let vert = dir(self);
        let (a, b) = ab();
        match vert {
            Orientation::Horizontal => self.split_h(a, b),
            Orientation::Vertical => self.split_v(a, b),
        }
    }

    /// Convert the `Quad` to a list of points in counter clockwise order
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

/// Subdived each quadrilateral in a `Vec`.
pub fn subdivide_vec(
    quads: &[Quad],
    mut dir: impl FnMut(&Quad) -> Orientation,
    mut ab: impl FnMut() -> (f32, f32),
) -> Vec<Quad> {
    let mut sub = vec![];
    for quad in quads {
        let q = quad.subdivide(&mut dir, &mut ab);
        sub.push(q.0);
        sub.push(q.1);
    }
    sub
}
