use wassily::prelude::*;
const WIDTH: u32 = 6000;
const HEIGHT: u32 = 1440;
const DEBUG: bool = true;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut rng = SmallRng::seed_from_u64(16);
    let ramp = NoiseColorRamp::new(
        rand_okhsl(&mut rng),
        rand_okhsl(&mut rng),
        rand_okhsl(&mut rng),
        rand_okhsl(&mut rng),
        Box::new(Fbm::<Perlin>::default().set_octaves(4)),
        10.0,
        0.333,
        2.0,
    );

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let x = i as f32 / WIDTH as f32;
            let y = j as f32 / HEIGHT as f32;
            let c = ramp.get_color(x, y);
            canvas.dot(i as f32, j as f32, c.rotate_hue(140.0));
        }
    }

    if DEBUG {
        for i in 0..HEIGHT {
            let t = i as f32 / HEIGHT as f32;
            let c = ramp.color_quad.get_color(t);
            Shape::new()
                .line(pt(0.0, i), pt(500, i))
                .stroke_color(c)
                .stroke_weight(2.0)
                .draw(&mut canvas);
            Shape::new()
                .line(pt(WIDTH - 500, i), pt(WIDTH, i))
                .stroke_color(c.rotate_hue(140.0))
                .stroke_weight(2.0)
                .draw(&mut canvas);
        }
    }
    canvas.save_png("./wheel.png");
}
