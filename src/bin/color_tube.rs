#![allow(dead_code)]

use tiny_skia::*;
use wassily::grid::*;
use wassily::shape::*;
use wassily::util::*;

const WIDTH: u32 = 8191;
const HEIGHT: u32 = 6144;
const GRID_SPACING: f32 = 4.0;
const XSTEP: f32 = 10.0;
const YSTEP: f32 = 15.0;
const LENGTH: usize = 800;
const X: f32  = 7.0;

fn main() {
    let mut wk = Wassily::new(8191.0, 6144.0);
    wk.set_seed(9);
    wk.set_noise_scale(0.0011);
    let mut canvas = Pixmap::new(wk.width_n(), wk.height_n()).unwrap();
    // let bg = Color::from_rgba8(255, 248, 220, 255);
    let bg = Color::from_rgba8(242, 240, 233, 255);
    background(&mut canvas, wk.width_n(), wk.height_n(), bg);

    let grid = Grid::new(1.1 * wk.width, 1.1 * wk.height, GRID_SPACING, |x, y| {
        TAU * wk.noise(x, y)
    });

    for i in 0..(grid.cols() / 4) {
        let mut l1 = pt2(GRID_SPACING * i as f32, wk.height / 2.0);
        let mut l2 = pt2(GRID_SPACING * (i as f32 + 1.5), wk.height / 2.0);
        let mut up = vec![];
        for _ in 0..LENGTH {
            if l1.x >= wk.width || l1.y >= wk.height {
                break;
            }
            up.push(l1);
            let angle = grid.get(l1.x, l1.y);
            l1.x += X + XSTEP * angle.cos();
            l1.y += YSTEP * angle.sin();
        }
        let mut dn = vec![];
        for _ in 0..LENGTH {
            if l2.x >= wk.width || l2.y >= wk.height {
                break;
            }
            dn.push(l2);
            let angle = &grid.get(l2.x, l2.y);
            l2.x += X + XSTEP * angle.cos();
            l2.y += YSTEP * angle.sin();
        }
        dn.reverse();
        up.extend(dn);

        let shape = ShapeBuilder::new()
            .fill_color(wk.rand_rgb())
            .no_stroke()
            .points(&up)
            .build();
        shape.draw(&mut canvas);
    }
    canvas.save_png("color_tube_9.png").unwrap();
}
