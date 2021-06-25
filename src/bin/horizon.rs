use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 1200;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let dots = stipple(WIDTH as f32, HEIGHT as f32, 1_00_000);
    let mut palette = Palette::with_img(file_path("weeds.png"), 1000);
    palette.sort_by_hue();
    palette.colors.reverse();
    canvas.fill(BLACK);
    // palette.rotate_hue(30.);
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
            d.x,
            d.y,
            100.0,
            tz,
            &Texture::solid_color(color.set_opacity(0.1)),
        );
    }
    canvas.save("horizon.png");
}
