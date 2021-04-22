#![allow(dead_code)]

use tiny_skia::*;
use wassily::shape::*;
use wassily::util::*;

const WIDTH: u32 = 8191;
const HEIGHT: u32 = 6144;
const XSTEP: f32 = 10.0;
const YSTEP: f32 = 80.0;
const LENGTH: usize = 800;
const X: f32 = 10.0;
const LINES: usize = 1000;

fn main() {
    let mut wk = Wassily::new(8191.0, 6144.0);
    wk.set_seed(4);
    wk.set_noise_scale(0.0019);
    let mut canvas = Pixmap::new(wk.width_n(), wk.height_n()).unwrap();
    let img = Pixmap::load_png("hl.png").expect("Can't loag image");
    wk.set_colors(img, 1000);
    let bg = wk.color(990);
    canvas.fill(bg);

    for i in 0..LINES {
        let mut l1 = pt2(5.0 * i as f32, wk.height / 2.0);
        let mut l2 = pt2(5.0 * i as f32, wk.height / 2.0);
        let mut up = vec![];
        for _ in 0..LENGTH {
            if l1.x >= wk.width || l1.y >= wk.height {
                break;
            }
            up.push(l1);
            let angle = wk.noise(l1.x, l1.y, 0.0);
            l1.x += X + XSTEP * angle.cos();
            l1.y += YSTEP * angle.sin();
        }
        let mut dn = vec![];
        for _ in 0..LENGTH {
            if l2.x >= wk.width || l2.y >= wk.height {
                break;
            }
            dn.push(l2);
            let angle = wk.noise(l2.x, l2.y, 1000.0);
            l2.x += X + XSTEP * angle.cos();
            l2.y += YSTEP * angle.sin();
        }
        dn.reverse();
        up.extend(dn);

        let shape = ShapeBuilder::new()
            // .fill_color(wk.rand_rgb())
            .fill_color(wk.rand_color())
            .no_stroke()
            .points(&up)
            .build();
        shape.draw(&mut canvas);
    }
    canvas.save_png("ct.png").unwrap();
}
