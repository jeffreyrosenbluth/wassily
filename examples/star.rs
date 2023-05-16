use wassily::prelude::*;

fn main() {
    let mut canvas = Canvas::new(500, 500);
    canvas.fill(*CORNFLOWERBLUE);
    let pos = center(500, 500);
    Shape::new()
        .star(pos, 100.0, 175.0, 8)
        .fill_color(*GREENYELLOW)
        .stroke_color(*MIDNIGHTBLUE)
        .stroke_weight(3.0)
        .draw(&mut canvas);
    canvas.save_png("./star.png");
}
