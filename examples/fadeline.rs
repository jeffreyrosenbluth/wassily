use wassily::prelude::*;

fn main() {
    let img = open("/Users/jeffreyrosenbluth/Desktop/Gino_Severini.jpg").unwrap();
    let img_large = img.resize(
        5 * img.width(),
        5 * img.height(),
        image::imageops::FilterType::Lanczos3,
    );
    let blurred_img = img_large.fast_blur(100.0);
    let mut canvas = Canvas::from_image(&blurred_img);

    let width = canvas.width();
    let height = canvas.height();
    let linecolor = *BLACK;
    let w = canvas.w_f32();
    let h = canvas.h_f32();
    let spacing = 15.0;
    let thickness = 0.50;
    let subdivisions = 75;
    let min_opacity = 0.15;
    let max_opacity = 1.0;
    let mut i = spacing;

    // let mut canvas = Canvas::new(width, height);
    // canvas.fill(rgb8(255, 255, 255));

    while i < h {
        let v0 = pt(0, i);
        let v1 = pt(width, i);
        let mut fl = FadeLine::new(v0, v1, 98731 + i as u64)
            .subdivisions(subdivisions)
            .thickness(thickness)
            .min_opacity(min_opacity)
            .max_opacity(max_opacity)
            .color(linecolor);
        fl.draw(&mut canvas);
        i += spacing;
    }
    i = spacing;
    while i < w {
        let v0 = pt(i, 0);
        let v1 = pt(i, height);
        let mut fl = FadeLine::new(v0, v1, 98731 + i as u64)
            .subdivisions(subdivisions)
            .thickness(thickness)
            .min_opacity(min_opacity)
            .max_opacity(max_opacity)
            .color(linecolor);
        fl.draw(&mut canvas);
        i += spacing;
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
    canvas.save_png("./fade.png");
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
