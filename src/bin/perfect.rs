use image::imageops::blur;
use image::*;
use rand::prelude::*;
// use rand_distr::uniform::SampleUniform;
use rand_pcg::Pcg64;
fn main() {
    let width = 1_000;
    let height = 1_000;
    let mut imgbuf = ImageBuffer::new(width, height);
    let mut rng = Pcg64::seed_from_u64(0);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if x % 2 == 0  {
            let r = rng.gen_range(0u8..255);
            let g = rng.gen_range(0..255);
            let b = rng.gen_range(0..255);
            *pixel = image::Rgb([r, 0, 0]);
        } 
    }
    let result = blur(&imgbuf, 4.0);
    result.save("blur.png").unwrap();
}
