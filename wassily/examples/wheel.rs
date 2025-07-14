use rayon::prelude::*;
use wassily::prelude::*;

const DEBUG: bool = true;
const SEED: u64 = 80;
const XSCALE: f64 = 15.0;
const YSCALE: f64 = 15.0;

const WIDTH: u32 = 8400;
const HEIGHT: u32 = 1440;

fn main() {
    let w = WIDTH * if DEBUG { 1 } else { 10 };
    let h = HEIGHT * if DEBUG { 1 } else { 10 };
    let mut canvas = Canvas::new(w, h);
    let mut rng = SmallRng::seed_from_u64(SEED);
    let c1 = rand_okhsl(&mut rng);
    let c2 = rand_okhsl(&mut rng);
    let c3 = rand_okhsl(&mut rng);
    let c4 = rand_okhsl(&mut rng);
    let c5 = rand_okhsl(&mut rng);

    println!("c1: {:?}", c1.as_u8s());
    println!("c2: {:?}", c2.as_u8s());
    println!("c3: {:?}", c3.as_u8s());
    println!("c4: {:?}", c4.as_u8s());
    println!("c5: {:?}", c5.as_u8s());

    let noise = Fbm::<Value>::default().set_octaves(1);
    // let noise = Fbm::<Perlin>::default().set_octaves(4);

    // *DARKGREEN, *NAVY, *DARKGREEN, *GOLD, *BLACK,
    let cs = ColorScale::new(c1, c2, c3, c4, c5);

    for i in 0..w {
        let values: Vec<_> = (0..h)
            .into_par_iter()
            .map(|j| {
                let x = i as f64 / w as f64;
                let y = j as f64 / h as f64;
                let t = noise.get([x * XSCALE, y * YSCALE]) as f32;
                cs.get_color_fractal(t, 15.0)
            })
            .collect();
        for (k, v) in values.into_iter().enumerate() {
            canvas.dot(i as f32, k as f32, v);
        }
    }

    if DEBUG {
        for i in 0..HEIGHT {
            let t = i as f32 / HEIGHT as f32;
            let c = cs.get_color_fractal(t, 15.0);
            Shape::new()
                .line(pt(0.0, i), pt(500, i))
                .stroke_color(c)
                .stroke_weight(2.0)
                .draw(&mut canvas);
        }
        canvas.save_png("scale.png");
    } else {
        let file_name = format!("scale_{}.png", SEED);
        canvas.save_png(file_name);
    }
}
