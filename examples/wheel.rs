use std::sync::Arc;
use wassily::prelude::*;
const WIDTH: u32 = 1080;
const HEIGHT: u32 = 1080;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut rng = SmallRng::seed_from_u64(23);
    let mut color_wheel = ColorWheel::new(&mut rng, 7, 3);
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let x = i as f32 / WIDTH as f32;
            let y = j as f32 / HEIGHT as f32;
            let c = color_wheel.get_color(&mut rng, x, y);
            canvas.dot(i as f32, j as f32, c);
        }
    }
    canvas.save_png("./wheel4.png");
}

#[derive(Clone)]
pub struct ColorWheel {
    pub octaves: usize,
    pub base_color: [f32; 3],
    pub phases: Vec<[f32; 3]>,
    pub frequencies: [f32; 7],
    pub noise: Arc<Fbm<Perlin>>,
    scale: f32,
}

impl ColorWheel {
    pub fn new<R: Rng>(rng: &mut R, octaves: usize, noise_octaves: usize) -> Self {
        assert!(octaves <= 7, "Maximum ocatves is 7");
        let mut phases = ColorWheel::PHASES;
        phases.shuffle(rng);
        let phases: Vec<[f32; 3]> = phases.into_iter().map(|v| Self::shuffle3(rng, v)).collect();
        let noise = Fbm::<Perlin>::default();
        let seed: u32 = rng.gen();
        let noise = noise.set_seed(seed).set_octaves(noise_octaves);
        let x: f32 = 0.5 + rng.gen_range(-0.1..0.1);
        let y: f32 = 0.3 + rng.gen_range(-0.1..0.1);
        let z: f32 = 0.4 + rng.gen_range(-0.1..0.1);
        let base_color = [x, y, z];
        let mut scale: f32 = Self::AMPLITUDES[0..octaves].iter().sum();
        scale += 0.4;
        Self {
            octaves,
            phases,
            frequencies: Self::FREQUENCIES,
            noise: Arc::new(noise),
            base_color,
            scale,
        }
    }

    const AMPLITUDES: [f32; 7] = [0.12, 0.11, 0.1, 0.09, 0.08, 0.07, 0.06];

    const PHASES: [[f32; 3]; 9] = [
        [0.0, 0.8, 1.0],
        [0.3, 0.4, 0.1],
        [0.1, 0.7, 1.1],
        [0.2, 0.8, 1.4],
        [0.2, 0.6, 0.7],
        [0.1, 0.6, 0.7],
        [0.0, 0.5, 0.8],
        [0.1, 0.4, 0.7],
        [1.1, 1.4, 2.7],
    ];

    const FREQUENCIES: [f32; 7] = [1.0, 3.1, 5.1, 9.1, 17.1, 31.1, 65.1];

    fn shuffle3<R: Rng>(rng: &mut R, p: [f32; 3]) -> [f32; 3] {
        let i: usize = rng.gen_range(0..=2);
        let j = (i + rng.gen_range(1..=2)) % 3;
        let x = p[i];
        let y = p[j];
        let z = p[3 - i - j];
        [x, y, z]
    }

    fn term(&self, t: f32, amplitude: f32, freq: f32, phases: [f32; 3]) -> [f32; 3] {
        phases.map(|p| amplitude * (TAU * t * freq + p).cos())
    }

    pub fn get_color<R: Rng>(&mut self, rng: &mut R, x: f32, y: f32) -> Color {
        let mut rgb = self.base_color;
        for i in 0..self.octaves {
            let t = 0.5
                + 0.5 * self.noise.get([3.0 * x as f64, 3.0 * y as f64]) as f32
                + 0.02 * (rng.gen_range(0.0..1.0) + rng.gen_range(0.0..1.0));
            let a = self.term(t, Self::AMPLITUDES[i], Self::FREQUENCIES[i], self.phases[i]);
            for j in 0..3 {
                rgb[j] += a[j];
            }
        }
        Color::from_rgba(
            (rgb[0] / self.scale).clamp(0.0, 1.0),
            (rgb[1] / self.scale).clamp(0.2, 0.8),
            (rgb[2] / self.scale).clamp(0.0, 1.0),
            1.0,
        )
        .unwrap()
    }
}
