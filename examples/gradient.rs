use wassily::prelude::*;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 1200.0;
const EDGE: f32 = 300.0;
const PAD: f32 = 100.0;

fn main() {
    let mut canvas = Canvas::with_scale(WIDTH as u32, HEIGHT as u32, 0.5);
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

    ShapeBuilder::new()
        .rect_xywh(pt(0, 0), pt(WIDTH, HEIGHT))
        .fill_paint(&linear)
        .no_stroke()
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(
            pt(WIDTH / 2.0 - EDGE / 2.0, HEIGHT / 2.0 - EDGE / 2.0),
            pt(EDGE, EDGE),
        )
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(pt(PAD, PAD), pt(EDGE, EDGE))
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(pt(WIDTH - EDGE - PAD, PAD), pt(EDGE, EDGE))
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(pt(PAD, WIDTH - EDGE - PAD), pt(EDGE, EDGE))
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(pt(WIDTH - EDGE - PAD, HEIGHT - EDGE - PAD), pt(EDGE, EDGE))
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);
    canvas.save_png("gradient.png");
}
