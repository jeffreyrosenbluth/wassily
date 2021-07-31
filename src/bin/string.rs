use wassily::prelude::*;
use wassily::skia::Canvas;

const N: usize = 60;
const WIDTH: u32 = 1200;
const HEIGHT: u32 = 1200;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.fill(BLACK);
    let mut ps = stipple(WIDTH as f32, HEIGHT as f32, 2 * N as u32);
    ps[..N].sort_by(|a,b| a.x.partial_cmp(&b.y).unwrap());
    ps[N..].sort_by(|a,b| a.x.partial_cmp(&b.y).unwrap());
    ShapeBuilder::new()
        .points(&ps[..N])
        .no_stroke()
        .fill_color(FIREBRICK)
        .fill_rule(FillRule::EvenOdd)
        .cubic()
        .build()
        .draw(&mut canvas);

    let texture = Texture::solid_color(INDIANRED).mode(BlendMode::Lighten);

    ShapeBuilder::new()
        .points(&ps[N..])
        .no_stroke()
        .fill_texture(&texture)
        .fill_rule(FillRule::EvenOdd)
        .cubic()
        .build()
        .draw(&mut canvas);
    canvas.save("string.png");
}
