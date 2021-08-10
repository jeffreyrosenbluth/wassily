use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 8100;
const HEIGHT: u32 = 6075;

#[derive(Debug, Clone, Copy)]
struct Data<P: AsRef<std::path::Path>> {
    width: u32,
    height: u32,
    n: i32,
    seed: u64,
    mean: f32,
    std: f32,
    image: P,
    output: P,
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.fill(WHITE);
    let tri1 = Tri::new(
        point2(0, 0),
        point2(0, HEIGHT),
        point2(WIDTH, HEIGHT),
    );
    let tri2 = Tri::new(
        point2(0, 0),
        point2(WIDTH, HEIGHT),
        point2(WIDTH, 0),
    );
    let mut qs = vec![tri1, tri2];
    let mut prng = Rand::new(764321);
    let n = 11;
    for _ in 0..n {
        qs = tri_divide_vec(
            &qs,
            |q| q.best_dir(),
            || {
                let a = prng.rand_normal(0.5, 0.05);
                let a = a.clamp(0.0, 1.0);
                a
            },
        );
    }
    let img = image::open("fireweed.png").unwrap();
    for q in qs {
        let c = get_color(&img, WIDTH as f32, HEIGHT as f32, q.bl);
        ShapeBuilder::new()
            .points(&q.to_vec())
            .fill_color(c)
            .no_stroke()
            .build()
            .draw(&mut canvas);
    }
    canvas.save("tri.png");
}