use wassily::prelude::*;

const SIZE: f32 = 4.0;

fn main() {
    let mut canvas = Canvas::new(1200, 1200);
    canvas.fill(*WHITE);
    let dots1 = halton_23(400, 1200, 1027, 0);
    for d in dots1 {
        Shape::new()
            .rect_xywh(pt(d.x + 400.0, d.y), pt(SIZE, SIZE))
            .fill_color(*BLACK)
            .draw(&mut canvas);
    }
    let dots2 = uniform(400, 1200, 1027, 0);
    for d in dots2 {
        Shape::new()
            .rect_xywh(d, pt(SIZE, SIZE))
            .fill_color(*BLACK)
            .draw(&mut canvas);
    }
    let dots3 = poisson_disk(400.0, 1200.0, 18.0, 0);
    for d in dots3 {
        Shape::new()
            .rect_xywh(pt(d.x + 800.0, d.y), pt(SIZE, SIZE))
            .fill_color(*BLACK)
            .draw(&mut canvas);
    }
    Shape::new()
        .line(pt(400, 0), pt(400, 1200))
        .stroke_color(*BLUE)
        .draw(&mut canvas);
    Shape::new()
        .line(pt(800, 0), pt(800, 1200))
        .stroke_color(*BLUE)
        .draw(&mut canvas);
    canvas.save_png("./stipple.png");
}
