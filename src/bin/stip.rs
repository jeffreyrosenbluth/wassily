use image::GenericImageView;
use wassily::prelude::*;
use wassily::skia::Canvas;

const FILE: &'static str = "sea.png";
const SKIP: f32 = 50.0;
const EDGE: f32 = 200.0;

fn main() {
    let img = image::open(FILE).expect("Cannot open file");
    let w = img.width() as f32 / SKIP * EDGE as f32;
    let h = img.height() as f32 / SKIP * EDGE as f32;
    let mut canvas = Canvas::new(w as u32, h as u32);
    canvas.fill(WHITE);
    let mut i = 0;
    let mut j = 0;
    while i < img.width() {
        while j < img.height() {
            let p = img.get_pixel(i, j);
            let q = RGBA::rgb8(p[0], p[1], p[2]);
            let mut c = q.lcha();
            let chroma = c.chroma;
            c.chroma = 181.0;
            let dots = stipple(EDGE, EDGE, (chroma * 400.0) as u32);
            let transform = Transform::identity()
                .post_translate(vec2(i as f32 / SKIP * EDGE, j as f32 / SKIP * EDGE));
            let stips = dots.iter().map(|p| transform.transform_point(*p));
            for s in stips {
                canvas.pixel(s.x, s.y, q)
            }
            j += SKIP as u32;
        }
        j = 0;
        i += SKIP as u32;
    }
    canvas.save("stip.png");
}
