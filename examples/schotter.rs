use wassily::prelude::*;

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 50;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut rng = SmallRng::from_entropy();
    let mut palette = Palette::default();
    canvas.fill(*WHITESMOKE);
    for y in 0..ROWS {
        for x in 0..COLS {
            let factor = y as f32 / ROWS as f32;
            let x_offset = factor * rng.gen_range(-0.5..0.5);
            let y_offset = factor * rng.gen_range(-0.5..0.5);
            let tx = (x * SIZE + MARGIN) as f32 + x_offset;
            let ty = (y * SIZE + MARGIN) as f32 + y_offset;
            let angle = factor * rng.gen_range(-45.0..45.0);
            let rotation = Transform::from_rotate_at(angle, tx as f32, ty as f32);
            let fill = palette.rand_laba();
            Shape::new()
                .rect_xywh(pt(tx, ty), pt(SIZE, SIZE))
                .fill_color(fill)
                .stroke_color(*BLACK)
                // .stroke_weight(1.0)
                .transform(&rotation)
                .draw(&mut canvas);
        }
    }
    canvas.save_png("./schotter.png");
}
