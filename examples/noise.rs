use wassily::prelude::*;

const SIZE: u32 = 1080;

fn main() {
    let mut drawing = Drawing::new(SIZE, SIZE, 4.0);
    let opts = NoiseOpts::with_wh(SIZE, SIZE).factor(1.0).scales(10.0);
    let nf = FMCross::new2(1.0, 0.2, 10.0, 1.0, 0.2, 10.0);
    for x in 0..(2 * drawing.width) {
        for y in 0..(2 * drawing.height) {
            let n = noise2d_01(&nf, &opts, x as f32 / 2.0 - 540., y as f32 / 2.0 - 540.);
            let g = n * 255.0;
            let c = gray(g as u8);
            let c = (*ORANGERED).lerp(&BLACK, n);
            ShapeBuilder::new()
                .rect_cwh(pt(x / 2, y / 2), pt(1.0, 1.0))
                .no_stroke()
                .fill_color(c)
                .build()
                .push(&mut drawing);
        }
    }
    drawing.render();
    drawing.save_png("./noise4.png");
    drawing.set_scale(1.0);
    ShapeBuilder::new()
        .circle(pt(drawing.w_f32() / 2.0, drawing.h_f32() / 2.0), 100.0)
        .build()
        .push(&mut drawing);
    drawing.render();
    drawing.save_png("./noise1.png");
}
