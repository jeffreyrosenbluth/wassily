use rand::prelude::SliceRandom;
use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

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
    let n = 18;
    for _ in 0..n {
        qs = subdivide_vec(
            &qs,
            |q| q.best_dir(),
            || {
                let a = prng.rand_normal(0.5, 0.1);
                let a = a.clamp(0.0, 1.0);
                let b = prng.rand_normal(0.5, 0.1);
                let b = b.clamp(0.0, 1.0);
                (0.5, 0.5)
            },
        );
    }
    let mut palette = Palette::with_img("tile.png", 2usize.pow(n) + 300);
    palette.sort_by_hue();
    palette.set_index(100);
    qs.sort();
    // qs.shuffle(&mut prng.rng);
    for q in qs {
        let c = palette.next();
        ShapeBuilder::new()
            .points(&q.to_vec())
            // .cubic()
            .fill_color(c)
            .stroke_color(c)
            .build()
            .draw(&mut canvas);
    }
    canvas.save("sub.png");
}
