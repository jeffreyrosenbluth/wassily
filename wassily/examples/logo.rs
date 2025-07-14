use wassily::prelude::*;

fn main() {
    let mut canvas = Canvas::new(500, 500);
    let center = center(canvas.width(), canvas.height()) - pt(25, 25);

    // Draw a background.
    let nf1 = Fbm::<Perlin>::default().set_octaves(4);
    let nf2: Turbulence<Fbm<Perlin>, Perlin> = Turbulence::new(nf1).set_power(2.0).set_roughness(6);
    let opts = NoiseOpts::default();
    for i in 0..canvas.width() {
        for j in 0..canvas.height() {
            let y = 255 - (40.0 * noise2d_01(&nf2, &opts, i as f32 * 0.005, j as f32 * 0.30)) as u8;
            let r = y as f32 / 255.0 * 220.0;
            let g = y as f32 / 255.0 * 205.0;
            let b = y as f32 / 255.0 * 170.0;
            let c = Color::from_rgba8(r as u8, g as u8, b as u8, 255);
            let mut paint = Paint::default();
            paint.set_color(c);
            paint.blend_mode = BlendMode::Multiply;
            Shape::new()
                .rect_xywh(pt(i, j), pt(1, 1))
                .fill_paint(&paint)
                .no_stroke()
                .draw(&mut canvas);
        }
    }

    let mut yellow = rgb8(235, 195, 55);
    yellow.set_alpha(0.8);
    Shape::new()
        .circle(center + pt(125, 125), 75.0)
        .fill_color(yellow)
        .no_stroke()
        .draw(&mut canvas);

    let pink = rgb8(225, 125, 115);
    let wood_pink = wood(500, 500, pink, (*MISTYROSE).darken_fixed(0.30), 5.0);
    let mut granite_pink_paint = PatternPaint::new(
        &wood_pink,
        bbox_circle(center, 185.0),
        canvas.width(),
        canvas.height(),
    );
    Shape::new()
        .circle(center, 185.0)
        .fill_paint(&granite_pink_paint.paint())
        .no_stroke()
        .draw(&mut canvas);

    Shape::new()
        .circle(center + pt(125, 125), 55.0)
        .fill_color(rgb8(185, 0, 10))
        .stroke_color(*BLACK)
        .stroke_weight(0.5)
        .draw(&mut canvas);

    Shape::new()
        .circle(center, 150.0)
        .fill_color(rgb8(10, 10, 10))
        .no_stroke()
        .draw(&mut canvas);

    let purple1 = rgb8(50, 25, 75);
    let purple2 = rgb8(100, 75, 125);
    let granite_purple = granite(500, 500, purple1, purple2, 5.0, 0);
    let mut granite_purple_paint = PatternPaint::new(
        &granite_purple,
        bbox_circle(center, 75.0),
        canvas.width(),
        canvas.height(),
    );
    Shape::new()
        .circle(center, 75.0)
        .fill_paint(&granite_purple_paint.paint())
        .no_stroke()
        .draw(&mut canvas);

    let tri_points = [pt(40, 455), pt(490, 445), pt(490, 450), pt(40, 485)];
    let tri = chaiken(&tri_points, 3, Trail::Closed);
    Shape::new()
        .points(&tri)
        .fill_color(*BLACK)
        .no_stroke()
        .draw(&mut canvas);

    canvas.save_png("./outputs/logo.png");
}

fn bbox_circle(center: Point, radius: f32) -> Rect {
    let x = center.x - radius;
    let y = center.y - radius;
    let w = radius * 2.0;
    let h = radius * 2.0;
    Rect::from_xywh(x, y, w, h).unwrap()
}
