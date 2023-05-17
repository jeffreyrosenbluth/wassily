//! Various sinusoidal noises.
use noise::NoiseFn;
#[derive(Clone, Copy, Debug)]

/// Parameters for the sinusoidal fumctions. One for each dimension.
/// The thrid dimension is ignored for 2d noise.
/// The outputs of the three sinusoids are averaged.
pub struct Sinusoid {
    pub phases: [f64; 3],
    pub frequencies: [f64; 3],
}

impl Sinusoid {
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

impl Default for Sinusoid {
    fn default() -> Self {
        Self {
            phases: [0.0, 0.0, 0.0],
            frequencies: [1.0, 1.0, 1.0],
        }
    }
}

impl NoiseFn<f64, 2> for Sinusoid {
    fn get(&self, point: [f64; 2]) -> f64 {
        let x = std::f64::consts::TAU * self.frequencies[0] * (point[0] + self.phases[0]);
        let y = std::f64::consts::TAU * self.frequencies[1] * (point[1] + self.phases[1]);
        0.5 * (x.sin() + y.sin())
    }
}

impl NoiseFn<f64, 3> for Sinusoid {
    fn get(&self, point: [f64; 3]) -> f64 {
        let x = std::f64::consts::TAU * self.frequencies[0] * (point[0] + self.phases[0]);
        let y = std::f64::consts::TAU * self.frequencies[1] * (point[1] + self.phases[1]);
        let z = std::f64::consts::TAU * self.frequencies[2] * (point[2] + self.phases[2]);
        (x.sin() + y.sin() + z.sin()) / 3.0
    }
}

/// Parameters for the carrier, modulator and index fumctions. One for each dimension.
/// The thrid dimension is ignored for 2d noise.
/// The outputs are multiplied for 2d noise and the aveage of the pairwise products
/// is returned for 3d noise.
#[derive(Clone, Copy, Debug)]
pub struct FMSynth {
    pub carrier_freqs: [f64; 3],
    pub modulator_freqs: [f64; 3],
    pub indices: [f64; 3],
}

impl FMSynth {
    pub fn new(carrier_freqs: [f64; 3], modulator_freqs: [f64; 3], indices: [f64; 3]) -> Self {
        Self {
            carrier_freqs,
            modulator_freqs,
            indices,
        }
    }

    pub fn new2(
        carrier_x: f64,
        modulator_x: f64,
        idx_x: f64,
        carrier_y: f64,
        modulator_y: f64,
        idx_y: f64,
    ) -> Self {
        Self {
            carrier_freqs: [carrier_x, carrier_y, 0.0],
            modulator_freqs: [modulator_x, modulator_y, 0.0],
            indices: [idx_x, idx_y, 0.0],
        }
    }

    pub fn new3(
        carrier_x: f64,
        modulator_x: f64,
        idx_x: f64,
        carrier_y: f64,
        modulator_y: f64,
        idx_y: f64,
        carrier_z: f64,
        modulator_z: f64,
        idx_z: f64,
    ) -> Self {
        Self {
            carrier_freqs: [carrier_x, carrier_y, carrier_z],
            modulator_freqs: [modulator_x, modulator_y, modulator_z],
            indices: [idx_x, idx_y, idx_z],
        }
    }

    pub fn carrier_x(mut self, freq: f64) -> Self {
        self.carrier_freqs[0] = freq;
        self
    }

    pub fn carrier_y(mut self, freq: f64) -> Self {
        self.carrier_freqs[1] = freq;
        self
    }

    pub fn carrier_z(mut self, freq: f64) -> Self {
        self.carrier_freqs[2] = freq;
        self
    }

    pub fn modulator_x(mut self, freq: f64) -> Self {
        self.modulator_freqs[0] = freq;
        self
    }

    pub fn modulator_y(mut self, freq: f64) -> Self {
        self.modulator_freqs[1] = freq;
        self
    }

    pub fn modulator_z(mut self, freq: f64) -> Self {
        self.modulator_freqs[2] = freq;
        self
    }

    pub fn idx_x(mut self, idx: f64) -> Self {
        self.indices[0] = idx;
        self
    }

    pub fn idx_y(mut self, idx: f64) -> Self {
        self.indices[1] = idx;
        self
    }

