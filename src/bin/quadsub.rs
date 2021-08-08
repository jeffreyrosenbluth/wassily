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
    let n = 14;
    for _ in 0..n {
        qs = quad_divide_vec(
            &qs,
            |q| q.best_dir(),
            || {
                let a = prng.rand_normal(0.5, 0.15);
                let a = a.clamp(0.0, 1.0);
                let b = prng.rand_normal(0.5, 0.15);
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
        // let (q1, _) = q.subdivide(|q| q.best_dir(), || (0.75, 0.75));
        // let (_, q2) = q.subdivide(|q| q.best_dir(), || (0.25, 0.25));
        let s = 4.0;
        let c = get_color(&img, WIDTH as f32, HEIGHT as f32, q.bl).set_opacity(0.4);
        let transform = Transform::identity()
            .post_scale(s, s)
            .post_translate(vec2(-&q.bl.x * 3.0, -&q.bl.y * 3.0));
        ShapeBuilder::new()
            .points(&q.to_vec())
            .fill_color(c)
            .no_stroke()
            .transform(&transform)
            .build()
            .draw(&mut canvas);
        // let d = get_color(&img, WIDTH as f32, HEIGHT as f32, q.tr).set_opacity(0.75);
        // ShapeBuilder::new()
        //     .points(&q2.to_vec())
        //     .fill_color(d)
        //     .no_stroke()
        //     .build()
        //     .draw(&mut canvas);
    }
    canvas.save("sub.png");
}
