use crate::core::points::*;
use crate::prelude::Point;

#[derive(Debug, Clone)]
pub struct ParametricPath {
    pub points: Vec<Point>,
    pub lengths: Vec<f32>,
    pub total_length: f32,
    pub params: Vec<f32>,
}

impl ParametricPath {
    pub fn new(points: Vec<Point>) -> Self {
        assert!(points.len() >= 2);
        let mut lengths = Vec::new();
        for p in points.windows(2) {
            lengths.push(p[0].distance(p[1]));
        }
        let total_length = lengths.iter().sum::<f32>();
        let mut params: Vec<f32> = lengths
            .iter()
            .scan(0.0, |state, &length| {
                *state += length;
                Some(*state / total_length)
            })
            .collect();
        params.insert(0, 0f32);
        Self {
            points,
            lengths: lengths.clone(),
            total_length,
            params,
        }
    }

    pub fn point_at(&self, t: f32) -> Point {
        assert!((0.0..=1.0).contains(&t));
        if t == 0.0 {
            return self.points[0];
        };
        if t == 1.0 {
            return self.points[self.points.len() - 1];
        };

        let (i, t1) = self
            .params
            .iter()
            .enumerate()
            .find(|&p| &t <= p.1)
            .unwrap_or_else(|| panic!("Cannot find param >= t ({t})"));
        let t0 = self.params[i - 1];
        self.points[i - 1].lerp(self.points[i], (t - t0) / (t1 - t0))
    }

    pub fn section(&self, t0: f32, t1: f32) -> Vec<Point> {
        assert!(t0 >= 0.0 && t0 < t1 && t1 <= 1.0);
        let idx0 = self.params.iter().position(|t| t >= &t0).unwrap();
        let idx1 = self.params.iter().position(|t| t >= &t1).unwrap();
        let mut points = Vec::new();
        if self.params[idx0] > t0 {
            points.push(self.point_at(t0));
        }
        for i in idx0..idx1 {
            points.push(self.points[i]);
        }
        points.push(self.point_at(t1));
        points
    }
}

/// Refine a curve by inserting points between existing points. Until the piecewise linear
/// segments are within `eps` of the curve defined by `f`.
pub fn refine(pts: &[Point], f: impl Fn(f32) -> f32, eps: f32) -> Vec<Point> {
    fn refiner(pts: &[Point], f: impl Fn(f32) -> f32, idx: usize, eps: f32) -> Vec<Point> {
        let mut refined_pts = pts.to_vec();
        if idx > refined_pts.len() - 2 {
            refined_pts
        } else {
            let a = refined_pts[idx];
            let b = refined_pts[idx + 1];
            let m = (a + b).scale(0.5);
            let q = pt(m.x, f(m.x));
            if (q.y - m.y).abs() > eps {
                refined_pts.insert(idx + 1, q);
                refiner(&refined_pts, f, idx, eps)
            } else {
                refiner(&refined_pts, f, idx + 1, eps)
            }
        }
    }
    refiner(pts, f, 0, eps)
}

/// Create a piecewise linear curve from a function so that it looks smooth with an
/// error of < `eps`.
/*
```rust
 let f = |x: f32| x.sin();
    let sine_path = curve(f, pt(0, 1080 / 2), 1080.0, 400.0, 0.1);
    for t in sine_path.clone().windows(2) {
        let c = (*WHITE)
            .lerp(&*BLUE, 1.0 - t[0].x / 1080.0)
        Shape::new()
            .points( &t,)
            .no_fill()
            .stroke_weight(16.0)
            .stroke_color(c)
            .line_cap(LineCap::Square)
            .draw(&mut canvas);
    }
````
*/
pub fn curve(f: fn(f32) -> f32, start: Point, width: f32, height: f32, eps: f32) -> Vec<Point> {
    let g = |x: f32| start.y + height * f(2.0 * PI * x / width);
    let mut pts = Vec::new();
    for i in 0..=10 {
        let s = width * i as f32 / 10.0;
        pts.push(pt(start.x + s, g(s)));
    }
    refine(&pts, g, eps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::points::pt;

    #[test]
    fn new_test() {
        let points = vec![pt(0, 0), pt(1, 1), pt(2, 3)];
        let pp = ParametricPath::new(points);
        assert_eq!(pp.lengths[0], 2_f32.sqrt());
        assert_eq!(pp.lengths[1], 5_f32.sqrt());
        assert_eq!(pp.total_length, 2_f32.sqrt() + 5_f32.sqrt());
        assert_eq!(pp.params, vec![0.0, 0.3874259, 1.0]);
    }

    #[test]
    fn point_at_test() {
        let points = vec![pt(0, 0), pt(1, 1), pt(2, 3)];
        let pp = ParametricPath::new(points);
        assert_eq!(pp.point_at(0.0), pt(0, 0));
        assert_eq!(pp.point_at(1.0), pt(2, 3));
        assert_eq!(pp.point_at(0.5), pt(1.1837722, 1.3675444));
        assert_eq!(pp.point_at(0.25), pt(0.6452847, 0.6452847));
        assert_eq!(pp.point_at(0.75), pt(1.591886, 2.1837723));
        assert_eq!(pp.point_at(0.3874259), pt(1.0, 1.0));
    }

    #[test]
    fn section_test() {
        let points = vec![pt(0, 0), pt(1, 1), pt(2, 3)];
        let pp = ParametricPath::new(points.clone());
        assert_eq!(pp.section(0.0, 1.0), points);
        assert_eq!(
            pp.section(0.5, 1.0),
            vec![pt(1.1837722, 1.3675444), pt(2, 3)]
        );
        assert_eq!(
            pp.section(0.25, 0.5),
            vec![
                pt(0.6452847, 0.6452847),
                pt(1.0, 1.0),
                pt(1.1837722, 1.3675444)
            ]
        );
        assert_eq!(
            pp.section(0.25, 0.75),
            vec![
                pt(0.6452847, 0.6452847),
                pt(1.0, 1.0),
                pt(1.591886, 2.1837723)
            ]
        )
    }
}
