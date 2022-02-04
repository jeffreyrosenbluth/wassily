use wassily::prelude::*;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 0.80 * WIDTH;

fn main() {
    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);
    let mut palette = Palette::steal("fruit.png", 16);
    palette.set_seed(71731);
    canvas.fill(Color::from_rgba8(122, 122, 122, 255));

    let lg = LinearGradient::new(
        pt(600.0, 480.0),
        pt(600.0, 480.0),
        vec![
            GradientStop::new(0.0, *ORANGE),
            GradientStop::new(1.0, *BLUE),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();
    let linear = paint_shader(lg);

    let rg = RadialGradient::new(
        pt(600.0, 480.0),
        pt(600.0, 480.0),
        600.0,
        vec![
            GradientStop::new(0.0, *ORANGE),
            GradientStop::new(1.0, *BLUE),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();
    let radial = paint_shader(rg);

    ShapeBuilder::new()
        .rect_xywh(pt(0, 0), pt(1200, 960))
        .fill_paint(&linear)
        .no_stroke()
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(pt(500, 380), pt(200, 200))
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(pt(100, 100), pt(200, 200))
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(pt(900, 100), pt(200, 200))
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);
    ShapeBuilder::new()
        .rect_xywh(pt(300, 660), pt(200, 200))
        .fill_paint(&radial)
        .build()
        .draw(&mut canvas);

    canvas.save_png("gradient.png");
}
