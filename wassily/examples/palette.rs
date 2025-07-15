use wassily::prelude::*;

const SEED: u64 = 92;
const WIDTH: u32 = 1225;
const HEIGHT: u32 = 600;
const PHASE: f32 = 15.0;
const STYLE: Style = Style::Fractal;
const FREQ: f32 = 1.0;

enum Style {
    Fractal,
    Clipped,
    Standard,
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut rng = SmallRng::seed_from_u64(SEED);
    let c1 = rand_okhsl(&mut rng);
    let c2 = rand_okhsl(&mut rng);
    let c3 = rand_okhsl(&mut rng);
    let c4 = rand_okhsl(&mut rng);
    let c5 = rand_okhsl(&mut rng);
    let colors = [c1, c2, c3, c4, c5];

    println!("c1: {:?}", c1.as_u8s());
    println!("c2: {:?}", c2.as_u8s());
    println!("c3: {:?}", c3.as_u8s());
    println!("c4: {:?}", c4.as_u8s());
    println!("c5: {:?}", c5.as_u8s());

    let cs = ColorScale::new(c1, c2, c3, c4, c5);
    let delta = WIDTH / 7;

    for (i, c) in colors.into_iter().enumerate() {
        Shape::new()
            .rect_xywh(
                pt(i * delta as usize, 0),
                pt((i + 1) * delta as usize, HEIGHT),
            )
            .stroke_color(c)
            .fill_color(c)
            .draw(&mut canvas);
    }
    for y in 0..HEIGHT {
        let t = y as f32 / HEIGHT as f32;
        let c = match STYLE {
            Style::Fractal => cs.get_color_fractal(FREQ * t, PHASE),
            Style::Clipped => cs.get_color_clip(FREQ * t, 0.1),
            Style::Standard => cs.get_color(FREQ * t),
        };
        Shape::new()
            .line(pt(5.0 * delta as f32, y), pt(WIDTH, y))
            .stroke_color(c)
            .draw(&mut canvas);
    }
    canvas.save_png("./outputs/palette.png");
}
