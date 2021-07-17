use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 8000;
const HEIGHT: u32 = 6000;
const COLOR1: RGBA = CORAL;
const COLOR2: RGBA = SADDLEBROWN;
const BG_COLOR: RGBA = WHITE;
const THICK: f32 = 1700.0;
const GRAINS: u32 = 5000;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.fill(BG_COLOR);
    let mut sb = SandBox::new(
        point2(0.05 * WIDTH as f32, 0.05 * HEIGHT as f32),
        point2(WIDTH as f32 * 0.9, HEIGHT as f32 * 0.9),
    )
    .set_bg(BG_COLOR)
    .set_color1(COLOR1)
    .set_color2(COLOR2);
    sb.draw(&mut canvas);

    let mut sl = SandLine::new(
        point2(0.05 * WIDTH as f32, 0.05 * HEIGHT as f32),
        point2(0.95 * WIDTH as f32, 0.05 * HEIGHT as f32),
    )
    .thickness(THICK)
    .color(BG_COLOR)
    .grains(GRAINS);
    sl.draw(&mut canvas);

    let mut sl = SandLine::new(
        point2(0.05 * WIDTH as f32, 0.95 * HEIGHT as f32),
        point2(0.95 * WIDTH as f32, 0.95 * HEIGHT as f32),
    )
    .thickness(THICK)
    .color(BG_COLOR)
    .grains(GRAINS);
    sl.draw(&mut canvas);

    let mut sl = SandLine::new(
        point2(0.05 * WIDTH as f32, 0.05 * HEIGHT as f32),
        point2(0.05 * WIDTH as f32, 0.95 * HEIGHT as f32),
    )
    .thickness(THICK)
    .color(BG_COLOR)
    .grains(GRAINS);
    sl.draw(&mut canvas);

    let mut sl = SandLine::new(
        point2(0.95 * WIDTH as f32, 0.05 * HEIGHT as f32),
        point2(0.95 * WIDTH as f32, 0.95 * HEIGHT as f32),
    )
    .thickness(THICK)
    .color(BG_COLOR)
    .grains(GRAINS);
    sl.draw(&mut canvas);



    canvas.save("window.png");
}
