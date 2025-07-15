//! # 2D Endomorphisms
//!
//! Mathematical transformations that map points from the unit square to itself.
//! These functions are commonly used in fractal systems, particularly Iterated
//! Function Systems (IFS), and provide various artistic and mathematical effects
//! for generative art applications.
//!
//! ## What are Endomorphisms?
//!
//! An endomorphism is a mathematical function that maps a space to itself.
//! In this context, all functions map points within or around the unit square
//! `[-1, 1] × [-1, 1]` to new positions, creating various visual effects.
//!
//! ## Available Transformations
//!
//! ### Trigonometric Functions
//! - [`sinusoid()`]: Applies sine function to both coordinates
//! - [`hankerchief()`]: Creates handkerchief-like distortions
//! - [`heart()`]: Creates heart-shaped transformations
//!
//! ### Geometric Transformations
//! - [`spherical()`]: Spherical inversion transformation
//! - [`swirl()`]: Creates swirling, spiral effects
//! - [`horseshoe()`]: Horseshoe-shaped transformation
//! - [`disc()`]: Disc-based coordinate transformation
//!
//! ### Coordinate System Changes
//! - [`to_polar()`]: Converts to polar coordinate representation
//!
//! ## Usage in Fractal Systems
//!
//! ```no_run
//! use wassily_algorithms::*;
//! use wassily_core::*;
//!
//! // Create an IFS (Iterated Function System)
//! let mut point = pt(0.5, 0.3);
//! let mut canvas = Canvas::new(800, 600);
//!
//! // Apply transformations iteratively
//! for _ in 0..10000 {
//!     // Randomly choose a transformation
//!     point = match rand::random::<u8>() % 3 {
//!         0 => swirl(point),
//!         1 => heart(point),
//!         _ => spherical(point),
//!     };
//!     
//!     // Plot the transformed point
//!     let screen_x = (point.x + 1.0) * 400.0;
//!     let screen_y = (point.y + 1.0) * 300.0;
//!     canvas.dot(screen_x, screen_y, *WHITE);
//! }
//! ```
//!
//! ## Mathematical Properties
//!
//! Each transformation has unique mathematical properties:
//!
//! - **Continuous**: All transformations are continuous functions
//! - **Bounded**: Most transformations keep points within reasonable bounds
//! - **Differentiable**: Most functions are smooth and differentiable
//! - **Invertible**: Some transformations are invertible, others are not
//!
//! ## Applications
//!
//! - **Fractal Generation**: IFS fractals and strange attractors
//! - **Artistic Effects**: Distortion effects for images and shapes
//! - **Mathematical Visualization**: Studying function behavior
//! - **Animation**: Creating organic, flowing motion patterns

use wassily_core::points::{pt, Algebra};
use tiny_skia::Point;
use std::f32::consts::PI;

/// Applies the sine function to both x and y coordinates.
/// 
/// This transformation creates a grid-like pattern with curved boundaries,
/// mapping the input coordinates through sine functions.
/// 
/// # Example
/// ```no_run
/// use wassily_algorithms::sinusoid;
/// use wassily_core::pt;
/// 
/// let input = pt(1.0, 0.5);
/// let result = sinusoid(input);
/// ```
pub fn sinusoid(p: Point) -> Point {
    pt(p.x.sin(), p.y.sin())
}

/// Spherical inversion transformation.
/// 
/// This transformation scales the point by its magnitude, creating a radial
/// distortion effect that resembles looking at the plane through a spherical lens.
/// 
/// # Example
/// ```no_run
/// use wassily_algorithms::spherical;
/// use wassily_core::pt;
/// 
/// let input = pt(0.5, 0.3);
/// let result = spherical(input);
/// ```
pub fn spherical(p: Point) -> Point {
    p.scale(p.mag())
}

/// Creates a swirling, spiral transformation.
/// 
/// This transformation rotates points around the origin with the rotation
/// angle proportional to the square of the distance from the origin,
/// creating a spiral or whirlpool effect.
/// 
/// # Example
/// ```no_run
/// use wassily_algorithms::swirl;
/// use wassily_core::pt;
/// 
/// let input = pt(0.8, 0.2);
/// let result = swirl(input);
/// ```
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
