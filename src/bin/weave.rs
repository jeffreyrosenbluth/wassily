use noise::Perlin;
use rand_distr::{Distribution, Normal};
use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 1200;
const SKIP: usize = 30;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    canvas.fill(WHITE);
    let dots = stipple(WIDTH, HEIGHT, 700_000);
    for dot in dots {
        canvas.pixel(dot.x, dot.y, GRAY);
    }

    let ns: Noise<_, 2> = Noise::new(WIDTH, HEIGHT, Perlin::default())
        .scales(22.0)
        .factor(300.0);
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 300.0).unwrap();

    let mut palette = Palette::with_img("blues.png", 1300);
    palette.rotate_hue(180.0);

    for i in (0..WIDTH).step_by(SKIP) {
        let mut ps = vec![point2(i, 0.0)];
        ps.push(point2(
            i as f32 + normal.sample(&mut rng),
            400.0 + ns.get(i as f32, 400.0).abs(),
        ));
        ps.push(point2(
            i as f32 + normal.sample(&mut rng),
            800.0 - ns.get(i as f32, 800.0).abs(),
        ));
        ps.push(point2(i, HEIGHT as f32));

        ShapeBuilder::new()
            .points(&ps)
            .cubic()
            .no_fill()
            .stroke_weight(25.0)
            .line_cap(LineCap::Round)
            .stroke_color(palette.rand_color().set_opacity(0.7))
            .build()
            .draw(&mut canvas);
        ShapeBuilder::new()
            .points(&ps)
            .cubic()
            .no_fill()
            .stroke_weight(15.0)
            .line_cap(LineCap::Round)
            .stroke_color(WHITE)
            .build()
            .draw(&mut canvas);
    }
    canvas.save("weave.png");
}
