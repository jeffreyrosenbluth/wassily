use std::f64::consts::PI;

use wassily::prelude::*;

const SIZE: u32 = 1080;

fn main() {
    let mut canvas = Canvas::new(SIZE, SIZE);
    let opts = NoiseOpts::with_wh(SIZE, SIZE).factor(1.0).scales(1.0);
    let mut nf = SinFM::default();
    // nf.xy.freq_x = 1.5;
    // nf.xy.freq_y = 1.75;
    // nf.xy.phase_x = PI / 3.0;
    // nf.xy.phase_y = 0.0;
    // nf.xy.amp = 20.0;
    // nf.yx.freq_x = 3.5;
    // nf.yx.freq_y = 2.5;
    // nf.yx.phase_x = PI / 5.0;
    // nf.yx.phase_y = PI / 2.0;
    // nf.yx.amp = 20.0;
    for x in 0..SIZE {
        for y in 0..SIZE {
            let n = 0.5 * noise2d(&nf, &opts, x as f32, y as f32) + 0.5;
            let c = if n < 0.0 {
                (*BLUEVIOLET).lerp(&BLACK, 2.0 * n)
            } else {
                (*WHITE).lerp(&CRIMSON, n)
            };
            canvas.dot(x as f32, y as f32, c);
        }
    }
    let g = Grain::new(SIZE, SIZE, 0.01, 0.5);
    g.canvas_grain(&mut canvas);
    canvas.save_png("./noise.png");
}
