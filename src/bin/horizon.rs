use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 1200;
const PIX_SZ: f32 = 32.0;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let dots = stipple(WIDTH as f32 - PIX_SZ, HEIGHT as f32 - PIX_SZ, 100_000);
    let mut palette = Palette::with_img(file_path("w1.png"), 1000);
    palette.sort_by_hue();
    palette.colors.reverse();
    canvas.fill(BLACK);
    palette.rotate_hue(330.);
    for d in dots {
        // let color = palette.rand_color();
        let sz = map_range(d.y, 0.0, HEIGHT as f32, 10.0, 100.0);
        let tz = map_range(d.y, 0.0, HEIGHT as f32, 100.0, 10.0);
        let cz = map_range(d.y, 0.0, HEIGHT as f32, 999.0, 0.0) as usize;
        let color = palette.colors[cz];
        // ShapeBuilder::new()
        //     .circle(point2(d.x, d.y), sz)
        //     .fill_color(color.set_opacity(0.5))
        //     .no_stroke()
        //     .build()
        //     .draw(&mut canvas);
        canvas.fill_rect(
            d.x - 10.0,
            d.y - 10.0,
            100.0,
            tz,
            &Texture::solid_color(color.set_opacity(0.5)),
        );
    }
    canvas.save("horizon.png");
}
