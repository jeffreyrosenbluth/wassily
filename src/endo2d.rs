//! Endomorphisms of the unit square.
//! Mappings from the unit square to itself.

use crate::points::Algebra;
use crate::prelude::{pt, Point, PI};

pub fn sinusoid(p: Point) -> Point {
    pt(p.x.sin(), p.y.sin())
}

pub fn spherical(p: Point) -> Point {
    p.scale(p.mag())
}

pub fn swirl(p: Point) -> Point {
    let r = p.mag();
    let r2 = r * r;
    pt(
        p.x * r2.sin() - p.y * r2.cos(),
        p.x * r2.cos() + p.y * r2.sin(),
    )
}

pub fn horseshoe(p: Point) -> Point {
    pt((p.x + p.y) * (p.x - p.y), 2.0 * p.x * p.y).scale(1.0 / p.mag())
}

pub fn to_polar(p: Point) -> Point {
    pt(p.x.atan2(p.y) / PI, p.mag() - 1.0)
}

pub fn hankerchief(p: Point) -> Point {
    let theta = p.x.atan2(p.y);
    let r = p.mag();
    pt((theta + r).sin(), (theta - r).cos()).scale(r)
}

pub fn heart(p: Point) -> Point {
    let theta = p.x.atan2(p.y);
    let r = p.mag();
    pt((theta * r).sin(), -(theta * r).cos()).scale(r)
}

pub fn disc(p: Point) -> Point {
    let theta = p.x.atan2(p.y);
    let r = p.mag();
    pt((PI * r).sin(), (PI * r).cos()).scale(theta / PI)
}

pub fn spiral(p: Point) -> Point {
    let theta = p.x.atan2(p.y);
    let r = p.mag();
    pt(theta.cos() + r.sin(), theta.sin() - r.cos()).scale(1.0 / p.mag())
}

pub fn hyperbolic(p: Point) -> Point {
    let theta = p.x.atan2(p.y);
    let r = p.mag();
    pt(theta.sin() / r, r * theta.cos())
}

pub fn diamond(p: Point) -> Point {
    let theta = p.x.atan2(p.y);
    let r = p.mag();
    pt(theta.sin() * r.cos(), theta.cos() * r.sin())
}

pub fn ex(p: Point) -> Point {
    let theta = p.x.atan2(p.y);
    let r = p.mag();
    let p0 = (theta + r).sin();
    let p1 = (theta - r).cos();
    pt(p0 * p0 * p0 + p1 * p1 * p1, p0 * p0 * p0 - p1 * p1 * p1)
}

pub fn fisheye(p: Point) -> Point {
    pt(p.x, p.y).scale(2.0 / (1.0 + p.mag()))
}

pub fn exponential(p: Point) -> Point {
    pt((PI * p.y).cos(), (PI * p.y).sin()).scale((p.x - 1.0).exp())
}

pub fn tangent(p: Point) -> Point {
    pt(p.x.sin() / p.y.cos(), p.y.tan())
}

pub fn cross(p: Point) -> Point {
    let s = 1.0 / ((p.x * p.x - p.y * p.y) * (p.x * p.x - p.y * p.y));
    p.scale(s)
}
