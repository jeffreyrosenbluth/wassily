#![allow(dead_code)]

use tiny_skia::*;
use wassily::shape::*;
use wassily::util::*;

const WIDTH: u32 = 8191;
const HEIGHT: u32 = 6144;
const XSTEP: f32 = 10.0;
const YSTEP: f32 = 20.0;
const LENGTH: usize = 1000;
const DIST: f32 = 50.0;
const LINES: usize = 1500;
const X: f32 = 7.0;

fn main() {
    let mut wk = Wassily::new(8191.0, 6144.0);
    wk.set_seed(1);
    wk.set_noise_scale(0.0071);
    let mut canvas = Pixmap::new(wk.width_n(), wk.height_n()).unwrap();
    let bg = Color::from_rgba8(255, 248, 220, 255);
    canvas.fill(bg);

    for l in 0..LINES {
        let mut l1 = pt2(0.0, 5.0 * l as f32);
        let mut up = vec![];
        for _ in 0..LENGTH {
            if l1.x >= wk.width || l1.x < 0.0 || l1.y >= wk.height || l1.y <= 0.0 {
                break;
            }
            up.push(l1);
            let angle = map_range(wk.noise(l1.x, l1.y, l as f32 * 3.0), -1.0, 1.0, -PI, PI);
            l1.x += X + XSTEP * angle.cos();
            l1.y += YSTEP * angle.sin();
        }

        if up.len() > 3 {
            let shape = ShapeBuilder::new()
                .no_fill() 
                .points(&up)
                .build();
            shape.draw(&mut canvas);
        }
    }
    for l in 0..LINES {
        let mut l1 = pt2(wk.width, 5.0 * l as f32);
        let mut up = vec![];
        for _ in 0..LENGTH {
            if l1.x > wk.width || l1.x < 0.0 || l1.y > wk.height || l1.y < 0.0 {
                break;
            }
            up.push(l1);
            let angle = map_range(wk.noise(l1.x, l1.y, l as f32 * 3.0), -1.0, 1.0, -PI, PI);
            l1.x += -X - XSTEP * angle.cos();
            l1.y += -YSTEP * angle.sin();
        }

        if up.len() > 3 {
            let shape = ShapeBuilder::new()
                .no_fill() 
                .points(&up)
                .build();
            shape.draw(&mut canvas);
        }
    }
    canvas.save_png("squirt.png").unwrap();
}
