use wassily::prelude::*;

fn main() {
    let width = 1080;
    let height = 1080;
    let linecolor = *WHITE;
    let mut canvas = Canvas::new(width, height);
    canvas.fill(rgb8(0, 5, 25));
    let w = width as f32;
    let h = height as f32;
    let spacing = 3.0;
    let thickness = 0.20;
    let mut i = spacing;
    while i < w {
        let p0 = pt(i, 0);
        let p1 = pt(0, i);
        let mut fl = FadeLine::new(p0, p1, 98731 + i as u64)
            .subdivisions(2 + 50 * i as u32 / (h * h + w * w).sqrt() as u32)
            .thickness(thickness)
            .color(linecolor);
        fl.draw(&mut canvas);
        let p0 = pt(i, height);
        let p1 = pt(width, i);
        let mut fl = FadeLine::new(p0, p1, 8317 + i as u64)
            .subdivisions(2 + 50 * (w - i) as u32 / (h * h + w * w).sqrt() as u32)
            .thickness(thickness)
            .color(linecolor);
        fl.draw(&mut canvas);
        i += spacing;
    }
    i = spacing;
    while i < w {
        let p0 = pt(i, 0);
        let p1 = pt(width, w - i);
        let mut fl = FadeLine::new(p0, p1, 137 + i as u64)
            .subdivisions(2 + 50 * (w - i) as u32 / (h * h + w * w).sqrt() as u32)
            .thickness(thickness)
            .color(linecolor);
        fl.draw(&mut canvas);
        let p0 = pt(i, height);
        let p1 = pt(0, h - i);
        let mut fl = FadeLine::new(p0, p1, 137 + i as u64)
            .subdivisions(2 + 50 * i as u32 / (h * h + w * w).sqrt() as u32)
            .thickness(thickness)
            .color(linecolor);
        fl.draw(&mut canvas);
        i += spacing;
    }
    canvas.save_png("./denim.png");
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
