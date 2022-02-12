use std::ops::{Add, Mul, Sub};

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

#[derive(Clone, Copy, Debug)]
pub struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_spherical(&self, center: Point3) -> Spherical {
        let x = self.x - center.x;
        let y = self.y - center.y;
        let z = self.z - center.z;
        let radius = (x * x + y * y + z * z).sqrt();
        let theta = (z / radius).acos();
        let phi = y.atan2(x);
        Spherical { phi, theta, radius }
    }

    pub fn dot_prod(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn mag(&self) -> f32 {
        self.dot_prod(&self).sqrt()
    }

    pub fn mag2(&self) -> f32 {
        self.dot_prod(&self)
    }

    pub fn rotate_x(&self, theta: f32) -> Self {
        let x = self.x;
        let y = self.y * theta.cos() - self.z * theta.sin();
        let z = self.y * theta.sin() + self.z * theta.cos();
        Self { x, y, z }
    }

    pub fn rotate_y(&self, theta: f32) -> Self {
        let x = self.x * theta.cos() + self.z * theta.sin();
        let y = self.y;
        let z = -self.x * theta.sin() + self.z * theta.cos();
        Self { x, y, z }
    }

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

impl Add for Point3 {
    type Output = Point3;

    fn add(self, rhs: Self) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f32> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: f32) -> Self::Output {
        Point3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
