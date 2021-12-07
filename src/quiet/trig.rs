use noise::NoiseFn;

pub struct Trig<'a> {
    pub phases: &'a [f64],
    pub frequencies: &'a [f64],
}

impl<'a> Trig<'a> {
    pub fn new(phases: &'a [f64], frequencies: &'a [f64]) -> Self {
        Self {
            phases, frequencies
        }
    }
}

impl<'a> Default for Trig<'a> {
    fn default() -> Self {
        Self {
            phases: &[0.0, 0.0, 0.0],
            frequencies: &[1.0, 1.0, 1.0],
        }
    }
}

impl<'a> NoiseFn<f64, 2> for Trig<'a> {
    fn get(&self, point: [f64; 2]) -> f64 {
        let x = std::f64::consts::TAU * self.frequencies[0] * (point[0] + self.phases[0]);
        let y = std::f64::consts::TAU * self.frequencies[1] * (point[1] + self.phases[1]);
        x.sin() * y.sin()
    }
}

impl<'a> NoiseFn<f64, 3> for Trig<'a> {
    fn get(&self, point: [f64; 3]) -> f64 {
        let x = std::f64::consts::TAU * self.frequencies[0] * (point[0] + self.phases[0]);
        let y = std::f64::consts::TAU * self.frequencies[1] * (point[1] + self.phases[1]);
        let z = std::f64::consts::TAU * self.frequencies[2] * (point[2] + self.phases[2]);
        x.sin() * y.sin() * z.sin()
    }
}