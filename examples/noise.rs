use wassily::prelude::*;

const SIZE: u32 = 1080;

fn main() {
    let mut canvas = Canvas::new(SIZE, SIZE);
    let opts = NoiseOpts::with_wh(SIZE, SIZE).factor(1.0).scales(10.0);
    let nf = FMCross::new2(1.0, 0.2, 10.0, 1.0, 0.2, 10.0);
    for x in 0..SIZE {
        for y in 0..SIZE {
            let n = noise2d_01(&nf, &opts, x as f32 - 540., y as f32 - 540.);
            let g = n * 255.0;
            let c = gray(g as u8);
            // let c = (*ORANGERED).lerp(&MIDNIGHTBLUE, n);
            canvas.dot(x as f32, y as f32, c);
        }
    }
    canvas.save_png("./noise.png");
}
