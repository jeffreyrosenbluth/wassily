use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 0.80 * WIDTH;

fn main() {
    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);
    let mut palette = Palette::steal("hudson.png", 16);
    palette.set_seed(71731);
    canvas.fill(RGBA::rgba8(122, 122, 122, 255));

    let gradient = Gradient::new(
        point2(600.0, 600.0),
        point2(600.0, 600.0),
        500.0,
        vec![
            GradientStop::new(0.0, RGBA::rgba8(50, 127, 150, 255)),
            GradientStop::new(1.0, RGBA::rgba8(50, 127, 150, 0)),
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
    canvas.fill_path(&path, &Texture::new(TextureKind::RadialGradient(gradient)));

    let mut circle = ShapeBuilder::new()
        .circle(point2(100.0 + WIDTH / 2.0, -100.0 + HEIGHT / 2.0), 100.0)
        .fill_color(palette.rand_lab())
        .stroke_color(palette[4])
        .stroke_weight(10.0)
        .build();

    circle.stroke.dash = Some(Dash::new(vec![10., 18.], 16.));
    circle.draw(&mut canvas);

    canvas.save("examine.png");
}
