use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 0.80 * WIDTH;

fn main() {
    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);
    let path = file_path("hudson.png");
    let palette = Palette::steal(path, 16);
    canvas.fill(RGBA::with_8(122, 122, 122, 255));

    let gradient = Gradient::new(
        point2(100.0, 100.0),
        point2(900.0, 900.0),
        0.0,
        vec![
            GradientStop::new(0.0, RGBA::with_8(50, 127, 150, 200)),
            GradientStop::new(1.0, RGBA::with_8(220, 140, 75, 255)),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    );

    let mut pb = PathBuilder::new();
    pb.move_to(60.0, 60.0);
    pb.line_to(160.0, 940.0);
    pb.cubic_to(380.0, 840.0, 660.0, 800.0, 940.0, 800.0);
    pb.cubic_to(740.0, 460.0, 440.0, 160.0, 60.0, 60.0);
    pb.close();
    let path = pb.finish();
    canvas.fill_path(&path, &Texture::LinearGradient(gradient));

    let circle = ShapeBuilder::new()
        .circle(point2(100.0 + WIDTH / 2.0, -100.0 + HEIGHT / 2.0), 100.0)
        .fill_color(palette.colors[2])
        .stroke_color(palette.colors[4])
        .stroke_weight(10.0)
        .build();
    circle.draw(&mut canvas);

    canvas.save("examine_skia.png");
}
