use wassily::prelude::*;

const SPACING: f32 = 5.0;
const THICKNESS: f32 = 0.50;
const SUBDIVISIONS: u32 = 75;
const MIN_OPACITY: f32 = 0.1;
const MAX_OPACITY: f32 = 0.4;
const BLUR_1: f32 = 100.0;
const BLUR_2: f32 = 75.0;
const WIDTH: u32 = 4032;
const HEIGHT: u32 = 3024;
const CONTAMINATION: f32 = 0.25;
const SCREEN: bool = true;

fn main() {
    let img_1 = open("/Users/jeffreyrosenbluth/Desktop/IMG_1755.jpeg")
        .unwrap()
        .resize_exact(WIDTH, HEIGHT, image::imageops::FilterType::Lanczos3);
    let img_2 = open("/Users/jeffreyrosenbluth/Desktop/Gino_Severini.jpg")
        .unwrap()
        .resize_exact(WIDTH, HEIGHT, image::imageops::FilterType::Lanczos3);
    let mut img = RgbaImage::new(WIDTH, HEIGHT);
    let blurred_img_1 = img_1.fast_blur(BLUR_1).to_rgba8();
    let blurred_img_2 = img_2.fast_blur(BLUR_2).to_rgba8();

    let opts = NoiseOpts::default()
        .scales(5.0)
        .width(img.width() as f32)
        .height(img.height() as f32);

    let nf = Fbm::<OpenSimplex>::default().set_seed(13).set_octaves(2);
    let mut rng = SmallRng::seed_from_u64(0);

    for x in 0..img.width() {
        for y in 0..img.height() {
            if noise2d(&nf, &opts, x as f32, y as f32) + CONTAMINATION * (0.5 - rng.random::<f32>())
                > 0.0
            {
                img.put_pixel(x, y, *blurred_img_1.get_pixel(x, y));
            } else {
                img.put_pixel(x, y, *blurred_img_2.get_pixel(x, y));
            }
        }
    }

    let mut canvas = Canvas::from_image(&DynamicImage::ImageRgba8(img));

    let width = canvas.width();
    let height = canvas.height();
    let linecolor = *BLACK;
    let w = canvas.w_f32();
    let h = canvas.h_f32();
    let mut i = SPACING;

    if SCREEN {
        while i < h {
            let v0 = pt(0, i);
            let v1 = pt(width, i);
            let mut fl = FadeLine::new(v0, v1, 98731 + i as u64)
                .subdivisions(SUBDIVISIONS)
                .thickness(THICKNESS)
                .min_opacity(MIN_OPACITY)
                .max_opacity(MAX_OPACITY)
                .color(linecolor);
            fl.draw(&mut canvas);
            i += SPACING;
        }
        i = SPACING;
        while i < w {
            let v0 = pt(i, 0);
            let v1 = pt(i, height);
            let mut fl = FadeLine::new(v0, v1, 98731 + i as u64)
                .subdivisions(SUBDIVISIONS)
                .thickness(THICKNESS)
                .min_opacity(MIN_OPACITY)
                .max_opacity(MAX_OPACITY)
                .color(linecolor);
            fl.draw(&mut canvas);
            i += SPACING;
        }
    }

    // --- Diagonal lines --------------------------------------------------------

    // while i < w {
    //     let p0 = pt(i, 0);
    //     let p1 = pt(0, i);
    //     let mut fl = FadeLine::new(p0, p1, 98731 + i as u64)
    //         .subdivisions(2 + 50 * i as u32 / (h * h + w * w).sqrt() as u32)
    //         .thickness(thickness)
    //         .color(linecolor);
    //     fl.draw(&mut canvas);
    //     let p0 = pt(i, height);
    //     let p1 = pt(width, i);
    //     let mut fl = FadeLine::new(p0, p1, 8317 + i as u64)
    //         .subdivisions(2 + 50 * (w - i) as u32 / (h * h + w * w).sqrt() as u32)
    //         .thickness(thickness)
    //         .color(linecolor);
    //     fl.draw(&mut canvas);
    //     i += spacing;
    // }
    // i = spacing;
    // while i < w {
    //     let p0 = pt(i, 0);
    //     let p1 = pt(width, w - i);
    //     let mut fl = FadeLine::new(p0, p1, 137 + i as u64)
    //         .subdivisions(2 + 50 * (w - i) as u32 / (h * h + w * w).sqrt() as u32)
    //         .thickness(thickness)
    //         .color(linecolor);
    //     fl.draw(&mut canvas);
    //     let p0 = pt(i, height);
    //     let p1 = pt(0, h - i);
    //     let mut fl = FadeLine::new(p0, p1, 137 + i as u64)
    //         .subdivisions(2 + 50 * i as u32 / (h * h + w * w).sqrt() as u32)
    //         .thickness(thickness)
    //         .color(linecolor);
    //     fl.draw(&mut canvas);
    //     i += spacing;
    // }
    canvas.save_png("./outputs/fade.png");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refine_test() {
        let pts = vec![pt(-1, -1), pt(0, 0), pt(1, -1)];
        let refined = refine(&pts, |x| -x * x, 0.01);
        dbg!(&refined);
    }

    #[test]
    fn curve_test() {
        let f = |x: f32| x.sin();
        let c = curve(f, pt(0, 1080 / 2), 1080.0, 540.0, 10.0);
        dbg!(c);
    }
}
