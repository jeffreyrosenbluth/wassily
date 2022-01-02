use wassily::prelude::*;
use tiny_skia::{Paint, Rect, Pixmap, Transform};

const WIDTH: u32 = 1024;
const NUM_COLORS: u8 = 32;
const FILE: &str = "fruit.png";
const ANGLE: f32 = 0.0;

fn main() {
    let width = WIDTH as f32;
    let mut canvas = Pixmap::new(WIDTH, WIDTH).unwrap();
    let mut palette = Palette::steal(FILE, NUM_COLORS);
    let num_colors = palette.len() as f32;
    let n = (0.5 + num_colors.sqrt()) as usize;
    let swatch_width = width / n as f32;
    println!("-----------------------------------");
    palette.rotate_hue(ANGLE);
    for i in 0..n {
        for j in 0..n {
            if palette.colors.len() <= (i * n + j) {
                break;
            }
            let c = palette[(i * n + j)];
            let c8 = as_u8s(c);
            let mut paint = Paint::default();
            paint.set_color(c);
            println!("({:2}, {:2}) : [{}, {}, {}]", j, i, c8.0, c8.1, c8.2);
            let x = j as f32 * swatch_width;
            let y = i as f32 * swatch_width;
            let rect = Rect::from_xywh(x, y, swatch_width, swatch_width).unwrap();
            canvas.fill_rect(rect, &paint, Transform::identity(), None);
        }
    }
    canvas.save_png("./colors.png");
}
