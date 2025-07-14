//! Functions for dealing with 2D and 3D points.
use num_complex::Complex32;
use num_traits::AsPrimitive;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use tiny_skia::Point;

pub use std::f32::consts::FRAC_PI_2 as HALF_PI;
pub use std::f32::consts::PI;
pub use std::f32::consts::TAU;

/// Create a `Point` from x and y coordinates.
pub fn pt<S, T>(x: S, y: T) -> Point
where
    S: AsPrimitive<f32>,
    T: AsPrimitive<f32>,
{
    Point::from_xy(x.as_(), y.as_())
}

pub fn pt3<S, T, U>(x: S, y: T, z: U) -> Point3
where
    S: AsPrimitive<f32>,
    T: AsPrimitive<f32>,
    U: AsPrimitive<f32>,
{
    Point3 {
        x: x.as_(),
        y: y.as_(),
        z: z.as_(),
    }
}

pub fn polar<S, T>(theta: S, r: T) -> Point
where
    S: AsPrimitive<f32>,
    T: AsPrimitive<f32>,
{
    Point::from_xy(r.as_() * theta.as_().cos(), r.as_() * theta.as_().sin())
}

pub fn complex(p: Point) -> Complex32 {
    Complex32::new(p.x, p.y)
}

pub fn center<S, T>(width: S, height: T) -> Point
where
    S: AsPrimitive<f32>,
    T: AsPrimitive<f32>,
{
    Point::from_xy(width.as_() / 2.0, height.as_() / 2.0)
}

/// A trait to unify the interface of tiny-skia `Point` and wassily `Point3`.
pub trait Algebra: Copy {
    const ZERO: Self;

    /// Multiply all coordinates by a scalar.
    fn scale(self, k: f32) -> Self;

    /// Linear interpolation between two points.
    fn lerp(self, other: Self, t: f32) -> Self;

    /// The square of the magnitude of the vector.
    fn mag2(self) -> f32;

    /// The distance squared between two points.
    fn dist2(self, other: Self) -> f32;

    /// The dot product of two vectors.
    fn dot(self, other: Self) -> f32;

    /// The magnitude of the vector.
    fn mag(self) -> f32 {
        self.mag2().sqrt()
    }

    /// Normalize the vector.
    fn normalize(self) -> Self {
        self.scale(1.0 / self.mag())
    }

    /// The average of two vectors.
    fn average(self, other: Self) -> Self {
        self.lerp(other, 0.5)
    }

    /// Distance between two points.
    fn dist(self, other: Self) -> f32 {
        self.dist2(other).sqrt()
    }
}

impl Algebra for Point {
    const ZERO: Self = Point { x: 0.0, y: 0.0 };

    fn mag2(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    fn scale(self, k: f32) -> Self {
        Point::from_xy(k * self.x, k * self.y)
    }

    fn lerp(self, other: Self, t: f32) -> Self {
        let x = self.x * (1.0 - t) + t * other.x;
        let y = self.y * (1.0 - t) + t * other.y;
        Self::from_xy(x, y)
    }

    fn dist2(self, other: Self) -> f32 {
        pt(self.x - other.x, self.y - other.y).mag2()
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

/// Spherical coordinates of a `Point3`.
#[derive(Clone, Copy, Debug)]
pub struct Spherical {
    pub phi: f32,
    pub theta: f32,
    pub radius: f32,
}

impl Spherical {
    pub fn new(phi: f32, theta: f32, radius: f32) -> Self {
        Self { phi, theta, radius }
    }
}

/// A point in 3D space.
#[derive(Clone, Copy, Debug)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Algebra for Point3 {
    const ZERO: Self = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    fn scale(self, k: f32) -> Self {
        self * k
    }

    fn lerp(self, other: Self, t: f32) -> Self {
        let x = self.x * (1.0 - t) + t * other.x;
        let y = self.y * (1.0 - t) + t * other.y;
        let z = self.z * (1.0 - t) + t * other.z;
        Point3 { x, y, z }
    }

    fn mag2(self) -> f32 {
        self.dot(self)
    }

    fn dist2(self, other: Self) -> f32 {
        pt3(self.x - other.x, self.y - other.y, self.z - other.z).mag2()
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Conert from cartesian to Spherical coordinates.
    pub fn to_spherical(&self, center: Point3) -> Spherical {
        let x = self.x - center.x;
        let y = self.y - center.y;
        let z = self.z - center.z;
        let radius = (x * x + y * y + z * z).sqrt();
        let theta = (z / radius).acos();
        let phi = y.atan2(x);
        Spherical { phi, theta, radius }
    }

    /// Rotate around the x-axis.
    pub fn rotate_x(&self, theta: f32) -> Self {
        let x = self.x;
        let y = self.y * theta.cos() - self.z * theta.sin();
        let z = self.y * theta.sin() + self.z * theta.cos();
        Self { x, y, z }
    }

    /// Rotate around the y-axis.
    pub fn rotate_y(&self, theta: f32) -> Self {
        let x = self.x * theta.cos() + self.z * theta.sin();
        let y = self.y;
        let z = -self.x * theta.sin() + self.z * theta.cos();
        Self { x, y, z }
    }

    /// Rotate around the z-axis.
    pub fn rotate_z(&self, theta: f32) -> Self {
        let x = self.x * theta.cos() - self.y * theta.sin();
        let y = self.x * theta.sin() + self.y * theta.cos();
        let z = self.z;
        Self { x, y, z }
    }
}

impl Sub for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Self) -> Self::Output {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Point3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Add for Point3 {
    type Output = Point3;

    fn add(self, rhs: Self) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Point3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<f32> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: f32) -> Self::Output {
        Point3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f32> for Point3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Point3 {
    type Output = Point3;

    fn div(self, rhs: f32) -> Self::Output {
        Point3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f32> for Point3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Mul<Point3> for f32 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output {
        rhs * self
    }
}

impl Div<Point3> for f32 {
    type Output = Point3;

    fn div(self, rhs: Point3) -> Self::Output {
        rhs / self
    }
}
