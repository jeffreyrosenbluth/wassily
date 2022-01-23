use std::f64::consts::PI;
use noise::NoiseFn;

pub struct Curl<T> {
    pub source: T,
    pub eps: f64,
}

impl<T> Curl<T> {
    pub fn new(source: T) -> Self {
        Self {
            source,
            eps: 0.0001,
        }
    }

    pub fn eps(self, eps: f64) -> Self {
        Self { eps, ..self }
    }
}

impl<T> NoiseFn<f64, 2> for Curl<T>
where
    T: NoiseFn<f64, 2>,
{
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
