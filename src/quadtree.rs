use crate::points::{pt, Algebra};
use tiny_skia::Point;

const CAPACITY: usize = 64;

pub fn blq(bl: Point, tr: Point) -> (Point, Point) {
    (bl, (bl + tr).scale(0.5))
}

pub fn brq(bl: Point, tr: Point) -> (Point, Point) {
    (pt((bl.x + tr.x) / 2.0, bl.y), pt(tr.x, (bl.y + tr.y) / 2.0))
}

pub fn tlq(bl: Point, tr: Point) -> (Point, Point) {
    (pt(bl.x, (bl.y + tr.y) / 2.0), pt((bl.x + tr.x) / 2.0, tr.y))
}

pub fn trq(bl: Point, tr: Point) -> (Point, Point) {
    ((bl + tr).scale(0.5), tr)
}

pub trait Position {
    fn pos(&self) -> Point;
}

impl Position for Point {
    fn pos(&self) -> Point {
        *self
    }
}

#[derive(Debug, Clone)]
pub struct Quadrants<T> {
    pub bl: Box<QNode<T>>,
    pub br: Box<QNode<T>>,
    pub tl: Box<QNode<T>>,
    pub tr: Box<QNode<T>>,
}

impl<T> Quadrants<T> {
    pub fn new(bl: QNode<T>, br: QNode<T>, tl: QNode<T>, tr: QNode<T>) -> Self {
        let bl = Box::new(bl);
        let br = Box::new(br);
        let tl = Box::new(tl);
        let tr = Box::new(tr);
        Self { bl, br, tl, tr }
    }
}

#[derive(Debug, Clone)]
pub enum QNode<T> {
    Points(Vec<T>),
    Quad(Quadrants<T>),
}

impl<T> Default for QNode<T> {
    fn default() -> Self {
        QNode::Points(vec![])
    }
}

impl<T: Position + Clone> QNode<T> {
    pub fn new(ps: Vec<T>, bl: Point, tr: Point) -> Self {
        let mut qt = QNode::default();
        for p in ps {
            qt.insert(p, bl, tr);
        }
        qt
    }

    pub fn split(&mut self, bl: Point, tr: Point) {
        let mut bl_quad = vec![];
        let mut br_quad = vec![];
        let mut tl_quad = vec![];
        let mut tr_quad = vec![];
        match self {
            QNode::Points(ps) => {
                let midx = (bl.x + tr.x) / 2.0;
                let midy = (bl.y + tr.y) / 2.0;
                for p in ps {
                    if p.pos().x <= midx {
                        if p.pos().y <= midy {
                            bl_quad.push(p.clone());
                        } else {
                            tl_quad.push(p.clone());
                        }
                    } else if p.pos().y <= midy {
                        br_quad.push(p.clone());
                    } else {
                        tr_quad.push(p.clone());
                    }
                }
            }
            _ => {
                println!("Warning: Only QNode::Points should be split");
                return;
            }
        }
        let quadrants = Quadrants::new(
            QNode::Points(bl_quad),
            QNode::Points(br_quad),
            QNode::Points(tl_quad),
            QNode::Points(tr_quad),
        );
        *self = QNode::Quad(quadrants);
    }

    pub fn insert(&mut self, p: T, bl: Point, tr: Point) {
        let midx = (bl.x + tr.x) / 2.0;
        match self {
            QNode::Points(pts) => {
                pts.push(p);
                if pts.len() > CAPACITY {
                    self.split(bl, tr);
                }
            }
            QNode::Quad(q) if p.pos().x <= midx => {
                let midy = (bl.y + tr.y) / 2.0;
                let mid = pt(midx, midy);
                if p.pos().y <= midy {
                    q.bl.insert(p, bl, mid);
                } else {
                    q.tl.insert(p, pt(bl.x, midy), pt(midx, tr.y));
                }
            }
            QNode::Quad(q) => {
                let midy = (bl.y + tr.y) / 2.0;
                let mid = pt(midx, midy);
                if p.pos().y <= midy {
                    q.br.insert(p, pt(midx, bl.y), pt(tr.x, midy));
                } else {
                    q.tr.insert(p, mid, tr);
                }
            }
        }
    }

    pub fn points_in_circle(&self, bl: Point, tr: Point, center: Point, radius: f32) -> Vec<T> {
        let mut pts = vec![];
        if !intersects(bl, tr, center, radius) {
            return pts;
        }
        match self {
            QNode::Points(ps) => {
                for p in ps {
                    if (p.pos().x - center.x) * (p.pos().x - center.x)
                        + (p.pos().y - center.y) * (p.pos().y - center.y)
                        <= radius * radius
                    {
                        pts.push(p.clone());
                    }
                }
            }
            QNode::Quad(q) => {
                let (a, b) = blq(bl, tr);
                pts.append(&mut q.bl.points_in_circle(a, b, center, radius));

                let (a, b) = brq(bl, tr);
                pts.append(&mut q.br.points_in_circle(a, b, center, radius));

                let (a, b) = tlq(bl, tr);
                pts.append(&mut q.tl.points_in_circle(a, b, center, radius));

                let (a, b) = trq(bl, tr);
                pts.append(&mut q.tr.points_in_circle(a, b, center, radius));
            }
        }
        pts
    }
}

pub fn intersects(bl: Point, tr: Point, center: Point, radius: f32) -> bool {
    center.x >= bl.x - radius
        && center.x < tr.x + radius
        && center.y >= bl.y - radius
        && center.y < tr.y + radius
}

#[cfg(test)]
mod tests {
    use rand::{rngs::SmallRng, Rng, SeedableRng};

    use super::*;

    #[test]
    fn test_split() {
        let mut rng = SmallRng::seed_from_u64(0);
        let pts = vec![];
        let mut qt = QNode::Points(pts);
        for _ in 0..1000 {
            qt.insert(
                pt(rng.gen_range(0.0..3.0), rng.gen_range(0.0..3.0)),
                pt(0., 0.),
                pt(3., 3.),
            );
        }
        let c = qt.points_in_circle(pt(0.0, 0.0), pt(3.0, 3.0), pt(2.0, 0.5), 0.2);
        dbg!(c);
    }
}