    pub fn idx_z(mut self, idx: f64) -> Self {
        self.indices[2] = idx;
        self
    }
}

impl Default for FMSynth {
    fn default() -> Self {
        Self {
            carrier_freqs: [1.0, 1.0, 1.0],
            modulator_freqs: [0.1, 0.1, 0.1],
            indices: [5.0, 5.0, 5.0],
        }
    }
}

impl NoiseFn<f64, 2> for FMSynth {
    fn get(&self, point: [f64; 2]) -> f64 {
        let fx = std::f64::consts::TAU * self.modulator_freqs[0] * point[0];
        let xm =
            std::f64::consts::TAU * self.carrier_freqs[0] * point[0] + self.indices[0] * fx.sin();

        let fy = std::f64::consts::TAU * self.modulator_freqs[1] * point[1];
        let ym =
            std::f64::consts::TAU * self.carrier_freqs[1] * point[1] + self.indices[1] * fy.sin();

        0.5 * (xm.sin() + ym.sin())
    }
}

impl NoiseFn<f64, 3> for FMSynth {
    fn get(&self, point: [f64; 3]) -> f64 {
        let fx = std::f64::consts::TAU * self.modulator_freqs[0] * point[0];
        let xm =
            std::f64::consts::TAU * self.carrier_freqs[0] * point[0] + self.indices[0] * fx.sin();

        let fy = std::f64::consts::TAU * self.modulator_freqs[1] * point[1];
        let ym =
            std::f64::consts::TAU * self.carrier_freqs[1] * point[1] + self.indices[1] * fy.sin();

        let fz = std::f64::consts::TAU * self.modulator_freqs[2] * point[2];
        let zm =
            std::f64::consts::TAU * self.carrier_freqs[2] * point[2] + self.indices[2] * fz.sin();

        (xm.sin() + ym.sin() + zm.sin()) / 3.0
    }
}

/// Parameters for the carrier, modulator and index fumctions. One for each dimension.
/// The thrid dimension is ignored for 2d noise.
/// The outputs are arveraged for 2d noise and the aveage of the pairwise products
/// is returned for 3d noise.
#[derive(Clone, Copy, Debug)]
pub struct FMCross {
    pub carrier_freqs: [f64; 3],
    pub modulator_freqs: [f64; 3],
    pub indices: [f64; 3],
}

impl FMCross {
    pub fn new(carrier_freqs: [f64; 3], modulator_freqs: [f64; 3], indices: [f64; 3]) -> Self {
        Self {
            carrier_freqs,
            modulator_freqs,
            indices,
        }
    }

    pub fn new2(
        carrier_x: f64,
        modulator_x: f64,
        idx_x: f64,
        carrier_y: f64,
        modulator_y: f64,
        idx_y: f64,
    ) -> Self {
        Self {
            carrier_freqs: [carrier_x, carrier_y, 0.0],
            modulator_freqs: [modulator_x, modulator_y, 0.0],
            indices: [idx_x, idx_y, 0.0],
        }
    }

    pub fn new3(
        carrier_x: f64,
        modulator_x: f64,
        idx_x: f64,
        carrier_y: f64,
        modulator_y: f64,
        idx_y: f64,
        carrier_z: f64,
        modulator_z: f64,
        idx_z: f64,
    ) -> Self {
        Self {
            carrier_freqs: [carrier_x, carrier_y, carrier_z],
            modulator_freqs: [modulator_x, modulator_y, modulator_z],
            indices: [idx_x, idx_y, idx_z],
        }
    }

    pub fn carrier_x(mut self, freq: f64) -> Self {
        self.carrier_freqs[0] = freq;
        self
    }

    pub fn carrier_y(mut self, freq: f64) -> Self {
        self.carrier_freqs[1] = freq;
        self
    }

    pub fn carrier_z(mut self, freq: f64) -> Self {
        self.carrier_freqs[2] = freq;
        self
    }

    pub fn modulator_x(mut self, freq: f64) -> Self {
        self.modulator_freqs[0] = freq;
        self
    }

    pub fn modulator_y(mut self, freq: f64) -> Self {
        self.modulator_freqs[1] = freq;
        self
    }

    pub fn modulator_z(mut self, freq: f64) -> Self {
        self.modulator_freqs[2] = freq;
        self
    }

    pub fn idx_x(mut self, idx: f64) -> Self {
        self.indices[0] = idx;
        self
    }

    pub fn idx_y(mut self, idx: f64) -> Self {
        self.indices[1] = idx;
        self
    }

    pub fn idx_z(mut self, idx: f64) -> Self {
        self.indices[2] = idx;
        self
    }
}

impl Default for FMCross {
    fn default() -> Self {
        Self {
            carrier_freqs: [1.0, 1.0, 1.0],
            modulator_freqs: [0.1, 0.1, 0.1],
            indices: [5.0, 5.0, 5.0],
        }
    }
}

impl NoiseFn<f64, 2> for FMCross {
    fn get(&self, point: [f64; 2]) -> f64 {
        let fx = std::f64::consts::TAU * self.modulator_freqs[0] * point[1];
        let xm =
            std::f64::consts::TAU * self.carrier_freqs[0] * point[0] + self.indices[0] * fx.sin();

        let fy = std::f64::consts::TAU * self.modulator_freqs[1] * point[0];
        let ym =
            std::f64::consts::TAU * self.carrier_freqs[1] * point[1] + self.indices[1] * fy.sin();

        0.5 * (xm.sin() + ym.sin())
    }
}

impl NoiseFn<f64, 3> for FMCross {
    fn get(&self, point: [f64; 3]) -> f64 {
        let fx = std::f64::consts::TAU * self.modulator_freqs[0] * point[2];
        let xm =
            std::f64::consts::TAU * self.carrier_freqs[0] * point[0] + self.indices[0] * fx.sin();

        let fy = std::f64::consts::TAU * self.modulator_freqs[1] * point[0];
        let ym =
            std::f64::consts::TAU * self.carrier_freqs[1] * point[1] + self.indices[1] * fy.sin();
        let fz = std::f64::consts::TAU * self.modulator_freqs[1] * point[1];
        let zm =
            std::f64::consts::TAU * self.carrier_freqs[2] * point[2] + self.indices[2] * fz.sin();

        (xm.sin() + ym.sin() + zm.sin()) / 3.0
    }
}
