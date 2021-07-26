use crate::prelude::Point;

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    bl: Point,
    tl: Point,
    tr: Point,
    br: Point,
}

impl Quad {
    pub fn new(bl: Point, tl: Point, tr: Point, br: Point) -> Self {
        Self { bl, tl, tr, br }
    }

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

    pub fn subdivide(
        &self,
        mut dir: impl FnMut(&Quad) -> bool,
        mut ab: impl FnMut() -> (f32, f32),
    ) -> (Self, Self) {
        let vert = dir(self);
        let (a, b) = ab();
        if vert {
            self.split_v(a, b)
        } else {
            self.split_h(a, b)
        }
    }

    pub fn to_vec(&self) -> Vec<Point> {
        vec![self.bl, self.tl, self.tr, self.br]
    }

    pub fn best_dir(&self) -> bool {
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
        x1 - x0 > y1 - y0
    }
}

pub fn subdivide_vec(
    quads: &[Quad],
    mut dir: impl FnMut(&Quad) -> bool,
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
