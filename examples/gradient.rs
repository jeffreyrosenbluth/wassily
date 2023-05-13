use wassily::prelude::*;

const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 1000.0;
const EDGE: f32 = 250.0;
const PAD: f32 = 100.0;

fn main() {
    let mut canvas = Canvas::with_scale(WIDTH as u32, HEIGHT as u32, 1.25);
    let mut palette = Palette::steal("fruit.png", 16);
    palette.set_seed(71731);
    canvas.fill(Color::from_rgba8(122, 122, 122, 255));

    let lg = LinearGradient::new(
        pt(0.0, 0.0),
        pt(0.0, HEIGHT),
        vec![
            GradientStop::new(0.0, *WHITE),
            GradientStop::new(1.0, *BLACK),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    )
    .unwrap();
    let linear = paint_shader(lg);

    let rg = RadialGradient::new(
        pt(WIDTH / 2.0, HEIGHT / 2.0),
        pt(WIDTH / 2.0, HEIGHT / 2.0),
        700.0,
        vec![
            GradientStop::new(0.0, *MAROON),
            GradientStop::new(0.4, *ORANGE),
            GradientStop::new(1.0, *BLUE),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    )
    .unwrap();
    let radial = paint_shader(rg);

    Shape::new()
        .rect_xywh(pt(0, 0), pt(WIDTH, HEIGHT))
        .fill_paint(&linear)
        .no_stroke()
        .draw(&mut canvas);

    let tr1 = Transform::from_rotate_at(30.0, canvas.w_f32() / 2.0, canvas.h_f32() / 2.0);
    let tr2 = Transform::from_scale(1.5, 0.5);
    let tr3 = Transform::from_translate(-310.0, 50.0);
    let transform = tr1.pre_concat(tr2);
    let transform = transform.post_concat(tr3);
    Shape::new()
        .rect_xywh(
            pt(WIDTH / 2.0 - EDGE / 2.0, HEIGHT / 2.0 - EDGE / 2.0),
            pt(EDGE, EDGE),
        )
        .fill_paint(&radial)
        .stroke_weight(4.0)
        .transform(&transform)
        .draw(&mut canvas);

    Shape::new()
        .rect_xywh(pt(PAD, PAD), pt(EDGE, EDGE))
        .fill_paint(&radial)
        .draw(&mut canvas);
    Shape::new()
        .rect_xywh(pt(WIDTH - EDGE - PAD, PAD), pt(EDGE, EDGE))
        .fill_paint(&radial)
        .draw(&mut canvas);
    Shape::new()
        .rect_xywh(pt(PAD, WIDTH - EDGE - PAD), pt(EDGE, EDGE))
        .fill_paint(&radial)
        .draw(&mut canvas);
    Shape::new()
        .rect_xywh(pt(WIDTH - EDGE - PAD, HEIGHT - EDGE - PAD), pt(EDGE, EDGE))
        .fill_paint(&radial)
        .draw(&mut canvas);
    canvas.save_png("gradient2.png");
}
