use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 8100;
const HEIGHT: u32 = 6075;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.fill(WHITE);
    let quad = Quad::new(
        point2(0, 0),
        point2(0, HEIGHT),
        point2(WIDTH, HEIGHT),
        point2(WIDTH, 0),
    );
    let mut qs = vec![quad];
    let mut prng = Rand::new(87654321);
    let n = 12;
    for _ in 0..n {
        qs = subdivide_vec(
            &qs,
            |q| q.best_dir(),
            || {
                let a = prng.rand_normal(0.5, 0.1);
                let a = a.clamp(0.0, 1.0);
                let b = prng.rand_normal(0.5, 0.1);
                let b = b.clamp(0.0, 1.0);
                (a, b)
            },
        );
    }
    // // let mut palette = Palette::with_img("church.png", Some(2usize.pow(n)));
    // // palette.jiggle(0, 0.05);
    // // palette.sort_by_hue();
    // // let mut iter = palette.into_iter().cycle();
    // qs.sort();
    // qs.shuffle(&mut prng.rng);
    let img = image::open("fireweed.png").unwrap();
    for q in qs {
        let (q1, _) = q.subdivide(|q| q.best_dir(), || (0.75, 0.75));
        let (_, q2) = q.subdivide(|q| q.best_dir(), || (0.25, 0.25));
        let c = get_color(&img, WIDTH as f32, HEIGHT as f32, q.bl).set_opacity(0.75);
        ShapeBuilder::new()
            .points(&q1.to_vec())
            .fill_color(c)
            // .stroke_color(c)
            .no_stroke()
            .build()
            .draw(&mut canvas);
        let d = get_color(&img, WIDTH as f32, HEIGHT as f32, q.tr).set_opacity(0.75);
        ShapeBuilder::new()
            .points(&q2.to_vec())
            .fill_color(d)
            // .stroke_color(d)
            .no_stroke()
            .build()
            .draw(&mut canvas);
    }
    canvas.save("sub.png");
}
