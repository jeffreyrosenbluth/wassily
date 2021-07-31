use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 7200;
const HEIGHT: u32 = 5400;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let dots = stipple(WIDTH as f32, HEIGHT as f32, 600_000);
    let mut palette = Palette::with_img("matches.png", Some(1000));
    palette.rotate_hue(150.0);
    palette.sort_by_chroma();
    palette.colors.reverse();
    canvas.fill(BLACK);
    // palette.rotate_hue(30.);
    for d in dots {
        let tz = map_range(d.y, 0.0, HEIGHT as f32, 600.0, 60.0);
        let cz = map_range(d.y, 0.0, HEIGHT as f32, 999.0, 0.0) as usize;
        let color = palette.colors[cz];
        canvas.fill_rect(
            d.x,
            d.y,
            600.0,
            tz,
            &Texture::solid_color(color.set_opacity(0.1)),
        );
    }
    canvas.save("horizon.png");
}
