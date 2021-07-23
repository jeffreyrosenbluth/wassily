use noise::Perlin;
use rand_distr::{Distribution, Normal};
use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 1200;
const SKIP: usize = 60;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.fill(CORNSILK);
    let ns: Noise<_, 2> = Noise::new(WIDTH as f32, HEIGHT as f32, Perlin::default())
        .set_scales(12.5)
        .set_noise_factor(300.0);
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 300.0).unwrap();
    let mut palette = Palette::with_img(file_path("blues.png"), 1300);
    for i in (0..WIDTH).step_by(SKIP) {
        let mut ps = vec![point2(i as f32, 0.0)];
        ps.push(point2(
            i as f32 + normal.sample(&mut rng),
            400.0 + ns.noise(i as f32, 400.0),
        ));
        ps.push(point2(
            i as f32 + normal.sample(&mut rng),
            800.0 + ns.noise(i as f32, 800.0),
        ));
        // ps.push(point2(i as f32 + ns.noise(i as f32, 400.0), 400.0));
        // ps.push(point2(i as f32 + ns.noise(i as f32, 800.0), 800.0));
        ps.push(point2(i as f32, HEIGHT as f32));
        ShapeBuilder::new()
            .points(&ps)
            .cubic()
            .no_fill()
            .stroke_weight(35.0)
            .line_cap(LineCap::Square)
            .stroke_color(palette.rand_color().set_opacity(0.6))
            .build()
            .draw(&mut canvas);
    }
    canvas.save("weave.png");
}
