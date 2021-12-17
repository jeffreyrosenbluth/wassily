use noise::NoiseFn;
use num_traits::{AsPrimitive, ToPrimitive};

pub mod gabor;
pub mod trig;
pub mod white;

#[derive(Debug, Clone, Copy)]
pub struct NoiseOpts {
    pub width: f32,
    pub height: f32,
    x_scale: f32,
    y_scale: f32,
    z_scale: f32,
    factor: f32,
}

impl NoiseOpts {
    pub fn new(
        width: f32,
        height: f32,
        x_scale: f32,
        y_scale: f32,
        z_scale: f32,
        factor: f32,
    ) -> Self {
        Self {
            width,
            height,
            x_scale,
            y_scale,
            z_scale,
            factor,
        }
    }

    pub fn with_wh<T: AsPrimitive<f32>>(width: T, height: T) -> Self {
        Self {
            width: width.as_(),
            height: height.as_(),
            ..Self::default()
        }
    }

    pub fn width(self, width: f32) -> Self {
        Self { width, ..self }
    }

    pub fn height(self, height: f32) -> Self {
        Self { height, ..self }
    }

    /// Multiplier for the noise value.
    pub fn factor(self, factor: f32) -> Self {
        Self { factor, ..self }
    }

    /// Used to scale the x-coordinate: `x = x_scale * x / width`.
    pub fn x_scale(self, x_scale: f32) -> Self {
        Self { x_scale, ..self }
    }

    /// Used to  the y-coordinate: `y = y_scale * y / height`.
    pub fn y_scale(self, y_scale: f32) -> Self {
        Self { y_scale, ..self }
    }

    /// Used ot scale the z-coordingate: z = z_scale * z.
    pub fn z_scale(self, z_scale: f32) -> Self {
        Self { z_scale, ..self }
    }

    /// Set both the x and y scales to the same value.
    pub fn xy_scales(self, scale: f32) -> Self {
        Self {
            x_scale: scale,
            y_scale: scale,
            ..self
        }
    }

    /// Set all scales to the same value.
    pub fn scales(self, scale: f32) -> Self {
        Self {
            x_scale: scale,
            y_scale: scale,
            z_scale: scale,
            ..self
        }
    }
}

impl Default for NoiseOpts {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            x_scale: 1.0,
            y_scale: 1.0,
            z_scale: 1.0,
            factor: 1.0,
        }
    }
}

pub fn get_f32<const N: usize>(nf: impl NoiseFn<f64, N>, point: [f32; N]) -> f32 {
    let coords = point.iter().map(|p| p.to_f64());
    let mut a: [f64; N] = [0.0; N];
    for (i, c) in coords.enumerate() {
        a[i] = c.unwrap();
    }
    nf.get(a) as f32
}

pub fn noise2d(nf: impl NoiseFn<f64, 2>, opts: &NoiseOpts, x: f32, y: f32) -> f32 {
    let cx = opts.width / 2.0;
    let cy = opts.height / 2.0;
    opts.factor
        * get_f32(
            nf,
            [
                (1.0 / cx * opts.x_scale * (x - cx)),
                (1.0 / cy * opts.y_scale * (y - cy)),
            ],
        )
}

pub fn noise3d(nf: impl NoiseFn<f64, 3>, opts: &NoiseOpts, x: f32, y: f32, z: f32) -> f32 {
    let cx = opts.width / 2.0;
    let cy = opts.height / 2.0;
    opts.factor
        * get_f32(
            nf,
            [
                (1.0 / cx * opts.x_scale * (x - cx)),
                (1.0 / cy * opts.y_scale * (y - cy)),
                opts.z_scale * z,
            ],
        )
}
