use noise::NoiseFn;

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub freq_x: f64,
    pub phase_x: f64,
    pub freq_y: f64,
    pub phase_y: f64,
    pub amp: f64,
}

impl Params {
    pub fn new(freq_x: f64, phase_x: f64, freq_y: f64, phase_y: f64, amp: f64) -> Self {
        Self {
            freq_x,
            phase_x,
            freq_y,
            phase_y,
            amp,
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            freq_x: 1.0,
            phase_x: 0.0,
            freq_y: 1.0,
            phase_y: 0.0,
            amp: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SinPM {
    pub xy: Params,
    pub yx: Params,
}

impl SinPM {
    pub fn new(xy: Params, yx: Params) -> Self {
        Self { xy, yx }
    }

    pub fn xy_freq_x(mut self, freq: f64) -> Self {
        self.xy.freq_x = freq;
        self
    }

    pub fn xy_freq_y(mut self, freq: f64) -> Self {
        self.xy.freq_y = freq;
        self
    }

    pub fn xy_phase_x(mut self, phase: f64) -> Self {
        self.xy.phase_x = phase;
        self
    }
    
    pub fn xy_phase_y(mut self, phase: f64) -> Self {
        self.xy.phase_y = phase;
        self
    }

    pub fn xy_amp(mut self, amp: f64) -> Self {
        self.xy.amp = amp;
        self
    }

    pub fn yx_freq_x(mut self, freq: f64) -> Self {
        self.yx.freq_x = freq;
        self
    }

    pub fn yx_phase_x(mut self, phase: f64) -> Self {
        self.yx.phase_x = phase;
        self
    }

    pub fn yx_freq_y(mut self, freq: f64) -> Self {
        self.yx.freq_y = freq;
        self
    }

    pub fn yx_phase_y(mut self, phase: f64) -> Self {
        self.yx.phase_y = phase;
        self
    }
    
    pub fn yx_amp(mut self, amp: f64) -> Self {
        self.yx.amp = amp;
        self
    }
    
}

impl Default for SinPM {
    fn default() -> Self {
        Self {
            xy: Default::default(),
            yx: Default::default(),
        }
    }
}

impl NoiseFn<f64, 2> for SinPM {
    fn get(&self, point: [f64; 2]) -> f64 {
        let s = std::f64::consts::TAU * self.xy.freq_y * point[1] + self.xy.phase_y;
        let t = std::f64::consts::TAU * self.xy.freq_x * point[0]
            + self.xy.phase_x
            + self.xy.amp * s.sin();
        let u = std::f64::consts::TAU * self.yx.freq_x * point[0] + self.yx.phase_x;
        let v = std::f64::consts::TAU * self.yx.freq_y * point[1]
            + self.yx.phase_y
            + self.yx.amp * u.sin();
        0.5 * (t.sin() + v.sin())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SinFM {
    pub xy: Params,
    pub yx: Params,
}

impl SinFM {
    pub fn new(xy: Params, yx: Params) -> Self {
        Self { xy, yx }
    }

    pub fn xy_freq_x(mut self, freq: f64) -> Self {
        self.xy.freq_x = freq;
        self
    }

    pub fn xy_freq_y(mut self, freq: f64) -> Self {
        self.xy.freq_y = freq;
        self
    }

    pub fn xy_phase_x(mut self, phase: f64) -> Self {
        self.xy.phase_x = phase;
        self
    }
    
    pub fn xy_phase_y(mut self, phase: f64) -> Self {
        self.xy.phase_y = phase;
        self
    }

    pub fn xy_amp(mut self, amp: f64) -> Self {
        self.xy.amp = amp;
        self
    }

    pub fn yx_freq_x(mut self, freq: f64) -> Self {
        self.yx.freq_x = freq;
        self
    }

    pub fn yx_phase_x(mut self, phase: f64) -> Self {
        self.yx.phase_x = phase;
        self
    }

    pub fn yx_freq_y(mut self, freq: f64) -> Self {
        self.yx.freq_y = freq;
        self
    }

    pub fn yx_phase_y(mut self, phase: f64) -> Self {
        self.yx.phase_y = phase;
        self
    }
    
    pub fn yx_amp(mut self, amp: f64) -> Self {
        self.yx.amp = amp;
        self
    }
}

impl Default for SinFM {
    fn default() -> Self {
        Self {
            xy: Default::default(),
            yx: Default::default(),
        }
    }
}

impl NoiseFn<f64, 2> for SinFM {
    fn get(&self, point: [f64; 2]) -> f64 {
        let s = std::f64::consts::TAU * self.xy.freq_y * point[1] + self.xy.phase_y;
        let t = std::f64::consts::TAU * self.xy.freq_x * point[0] * s.sin() * self.xy.amp
            + self.xy.phase_x;
        let u = std::f64::consts::TAU * self.yx.freq_x * point[0] + self.yx.phase_x;
        let v = std::f64::consts::TAU * self.yx.freq_y * point[1] * u.sin() * self.yx.amp
            + self.yx.phase_y;
        0.5 * (t.sin() + v.sin())
    }
}
