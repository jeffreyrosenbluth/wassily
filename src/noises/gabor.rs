use noise::NoiseFn;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

// Gabor Noise
//
// Lagae, Ares & Lefebvre, Sylvain & Drettakis, George & Dutré, Philip. (2009).
// Procedural Noise using Sparse Gabor Convolution.
// ACM Transactions on Graphics. 28. 10.1145/1576246.1531360.
//
// Lagae A, Lefebvre S, Dutré P. Improving Gabor noise. IEEE Trans Vis Comput Graph.
// 2011 Aug;17(8):1096-107. doi: 10.1109/TVCG.2010.238. PMID: 21041873.
//
// Vincent Tavernier, Fabrice Neyret, Romain Vergne, Joëlle Thollot. Making Gabor Noise Fast and
// Normalized. Eurographics 2019 - 40th Annual Conference of the European Association for Computer
// Graphics, May 2019, Gênes, Italy. pp.37-40, ff10.2312/egs.20191009ff. ffhal-02104389f
const PI64: f64 = std::f64::consts::PI;
const TAU64: f64 = std::f64::consts::TAU;

fn gabor(k: f64, r: f64, f0: f64, omega: f64, x: f64, y: f64) -> f64 {
    let guass = k * (-PI64 / (r * r) * ((x * x) + (y * y))).exp();
    let sin = (TAU64 * f0 * (x * omega.cos() + y * omega.sin())).sin();
    guass * sin
}

// z-curve ordering.
fn morton(x: u32, y: u32) -> u32 {
    let mut z = 0;
    for i in 0..32 {
        z |= ((x & (1 << i)) << i) | ((y & (1 << i)) << (i + 1));
    }
    z
}

#[derive(Debug, Clone, Copy)]
pub struct Gabor {
    k: f64,
    r: f64,
    f0: f64,
    omega0: Option<f64>,
    kernel_radius: f64,
    impulses_per_cell: u32,
    scale: f64,
}

impl Default for Gabor {
    fn default() -> Self {
        Self::new(1.0, 64.0, 0.01, None, 64.0)
    }
}

impl Gabor {
    pub fn new(k: f64, r: f64, f0: f64, omega0: Option<f64>, impulses_per_kernel: f64) -> Self {
        let kernel_radius = (-(0.05f64).ln() / PI64).sqrt() * r;
        let impulse_density = impulses_per_kernel / (PI64 * kernel_radius * kernel_radius);
        let integral_gabor_filter_squared =
            0.25 * k * k * r * r * (1.0 + (-TAU64 * f0 * f0 * r * r).exp());
        let impulses_per_cell = (impulses_per_kernel / PI64) as u32;
        let scale = 3.0 * (impulse_density * integral_gabor_filter_squared).sqrt();
        Self {
            k,
            r,
            f0,
            omega0,
            kernel_radius,
            impulses_per_cell,
            scale,
        }
    }

    pub fn k(self, k: f64) -> Self {
        Self { k, ..self }
    }

    pub fn r(self, r: f64) -> Self {
        let kernel_radius = (-(0.05f64).ln() / PI64).sqrt() * r;
        Self {
            r,
            kernel_radius,
            ..self
        }
    }

    pub fn a(self, a: f64) -> Self {
        let r = 1.0 / a;
        let kernel_radius = (-(0.05f64).ln() / PI64).sqrt() * r;
        Self {
            r,
            kernel_radius,
            ..self
        }
    }

    pub fn f0(self, f0: f64) -> Self {
        Self { f0, ..self }
    }

    pub fn omega0(self, omega0: Option<f64>) -> Self {
        Self { omega0, ..self }
    }

    pub fn get(&self, x: f64, y: f64) -> f64 {
        let x = x / self.kernel_radius;
        let y = y / self.kernel_radius;
        let int_x = x.floor();
        let int_y = y.floor();
        let frac_x = x - int_x;
        let frac_y = y - int_y;
        let int_x = int_x as i32;
        let int_y = int_y as i32;
        let mut ns = 0.0;
        for di in -1..=1 {
            for dj in -1..=1 {
                ns += self.cell(
                    int_x + di,
                    int_y + dj,
                    frac_x - di as f64,
                    frac_y - dj as f64,
                );
            }
        }
        ns / self.scale
    }

    fn cell(&self, i: i32, j: i32, x: f64, y: f64) -> f64 {
        let mut rnd = SmallRng::seed_from_u64(morton(i as u32, j as u32) as u64);
        let mut noise = 0.0;
        for _ in 0..self.impulses_per_cell {
            let xi:f64 = rnd.gen();
            let yi:f64 = rnd.gen();
            let wi: f64 = rnd.gen::<f64>() * 2.0 - 1.0;
            let omega0i: f64;
            if let Some(o) = self.omega0 {
                omega0i = o;
            } else {
                omega0i = rnd.gen_range(0.0..TAU64);
            }
            let xix = x - xi;
            let yiy = y - yi;
            if xix * xix + yiy * yiy < 1.0 {
                noise += wi
                    * gabor(
                        self.k,
                        self.r,
                        self.f0,
                        omega0i,
                        xix * self.kernel_radius,
                        yiy * self.kernel_radius,
                    );
            }
        }
        noise
    }
}

impl NoiseFn<f64, 2> for Gabor {
    fn get(&self, point: [f64; 2]) -> f64 {
        self.get(point[0], point[1])
    }
}
