use std::f64::consts::PI;

use wassily::prelude::*;

const SIZE: u32 = 1080;

fn main() {
    let mut canvas = Canvas::new(SIZE, SIZE);
    let opts = NoiseOpts::with_wh(SIZE, SIZE).factor(1.0).scales(1.0);
    let nf = SinFM::default()
        .xy_freq_x(1.5)
        .xy_freq_y(1.1)
        .xy_phase_x(PI / 3.0)
        .xy_phase_y(0.0)
        .xy_amp(3.0)
        .yx_freq_x(1.1)
        .yx_freq_y(1.5)
        .yx_phase_x(0.0)
        .yx_phase_y(PI / 3.0)
        .yx_amp(1.5);
    for x in 0..SIZE {
        for y in 0..SIZE {
            let n = noise2d_01(&nf, &opts, x as f32, y as f32);
            let c = (*CORNFLOWERBLUE).lerp(&CORNSILK, n);
            canvas.dot(x as f32, y as f32, c);
        }
    }
    canvas.save_png("./noise.png");
}
