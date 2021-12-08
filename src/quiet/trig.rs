use noise::NoiseFn;

pub struct Trig {
    pub phases: [f64; 3],
    pub frequencies: [f64; 3],
}

impl Trig {
    pub fn new2(phase_x: f64, frequency_x: f64, phase_y: f64, frequency_y: f64) -> Self {
        Self {
            phases: [phase_x, phase_y, 0.0],
            frequencies: [frequency_x, frequency_y, 1.0],
        }
    }

    pub fn new3(
        phase_x: f64,
        frequency_x: f64,
        phase_y: f64,
        frequency_y: f64,
        phase_z: f64,
        frequency_z: f64,
    ) -> Self {
        Self {
            phases: [phase_x, phase_y, phase_z],
            frequencies: [frequency_x, frequency_y, frequency_z],
        }
    }

    pub fn new(phases: [f64; 3], frequencies: [f64; 3]) -> Self {
        Self {
            phases,
            frequencies,
        }
    }
}

impl Default for Trig {
    fn default() -> Self {
        Self {
            phases: [0.0, 0.0, 0.0],
            frequencies: [1.0, 1.0, 1.0],
        }
    }
}

impl NoiseFn<f64, 2> for Trig {
    fn get(&self, point: [f64; 2]) -> f64 {
        let x = std::f64::consts::TAU * self.frequencies[0] * (point[0] + self.phases[0]);
        let y = std::f64::consts::TAU * self.frequencies[1] * (point[1] + self.phases[1]);
        x.sin() * y.sin()
    }
}

impl NoiseFn<f64, 3> for Trig {
    fn get(&self, point: [f64; 3]) -> f64 {
        let x = std::f64::consts::TAU * self.frequencies[0] * (point[0] + self.phases[0]);
        let y = std::f64::consts::TAU * self.frequencies[1] * (point[1] + self.phases[1]);
        let z = std::f64::consts::TAU * self.frequencies[2] * (point[2] + self.phases[2]);
        x.sin() * y.sin() * z.sin()
    }
}
