use wassily::prelude::*;

fn draw(canvas: &mut Canvas) {
    canvas.fill(*GRAY);
    let center = pt(canvas.width() / 2, canvas.height() / 2);
    Shape::new()
        .ellipse(center, 300.0, 200.0)
        .fill_color(*INDIGO)
        .stroke_color(*ORANGE)
        .stroke_weight(10.0)
        .draw(canvas);
}

fn main() {
    let mut canvas1 = Canvas::new(500, 500);
    // Draw a 300x200 ellipse at the center of a 500 x 500 canvas.
    draw(&mut canvas1);

    let mut canvas2 = Canvas::with_scale(500, 500, 2.0);
    // Draw a 600x400 ellipse at the center of a 1000 x 1000 canvas. That is,
    // the exact same image as above, but, double the width and height.
    draw(&mut canvas2);

    canvas1.save_png("./scale1.png");
    canvas2.save_png("./scale2.png");
}
