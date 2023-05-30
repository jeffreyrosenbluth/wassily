use crate::points::Algebra;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use tiny_skia::{Point, Rect};

pub fn calculate_hash<T: Hash>(t: T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// Map a value from one range to anothe.
pub fn map_range(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}

/// Calculate the curl of a function from R2 -> R
pub fn curl(f: impl Fn(f32, f32) -> f32, x: f32, y: f32, eps: f32) -> f32 {
    let x0 = x - eps;
    let x1 = x + eps;
    let y0 = y - eps;
    let y1 = y + eps;
    let dfdx = (f(x1, y) - f(x0, y)) / (2.0 * eps);
    let dfdy = (f(x, y1) - f(x, y0)) / (2.0 * eps);
    dfdy.atan2(-dfdx)
}

pub fn bias(b: f32, t: f32) -> f32 {
    t / ((1.0 / b - 2.0) * (1.0 - t) + 1.0)
}

pub fn gain(g: f32, t: f32) -> f32 {
    if t < 0.5 {
        bias(g, 2.0 * t) / 2.0
    } else {
        bias(1.0 - g, 2.0 * t - 1.0) / 2.0 + 0.5
    }
}

/// Smoothstep function - Cubic.
pub fn smooth_step(t: f32) -> f32 {
    let s = t.clamp(0.0, 1.0);
    s * s * (3.0 - 2.0 * s)
}

/// Smootherstep function - Quintic.
pub fn smoother_step(t: f32) -> f32 {
    let s = t.clamp(0.0, 1.0);
    s * s * s * (6.0 * s * s - 15.0 * s + 10.0)
}

/// The bounding rectangle of a set of points.
pub fn bounding_box(points: &[Point], min_size: f32) -> Rect {
    let (left, top, right, bottom) =
        points
            .iter()
            .fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |mut acc, p| {
                if p.x < acc.0 {
                    acc.0 = p.x
                };
                if p.x > acc.2 {
                    acc.2 = p.x
                };
                if p.y < acc.1 {
                    acc.1 = p.y
                };
                if p.y > acc.3 {
                    acc.3 = p.y
                };
                acc
            });
    let right = right.max(left + min_size);
    let bottom = bottom.max(top + min_size);
    Rect::from_ltrb(left, top, right, bottom).unwrap()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Trail {
    Open,
    Closed,
}

/// Chaiken's algorithm for curve smoothing.
pub fn chaiken(pts: &[Point], smoothness: u32, trail: Trail) -> Vec<Point> {
    let mut pts = pts.to_vec();
    const RATIO: f32 = 0.25;
    if smoothness == 0 || pts.len() < 3 {
        if trail == Trail::Closed {
            pts.push(pts[0])
        }
        return pts;
    }
    if trail == Trail::Closed {
        pts.push(pts[0]);
    }
    let mut c: Vec<Point> = pts
        .windows(2)
        .flat_map(|ps| [ps[0].lerp(ps[1], RATIO), ps[1].lerp(ps[0], RATIO)])
        .collect();
    if trail == Trail::Open {
        c.insert(0, pts[0]);
        c.push(pts[pts.len() - 1]);
    }
    chaiken(&c, smoothness - 1, trail)
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::prelude::pt;

    #[test]
    fn smooth_step_test() {
        assert_eq!(smooth_step(0.0), 0.0);
        assert_eq!(smooth_step(1.0), 1.0);
        assert_eq!(smooth_step(0.5), 0.5);
        assert_eq!(smooth_step(0.25), 0.15625);
        assert_eq!(smooth_step(0.75), 0.84375);
    }

    #[test]
    fn smoother_step_test() {
        assert_eq!(smoother_step(0.0), 0.0);
        assert_eq!(smoother_step(1.0), 1.0);
        assert_eq!(smoother_step(0.5), 0.5);
        assert_eq!(smoother_step(0.25), 0.103515625);
        assert_eq!(smoother_step(0.75), 0.8964844);
    }

    #[test]
    fn bounding_box_test() {
        let points = vec![pt(10, 10), pt(-100, 90), pt(100, -80), pt(80, 100)];
        let bbox = bounding_box(&points, 0.0);
        assert_eq!(bbox, Rect::from_ltrb(-100.0, -80.0, 100.0, 100.0).unwrap());
    }
}
