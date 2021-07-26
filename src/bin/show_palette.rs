use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1200;
const NUM_COLORS: u8 = 16;
const FILE: &'static str = "hl.png";
const ANGLE: f32 = 180.0;

fn main() {
    let width = WIDTH as f32;
    let mut canvas = Canvas::new(WIDTH, WIDTH);
    let n = (0.5 + (NUM_COLORS as f32).sqrt()) as usize;
    let swatch_width = width / (NUM_COLORS as f32).sqrt();
    let mut palette = Palette::steal(FILE, NUM_COLORS);
    println!("-----------------------------------");
    println!("Primary Hue: {:3.1}",palette.colors[0].lcha().hue.to_positive_degrees());
    println!("-----------------------------------");
    palette.rotate_hue(ANGLE);
    for i in 0..n {
        for j in 0..n {
            if palette.colors.len() <= (i * n + j) {
                break;
            }
            let c = palette.colors[(i * n + j)];
            let c8 = c.as_8();
            let texture = Texture::solid_color(c);
            println!("({:2}, {:2}) : [{}, {}, {}]", i, j, c8.0, c8.1, c8.2);
            let x = j as f32 * swatch_width;
            let y = i as f32 * swatch_width;
            canvas.fill_rect(x, y, swatch_width, swatch_width, &texture)
        }
    }
    canvas.save("colors.png");
}
