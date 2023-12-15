use wassily::prelude::*;

const SEED: u64 = 56;

const WIDTH: u32 = 2000;
const HEIGHT: u32 = 1000;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut rng = SmallRng::seed_from_u64(SEED);
    let c1 = rand_okhsl(&mut rng);
    let c2 = rand_okhsl(&mut rng);
    let c3 = rand_okhsl(&mut rng);
    let c4 = rand_okhsl(&mut rng);
    let c5 = rand_okhsl(&mut rng);
    let x_delta = WIDTH / 5;
    let y_delta = HEIGHT / 5;

    let cs = ColorScale::new(c1, c2, c3, c4, c5);

    // Panel 1: The starting colors
    Shape::new()
        .rect_xywh(pt(0.0, 0.0), pt(x_delta, y_delta))
        .fill_color(c1)
        .stroke_color(c1)
        .draw(&mut canvas);
    Shape::new()
        .rect_xywh(pt(0.0, y_delta), pt(x_delta, y_delta))
        .fill_color(c2)
        .stroke_color(c2)
        .draw(&mut canvas);
    Shape::new()
        .rect_xywh(pt(0.0, 2 * y_delta), pt(x_delta, y_delta))
        .fill_color(c3)
        .stroke_color(c3)
        .draw(&mut canvas);
    Shape::new()
        .rect_xywh(pt(0.0, 3 * y_delta), pt(x_delta, y_delta))
        .fill_color(c4)
        .stroke_color(c4)
        .draw(&mut canvas);
    Shape::new()
        .rect_xywh(pt(0.0, 4 * y_delta), pt(x_delta, y_delta))
        .fill_color(c5)
        .stroke_color(c5)
        .draw(&mut canvas);

    // Panel 2: The generated fractal palette
    for i in 0..HEIGHT {
        let t = i as f32 / HEIGHT as f32;
        let c = cs.get_color_fractal(t, 15.0);
        Shape::new()
            .line(pt(x_delta, i), pt(3 * x_delta, i))
            .stroke_color(c)
            .stroke_weight(2.0)
            .draw(&mut canvas);
    }

    // Panel 3: The generated clipped palette
    for i in 0..HEIGHT {
        let t = i as f32 / HEIGHT as f32;
        let c = cs.get_color_clip(t, 0.1);
        Shape::new()
            .line(pt(2 * x_delta, i), pt(3 * x_delta, i))
            .stroke_color(c)
            .stroke_weight(2.0)
            .draw(&mut canvas);
    }

    // Panel 5: The staggered gradient
    let gradient = LinearGradient::new(
        pt(0.0, 0.0),
        pt(0.0, HEIGHT),
        vec![
            GradientStop::new(1.0 / 6.0, c1),
            GradientStop::new(1.0 / 3.0, c2),
            GradientStop::new(0.5, c3),
            GradientStop::new(2.0 / 3.0, c4),
            GradientStop::new(5.0 / 6.0, c5),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    )
    .unwrap();
    let paint = paint_shader(gradient);
    Shape::new()
        .rect_xywh(pt(4 * x_delta, 0), pt(x_delta, HEIGHT))
        .fill_paint(&paint)
        .stroke_paint(&paint)
        .draw(&mut canvas);

    // Panel 4: The linear gradient
    let gradient = LinearGradient::new(
        pt(0.0, 0.0),
        pt(0.0, HEIGHT),
        vec![
            GradientStop::new(0.0, c1),
            GradientStop::new(0.25, c2),
            GradientStop::new(0.5, c3),
            GradientStop::new(0.75, c4),
            GradientStop::new(1.0, c5),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    )
    .unwrap();
    let paint = paint_shader(gradient);
    Shape::new()
        .rect_xywh(pt(3 * x_delta, 0), pt(x_delta, HEIGHT))
        .fill_paint(&paint)
        .stroke_paint(&paint)
        .draw(&mut canvas);

    let file_name = format!("color_scale_{}.png", SEED);
    canvas.save_png("color_scale.png");
}
