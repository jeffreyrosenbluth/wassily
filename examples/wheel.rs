use wassily::prelude::*;
const WIDTH: u32 = 6000;
const HEIGHT: u32 = 1440;
const DEBUG: bool = true;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut rng = SmallRng::seed_from_u64(5);
    let c1 = rand_okhsl(&mut rng);
    let c2 = rand_okhsl(&mut rng);
    let c3 = rand_okhsl(&mut rng);
    let c4 = rand_okhsl(&mut rng);
    let c5 = rand_okhsl(&mut rng);
    let ramp = NoiseColorRamp::new(
        c1,
        c2,
        c3,
        c4,
        Box::new(Fbm::<Perlin>::default().set_octaves(1)),
        8.0,
        0.333,
        2.0,
    );

    let color_scale = NoiseColorScale::new(
        c1,
        c2,
        c3,
        c4,
        c1,
        Box::new(Fbm::<Perlin>::default().set_octaves(1)),
        8.0,
    );
    let color_scale2 = NoiseColorScale::new(
        c1,
        c2,
        c3,
        c4,
        c5,
        Box::new(Fbm::<Perlin>::default().set_octaves(3)),
        8.0,
    );

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let x = i as f32 / WIDTH as f32;
            let y = j as f32 / HEIGHT as f32;
            let c = color_scale2.get_color(x, y);
            canvas.dot(i as f32, j as f32, c);
        }
    }

    if DEBUG {
        for i in 0..HEIGHT {
            let t = i as f32 / HEIGHT as f32;
            let c = color_scale2.color_scale.get_color(t);
            Shape::new()
                .line(pt(0.0, i), pt(500, i))
                .stroke_color(c)
                .stroke_weight(2.0)
                .draw(&mut canvas);
            let c = color_scale.color_scale.get_color(t);
            Shape::new()
                .line(
                    pt(WIDTH as f32 / 2.0 - 250.0, i),
                    pt(WIDTH as f32 / 2.0 + 250.0, i),
                )
                .stroke_color(c)
                .stroke_weight(2.0)
                .draw(&mut canvas);
            let c = ramp.color_quad.get_color(t);
            Shape::new()
                .line(pt(WIDTH - 500, i), pt(WIDTH, i))
                .stroke_color(c)
                .stroke_weight(2.0)
                .draw(&mut canvas);
        }
    }
    canvas.save_png("./scale.png");
}
