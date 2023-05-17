//! A Curl noise struct that holds a source noise function and an epsilon value.
use noise::NoiseFn;
use std::f64::consts::PI;

pub struct Curl<T> {
    pub source: T,
    pub eps: f64,
}

impl<T> Curl<T> {
    /// Creates a new `Curl` noise function with the given source noise function and a default
    /// epsilon value of 0.0001.
    pub fn new(source: T) -> Self {
        Self {
            source,
            eps: 0.0001,
        }
    }

    /// Sets a custom epsilon value for the `Curl` noise function and returns a new instance.
    pub fn eps(self, eps: f64) -> Self {
        Self { eps, ..self }
    }
}

impl<T> NoiseFn<f64, 2> for Curl<T>
where
    T: NoiseFn<f64, 2>,
{
    /// Calculates the Curl noise function at a given point using the source noise function and the epsilon value.
    /// The Curl value at the given point, represented as a f64 value.
    fn get(&self, point: [f64; 2]) -> f64 {
        let x = point[0];
        let y = point[1];
        let x0 = x - self.eps;
        let x1 = x + self.eps;
        let y0 = y - self.eps;
        let y1 = y + self.eps;
        let dfdx = (self.source.get([x1, y]) - self.source.get([x0, y])) / (2.0 * self.eps);
        let dfdy = (self.source.get([x, y1]) - self.source.get([x, y0])) / (2.0 * self.eps);
        dfdy.atan2(-dfdx) / PI
    }
}
